#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeBackendServiceData_ {
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

struct DataComputeBackendService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeBackendServiceData_>,
}

#[derive(Clone)]
pub struct DataComputeBackendService(std::rc::Rc<DataComputeBackendService_>);

impl DataComputeBackendService {
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
        "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.affinity_cookie_ttl_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nThe set of backends that serve this BackendService."]
    pub fn backend(&self) -> terrars::SetRef<DataComputeBackendService_BackendRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this BackendService."]
    pub fn cdn_policy(&self) -> terrars::ListRef<DataComputeBackendService_CdnPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `circuit_breakers` after provisioning.\nSettings controlling the volume of connections to a backend service. This field\nis applicable only when the load_balancing_scheme is set to INTERNAL_SELF_MANAGED."]
    pub fn circuit_breakers(&self) -> terrars::ListRef<DataComputeBackendService_CircuitBreakersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.circuit_breakers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_draining_timeout_sec` after provisioning.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn connection_draining_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout_sec", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `consistent_hash` after provisioning.\nConsistent Hash-based load balancing can be used to provide soft session\naffinity based on HTTP headers, cookies or other properties. This load balancing\npolicy is applicable only for HTTP connections. The affinity to a particular\ndestination host will be lost when one or more hosts are added/removed from the\ndestination service. This field specifies parameters that control consistent\nhashing. This field only applies if the load_balancing_scheme is set to\nINTERNAL_SELF_MANAGED. This field is only applicable when locality_lb_policy is\nset to MAGLEV or RING_HASH."]
    pub fn consistent_hash(&self) -> terrars::ListRef<DataComputeBackendService_ConsistentHashRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consistent_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_metrics` after provisioning.\nList of custom metrics that are used for the WEIGHTED_ROUND_ROBIN locality_lb_policy."]
    pub fn custom_metrics(&self) -> terrars::ListRef<DataComputeBackendService_CustomMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metrics", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_request_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nrequests."]
    pub fn custom_request_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_request_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nresponses."]
    pub fn custom_response_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_response_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe resource URL for the edge security policy associated with this backend service."]
    pub fn edge_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendService."]
    pub fn enable_cdn(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cdn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_managed_migration_state` after provisioning.\nSpecifies the canary migration state. Possible values are PREPARE, TEST_BY_PERCENTAGE, and\nTEST_ALL_TRAFFIC.\n\nTo begin the migration from EXTERNAL to EXTERNAL_MANAGED, the state must be changed to\nPREPARE. The state must be changed to TEST_ALL_TRAFFIC before the loadBalancingScheme can be\nchanged to EXTERNAL_MANAGED. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate\ntraffic by percentage using externalManagedMigrationTestingPercentage.\n\nRolling back a migration requires the states to be set in reverse order. So changing the\nscheme from EXTERNAL_MANAGED to EXTERNAL requires the state to be set to TEST_ALL_TRAFFIC at\nthe same time. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate some traffic\nback to EXTERNAL or PREPARE can be used to migrate all traffic back to EXTERNAL. Possible values: [\"PREPARE\", \"TEST_BY_PERCENTAGE\", \"TEST_ALL_TRAFFIC\"]"]
    pub fn external_managed_migration_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_managed_migration_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_managed_migration_testing_percentage` after provisioning.\nDetermines the fraction of requests that should be processed by the Global external\nApplication Load Balancer.\n\nThe value of this field must be in the range [0, 100].\n\nSession affinity options will slightly affect this routing behavior, for more details,\nsee: Session Affinity.\n\nThis value can only be set if the loadBalancingScheme in the backend service is set to\nEXTERNAL (when using the Classic ALB) and the migration state is TEST_BY_PERCENTAGE."]
    pub fn external_managed_migration_testing_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_managed_migration_testing_percentage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this\nobject. This field is used in optimistic locking."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn generated_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to the HttpHealthCheck or HttpsHealthCheck resource\nfor health checking this BackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend.\n\nFor internal load balancing, a URL to a HealthCheck resource must be specified instead."]
    pub fn health_checks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.health_checks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iap` after provisioning.\nSettings for enabling Cloud Identity Aware Proxy.\nIf OAuth client is not set, the Google-managed OAuth client is used."]
    pub fn iap(&self) -> terrars::ListRef<DataComputeBackendService_IapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address_selection_policy` after provisioning.\nSpecifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC). Possible values: [\"IPV4_ONLY\", \"PREFER_IPV6\", \"IPV6_ONLY\"]"]
    pub fn ip_address_selection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_selection_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nIndicates whether the backend service will be used with internal or\nexternal load balancing. A backend service created for one type of\nload balancing cannot be used with the other. For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"EXTERNAL_MANAGED\" Possible values: [\"EXTERNAL\", \"INTERNAL_SELF_MANAGED\", \"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locality_lb_policies` after provisioning.\nA list of locality load balancing policies to be used in order of\npreference. Either the policy or the customPolicy field should be set.\nOverrides any value set in the localityLbPolicy field.\n\nlocalityLbPolicies is only supported when the BackendService is referenced\nby a URL Map that is referenced by a target gRPC proxy that has the\nvalidateForProxyless field set to true."]
    pub fn locality_lb_policies(&self) -> terrars::ListRef<DataComputeBackendService_LocalityLbPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locality_lb_policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality_lb_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_config` after provisioning.\nThis field denotes the logging options for the load balancer traffic served by this backend service.\nIf logging is enabled, logs will be exported to Stackdriver."]
    pub fn log_config(&self) -> terrars::ListRef<DataComputeBackendService_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_stream_duration` after provisioning.\nSpecifies the default maximum duration (timeout) for streams to this service. Duration is computed from the\nbeginning of the stream until the response has been completely processed, including all retries. A stream that\ndoes not complete in this duration is closed.\nIf not specified, there will be no timeout limit, i.e. the maximum duration is infinite.\nThis value can be overridden in the PathMatcher configuration of the UrlMap that references this backend service.\nThis field is only allowed when the loadBalancingScheme of the backend service is INTERNAL_SELF_MANAGED."]
    pub fn max_stream_duration(&self) -> terrars::ListRef<DataComputeBackendService_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `outlier_detection` after provisioning.\nSettings controlling eviction of unhealthy hosts from the load balancing pool.\nApplicable backend service types can be a global backend service with the\nloadBalancingScheme set to INTERNAL_SELF_MANAGED or EXTERNAL_MANAGED."]
    pub fn outlier_detection(&self) -> terrars::ListRef<DataComputeBackendService_OutlierDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> terrars::ListRef<DataComputeBackendService_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nName of backend port. The same name should appear in the instance\ngroups referenced by this service. Required when the load balancing\nscheme is EXTERNAL."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Must be set to GRPC when the backend service is referenced\nby a URL map that is bound to target gRPC proxy. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_policy` after provisioning.\nThe security policy associated with this backend service."]
    pub fn security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nThe security settings that apply to this backend service. This field is applicable to either\na regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C, and\nload_balancing_scheme set to INTERNAL_MANAGED; or a global backend service with the\nload_balancing_scheme set to INTERNAL_SELF_MANAGED."]
    pub fn security_settings(&self) -> terrars::ListRef<DataComputeBackendService_SecuritySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_lb_policy` after provisioning.\nURL to networkservices.ServiceLbPolicy resource.\nCan only be set if load balancing scheme is EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED for a global backend service, and EXTERNAL_MANAGED or INTERNAL_MANAGED for a regional backend service. For a global backend service, the service lb policy must be global. For a regional backend service, the service lb policy must be regional and in the same region."]
    pub fn service_lb_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_lb_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"STRONG_COOKIE_AFFINITY\"]"]
    pub fn session_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\nDescribes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY."]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> terrars::ListRef<DataComputeBackendService_StrongSessionAffinityCookieRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_settings` after provisioning.\nConfiguration for Backend Authenticated TLS and mTLS. May only be specified when the backend protocol is SSL, HTTPS or HTTP2."]
    pub fn tls_settings(&self) -> terrars::ListRef<DataComputeBackendService_TlsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_settings", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeBackendService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeBackendService { }

impl terrars::ToListMappable for DataComputeBackendService {
    type O = terrars::ListRef<DataComputeBackendServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeBackendService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_backend_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeBackendService {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeBackendService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeBackendService {
        let out = DataComputeBackendService(std::rc::Rc::new(DataComputeBackendService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeBackendServiceData_ {
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

pub struct DataComputeBackendServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeBackendServiceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.affinity_cookie_ttl_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nThe set of backends that serve this BackendService."]
    pub fn backend(&self) -> terrars::SetRef<DataComputeBackendService_BackendRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this BackendService."]
    pub fn cdn_policy(&self) -> terrars::ListRef<DataComputeBackendService_CdnPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `circuit_breakers` after provisioning.\nSettings controlling the volume of connections to a backend service. This field\nis applicable only when the load_balancing_scheme is set to INTERNAL_SELF_MANAGED."]
    pub fn circuit_breakers(&self) -> terrars::ListRef<DataComputeBackendService_CircuitBreakersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.circuit_breakers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_draining_timeout_sec` after provisioning.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn connection_draining_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout_sec", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `consistent_hash` after provisioning.\nConsistent Hash-based load balancing can be used to provide soft session\naffinity based on HTTP headers, cookies or other properties. This load balancing\npolicy is applicable only for HTTP connections. The affinity to a particular\ndestination host will be lost when one or more hosts are added/removed from the\ndestination service. This field specifies parameters that control consistent\nhashing. This field only applies if the load_balancing_scheme is set to\nINTERNAL_SELF_MANAGED. This field is only applicable when locality_lb_policy is\nset to MAGLEV or RING_HASH."]
    pub fn consistent_hash(&self) -> terrars::ListRef<DataComputeBackendService_ConsistentHashRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consistent_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_metrics` after provisioning.\nList of custom metrics that are used for the WEIGHTED_ROUND_ROBIN locality_lb_policy."]
    pub fn custom_metrics(&self) -> terrars::ListRef<DataComputeBackendService_CustomMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metrics", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_request_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nrequests."]
    pub fn custom_request_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_request_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nresponses."]
    pub fn custom_response_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_response_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe resource URL for the edge security policy associated with this backend service."]
    pub fn edge_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendService."]
    pub fn enable_cdn(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cdn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_managed_migration_state` after provisioning.\nSpecifies the canary migration state. Possible values are PREPARE, TEST_BY_PERCENTAGE, and\nTEST_ALL_TRAFFIC.\n\nTo begin the migration from EXTERNAL to EXTERNAL_MANAGED, the state must be changed to\nPREPARE. The state must be changed to TEST_ALL_TRAFFIC before the loadBalancingScheme can be\nchanged to EXTERNAL_MANAGED. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate\ntraffic by percentage using externalManagedMigrationTestingPercentage.\n\nRolling back a migration requires the states to be set in reverse order. So changing the\nscheme from EXTERNAL_MANAGED to EXTERNAL requires the state to be set to TEST_ALL_TRAFFIC at\nthe same time. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate some traffic\nback to EXTERNAL or PREPARE can be used to migrate all traffic back to EXTERNAL. Possible values: [\"PREPARE\", \"TEST_BY_PERCENTAGE\", \"TEST_ALL_TRAFFIC\"]"]
    pub fn external_managed_migration_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_managed_migration_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_managed_migration_testing_percentage` after provisioning.\nDetermines the fraction of requests that should be processed by the Global external\nApplication Load Balancer.\n\nThe value of this field must be in the range [0, 100].\n\nSession affinity options will slightly affect this routing behavior, for more details,\nsee: Session Affinity.\n\nThis value can only be set if the loadBalancingScheme in the backend service is set to\nEXTERNAL (when using the Classic ALB) and the migration state is TEST_BY_PERCENTAGE."]
    pub fn external_managed_migration_testing_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_managed_migration_testing_percentage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this\nobject. This field is used in optimistic locking."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn generated_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to the HttpHealthCheck or HttpsHealthCheck resource\nfor health checking this BackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend.\n\nFor internal load balancing, a URL to a HealthCheck resource must be specified instead."]
    pub fn health_checks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.health_checks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iap` after provisioning.\nSettings for enabling Cloud Identity Aware Proxy.\nIf OAuth client is not set, the Google-managed OAuth client is used."]
    pub fn iap(&self) -> terrars::ListRef<DataComputeBackendService_IapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address_selection_policy` after provisioning.\nSpecifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC). Possible values: [\"IPV4_ONLY\", \"PREFER_IPV6\", \"IPV6_ONLY\"]"]
    pub fn ip_address_selection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_selection_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nIndicates whether the backend service will be used with internal or\nexternal load balancing. A backend service created for one type of\nload balancing cannot be used with the other. For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"EXTERNAL_MANAGED\" Possible values: [\"EXTERNAL\", \"INTERNAL_SELF_MANAGED\", \"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locality_lb_policies` after provisioning.\nA list of locality load balancing policies to be used in order of\npreference. Either the policy or the customPolicy field should be set.\nOverrides any value set in the localityLbPolicy field.\n\nlocalityLbPolicies is only supported when the BackendService is referenced\nby a URL Map that is referenced by a target gRPC proxy that has the\nvalidateForProxyless field set to true."]
    pub fn locality_lb_policies(&self) -> terrars::ListRef<DataComputeBackendService_LocalityLbPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locality_lb_policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality_lb_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_config` after provisioning.\nThis field denotes the logging options for the load balancer traffic served by this backend service.\nIf logging is enabled, logs will be exported to Stackdriver."]
    pub fn log_config(&self) -> terrars::ListRef<DataComputeBackendService_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_stream_duration` after provisioning.\nSpecifies the default maximum duration (timeout) for streams to this service. Duration is computed from the\nbeginning of the stream until the response has been completely processed, including all retries. A stream that\ndoes not complete in this duration is closed.\nIf not specified, there will be no timeout limit, i.e. the maximum duration is infinite.\nThis value can be overridden in the PathMatcher configuration of the UrlMap that references this backend service.\nThis field is only allowed when the loadBalancingScheme of the backend service is INTERNAL_SELF_MANAGED."]
    pub fn max_stream_duration(&self) -> terrars::ListRef<DataComputeBackendService_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `outlier_detection` after provisioning.\nSettings controlling eviction of unhealthy hosts from the load balancing pool.\nApplicable backend service types can be a global backend service with the\nloadBalancingScheme set to INTERNAL_SELF_MANAGED or EXTERNAL_MANAGED."]
    pub fn outlier_detection(&self) -> terrars::ListRef<DataComputeBackendService_OutlierDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> terrars::ListRef<DataComputeBackendService_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nName of backend port. The same name should appear in the instance\ngroups referenced by this service. Required when the load balancing\nscheme is EXTERNAL."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Must be set to GRPC when the backend service is referenced\nby a URL map that is bound to target gRPC proxy. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_policy` after provisioning.\nThe security policy associated with this backend service."]
    pub fn security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nThe security settings that apply to this backend service. This field is applicable to either\na regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C, and\nload_balancing_scheme set to INTERNAL_MANAGED; or a global backend service with the\nload_balancing_scheme set to INTERNAL_SELF_MANAGED."]
    pub fn security_settings(&self) -> terrars::ListRef<DataComputeBackendService_SecuritySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_lb_policy` after provisioning.\nURL to networkservices.ServiceLbPolicy resource.\nCan only be set if load balancing scheme is EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED for a global backend service, and EXTERNAL_MANAGED or INTERNAL_MANAGED for a regional backend service. For a global backend service, the service lb policy must be global. For a regional backend service, the service lb policy must be regional and in the same region."]
    pub fn service_lb_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_lb_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"STRONG_COOKIE_AFFINITY\"]"]
    pub fn session_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\nDescribes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY."]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> terrars::ListRef<DataComputeBackendService_StrongSessionAffinityCookieRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_settings` after provisioning.\nConfiguration for Backend Authenticated TLS and mTLS. May only be specified when the backend protocol is SSL, HTTPS or HTTP2."]
    pub fn tls_settings(&self) -> terrars::ListRef<DataComputeBackendService_TlsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_Backend_CustomMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dry_run: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_utilization: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_Backend_CustomMetrics {
    #[doc = "Set the field `dry_run`.\n"]
    pub fn set_dry_run(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dry_run = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_utilization`.\n"]
    pub fn set_max_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_Backend_CustomMetrics {
    type O = terrars::BlockAssignable<DataComputeBackendService_Backend_CustomMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_Backend_CustomMetrics {}

impl BuildDataComputeBackendService_Backend_CustomMetrics {
    pub fn build(self) -> DataComputeBackendService_Backend_CustomMetrics {
        DataComputeBackendService_Backend_CustomMetrics {
            dry_run: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_Backend_CustomMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_Backend_CustomMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_Backend_CustomMetricsRef {
        DataComputeBackendService_Backend_CustomMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_Backend_CustomMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dry_run` after provisioning.\n"]
    pub fn dry_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }

    #[doc = "Get a reference to the value of field `max_utilization` after provisioning.\n"]
    pub fn max_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_Backend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    balancing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_scaler: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metrics: std::option::Option<terrars::ListField<DataComputeBackendService_Backend_CustomMetrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_connections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_connections_per_endpoint: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_connections_per_instance: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_rate_per_endpoint: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_rate_per_instance: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_utilization: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_Backend {
    #[doc = "Set the field `balancing_mode`.\n"]
    pub fn set_balancing_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.balancing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_scaler`.\n"]
    pub fn set_capacity_scaler(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.capacity_scaler = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_metrics`.\n"]
    pub fn set_custom_metrics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_Backend_CustomMetrics>>,
    ) -> Self {
        self.custom_metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_connections`.\n"]
    pub fn set_max_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_connections_per_endpoint`.\n"]
    pub fn set_max_connections_per_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections_per_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_connections_per_instance`.\n"]
    pub fn set_max_connections_per_instance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections_per_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_rate`.\n"]
    pub fn set_max_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_rate_per_endpoint`.\n"]
    pub fn set_max_rate_per_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_rate_per_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_rate_per_instance`.\n"]
    pub fn set_max_rate_per_instance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_rate_per_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_utilization`.\n"]
    pub fn set_max_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preference`.\n"]
    pub fn set_preference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_Backend {
    type O = terrars::BlockAssignable<DataComputeBackendService_Backend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_Backend {}

impl BuildDataComputeBackendService_Backend {
    pub fn build(self) -> DataComputeBackendService_Backend {
        DataComputeBackendService_Backend {
            balancing_mode: core::default::Default::default(),
            capacity_scaler: core::default::Default::default(),
            custom_metrics: core::default::Default::default(),
            description: core::default::Default::default(),
            group: core::default::Default::default(),
            max_connections: core::default::Default::default(),
            max_connections_per_endpoint: core::default::Default::default(),
            max_connections_per_instance: core::default::Default::default(),
            max_rate: core::default::Default::default(),
            max_rate_per_endpoint: core::default::Default::default(),
            max_rate_per_instance: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            preference: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_BackendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_BackendRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_BackendRef {
        DataComputeBackendService_BackendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_BackendRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `balancing_mode` after provisioning.\n"]
    pub fn balancing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.balancing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_scaler` after provisioning.\n"]
    pub fn capacity_scaler(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_scaler", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> terrars::ListRef<DataComputeBackendService_Backend_CustomMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `max_connections_per_endpoint` after provisioning.\n"]
    pub fn max_connections_per_endpoint(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections_per_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `max_connections_per_instance` after provisioning.\n"]
    pub fn max_connections_per_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections_per_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `max_rate` after provisioning.\n"]
    pub fn max_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `max_rate_per_endpoint` after provisioning.\n"]
    pub fn max_rate_per_endpoint(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rate_per_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `max_rate_per_instance` after provisioning.\n"]
    pub fn max_rate_per_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rate_per_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `max_utilization` after provisioning.\n"]
    pub fn max_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    type O = terrars::BlockAssignable<DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {}

impl BuildDataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    pub fn build(self) -> DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
        DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
            header_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
        DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_CdnPolicy_CacheKeyPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_host: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_http_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_named_cookies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_protocol: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_query_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_blacklist: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_whitelist: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeBackendService_CdnPolicy_CacheKeyPolicy {
    #[doc = "Set the field `include_host`.\n"]
    pub fn set_include_host(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_http_headers`.\n"]
    pub fn set_include_http_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_http_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_named_cookies`.\n"]
    pub fn set_include_named_cookies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_named_cookies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_protocol`.\n"]
    pub fn set_include_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_query_string`.\n"]
    pub fn set_include_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_blacklist`.\n"]
    pub fn set_query_string_blacklist(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_blacklist = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_whitelist`.\n"]
    pub fn set_query_string_whitelist(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_whitelist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_CdnPolicy_CacheKeyPolicy {
    type O = terrars::BlockAssignable<DataComputeBackendService_CdnPolicy_CacheKeyPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_CdnPolicy_CacheKeyPolicy {}

impl BuildDataComputeBackendService_CdnPolicy_CacheKeyPolicy {
    pub fn build(self) -> DataComputeBackendService_CdnPolicy_CacheKeyPolicy {
        DataComputeBackendService_CdnPolicy_CacheKeyPolicy {
            include_host: core::default::Default::default(),
            include_http_headers: core::default::Default::default(),
            include_named_cookies: core::default::Default::default(),
            include_protocol: core::default::Default::default(),
            include_query_string: core::default::Default::default(),
            query_string_blacklist: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
        DataComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_host` after provisioning.\n"]
    pub fn include_host(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_host", self.base))
    }

    #[doc = "Get a reference to the value of field `include_http_headers` after provisioning.\n"]
    pub fn include_http_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_http_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `include_named_cookies` after provisioning.\n"]
    pub fn include_named_cookies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_named_cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `include_protocol` after provisioning.\n"]
    pub fn include_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `include_query_string` after provisioning.\n"]
    pub fn include_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_blacklist` after provisioning.\n"]
    pub fn query_string_blacklist(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.query_string_blacklist", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_whitelist` after provisioning.\n"]
    pub fn query_string_whitelist(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.query_string_whitelist", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_CdnPolicy_NegativeCachingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_CdnPolicy_NegativeCachingPolicy {
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

impl terrars::ToListMappable for DataComputeBackendService_CdnPolicy_NegativeCachingPolicy {
    type O = terrars::BlockAssignable<DataComputeBackendService_CdnPolicy_NegativeCachingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_CdnPolicy_NegativeCachingPolicy {}

impl BuildDataComputeBackendService_CdnPolicy_NegativeCachingPolicy {
    pub fn build(self) -> DataComputeBackendService_CdnPolicy_NegativeCachingPolicy {
        DataComputeBackendService_CdnPolicy_NegativeCachingPolicy {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
        DataComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
    #[allow(dead_code)]
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
pub struct DataComputeBackendService_CdnPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_cache_on_request_headers: std
    ::option
    ::Option<terrars::ListField<DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_policy: std::option::Option<terrars::ListField<DataComputeBackendService_CdnPolicy_CacheKeyPolicy>>,
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
    ::Option<terrars::ListField<DataComputeBackendService_CdnPolicy_NegativeCachingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_coalescing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_while_stale: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_url_cache_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_CdnPolicy {
    #[doc = "Set the field `bypass_cache_on_request_headers`.\n"]
    pub fn set_bypass_cache_on_request_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders>>,
    ) -> Self {
        self.bypass_cache_on_request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_CdnPolicy_CacheKeyPolicy>>,
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
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_CdnPolicy_NegativeCachingPolicy>>,
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

impl terrars::ToListMappable for DataComputeBackendService_CdnPolicy {
    type O = terrars::BlockAssignable<DataComputeBackendService_CdnPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_CdnPolicy {}

impl BuildDataComputeBackendService_CdnPolicy {
    pub fn build(self) -> DataComputeBackendService_CdnPolicy {
        DataComputeBackendService_CdnPolicy {
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

#[allow(dead_code)]
pub struct DataComputeBackendService_CdnPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_CdnPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_CdnPolicyRef {
        DataComputeBackendService_CdnPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_CdnPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> terrars::ListRef<DataComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bypass_cache_on_request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(&self) -> terrars::ListRef<DataComputeBackendService_CdnPolicy_CacheKeyPolicyRef> {
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
    ) -> terrars::ListRef<DataComputeBackendService_CdnPolicy_NegativeCachingPolicyRef> {
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

#[derive(Serialize)]
pub struct DataComputeBackendService_CircuitBreakers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_connections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_requests: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_requests: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_requests_per_connection: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_CircuitBreakers {
    #[doc = "Set the field `max_connections`.\n"]
    pub fn set_max_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_requests_per_connection`.\n"]
    pub fn set_max_requests_per_connection(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_requests_per_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_CircuitBreakers {
    type O = terrars::BlockAssignable<DataComputeBackendService_CircuitBreakers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_CircuitBreakers {}

impl BuildDataComputeBackendService_CircuitBreakers {
    pub fn build(self) -> DataComputeBackendService_CircuitBreakers {
        DataComputeBackendService_CircuitBreakers {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
            max_requests: core::default::Default::default(),
            max_requests_per_connection: core::default::Default::default(),
            max_retries: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_CircuitBreakersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_CircuitBreakersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_CircuitBreakersRef {
        DataComputeBackendService_CircuitBreakersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_CircuitBreakersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `max_pending_requests` after provisioning.\n"]
    pub fn max_pending_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_requests", self.base))
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }

    #[doc = "Get a reference to the value of field `max_requests_per_connection` after provisioning.\n"]
    pub fn max_requests_per_connection(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests_per_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    type O = terrars::BlockAssignable<DataComputeBackendService_ConsistentHash_HttpCookie_Ttl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_ConsistentHash_HttpCookie_Ttl {}

impl BuildDataComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    pub fn build(self) -> DataComputeBackendService_ConsistentHash_HttpCookie_Ttl {
        DataComputeBackendService_ConsistentHash_HttpCookie_Ttl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
        DataComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_ConsistentHash_HttpCookie {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::ListField<DataComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
}

impl DataComputeBackendService_ConsistentHash_HttpCookie {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
    ) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_ConsistentHash_HttpCookie {
    type O = terrars::BlockAssignable<DataComputeBackendService_ConsistentHash_HttpCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_ConsistentHash_HttpCookie {}

impl BuildDataComputeBackendService_ConsistentHash_HttpCookie {
    pub fn build(self) -> DataComputeBackendService_ConsistentHash_HttpCookie {
        DataComputeBackendService_ConsistentHash_HttpCookie {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_ConsistentHash_HttpCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_ConsistentHash_HttpCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_ConsistentHash_HttpCookieRef {
        DataComputeBackendService_ConsistentHash_HttpCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_ConsistentHash_HttpCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<DataComputeBackendService_ConsistentHash_HttpCookie_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_ConsistentHash {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_cookie: std::option::Option<terrars::ListField<DataComputeBackendService_ConsistentHash_HttpCookie>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_ring_size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_ConsistentHash {
    #[doc = "Set the field `http_cookie`.\n"]
    pub fn set_http_cookie(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_ConsistentHash_HttpCookie>>,
    ) -> Self {
        self.http_cookie = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_header_name`.\n"]
    pub fn set_http_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_header_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_ring_size`.\n"]
    pub fn set_minimum_ring_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_ring_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_ConsistentHash {
    type O = terrars::BlockAssignable<DataComputeBackendService_ConsistentHash>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_ConsistentHash {}

impl BuildDataComputeBackendService_ConsistentHash {
    pub fn build(self) -> DataComputeBackendService_ConsistentHash {
        DataComputeBackendService_ConsistentHash {
            http_cookie: core::default::Default::default(),
            http_header_name: core::default::Default::default(),
            minimum_ring_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_ConsistentHashRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_ConsistentHashRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_ConsistentHashRef {
        DataComputeBackendService_ConsistentHashRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_ConsistentHashRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_cookie` after provisioning.\n"]
    pub fn http_cookie(&self) -> terrars::ListRef<DataComputeBackendService_ConsistentHash_HttpCookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header_name` after provisioning.\n"]
    pub fn http_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_ring_size` after provisioning.\n"]
    pub fn minimum_ring_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_ring_size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_CustomMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dry_run: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_CustomMetrics {
    #[doc = "Set the field `dry_run`.\n"]
    pub fn set_dry_run(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dry_run = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_CustomMetrics {
    type O = terrars::BlockAssignable<DataComputeBackendService_CustomMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_CustomMetrics {}

impl BuildDataComputeBackendService_CustomMetrics {
    pub fn build(self) -> DataComputeBackendService_CustomMetrics {
        DataComputeBackendService_CustomMetrics {
            dry_run: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_CustomMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_CustomMetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_CustomMetricsRef {
        DataComputeBackendService_CustomMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_CustomMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dry_run` after provisioning.\n"]
    pub fn dry_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_Iap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_id_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret_sha256: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_Iap {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_id`.\n"]
    pub fn set_oauth2_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oauth2_client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_id_wo`.\n"]
    pub fn set_oauth2_client_id_wo(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_id_wo_version`.\n"]
    pub fn set_oauth2_client_id_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_id_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_secret`.\n"]
    pub fn set_oauth2_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_secret_sha256`.\n"]
    pub fn set_oauth2_client_secret_sha256(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret_sha256 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_secret_wo`.\n"]
    pub fn set_oauth2_client_secret_wo(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_secret_wo_version`.\n"]
    pub fn set_oauth2_client_secret_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_Iap {
    type O = terrars::BlockAssignable<DataComputeBackendService_Iap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_Iap {}

impl BuildDataComputeBackendService_Iap {
    pub fn build(self) -> DataComputeBackendService_Iap {
        DataComputeBackendService_Iap {
            enabled: core::default::Default::default(),
            oauth2_client_id: core::default::Default::default(),
            oauth2_client_id_wo: core::default::Default::default(),
            oauth2_client_id_wo_version: core::default::Default::default(),
            oauth2_client_secret: core::default::Default::default(),
            oauth2_client_secret_sha256: core::default::Default::default(),
            oauth2_client_secret_wo: core::default::Default::default(),
            oauth2_client_secret_wo_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_IapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_IapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_IapRef {
        DataComputeBackendService_IapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_IapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_id` after provisioning.\n"]
    pub fn oauth2_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_id_wo` after provisioning.\n"]
    pub fn oauth2_client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_id_wo_version` after provisioning.\n"]
    pub fn oauth2_client_id_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_secret` after provisioning.\n"]
    pub fn oauth2_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_secret_sha256` after provisioning.\n"]
    pub fn oauth2_client_secret_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_sha256", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_secret_wo` after provisioning.\n"]
    pub fn oauth2_client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_secret_wo_version` after provisioning.\n"]
    pub fn oauth2_client_secret_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_wo_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_LocalityLbPolicies_CustomPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_LocalityLbPolicies_CustomPolicy {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_LocalityLbPolicies_CustomPolicy {
    type O = terrars::BlockAssignable<DataComputeBackendService_LocalityLbPolicies_CustomPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_LocalityLbPolicies_CustomPolicy {}

impl BuildDataComputeBackendService_LocalityLbPolicies_CustomPolicy {
    pub fn build(self) -> DataComputeBackendService_LocalityLbPolicies_CustomPolicy {
        DataComputeBackendService_LocalityLbPolicies_CustomPolicy {
            data: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
        DataComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_LocalityLbPolicies_Policy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_LocalityLbPolicies_Policy {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_LocalityLbPolicies_Policy {
    type O = terrars::BlockAssignable<DataComputeBackendService_LocalityLbPolicies_Policy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_LocalityLbPolicies_Policy {}

impl BuildDataComputeBackendService_LocalityLbPolicies_Policy {
    pub fn build(self) -> DataComputeBackendService_LocalityLbPolicies_Policy {
        DataComputeBackendService_LocalityLbPolicies_Policy { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_LocalityLbPolicies_PolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_LocalityLbPolicies_PolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_LocalityLbPolicies_PolicyRef {
        DataComputeBackendService_LocalityLbPolicies_PolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_LocalityLbPolicies_PolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_LocalityLbPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_policy: std::option::Option<terrars::ListField<DataComputeBackendService_LocalityLbPolicies_CustomPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::ListField<DataComputeBackendService_LocalityLbPolicies_Policy>>,
}

impl DataComputeBackendService_LocalityLbPolicies {
    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_LocalityLbPolicies_CustomPolicy>>,
    ) -> Self {
        self.custom_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_LocalityLbPolicies_Policy>>,
    ) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_LocalityLbPolicies {
    type O = terrars::BlockAssignable<DataComputeBackendService_LocalityLbPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_LocalityLbPolicies {}

impl BuildDataComputeBackendService_LocalityLbPolicies {
    pub fn build(self) -> DataComputeBackendService_LocalityLbPolicies {
        DataComputeBackendService_LocalityLbPolicies {
            custom_policy: core::default::Default::default(),
            policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_LocalityLbPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_LocalityLbPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_LocalityLbPoliciesRef {
        DataComputeBackendService_LocalityLbPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_LocalityLbPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(&self) -> terrars::ListRef<DataComputeBackendService_LocalityLbPolicies_CustomPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::ListRef<DataComputeBackendService_LocalityLbPolicies_PolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_LogConfig_RequestHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_LogConfig_RequestHeaders {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_LogConfig_RequestHeaders {
    type O = terrars::BlockAssignable<DataComputeBackendService_LogConfig_RequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_LogConfig_RequestHeaders {}

impl BuildDataComputeBackendService_LogConfig_RequestHeaders {
    pub fn build(self) -> DataComputeBackendService_LogConfig_RequestHeaders {
        DataComputeBackendService_LogConfig_RequestHeaders { header_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_LogConfig_RequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_LogConfig_RequestHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_LogConfig_RequestHeadersRef {
        DataComputeBackendService_LogConfig_RequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_LogConfig_RequestHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_LogConfig_ResponseHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_LogConfig_ResponseHeaders {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_LogConfig_ResponseHeaders {
    type O = terrars::BlockAssignable<DataComputeBackendService_LogConfig_ResponseHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_LogConfig_ResponseHeaders {}

impl BuildDataComputeBackendService_LogConfig_ResponseHeaders {
    pub fn build(self) -> DataComputeBackendService_LogConfig_ResponseHeaders {
        DataComputeBackendService_LogConfig_ResponseHeaders { header_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_LogConfig_ResponseHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_LogConfig_ResponseHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_LogConfig_ResponseHeadersRef {
        DataComputeBackendService_LogConfig_ResponseHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_LogConfig_ResponseHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_LogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers: std::option::Option<terrars::ListField<DataComputeBackendService_LogConfig_RequestHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers: std::option::Option<terrars::ListField<DataComputeBackendService_LogConfig_ResponseHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_LogConfig {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `optional_fields`.\n"]
    pub fn set_optional_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.optional_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `optional_mode`.\n"]
    pub fn set_optional_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.optional_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers`.\n"]
    pub fn set_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_LogConfig_RequestHeaders>>,
    ) -> Self {
        self.request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_headers`.\n"]
    pub fn set_response_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_LogConfig_ResponseHeaders>>,
    ) -> Self {
        self.response_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_LogConfig {
    type O = terrars::BlockAssignable<DataComputeBackendService_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_LogConfig {}

impl BuildDataComputeBackendService_LogConfig {
    pub fn build(self) -> DataComputeBackendService_LogConfig {
        DataComputeBackendService_LogConfig {
            enable: core::default::Default::default(),
            optional_fields: core::default::Default::default(),
            optional_mode: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            response_headers: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_LogConfigRef {
        DataComputeBackendService_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_LogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\n"]
    pub fn optional_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optional_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `optional_mode` after provisioning.\n"]
    pub fn optional_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optional_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `request_headers` after provisioning.\n"]
    pub fn request_headers(&self) -> terrars::ListRef<DataComputeBackendService_LogConfig_RequestHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers` after provisioning.\n"]
    pub fn response_headers(&self) -> terrars::ListRef<DataComputeBackendService_LogConfig_ResponseHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_MaxStreamDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_MaxStreamDuration {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_MaxStreamDuration {
    type O = terrars::BlockAssignable<DataComputeBackendService_MaxStreamDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_MaxStreamDuration {}

impl BuildDataComputeBackendService_MaxStreamDuration {
    pub fn build(self) -> DataComputeBackendService_MaxStreamDuration {
        DataComputeBackendService_MaxStreamDuration {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_MaxStreamDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_MaxStreamDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_MaxStreamDurationRef {
        DataComputeBackendService_MaxStreamDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_MaxStreamDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_OutlierDetection_BaseEjectionTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_OutlierDetection_BaseEjectionTime {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_OutlierDetection_BaseEjectionTime {
    type O = terrars::BlockAssignable<DataComputeBackendService_OutlierDetection_BaseEjectionTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_OutlierDetection_BaseEjectionTime {}

impl BuildDataComputeBackendService_OutlierDetection_BaseEjectionTime {
    pub fn build(self) -> DataComputeBackendService_OutlierDetection_BaseEjectionTime {
        DataComputeBackendService_OutlierDetection_BaseEjectionTime {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
        DataComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_OutlierDetection_Interval {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_OutlierDetection_Interval {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_OutlierDetection_Interval {
    type O = terrars::BlockAssignable<DataComputeBackendService_OutlierDetection_Interval>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_OutlierDetection_Interval {}

impl BuildDataComputeBackendService_OutlierDetection_Interval {
    pub fn build(self) -> DataComputeBackendService_OutlierDetection_Interval {
        DataComputeBackendService_OutlierDetection_Interval {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_OutlierDetection_IntervalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_OutlierDetection_IntervalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_OutlierDetection_IntervalRef {
        DataComputeBackendService_OutlierDetection_IntervalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_OutlierDetection_IntervalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_OutlierDetection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_ejection_time: std
    ::option
    ::Option<terrars::ListField<DataComputeBackendService_OutlierDetection_BaseEjectionTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consecutive_errors: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consecutive_gateway_failure: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforcing_consecutive_errors: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforcing_consecutive_gateway_failure: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforcing_success_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::ListField<DataComputeBackendService_OutlierDetection_Interval>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ejection_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_rate_minimum_hosts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_rate_request_volume: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_rate_stdev_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendService_OutlierDetection {
    #[doc = "Set the field `base_ejection_time`.\n"]
    pub fn set_base_ejection_time(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_OutlierDetection_BaseEjectionTime>>,
    ) -> Self {
        self.base_ejection_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consecutive_errors`.\n"]
    pub fn set_consecutive_errors(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.consecutive_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consecutive_gateway_failure`.\n"]
    pub fn set_consecutive_gateway_failure(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.consecutive_gateway_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforcing_consecutive_errors`.\n"]
    pub fn set_enforcing_consecutive_errors(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.enforcing_consecutive_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforcing_consecutive_gateway_failure`.\n"]
    pub fn set_enforcing_consecutive_gateway_failure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.enforcing_consecutive_gateway_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforcing_success_rate`.\n"]
    pub fn set_enforcing_success_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.enforcing_success_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_OutlierDetection_Interval>>,
    ) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_ejection_percent`.\n"]
    pub fn set_max_ejection_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_ejection_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_rate_minimum_hosts`.\n"]
    pub fn set_success_rate_minimum_hosts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_rate_minimum_hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_rate_request_volume`.\n"]
    pub fn set_success_rate_request_volume(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_rate_request_volume = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_rate_stdev_factor`.\n"]
    pub fn set_success_rate_stdev_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_rate_stdev_factor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_OutlierDetection {
    type O = terrars::BlockAssignable<DataComputeBackendService_OutlierDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_OutlierDetection {}

impl BuildDataComputeBackendService_OutlierDetection {
    pub fn build(self) -> DataComputeBackendService_OutlierDetection {
        DataComputeBackendService_OutlierDetection {
            base_ejection_time: core::default::Default::default(),
            consecutive_errors: core::default::Default::default(),
            consecutive_gateway_failure: core::default::Default::default(),
            enforcing_consecutive_errors: core::default::Default::default(),
            enforcing_consecutive_gateway_failure: core::default::Default::default(),
            enforcing_success_rate: core::default::Default::default(),
            interval: core::default::Default::default(),
            max_ejection_percent: core::default::Default::default(),
            success_rate_minimum_hosts: core::default::Default::default(),
            success_rate_request_volume: core::default::Default::default(),
            success_rate_stdev_factor: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_OutlierDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_OutlierDetectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_OutlierDetectionRef {
        DataComputeBackendService_OutlierDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_OutlierDetectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_ejection_time` after provisioning.\n"]
    pub fn base_ejection_time(&self) -> terrars::ListRef<DataComputeBackendService_OutlierDetection_BaseEjectionTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_ejection_time", self.base))
    }

    #[doc = "Get a reference to the value of field `consecutive_errors` after provisioning.\n"]
    pub fn consecutive_errors(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `consecutive_gateway_failure` after provisioning.\n"]
    pub fn consecutive_gateway_failure(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_gateway_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `enforcing_consecutive_errors` after provisioning.\n"]
    pub fn enforcing_consecutive_errors(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcing_consecutive_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `enforcing_consecutive_gateway_failure` after provisioning.\n"]
    pub fn enforcing_consecutive_gateway_failure(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcing_consecutive_gateway_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `enforcing_success_rate` after provisioning.\n"]
    pub fn enforcing_success_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcing_success_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::ListRef<DataComputeBackendService_OutlierDetection_IntervalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `max_ejection_percent` after provisioning.\n"]
    pub fn max_ejection_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ejection_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `success_rate_minimum_hosts` after provisioning.\n"]
    pub fn success_rate_minimum_hosts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_rate_minimum_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `success_rate_request_volume` after provisioning.\n"]
    pub fn success_rate_request_volume(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_rate_request_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `success_rate_stdev_factor` after provisioning.\n"]
    pub fn success_rate_stdev_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_rate_stdev_factor", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeBackendService_Params {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_Params {
    type O = terrars::BlockAssignable<DataComputeBackendService_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_Params {}

impl BuildDataComputeBackendService_Params {
    pub fn build(self) -> DataComputeBackendService_Params {
        DataComputeBackendService_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_ParamsRef {
        DataComputeBackendService_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_SecuritySettings_AwsV4Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_SecuritySettings_AwsV4Authentication {
    #[doc = "Set the field `access_key`.\n"]
    pub fn set_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_key_id`.\n"]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_key_version`.\n"]
    pub fn set_access_key_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.access_key_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_region`.\n"]
    pub fn set_origin_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.origin_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_SecuritySettings_AwsV4Authentication {
    type O = terrars::BlockAssignable<DataComputeBackendService_SecuritySettings_AwsV4Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_SecuritySettings_AwsV4Authentication {}

impl BuildDataComputeBackendService_SecuritySettings_AwsV4Authentication {
    pub fn build(self) -> DataComputeBackendService_SecuritySettings_AwsV4Authentication {
        DataComputeBackendService_SecuritySettings_AwsV4Authentication {
            access_key: core::default::Default::default(),
            access_key_id: core::default::Default::default(),
            access_key_version: core::default::Default::default(),
            origin_region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
        DataComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key` after provisioning.\n"]
    pub fn access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }

    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\n"]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `access_key_version` after provisioning.\n"]
    pub fn access_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_version", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_region` after provisioning.\n"]
    pub fn origin_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_SecuritySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_v4_authentication: std
    ::option
    ::Option<terrars::ListField<DataComputeBackendService_SecuritySettings_AwsV4Authentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alt_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeBackendService_SecuritySettings {
    #[doc = "Set the field `aws_v4_authentication`.\n"]
    pub fn set_aws_v4_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeBackendService_SecuritySettings_AwsV4Authentication>>,
    ) -> Self {
        self.aws_v4_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_tls_policy`.\n"]
    pub fn set_client_tls_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_tls_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alt_names`.\n"]
    pub fn set_subject_alt_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subject_alt_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_SecuritySettings {
    type O = terrars::BlockAssignable<DataComputeBackendService_SecuritySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_SecuritySettings {}

impl BuildDataComputeBackendService_SecuritySettings {
    pub fn build(self) -> DataComputeBackendService_SecuritySettings {
        DataComputeBackendService_SecuritySettings {
            aws_v4_authentication: core::default::Default::default(),
            client_tls_policy: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_SecuritySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_SecuritySettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_SecuritySettingsRef {
        DataComputeBackendService_SecuritySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_SecuritySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(
        &self,
    ) -> terrars::ListRef<DataComputeBackendService_SecuritySettings_AwsV4AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_v4_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `client_tls_policy` after provisioning.\n"]
    pub fn client_tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_tls_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_alt_names` after provisioning.\n"]
    pub fn subject_alt_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alt_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_StrongSessionAffinityCookie {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::ListField<DataComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
}

impl DataComputeBackendService_StrongSessionAffinityCookie {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
    ) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_StrongSessionAffinityCookie {
    type O = terrars::BlockAssignable<DataComputeBackendService_StrongSessionAffinityCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_StrongSessionAffinityCookie {}

impl BuildDataComputeBackendService_StrongSessionAffinityCookie {
    pub fn build(self) -> DataComputeBackendService_StrongSessionAffinityCookie {
        DataComputeBackendService_StrongSessionAffinityCookie {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_StrongSessionAffinityCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_StrongSessionAffinityCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_StrongSessionAffinityCookieRef {
        DataComputeBackendService_StrongSessionAffinityCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_StrongSessionAffinityCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<DataComputeBackendService_ConsistentHash_HttpCookie_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_TlsSettings_SubjectAltNames {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uniform_resource_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendService_TlsSettings_SubjectAltNames {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uniform_resource_identifier`.\n"]
    pub fn set_uniform_resource_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.uniform_resource_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_TlsSettings_SubjectAltNames {
    type O = terrars::BlockAssignable<DataComputeBackendService_TlsSettings_SubjectAltNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_TlsSettings_SubjectAltNames {}

impl BuildDataComputeBackendService_TlsSettings_SubjectAltNames {
    pub fn build(self) -> DataComputeBackendService_TlsSettings_SubjectAltNames {
        DataComputeBackendService_TlsSettings_SubjectAltNames {
            dns_name: core::default::Default::default(),
            uniform_resource_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_TlsSettings_SubjectAltNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_TlsSettings_SubjectAltNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendService_TlsSettings_SubjectAltNamesRef {
        DataComputeBackendService_TlsSettings_SubjectAltNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_TlsSettings_SubjectAltNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `uniform_resource_identifier` after provisioning.\n"]
    pub fn uniform_resource_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uniform_resource_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendService_TlsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sni: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alt_names: std::option::Option<terrars::ListField<DataComputeBackendService_TlsSettings_SubjectAltNames>>,
}

impl DataComputeBackendService_TlsSettings {
    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity`.\n"]
    pub fn set_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sni`.\n"]
    pub fn set_sni(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sni = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alt_names`.\n"]
    pub fn set_subject_alt_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendService_TlsSettings_SubjectAltNames>>,
    ) -> Self {
        self.subject_alt_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendService_TlsSettings {
    type O = terrars::BlockAssignable<DataComputeBackendService_TlsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendService_TlsSettings {}

impl BuildDataComputeBackendService_TlsSettings {
    pub fn build(self) -> DataComputeBackendService_TlsSettings {
        DataComputeBackendService_TlsSettings {
            authentication_config: core::default::Default::default(),
            identity: core::default::Default::default(),
            sni: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeBackendService_TlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendService_TlsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendService_TlsSettingsRef {
        DataComputeBackendService_TlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendService_TlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_config", self.base))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }

    #[doc = "Get a reference to the value of field `sni` after provisioning.\n"]
    pub fn sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_alt_names` after provisioning.\n"]
    pub fn subject_alt_names(&self) -> terrars::ListRef<DataComputeBackendService_TlsSettings_SubjectAltNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alt_names", self.base))
    }
}
