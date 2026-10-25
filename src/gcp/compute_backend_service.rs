#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeBackendServiceData_ {
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
    affinity_cookie_ttl_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_draining_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cdn: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_managed_migration_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_managed_migration_testing_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_checks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_selection_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancing_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality_lb_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_lb_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend: std::option::Option<std::vec::Vec<ComputeBackendService_Backend>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdn_policy: std::option::Option<std::vec::Vec<ComputeBackendService_CdnPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    circuit_breakers: std::option::Option<std::vec::Vec<ComputeBackendService_CircuitBreakers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consistent_hash: std::option::Option<std::vec::Vec<ComputeBackendService_ConsistentHash>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metrics: std::option::Option<std::vec::Vec<ComputeBackendService_CustomMetrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iap: std::option::Option<std::vec::Vec<ComputeBackendService_Iap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality_lb_policies: std::option::Option<std::vec::Vec<ComputeBackendService_LocalityLbPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<ComputeBackendService_LogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_stream_duration: std::option::Option<std::vec::Vec<ComputeBackendService_MaxStreamDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outlier_detection: std::option::Option<std::vec::Vec<ComputeBackendService_OutlierDetection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeBackendService_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_settings: std::option::Option<std::vec::Vec<ComputeBackendService_SecuritySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strong_session_affinity_cookie: std::option::Option<std::vec::Vec<ComputeBackendService_StrongSessionAffinityCookie>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeBackendService_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_settings: std::option::Option<std::vec::Vec<ComputeBackendService_TlsSettings>>,
    dynamic: ComputeBackendServiceDynamic,
}

struct ComputeBackendService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeBackendServiceData_>,
}

#[derive(Clone)]
pub struct ComputeBackendService(std::rc::Rc<ComputeBackendService_>);

impl ComputeBackendService {
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
        "Set the field `affinity_cookie_ttl_sec`.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn set_affinity_cookie_ttl_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().affinity_cookie_ttl_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `compression_mode`.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn set_compression_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compression_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_draining_timeout_sec`.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn set_connection_draining_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().connection_draining_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_request_headers`.\nHeaders that the HTTP/S load balancer should add to proxied\nrequests."]
    pub fn set_custom_request_headers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_response_headers`.\nHeaders that the HTTP/S load balancer should add to proxied\nresponses."]
    pub fn set_custom_response_headers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
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

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `edge_security_policy`.\nThe resource URL for the edge security policy associated with this backend service."]
    pub fn set_edge_security_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().edge_security_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_cdn`.\nIf true, enable Cloud CDN for this BackendService."]
    pub fn set_enable_cdn(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_cdn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_managed_migration_state`.\nSpecifies the canary migration state. Possible values are PREPARE, TEST_BY_PERCENTAGE, and\nTEST_ALL_TRAFFIC.\n\nTo begin the migration from EXTERNAL to EXTERNAL_MANAGED, the state must be changed to\nPREPARE. The state must be changed to TEST_ALL_TRAFFIC before the loadBalancingScheme can be\nchanged to EXTERNAL_MANAGED. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate\ntraffic by percentage using externalManagedMigrationTestingPercentage.\n\nRolling back a migration requires the states to be set in reverse order. So changing the\nscheme from EXTERNAL_MANAGED to EXTERNAL requires the state to be set to TEST_ALL_TRAFFIC at\nthe same time. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate some traffic\nback to EXTERNAL or PREPARE can be used to migrate all traffic back to EXTERNAL. Possible values: [\"PREPARE\", \"TEST_BY_PERCENTAGE\", \"TEST_ALL_TRAFFIC\"]"]
    pub fn set_external_managed_migration_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().external_managed_migration_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_managed_migration_testing_percentage`.\nDetermines the fraction of requests that should be processed by the Global external\nApplication Load Balancer.\n\nThe value of this field must be in the range [0, 100].\n\nSession affinity options will slightly affect this routing behavior, for more details,\nsee: Session Affinity.\n\nThis value can only be set if the loadBalancingScheme in the backend service is set to\nEXTERNAL (when using the Classic ALB) and the migration state is TEST_BY_PERCENTAGE."]
    pub fn set_external_managed_migration_testing_percentage(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().external_managed_migration_testing_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `health_checks`.\nThe set of URLs to the HttpHealthCheck or HttpsHealthCheck resource\nfor health checking this BackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend.\n\nFor internal load balancing, a URL to a HealthCheck resource must be specified instead."]
    pub fn set_health_checks(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().health_checks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_address_selection_policy`.\nSpecifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC). Possible values: [\"IPV4_ONLY\", \"PREFER_IPV6\", \"IPV6_ONLY\"]"]
    pub fn set_ip_address_selection_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().ip_address_selection_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `load_balancing_scheme`.\nIndicates whether the backend service will be used with internal or\nexternal load balancing. A backend service created for one type of\nload balancing cannot be used with the other. For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"EXTERNAL_MANAGED\" Possible values: [\"EXTERNAL\", \"INTERNAL_SELF_MANAGED\", \"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn set_load_balancing_scheme(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `locality_lb_policy`.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn set_locality_lb_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().locality_lb_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_name`.\nName of backend port. The same name should appear in the instance\ngroups referenced by this service. Required when the load balancing\nscheme is EXTERNAL."]
    pub fn set_port_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protocol`.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Must be set to GRPC when the backend service is referenced\nby a URL map that is bound to target gRPC proxy. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_policy`.\nThe security policy associated with this backend service."]
    pub fn set_security_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_lb_policy`.\nURL to networkservices.ServiceLbPolicy resource.\nCan only be set if load balancing scheme is EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED for a global backend service, and EXTERNAL_MANAGED or INTERNAL_MANAGED for a regional backend service. For a global backend service, the service lb policy must be global. For a regional backend service, the service lb policy must be regional and in the same region."]
    pub fn set_service_lb_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_lb_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_affinity`.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"STRONG_COOKIE_AFFINITY\"]"]
    pub fn set_session_affinity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().session_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_sec`.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn set_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backend`.\n"]
    pub fn set_backend(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_Backend>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backend = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backend = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cdn_policy`.\n"]
    pub fn set_cdn_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_CdnPolicy>>,
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

    #[doc = "Set the field `circuit_breakers`.\n"]
    pub fn set_circuit_breakers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_CircuitBreakers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().circuit_breakers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.circuit_breakers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `consistent_hash`.\n"]
    pub fn set_consistent_hash(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_ConsistentHash>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().consistent_hash = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.consistent_hash = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_metrics`.\n"]
    pub fn set_custom_metrics(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_CustomMetrics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iap`.\n"]
    pub fn set_iap(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_Iap>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iap = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iap = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `locality_lb_policies`.\n"]
    pub fn set_locality_lb_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_LocalityLbPolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().locality_lb_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.locality_lb_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_LogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `max_stream_duration`.\n"]
    pub fn set_max_stream_duration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_MaxStreamDuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_stream_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_stream_duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outlier_detection`.\n"]
    pub fn set_outlier_detection(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_OutlierDetection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outlier_detection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outlier_detection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_Params>>) -> Self {
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

    #[doc = "Set the field `security_settings`.\n"]
    pub fn set_security_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_SecuritySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `strong_session_affinity_cookie`.\n"]
    pub fn set_strong_session_affinity_cookie(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_StrongSessionAffinityCookie>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().strong_session_affinity_cookie = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.strong_session_affinity_cookie = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeBackendService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_settings`.\n"]
    pub fn set_tls_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_TlsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.affinity_cookie_ttl_sec", self.extract_ref()))
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
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
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
        "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality_lb_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
        "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> terrars::ListRef<ComputeBackendService_CdnPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\n"]
    pub fn circuit_breakers(&self) -> terrars::ListRef<ComputeBackendService_CircuitBreakersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.circuit_breakers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\n"]
    pub fn consistent_hash(&self) -> terrars::ListRef<ComputeBackendService_ConsistentHashRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consistent_hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> terrars::ListRef<ComputeBackendService_CustomMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> terrars::ListRef<ComputeBackendService_IapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locality_lb_policies` after provisioning.\n"]
    pub fn locality_lb_policies(&self) -> terrars::ListRef<ComputeBackendService_LocalityLbPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locality_lb_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeBackendService_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(&self) -> terrars::ListRef<ComputeBackendService_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> terrars::ListRef<ComputeBackendService_OutlierDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeBackendService_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\n"]
    pub fn security_settings(&self) -> terrars::ListRef<ComputeBackendService_SecuritySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\n"]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> terrars::ListRef<ComputeBackendService_StrongSessionAffinityCookieRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendService_TimeoutsRef {
        ComputeBackendService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\n"]
    pub fn tls_settings(&self) -> terrars::ListRef<ComputeBackendService_TlsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_settings", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeBackendService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeBackendService { }

impl terrars::ToListMappable for ComputeBackendService {
    type O = terrars::ListRef<ComputeBackendServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeBackendService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_backend_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeBackendService {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeBackendService {
        let out = ComputeBackendService(std::rc::Rc::new(ComputeBackendService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeBackendServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                affinity_cookie_ttl_sec: core::default::Default::default(),
                compression_mode: core::default::Default::default(),
                connection_draining_timeout_sec: core::default::Default::default(),
                custom_request_headers: core::default::Default::default(),
                custom_response_headers: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                edge_security_policy: core::default::Default::default(),
                enable_cdn: core::default::Default::default(),
                external_managed_migration_state: core::default::Default::default(),
                external_managed_migration_testing_percentage: core::default::Default::default(),
                health_checks: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_selection_policy: core::default::Default::default(),
                load_balancing_scheme: core::default::Default::default(),
                locality_lb_policy: core::default::Default::default(),
                name: self.name,
                port_name: core::default::Default::default(),
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                security_policy: core::default::Default::default(),
                service_lb_policy: core::default::Default::default(),
                session_affinity: core::default::Default::default(),
                timeout_sec: core::default::Default::default(),
                backend: core::default::Default::default(),
                cdn_policy: core::default::Default::default(),
                circuit_breakers: core::default::Default::default(),
                consistent_hash: core::default::Default::default(),
                custom_metrics: core::default::Default::default(),
                iap: core::default::Default::default(),
                locality_lb_policies: core::default::Default::default(),
                log_config: core::default::Default::default(),
                max_stream_duration: core::default::Default::default(),
                outlier_detection: core::default::Default::default(),
                params: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                strong_session_affinity_cookie: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tls_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeBackendServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeBackendServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.affinity_cookie_ttl_sec", self.extract_ref()))
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
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
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
        "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality_lb_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
        "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> terrars::ListRef<ComputeBackendService_CdnPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\n"]
    pub fn circuit_breakers(&self) -> terrars::ListRef<ComputeBackendService_CircuitBreakersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.circuit_breakers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\n"]
    pub fn consistent_hash(&self) -> terrars::ListRef<ComputeBackendService_ConsistentHashRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consistent_hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> terrars::ListRef<ComputeBackendService_CustomMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> terrars::ListRef<ComputeBackendService_IapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locality_lb_policies` after provisioning.\n"]
    pub fn locality_lb_policies(&self) -> terrars::ListRef<ComputeBackendService_LocalityLbPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locality_lb_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeBackendService_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(&self) -> terrars::ListRef<ComputeBackendService_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> terrars::ListRef<ComputeBackendService_OutlierDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeBackendService_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\n"]
    pub fn security_settings(&self) -> terrars::ListRef<ComputeBackendService_SecuritySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\n"]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> terrars::ListRef<ComputeBackendService_StrongSessionAffinityCookieRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendService_TimeoutsRef {
        ComputeBackendService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\n"]
    pub fn tls_settings(&self) -> terrars::ListRef<ComputeBackendService_TlsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendServiceDynamic {
    backend: std::option::Option<terrars::DynamicBlock<ComputeBackendService_Backend>>,
    cdn_policy: std::option::Option<terrars::DynamicBlock<ComputeBackendService_CdnPolicy>>,
    circuit_breakers: std::option::Option<terrars::DynamicBlock<ComputeBackendService_CircuitBreakers>>,
    consistent_hash: std::option::Option<terrars::DynamicBlock<ComputeBackendService_ConsistentHash>>,
    custom_metrics: std::option::Option<terrars::DynamicBlock<ComputeBackendService_CustomMetrics>>,
    iap: std::option::Option<terrars::DynamicBlock<ComputeBackendService_Iap>>,
    locality_lb_policies: std::option::Option<terrars::DynamicBlock<ComputeBackendService_LocalityLbPolicies>>,
    log_config: std::option::Option<terrars::DynamicBlock<ComputeBackendService_LogConfig>>,
    max_stream_duration: std::option::Option<terrars::DynamicBlock<ComputeBackendService_MaxStreamDuration>>,
    outlier_detection: std::option::Option<terrars::DynamicBlock<ComputeBackendService_OutlierDetection>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeBackendService_Params>>,
    security_settings: std::option::Option<terrars::DynamicBlock<ComputeBackendService_SecuritySettings>>,
    strong_session_affinity_cookie: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeBackendService_StrongSessionAffinityCookie>>,
    tls_settings: std::option::Option<terrars::DynamicBlock<ComputeBackendService_TlsSettings>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_Backend_CustomMetrics {
    dry_run: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_utilization: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_Backend_CustomMetrics {
    #[doc =
        "Set the field `max_utilization`.\nOptional parameter to define a target utilization for the Custom Metrics\nbalancing mode. The valid range is <code>[0.0, 1.0]</code>."]
    pub fn set_max_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_utilization = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_Backend_CustomMetrics {
    type O = terrars::BlockAssignable<ComputeBackendService_Backend_CustomMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_Backend_CustomMetrics {
    #[doc =
        "If true, the metric data is collected and reported to Cloud\nMonitoring, but is not used for load balancing."]
    pub dry_run: terrars::PrimField<bool>,
    #[doc =
        "Name of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_Backend_CustomMetrics {
    pub fn build(self) -> ComputeBackendService_Backend_CustomMetrics {
        ComputeBackendService_Backend_CustomMetrics {
            dry_run: self.dry_run,
            max_utilization: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_Backend_CustomMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_Backend_CustomMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_Backend_CustomMetricsRef {
        ComputeBackendService_Backend_CustomMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_Backend_CustomMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dry_run` after provisioning.\nIf true, the metric data is collected and reported to Cloud\nMonitoring, but is not used for load balancing."]
    pub fn dry_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_utilization` after provisioning.\nOptional parameter to define a target utilization for the Custom Metrics\nbalancing mode. The valid range is <code>[0.0, 1.0]</code>."]
    pub fn max_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_utilization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_BackendDynamic {
    custom_metrics: std::option::Option<terrars::DynamicBlock<ComputeBackendService_Backend_CustomMetrics>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_Backend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    balancing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_scaler: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    group: terrars::PrimField<std::string::String>,
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metrics: std::option::Option<std::vec::Vec<ComputeBackendService_Backend_CustomMetrics>>,
    dynamic: ComputeBackendService_BackendDynamic,
}

impl ComputeBackendService_Backend {
    #[doc =
        "Set the field `balancing_mode`.\nSpecifies the balancing mode for this backend.\n\nFor global HTTP(S) or TCP/SSL load balancing, the default is\nUTILIZATION. Valid values are UTILIZATION, RATE (for HTTP(S)),\nCUSTOM_METRICS (for HTTP(s)) and CONNECTION (for TCP/SSL).\n\nSee the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)\nfor an explanation of load balancing modes. Default value: \"UTILIZATION\" Possible values: [\"UTILIZATION\", \"RATE\", \"CONNECTION\", \"CUSTOM_METRICS\", \"IN_FLIGHT\"]"]
    pub fn set_balancing_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.balancing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `capacity_scaler`.\nA multiplier applied to the group's maximum servicing capacity\n(based on UTILIZATION, RATE or CONNECTION).\n\nDefault value is 1, which means the group will serve up to 100%\nof its configured capacity (depending on balancingMode). A\nsetting of 0 means the group is completely drained, offering\n0% of its available Capacity. Valid range is [0.0,1.0]."]
    pub fn set_capacity_scaler(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.capacity_scaler = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn optional description of this resource.\nProvide this property when you create the resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_connections`.\nThe max number of simultaneous connections for the group. Can\nbe used with either CONNECTION or UTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or one\nof maxConnectionsPerInstance or maxConnectionsPerEndpoint,\nas appropriate for group type, must be set."]
    pub fn set_max_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_connections_per_endpoint`.\nThe max number of simultaneous connections that a single backend\nnetwork endpoint can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either\nmaxConnections or maxConnectionsPerEndpoint must be set."]
    pub fn set_max_connections_per_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections_per_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_connections_per_instance`.\nThe max number of simultaneous connections that a single\nbackend instance can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or\nmaxConnectionsPerInstance must be set."]
    pub fn set_max_connections_per_instance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections_per_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_rate`.\nThe max requests per second (RPS) of the group.\n\nCan be used with either RATE or UTILIZATION balancing modes,\nbut required if RATE mode. For RATE mode, either maxRate or one\nof maxRatePerInstance or maxRatePerEndpoint, as appropriate for\ngroup type, must be set."]
    pub fn set_max_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_rate_per_endpoint`.\nThe max requests per second (RPS) that a single backend network\nendpoint can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerEndpoint must be set."]
    pub fn set_max_rate_per_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_rate_per_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_rate_per_instance`.\nThe max requests per second (RPS) that a single backend\ninstance can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerInstance must be set."]
    pub fn set_max_rate_per_instance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_rate_per_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_utilization`.\nUsed when balancingMode is UTILIZATION. This ratio defines the\nCPU utilization target for the group. Valid range is [0.0, 1.0]."]
    pub fn set_max_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preference`.\nThis field indicates whether this backend should be fully utilized before sending traffic to backends\nwith default preference. This field cannot be set when loadBalancingScheme is set to 'EXTERNAL'. The possible values are:\n  - PREFERRED: Backends with this preference level will be filled up to their capacity limits first,\n    based on RTT.\n  - DEFAULT: If preferred backends don't have enough capacity, backends in this layer would be used and\n    traffic would be assigned based on the load balancing algorithm you use. This is the default Possible values: [\"PREFERRED\", \"DEFAULT\"]"]
    pub fn set_preference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_metrics`.\n"]
    pub fn set_custom_metrics(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_Backend_CustomMetrics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_Backend {
    type O = terrars::BlockAssignable<ComputeBackendService_Backend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_Backend {
    #[doc =
        "The fully-qualified URL of an Instance Group or Network Endpoint\nGroup resource. In case of instance group this defines the list\nof instances that serve traffic. Member virtual machine\ninstances from each instance group must live in the same zone as\nthe instance group itself. No two backends in a backend service\nare allowed to use same Instance Group resource.\n\nFor Network Endpoint Groups this defines list of endpoints. All\nendpoints of Network Endpoint Group must be hosted on instances\nlocated in the same zone as the Network Endpoint Group.\n\nBackend services cannot mix Instance Group and\nNetwork Endpoint Group backends.\n\nNote that you must specify an Instance Group or Network Endpoint\nGroup resource using the fully-qualified URL, rather than a\npartial URL."]
    pub group: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_Backend {
    pub fn build(self) -> ComputeBackendService_Backend {
        ComputeBackendService_Backend {
            balancing_mode: core::default::Default::default(),
            capacity_scaler: core::default::Default::default(),
            description: core::default::Default::default(),
            group: self.group,
            max_connections: core::default::Default::default(),
            max_connections_per_endpoint: core::default::Default::default(),
            max_connections_per_instance: core::default::Default::default(),
            max_rate: core::default::Default::default(),
            max_rate_per_endpoint: core::default::Default::default(),
            max_rate_per_instance: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            preference: core::default::Default::default(),
            custom_metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_BackendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_BackendRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_BackendRef {
        ComputeBackendService_BackendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_BackendRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `balancing_mode` after provisioning.\nSpecifies the balancing mode for this backend.\n\nFor global HTTP(S) or TCP/SSL load balancing, the default is\nUTILIZATION. Valid values are UTILIZATION, RATE (for HTTP(S)),\nCUSTOM_METRICS (for HTTP(s)) and CONNECTION (for TCP/SSL).\n\nSee the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)\nfor an explanation of load balancing modes. Default value: \"UTILIZATION\" Possible values: [\"UTILIZATION\", \"RATE\", \"CONNECTION\", \"CUSTOM_METRICS\", \"IN_FLIGHT\"]"]
    pub fn balancing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.balancing_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `capacity_scaler` after provisioning.\nA multiplier applied to the group's maximum servicing capacity\n(based on UTILIZATION, RATE or CONNECTION).\n\nDefault value is 1, which means the group will serve up to 100%\nof its configured capacity (depending on balancingMode). A\nsetting of 0 means the group is completely drained, offering\n0% of its available Capacity. Valid range is [0.0,1.0]."]
    pub fn capacity_scaler(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_scaler", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource.\nProvide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group` after provisioning.\nThe fully-qualified URL of an Instance Group or Network Endpoint\nGroup resource. In case of instance group this defines the list\nof instances that serve traffic. Member virtual machine\ninstances from each instance group must live in the same zone as\nthe instance group itself. No two backends in a backend service\nare allowed to use same Instance Group resource.\n\nFor Network Endpoint Groups this defines list of endpoints. All\nendpoints of Network Endpoint Group must be hosted on instances\nlocated in the same zone as the Network Endpoint Group.\n\nBackend services cannot mix Instance Group and\nNetwork Endpoint Group backends.\n\nNote that you must specify an Instance Group or Network Endpoint\nGroup resource using the fully-qualified URL, rather than a\npartial URL."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_connections` after provisioning.\nThe max number of simultaneous connections for the group. Can\nbe used with either CONNECTION or UTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or one\nof maxConnectionsPerInstance or maxConnectionsPerEndpoint,\nas appropriate for group type, must be set."]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_connections_per_endpoint` after provisioning.\nThe max number of simultaneous connections that a single backend\nnetwork endpoint can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either\nmaxConnections or maxConnectionsPerEndpoint must be set."]
    pub fn max_connections_per_endpoint(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections_per_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_connections_per_instance` after provisioning.\nThe max number of simultaneous connections that a single\nbackend instance can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or\nmaxConnectionsPerInstance must be set."]
    pub fn max_connections_per_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections_per_instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_rate` after provisioning.\nThe max requests per second (RPS) of the group.\n\nCan be used with either RATE or UTILIZATION balancing modes,\nbut required if RATE mode. For RATE mode, either maxRate or one\nof maxRatePerInstance or maxRatePerEndpoint, as appropriate for\ngroup type, must be set."]
    pub fn max_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_rate_per_endpoint` after provisioning.\nThe max requests per second (RPS) that a single backend network\nendpoint can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerEndpoint must be set."]
    pub fn max_rate_per_endpoint(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rate_per_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_rate_per_instance` after provisioning.\nThe max requests per second (RPS) that a single backend\ninstance can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerInstance must be set."]
    pub fn max_rate_per_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rate_per_instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_utilization` after provisioning.\nUsed when balancingMode is UTILIZATION. This ratio defines the\nCPU utilization target for the group. Valid range is [0.0, 1.0]."]
    pub fn max_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_utilization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preference` after provisioning.\nThis field indicates whether this backend should be fully utilized before sending traffic to backends\nwith default preference. This field cannot be set when loadBalancingScheme is set to 'EXTERNAL'. The possible values are:\n  - PREFERRED: Backends with this preference level will be filled up to their capacity limits first,\n    based on RTT.\n  - DEFAULT: If preferred backends don't have enough capacity, backends in this layer would be used and\n    traffic would be assigned based on the load balancing algorithm you use. This is the default Possible values: [\"PREFERRED\", \"DEFAULT\"]"]
    pub fn preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> terrars::ListRef<ComputeBackendService_Backend_CustomMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    header_name: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders { }

impl terrars::ToListMappable for ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    type O = terrars::BlockAssignable<ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    #[doc = "The header field name to match on when bypassing cache. Values are case-insensitive."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
    pub fn build(self) -> ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders {
        ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders { header_name: self.header_name }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
        ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef {
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
pub struct ComputeBackendService_CdnPolicy_CacheKeyPolicy {
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

impl ComputeBackendService_CdnPolicy_CacheKeyPolicy {
    #[doc = "Set the field `include_host`.\nIf true requests to different hosts will be cached separately."]
    pub fn set_include_host(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_http_headers`.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn set_include_http_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_http_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_named_cookies`.\nNames of cookies to include in cache keys."]
    pub fn set_include_named_cookies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_named_cookies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_protocol`.\nIf true, http and https requests will be cached separately."]
    pub fn set_include_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_query_string`.\nIf true, include query string parameters in the cache key\naccording to query_string_whitelist and\nquery_string_blacklist. If neither is set, the entire query\nstring will be included.\n\nIf false, the query string will be excluded from the cache\nkey entirely."]
    pub fn set_include_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_string_blacklist`.\nNames of query string parameters to exclude in cache keys.\n\nAll other parameters will be included. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn set_query_string_blacklist(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_blacklist = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_string_whitelist`.\nNames of query string parameters to include in cache keys.\n\nAll other parameters will be excluded. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn set_query_string_whitelist(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_whitelist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_CdnPolicy_CacheKeyPolicy {
    type O = terrars::BlockAssignable<ComputeBackendService_CdnPolicy_CacheKeyPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_CdnPolicy_CacheKeyPolicy {}

impl BuildComputeBackendService_CdnPolicy_CacheKeyPolicy {
    pub fn build(self) -> ComputeBackendService_CdnPolicy_CacheKeyPolicy {
        ComputeBackendService_CdnPolicy_CacheKeyPolicy {
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
pub struct ComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
        ComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_CdnPolicy_CacheKeyPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `include_host` after provisioning.\nIf true requests to different hosts will be cached separately."]
    pub fn include_host(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_http_headers` after provisioning.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn include_http_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_http_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_named_cookies` after provisioning.\nNames of cookies to include in cache keys."]
    pub fn include_named_cookies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_named_cookies", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_protocol` after provisioning.\nIf true, http and https requests will be cached separately."]
    pub fn include_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_query_string` after provisioning.\nIf true, include query string parameters in the cache key\naccording to query_string_whitelist and\nquery_string_blacklist. If neither is set, the entire query\nstring will be included.\n\nIf false, the query string will be excluded from the cache\nkey entirely."]
    pub fn include_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_query_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_string_blacklist` after provisioning.\nNames of query string parameters to exclude in cache keys.\n\nAll other parameters will be included. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn query_string_blacklist(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.query_string_blacklist", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_string_whitelist` after provisioning.\nNames of query string parameters to include in cache keys.\n\nAll other parameters will be excluded. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn query_string_whitelist(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.query_string_whitelist", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_CdnPolicy_NegativeCachingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeBackendService_CdnPolicy_NegativeCachingPolicy {
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

impl terrars::ToListMappable for ComputeBackendService_CdnPolicy_NegativeCachingPolicy {
    type O = terrars::BlockAssignable<ComputeBackendService_CdnPolicy_NegativeCachingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_CdnPolicy_NegativeCachingPolicy {}

impl BuildComputeBackendService_CdnPolicy_NegativeCachingPolicy {
    pub fn build(self) -> ComputeBackendService_CdnPolicy_NegativeCachingPolicy {
        ComputeBackendService_CdnPolicy_NegativeCachingPolicy {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
        ComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_CdnPolicy_NegativeCachingPolicyRef {
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
struct ComputeBackendService_CdnPolicyDynamic {
    bypass_cache_on_request_headers: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders>>,
    cache_key_policy: std::option::Option<terrars::DynamicBlock<ComputeBackendService_CdnPolicy_CacheKeyPolicy>>,
    negative_caching_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeBackendService_CdnPolicy_NegativeCachingPolicy>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_CdnPolicy {
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
    ::Option<std::vec::Vec<ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_policy: std::option::Option<std::vec::Vec<ComputeBackendService_CdnPolicy_CacheKeyPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching_policy: std::option::Option<std::vec::Vec<ComputeBackendService_CdnPolicy_NegativeCachingPolicy>>,
    dynamic: ComputeBackendService_CdnPolicyDynamic,
}

impl ComputeBackendService_CdnPolicy {
    #[doc =
        "Set the field `cache_mode`.\nSpecifies the cache setting for all responses from this backend.\nThe possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn set_cache_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
    pub fn set_client_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_ttl`.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age)."]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
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
        "Set the field `request_coalescing`.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests\nto the origin."]
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
        "Set the field `signed_url_cache_max_age_sec`.\nMaximum number of seconds the response to a signed URL request\nwill be considered fresh, defaults to 1hr (3600s). After this\ntime period, the response will be revalidated before\nbeing served.\n\nWhen serving responses to signed URL requests, Cloud CDN will\ninternally behave as though all responses from this backend had a\n\"Cache-Control: public, max-age=[TTL]\" header, regardless of any\nexisting Cache-Control header. The actual headers served in\nresponses will not be altered."]
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
                    ::Into<terrars::BlockAssignable<ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeaders>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_CdnPolicy_CacheKeyPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_CdnPolicy_NegativeCachingPolicy>>,
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

impl terrars::ToListMappable for ComputeBackendService_CdnPolicy {
    type O = terrars::BlockAssignable<ComputeBackendService_CdnPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_CdnPolicy {}

impl BuildComputeBackendService_CdnPolicy {
    pub fn build(self) -> ComputeBackendService_CdnPolicy {
        ComputeBackendService_CdnPolicy {
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
pub struct ComputeBackendService_CdnPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_CdnPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_CdnPolicyRef {
        ComputeBackendService_CdnPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_CdnPolicyRef {
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
        "Get a reference to the value of field `client_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
    pub fn client_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_ttl` after provisioning.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age)."]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
    pub fn max_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `negative_caching` after provisioning.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects."]
    pub fn negative_caching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negative_caching", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_coalescing` after provisioning.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests\nto the origin."]
    pub fn request_coalescing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_coalescing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serve_while_stale` after provisioning.\nServe existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache."]
    pub fn serve_while_stale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serve_while_stale", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signed_url_cache_max_age_sec` after provisioning.\nMaximum number of seconds the response to a signed URL request\nwill be considered fresh, defaults to 1hr (3600s). After this\ntime period, the response will be revalidated before\nbeing served.\n\nWhen serving responses to signed URL requests, Cloud CDN will\ninternally behave as though all responses from this backend had a\n\"Cache-Control: public, max-age=[TTL]\" header, regardless of any\nexisting Cache-Control header. The actual headers served in\nresponses will not be altered."]
    pub fn signed_url_cache_max_age_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signed_url_cache_max_age_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> terrars::ListRef<ComputeBackendService_CdnPolicy_BypassCacheOnRequestHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bypass_cache_on_request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(&self) -> terrars::ListRef<ComputeBackendService_CdnPolicy_CacheKeyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(&self) -> terrars::ListRef<ComputeBackendService_CdnPolicy_NegativeCachingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.negative_caching_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_CircuitBreakers {
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

impl ComputeBackendService_CircuitBreakers {
    #[doc =
        "Set the field `max_connections`.\nThe maximum number of connections to the backend cluster.\nDefaults to 1024."]
    pub fn set_max_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_pending_requests`.\nThe maximum number of pending requests to the backend cluster.\nDefaults to 1024."]
    pub fn set_max_pending_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_requests`.\nThe maximum number of parallel requests to the backend cluster.\nDefaults to 1024."]
    pub fn set_max_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_requests_per_connection`.\nMaximum requests for a single backend connection. This parameter\nis respected by both the HTTP/1.1 and HTTP/2 implementations. If\nnot specified, there is no limit. Setting this parameter to 1\nwill effectively disable keep alive."]
    pub fn set_max_requests_per_connection(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_requests_per_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_retries`.\nThe maximum number of parallel retries to the backend cluster.\nDefaults to 3."]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_CircuitBreakers {
    type O = terrars::BlockAssignable<ComputeBackendService_CircuitBreakers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_CircuitBreakers {}

impl BuildComputeBackendService_CircuitBreakers {
    pub fn build(self) -> ComputeBackendService_CircuitBreakers {
        ComputeBackendService_CircuitBreakers {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
            max_requests: core::default::Default::default(),
            max_requests_per_connection: core::default::Default::default(),
            max_retries: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_CircuitBreakersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_CircuitBreakersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_CircuitBreakersRef {
        ComputeBackendService_CircuitBreakersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_CircuitBreakersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_connections` after provisioning.\nThe maximum number of connections to the backend cluster.\nDefaults to 1024."]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_pending_requests` after provisioning.\nThe maximum number of pending requests to the backend cluster.\nDefaults to 1024."]
    pub fn max_pending_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_requests` after provisioning.\nThe maximum number of parallel requests to the backend cluster.\nDefaults to 1024."]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_requests_per_connection` after provisioning.\nMaximum requests for a single backend connection. This parameter\nis respected by both the HTTP/1.1 and HTTP/2 implementations. If\nnot specified, there is no limit. Setting this parameter to 1\nwill effectively disable keep alive."]
    pub fn max_requests_per_connection(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests_per_connection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_retries` after provisioning.\nThe maximum number of parallel retries to the backend cluster.\nDefaults to 3."]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<f64>,
}

impl ComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    type O = terrars::BlockAssignable<ComputeBackendService_ConsistentHash_HttpCookie_Ttl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildComputeBackendService_ConsistentHash_HttpCookie_Ttl {
    pub fn build(self) -> ComputeBackendService_ConsistentHash_HttpCookie_Ttl {
        ComputeBackendService_ConsistentHash_HttpCookie_Ttl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
        ComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_ConsistentHash_HttpCookie_TtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_ConsistentHash_HttpCookieDynamic {
    ttl: std::option::Option<terrars::DynamicBlock<ComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_ConsistentHash_HttpCookie {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<std::vec::Vec<ComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
    dynamic: ComputeBackendService_ConsistentHash_HttpCookieDynamic,
}

impl ComputeBackendService_ConsistentHash_HttpCookie {
    #[doc = "Set the field `name`.\nName of the cookie."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nPath to set for the cookie."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ttl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_ConsistentHash_HttpCookie {
    type O = terrars::BlockAssignable<ComputeBackendService_ConsistentHash_HttpCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_ConsistentHash_HttpCookie {}

impl BuildComputeBackendService_ConsistentHash_HttpCookie {
    pub fn build(self) -> ComputeBackendService_ConsistentHash_HttpCookie {
        ComputeBackendService_ConsistentHash_HttpCookie {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_ConsistentHash_HttpCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_ConsistentHash_HttpCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_ConsistentHash_HttpCookieRef {
        ComputeBackendService_ConsistentHash_HttpCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_ConsistentHash_HttpCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the cookie."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to set for the cookie."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<ComputeBackendService_ConsistentHash_HttpCookie_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_ConsistentHashDynamic {
    http_cookie: std::option::Option<terrars::DynamicBlock<ComputeBackendService_ConsistentHash_HttpCookie>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_ConsistentHash {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_ring_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_cookie: std::option::Option<std::vec::Vec<ComputeBackendService_ConsistentHash_HttpCookie>>,
    dynamic: ComputeBackendService_ConsistentHashDynamic,
}

impl ComputeBackendService_ConsistentHash {
    #[doc =
        "Set the field `http_header_name`.\nThe hash based on the value of the specified header field.\nThis field is applicable if the sessionAffinity is set to HEADER_FIELD."]
    pub fn set_http_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_header_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_ring_size`.\nThe minimum number of virtual nodes to use for the hash ring.\nLarger ring sizes result in more granular load\ndistributions. If the number of hosts in the load balancing pool\nis larger than the ring size, each host will be assigned a single\nvirtual node.\nDefaults to 1024."]
    pub fn set_minimum_ring_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_ring_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_cookie`.\n"]
    pub fn set_http_cookie(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_ConsistentHash_HttpCookie>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_cookie = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_cookie = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_ConsistentHash {
    type O = terrars::BlockAssignable<ComputeBackendService_ConsistentHash>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_ConsistentHash {}

impl BuildComputeBackendService_ConsistentHash {
    pub fn build(self) -> ComputeBackendService_ConsistentHash {
        ComputeBackendService_ConsistentHash {
            http_header_name: core::default::Default::default(),
            minimum_ring_size: core::default::Default::default(),
            http_cookie: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_ConsistentHashRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_ConsistentHashRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_ConsistentHashRef {
        ComputeBackendService_ConsistentHashRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_ConsistentHashRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `http_header_name` after provisioning.\nThe hash based on the value of the specified header field.\nThis field is applicable if the sessionAffinity is set to HEADER_FIELD."]
    pub fn http_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_header_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_ring_size` after provisioning.\nThe minimum number of virtual nodes to use for the hash ring.\nLarger ring sizes result in more granular load\ndistributions. If the number of hosts in the load balancing pool\nis larger than the ring size, each host will be assigned a single\nvirtual node.\nDefaults to 1024."]
    pub fn minimum_ring_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_ring_size", self.base))
    }

    #[doc = "Get a reference to the value of field `http_cookie` after provisioning.\n"]
    pub fn http_cookie(&self) -> terrars::ListRef<ComputeBackendService_ConsistentHash_HttpCookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_cookie", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_CustomMetrics {
    dry_run: terrars::PrimField<bool>,
    name: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_CustomMetrics { }

impl terrars::ToListMappable for ComputeBackendService_CustomMetrics {
    type O = terrars::BlockAssignable<ComputeBackendService_CustomMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_CustomMetrics {
    #[doc = "If true, the metric data is not used for load balancing."]
    pub dry_run: terrars::PrimField<bool>,
    #[doc =
        "Name of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_CustomMetrics {
    pub fn build(self) -> ComputeBackendService_CustomMetrics {
        ComputeBackendService_CustomMetrics {
            dry_run: self.dry_run,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_CustomMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_CustomMetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_CustomMetricsRef {
        ComputeBackendService_CustomMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_CustomMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dry_run` after provisioning.\nIf true, the metric data is not used for load balancing."]
    pub fn dry_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_Iap {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_id_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeBackendService_Iap {
    #[doc = "Set the field `oauth2_client_id`.\nOAuth2 Client ID for IAP"]
    pub fn set_oauth2_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oauth2_client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_id_wo`.\nOAuth2 Client ID for IAP"]
    pub fn set_oauth2_client_id_wo(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `oauth2_client_id_wo_version`.\nTriggers update of 'oauth2_client_id_wo' write-only. Increment this value when an update to 'oauth2_client_id_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_oauth2_client_id_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_id_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_secret`.\nOAuth2 Client Secret for IAP"]
    pub fn set_oauth2_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_client_secret_wo`.\nOAuth2 Client Secret for IAP"]
    pub fn set_oauth2_client_secret_wo(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `oauth2_client_secret_wo_version`.\nTriggers update of 'oauth2_client_secret_wo' write-only. Increment this value when an update to 'oauth2_client_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_oauth2_client_secret_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_Iap {
    type O = terrars::BlockAssignable<ComputeBackendService_Iap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_Iap {
    #[doc = "Whether the serving infrastructure will authenticate and authorize all incoming requests."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComputeBackendService_Iap {
    pub fn build(self) -> ComputeBackendService_Iap {
        ComputeBackendService_Iap {
            enabled: self.enabled,
            oauth2_client_id: core::default::Default::default(),
            oauth2_client_id_wo: core::default::Default::default(),
            oauth2_client_id_wo_version: core::default::Default::default(),
            oauth2_client_secret: core::default::Default::default(),
            oauth2_client_secret_wo: core::default::Default::default(),
            oauth2_client_secret_wo_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_IapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_IapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_IapRef {
        ComputeBackendService_IapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_IapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the serving infrastructure will authenticate and authorize all incoming requests."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_id` after provisioning.\nOAuth2 Client ID for IAP"]
    pub fn oauth2_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_id_wo` after provisioning.\nOAuth2 Client ID for IAP"]
    pub fn oauth2_client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_id_wo_version` after provisioning.\nTriggers update of 'oauth2_client_id_wo' write-only. Increment this value when an update to 'oauth2_client_id_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn oauth2_client_id_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id_wo_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret` after provisioning.\nOAuth2 Client Secret for IAP"]
    pub fn oauth2_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret_sha256` after provisioning.\nOAuth2 Client Secret SHA-256 for IAP"]
    pub fn oauth2_client_secret_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret_wo` after provisioning.\nOAuth2 Client Secret for IAP"]
    pub fn oauth2_client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret_wo_version` after provisioning.\nTriggers update of 'oauth2_client_secret_wo' write-only. Increment this value when an update to 'oauth2_client_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn oauth2_client_secret_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_wo_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_LocalityLbPolicies_CustomPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_LocalityLbPolicies_CustomPolicy {
    #[doc =
        "Set the field `data`.\nAn optional, arbitrary JSON object with configuration data, understood\nby a locally installed custom policy implementation."]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_LocalityLbPolicies_CustomPolicy {
    type O = terrars::BlockAssignable<ComputeBackendService_LocalityLbPolicies_CustomPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_LocalityLbPolicies_CustomPolicy {
    #[doc =
        "Identifies the custom policy.\n\nThe value should match the type the custom implementation is registered\nwith on the gRPC clients. It should follow protocol buffer\nmessage naming conventions and include the full path (e.g.\nmyorg.CustomLbPolicy). The maximum length is 256 characters.\n\nNote that specifying the same custom policy more than once for a\nbackend is not a valid configuration and will be rejected."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_LocalityLbPolicies_CustomPolicy {
    pub fn build(self) -> ComputeBackendService_LocalityLbPolicies_CustomPolicy {
        ComputeBackendService_LocalityLbPolicies_CustomPolicy {
            data: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
        ComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_LocalityLbPolicies_CustomPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nAn optional, arbitrary JSON object with configuration data, understood\nby a locally installed custom policy implementation."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifies the custom policy.\n\nThe value should match the type the custom implementation is registered\nwith on the gRPC clients. It should follow protocol buffer\nmessage naming conventions and include the full path (e.g.\nmyorg.CustomLbPolicy). The maximum length is 256 characters.\n\nNote that specifying the same custom policy more than once for a\nbackend is not a valid configuration and will be rejected."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_LocalityLbPolicies_Policy {
    name: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_LocalityLbPolicies_Policy { }

impl terrars::ToListMappable for ComputeBackendService_LocalityLbPolicies_Policy {
    type O = terrars::BlockAssignable<ComputeBackendService_LocalityLbPolicies_Policy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_LocalityLbPolicies_Policy {
    #[doc =
        "The name of a locality load balancer policy to be used. The value\nshould be one of the predefined ones as supported by localityLbPolicy,\nalthough at the moment only ROUND_ROBIN is supported.\n\nThis field should only be populated when the customPolicy field is not\nused.\n\nNote that specifying the same policy more than once for a backend is\nnot a valid configuration and will be rejected.\n\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                  hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n              hashing to backends. The algorithm has the property that the\n              addition/removal of a host from a set of N hosts only affects\n              1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824 Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\"]"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_LocalityLbPolicies_Policy {
    pub fn build(self) -> ComputeBackendService_LocalityLbPolicies_Policy {
        ComputeBackendService_LocalityLbPolicies_Policy { name: self.name }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_LocalityLbPolicies_PolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_LocalityLbPolicies_PolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_LocalityLbPolicies_PolicyRef {
        ComputeBackendService_LocalityLbPolicies_PolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_LocalityLbPolicies_PolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of a locality load balancer policy to be used. The value\nshould be one of the predefined ones as supported by localityLbPolicy,\nalthough at the moment only ROUND_ROBIN is supported.\n\nThis field should only be populated when the customPolicy field is not\nused.\n\nNote that specifying the same policy more than once for a backend is\nnot a valid configuration and will be rejected.\n\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                  hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n              hashing to backends. The algorithm has the property that the\n              addition/removal of a host from a set of N hosts only affects\n              1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824 Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\"]"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_LocalityLbPoliciesDynamic {
    custom_policy: std::option::Option<terrars::DynamicBlock<ComputeBackendService_LocalityLbPolicies_CustomPolicy>>,
    policy: std::option::Option<terrars::DynamicBlock<ComputeBackendService_LocalityLbPolicies_Policy>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_LocalityLbPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_policy: std::option::Option<std::vec::Vec<ComputeBackendService_LocalityLbPolicies_CustomPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<std::vec::Vec<ComputeBackendService_LocalityLbPolicies_Policy>>,
    dynamic: ComputeBackendService_LocalityLbPoliciesDynamic,
}

impl ComputeBackendService_LocalityLbPolicies {
    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_LocalityLbPolicies_CustomPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_LocalityLbPolicies_Policy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_LocalityLbPolicies {
    type O = terrars::BlockAssignable<ComputeBackendService_LocalityLbPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_LocalityLbPolicies {}

impl BuildComputeBackendService_LocalityLbPolicies {
    pub fn build(self) -> ComputeBackendService_LocalityLbPolicies {
        ComputeBackendService_LocalityLbPolicies {
            custom_policy: core::default::Default::default(),
            policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_LocalityLbPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_LocalityLbPoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_LocalityLbPoliciesRef {
        ComputeBackendService_LocalityLbPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_LocalityLbPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(&self) -> terrars::ListRef<ComputeBackendService_LocalityLbPolicies_CustomPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::ListRef<ComputeBackendService_LocalityLbPolicies_PolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_LogConfig_RequestHeaders {
    header_name: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_LogConfig_RequestHeaders { }

impl terrars::ToListMappable for ComputeBackendService_LogConfig_RequestHeaders {
    type O = terrars::BlockAssignable<ComputeBackendService_LogConfig_RequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_LogConfig_RequestHeaders {
    #[doc = "The header name to match on for logging."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_LogConfig_RequestHeaders {
    pub fn build(self) -> ComputeBackendService_LogConfig_RequestHeaders {
        ComputeBackendService_LogConfig_RequestHeaders { header_name: self.header_name }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_LogConfig_RequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_LogConfig_RequestHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_LogConfig_RequestHeadersRef {
        ComputeBackendService_LogConfig_RequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_LogConfig_RequestHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `header_name` after provisioning.\nThe header name to match on for logging."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_LogConfig_ResponseHeaders {
    header_name: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_LogConfig_ResponseHeaders { }

impl terrars::ToListMappable for ComputeBackendService_LogConfig_ResponseHeaders {
    type O = terrars::BlockAssignable<ComputeBackendService_LogConfig_ResponseHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_LogConfig_ResponseHeaders {
    #[doc = "The header name to match on for logging."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_LogConfig_ResponseHeaders {
    pub fn build(self) -> ComputeBackendService_LogConfig_ResponseHeaders {
        ComputeBackendService_LogConfig_ResponseHeaders { header_name: self.header_name }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_LogConfig_ResponseHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_LogConfig_ResponseHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_LogConfig_ResponseHeadersRef {
        ComputeBackendService_LogConfig_ResponseHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_LogConfig_ResponseHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `header_name` after provisioning.\nThe header name to match on for logging."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_LogConfigDynamic {
    request_headers: std::option::Option<terrars::DynamicBlock<ComputeBackendService_LogConfig_RequestHeaders>>,
    response_headers: std::option::Option<terrars::DynamicBlock<ComputeBackendService_LogConfig_ResponseHeaders>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_LogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers: std::option::Option<std::vec::Vec<ComputeBackendService_LogConfig_RequestHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers: std::option::Option<std::vec::Vec<ComputeBackendService_LogConfig_ResponseHeaders>>,
    dynamic: ComputeBackendService_LogConfigDynamic,
}

impl ComputeBackendService_LogConfig {
    #[doc =
        "Set the field `enable`.\nWhether to enable logging for the load balancer traffic served by this backend service."]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `optional_fields`.\nThis field can only be specified if logging is enabled for this backend service and \"logConfig.optionalMode\"\nwas set to CUSTOM. Contains a list of optional fields you want to include in the logs.\nFor example: serverInstance, serverGkeDetails.cluster, serverGkeDetails.pod.podNamespace\nFor example: orca_load_report, tls.protocol"]
    pub fn set_optional_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.optional_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `optional_mode`.\nSpecifies the optional logging mode for the load balancer traffic.\nSupported values: INCLUDE_ALL_OPTIONAL, EXCLUDE_ALL_OPTIONAL, CUSTOM. Possible values: [\"INCLUDE_ALL_OPTIONAL\", \"EXCLUDE_ALL_OPTIONAL\", \"CUSTOM\"]"]
    pub fn set_optional_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.optional_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sample_rate`.\nThis field can only be specified if logging is enabled for this backend service. The value of\nthe field must be in [0, 1]. This configures the sampling rate of requests to the load balancer\nwhere 1.0 means all logged requests are reported and 0.0 means no logged requests are reported.\nThe default value is 1.0."]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers`.\n"]
    pub fn set_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_LogConfig_RequestHeaders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_headers`.\n"]
    pub fn set_response_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_LogConfig_ResponseHeaders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_LogConfig {
    type O = terrars::BlockAssignable<ComputeBackendService_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_LogConfig {}

impl BuildComputeBackendService_LogConfig {
    pub fn build(self) -> ComputeBackendService_LogConfig {
        ComputeBackendService_LogConfig {
            enable: core::default::Default::default(),
            optional_fields: core::default::Default::default(),
            optional_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            response_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_LogConfigRef {
        ComputeBackendService_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_LogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nWhether to enable logging for the load balancer traffic served by this backend service."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `optional_fields` after provisioning.\nThis field can only be specified if logging is enabled for this backend service and \"logConfig.optionalMode\"\nwas set to CUSTOM. Contains a list of optional fields you want to include in the logs.\nFor example: serverInstance, serverGkeDetails.cluster, serverGkeDetails.pod.podNamespace\nFor example: orca_load_report, tls.protocol"]
    pub fn optional_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optional_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `optional_mode` after provisioning.\nSpecifies the optional logging mode for the load balancer traffic.\nSupported values: INCLUDE_ALL_OPTIONAL, EXCLUDE_ALL_OPTIONAL, CUSTOM. Possible values: [\"INCLUDE_ALL_OPTIONAL\", \"EXCLUDE_ALL_OPTIONAL\", \"CUSTOM\"]"]
    pub fn optional_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optional_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_rate` after provisioning.\nThis field can only be specified if logging is enabled for this backend service. The value of\nthe field must be in [0, 1]. This configures the sampling rate of requests to the load balancer\nwhere 1.0 means all logged requests are reported and 0.0 means no logged requests are reported.\nThe default value is 1.0."]
    pub fn sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `request_headers` after provisioning.\n"]
    pub fn request_headers(&self) -> terrars::ListRef<ComputeBackendService_LogConfig_RequestHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers` after provisioning.\n"]
    pub fn response_headers(&self) -> terrars::ListRef<ComputeBackendService_LogConfig_ResponseHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_MaxStreamDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeBackendService_MaxStreamDuration {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution.\nDurations less than one second are represented with a 0 seconds field and a positive nanos field.\nMust be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_MaxStreamDuration {
    type O = terrars::BlockAssignable<ComputeBackendService_MaxStreamDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_MaxStreamDuration {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. (int64 format)"]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendService_MaxStreamDuration {
    pub fn build(self) -> ComputeBackendService_MaxStreamDuration {
        ComputeBackendService_MaxStreamDuration {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_MaxStreamDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_MaxStreamDurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_MaxStreamDurationRef {
        ComputeBackendService_MaxStreamDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_MaxStreamDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution.\nDurations less than one second are represented with a 0 seconds field and a positive nanos field.\nMust be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. (int64 format)"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_OutlierDetection_BaseEjectionTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<f64>,
}

impl ComputeBackendService_OutlierDetection_BaseEjectionTime {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_OutlierDetection_BaseEjectionTime {
    type O = terrars::BlockAssignable<ComputeBackendService_OutlierDetection_BaseEjectionTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_OutlierDetection_BaseEjectionTime {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildComputeBackendService_OutlierDetection_BaseEjectionTime {
    pub fn build(self) -> ComputeBackendService_OutlierDetection_BaseEjectionTime {
        ComputeBackendService_OutlierDetection_BaseEjectionTime {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
        ComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_OutlierDetection_BaseEjectionTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_OutlierDetection_Interval {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<f64>,
}

impl ComputeBackendService_OutlierDetection_Interval {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_OutlierDetection_Interval {
    type O = terrars::BlockAssignable<ComputeBackendService_OutlierDetection_Interval>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_OutlierDetection_Interval {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildComputeBackendService_OutlierDetection_Interval {
    pub fn build(self) -> ComputeBackendService_OutlierDetection_Interval {
        ComputeBackendService_OutlierDetection_Interval {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_OutlierDetection_IntervalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_OutlierDetection_IntervalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_OutlierDetection_IntervalRef {
        ComputeBackendService_OutlierDetection_IntervalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_OutlierDetection_IntervalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_OutlierDetectionDynamic {
    base_ejection_time: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeBackendService_OutlierDetection_BaseEjectionTime>>,
    interval: std::option::Option<terrars::DynamicBlock<ComputeBackendService_OutlierDetection_Interval>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_OutlierDetection {
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
    max_ejection_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_rate_minimum_hosts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_rate_request_volume: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_rate_stdev_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_ejection_time: std::option::Option<std::vec::Vec<ComputeBackendService_OutlierDetection_BaseEjectionTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<std::vec::Vec<ComputeBackendService_OutlierDetection_Interval>>,
    dynamic: ComputeBackendService_OutlierDetectionDynamic,
}

impl ComputeBackendService_OutlierDetection {
    #[doc =
        "Set the field `consecutive_errors`.\nNumber of errors before a host is ejected from the connection pool. When the\nbackend host is accessed over HTTP, a 5xx return code qualifies as an error.\nDefaults to 5."]
    pub fn set_consecutive_errors(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.consecutive_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consecutive_gateway_failure`.\nThe number of consecutive gateway failures (502, 503, 504 status or connection\nerrors that are mapped to one of those status codes) before a consecutive\ngateway failure ejection occurs. Defaults to 5."]
    pub fn set_consecutive_gateway_failure(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.consecutive_gateway_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforcing_consecutive_errors`.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive 5xx. This setting can be used to disable\nejection or to ramp it up slowly. Defaults to 100."]
    pub fn set_enforcing_consecutive_errors(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.enforcing_consecutive_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforcing_consecutive_gateway_failure`.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive gateway failures. This setting can be\nused to disable ejection or to ramp it up slowly. Defaults to 0."]
    pub fn set_enforcing_consecutive_gateway_failure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.enforcing_consecutive_gateway_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforcing_success_rate`.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through success rate statistics. This setting can be used to\ndisable ejection or to ramp it up slowly. Defaults to 100."]
    pub fn set_enforcing_success_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.enforcing_success_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_ejection_percent`.\nMaximum percentage of hosts in the load balancing pool for the backend service\nthat can be ejected. Defaults to 10%."]
    pub fn set_max_ejection_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_ejection_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_rate_minimum_hosts`.\nThe number of hosts in a cluster that must have enough request volume to detect\nsuccess rate outliers. If the number of hosts is less than this setting, outlier\ndetection via success rate statistics is not performed for any host in the\ncluster. Defaults to 5."]
    pub fn set_success_rate_minimum_hosts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_rate_minimum_hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_rate_request_volume`.\nThe minimum number of total requests that must be collected in one interval (as\ndefined by the interval duration above) to include this host in success rate\nbased outlier detection. If the volume is lower than this setting, outlier\ndetection via success rate statistics is not performed for that host. Defaults\nto 100."]
    pub fn set_success_rate_request_volume(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_rate_request_volume = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_rate_stdev_factor`.\nThis factor is used to determine the ejection threshold for success rate outlier\nejection. The ejection threshold is the difference between the mean success\nrate, and the product of this factor and the standard deviation of the mean\nsuccess rate: mean - (stdev * success_rate_stdev_factor). This factor is divided\nby a thousand to get a double. That is, if the desired factor is 1.9, the\nruntime value should be 1900. Defaults to 1900."]
    pub fn set_success_rate_stdev_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_rate_stdev_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `base_ejection_time`.\n"]
    pub fn set_base_ejection_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeBackendService_OutlierDetection_BaseEjectionTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.base_ejection_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.base_ejection_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_OutlierDetection_Interval>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.interval = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.interval = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_OutlierDetection {
    type O = terrars::BlockAssignable<ComputeBackendService_OutlierDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_OutlierDetection {}

impl BuildComputeBackendService_OutlierDetection {
    pub fn build(self) -> ComputeBackendService_OutlierDetection {
        ComputeBackendService_OutlierDetection {
            consecutive_errors: core::default::Default::default(),
            consecutive_gateway_failure: core::default::Default::default(),
            enforcing_consecutive_errors: core::default::Default::default(),
            enforcing_consecutive_gateway_failure: core::default::Default::default(),
            enforcing_success_rate: core::default::Default::default(),
            max_ejection_percent: core::default::Default::default(),
            success_rate_minimum_hosts: core::default::Default::default(),
            success_rate_request_volume: core::default::Default::default(),
            success_rate_stdev_factor: core::default::Default::default(),
            base_ejection_time: core::default::Default::default(),
            interval: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_OutlierDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_OutlierDetectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_OutlierDetectionRef {
        ComputeBackendService_OutlierDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_OutlierDetectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consecutive_errors` after provisioning.\nNumber of errors before a host is ejected from the connection pool. When the\nbackend host is accessed over HTTP, a 5xx return code qualifies as an error.\nDefaults to 5."]
    pub fn consecutive_errors(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_errors", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_gateway_failure` after provisioning.\nThe number of consecutive gateway failures (502, 503, 504 status or connection\nerrors that are mapped to one of those status codes) before a consecutive\ngateway failure ejection occurs. Defaults to 5."]
    pub fn consecutive_gateway_failure(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_gateway_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforcing_consecutive_errors` after provisioning.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive 5xx. This setting can be used to disable\nejection or to ramp it up slowly. Defaults to 100."]
    pub fn enforcing_consecutive_errors(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcing_consecutive_errors", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforcing_consecutive_gateway_failure` after provisioning.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive gateway failures. This setting can be\nused to disable ejection or to ramp it up slowly. Defaults to 0."]
    pub fn enforcing_consecutive_gateway_failure(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcing_consecutive_gateway_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforcing_success_rate` after provisioning.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through success rate statistics. This setting can be used to\ndisable ejection or to ramp it up slowly. Defaults to 100."]
    pub fn enforcing_success_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcing_success_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_ejection_percent` after provisioning.\nMaximum percentage of hosts in the load balancing pool for the backend service\nthat can be ejected. Defaults to 10%."]
    pub fn max_ejection_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ejection_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_rate_minimum_hosts` after provisioning.\nThe number of hosts in a cluster that must have enough request volume to detect\nsuccess rate outliers. If the number of hosts is less than this setting, outlier\ndetection via success rate statistics is not performed for any host in the\ncluster. Defaults to 5."]
    pub fn success_rate_minimum_hosts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_rate_minimum_hosts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_rate_request_volume` after provisioning.\nThe minimum number of total requests that must be collected in one interval (as\ndefined by the interval duration above) to include this host in success rate\nbased outlier detection. If the volume is lower than this setting, outlier\ndetection via success rate statistics is not performed for that host. Defaults\nto 100."]
    pub fn success_rate_request_volume(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_rate_request_volume", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_rate_stdev_factor` after provisioning.\nThis factor is used to determine the ejection threshold for success rate outlier\nejection. The ejection threshold is the difference between the mean success\nrate, and the product of this factor and the standard deviation of the mean\nsuccess rate: mean - (stdev * success_rate_stdev_factor). This factor is divided\nby a thousand to get a double. That is, if the desired factor is 1.9, the\nruntime value should be 1900. Defaults to 1900."]
    pub fn success_rate_stdev_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_rate_stdev_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `base_ejection_time` after provisioning.\n"]
    pub fn base_ejection_time(&self) -> terrars::ListRef<ComputeBackendService_OutlierDetection_BaseEjectionTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_ejection_time", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::ListRef<ComputeBackendService_OutlierDetection_IntervalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeBackendService_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the backend service. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_Params {
    type O = terrars::BlockAssignable<ComputeBackendService_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_Params {}

impl BuildComputeBackendService_Params {
    pub fn build(self) -> ComputeBackendService_Params {
        ComputeBackendService_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_ParamsRef {
        ComputeBackendService_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the backend service. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_SecuritySettings_AwsV4Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeBackendService_SecuritySettings_AwsV4Authentication {
    #[doc =
        "Set the field `access_key`.\nThe access key used for s3 bucket authentication.\nRequired for updating or creating a backend that uses AWS v4 signature authentication, but will not be returned as part of the configuration when queried with a REST API GET request."]
    pub fn set_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_key_id`.\nThe identifier of an access key used for s3 bucket authentication."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `access_key_version`.\nThe optional version identifier for the access key. You can use this to keep track of different iterations of your access key."]
    pub fn set_access_key_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.access_key_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_region`.\nThe name of the cloud region of your origin. This is a free-form field with the name of the region your cloud uses to host your origin.\nFor example, \"us-east-1\" for AWS or \"us-ashburn-1\" for OCI."]
    pub fn set_origin_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.origin_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_SecuritySettings_AwsV4Authentication {
    type O = terrars::BlockAssignable<ComputeBackendService_SecuritySettings_AwsV4Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_SecuritySettings_AwsV4Authentication {}

impl BuildComputeBackendService_SecuritySettings_AwsV4Authentication {
    pub fn build(self) -> ComputeBackendService_SecuritySettings_AwsV4Authentication {
        ComputeBackendService_SecuritySettings_AwsV4Authentication {
            access_key: core::default::Default::default(),
            access_key_id: core::default::Default::default(),
            access_key_version: core::default::Default::default(),
            origin_region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
        ComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_SecuritySettings_AwsV4AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key` after provisioning.\nThe access key used for s3 bucket authentication.\nRequired for updating or creating a backend that uses AWS v4 signature authentication, but will not be returned as part of the configuration when queried with a REST API GET request."]
    pub fn access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nThe identifier of an access key used for s3 bucket authentication."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `access_key_version` after provisioning.\nThe optional version identifier for the access key. You can use this to keep track of different iterations of your access key."]
    pub fn access_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_region` after provisioning.\nThe name of the cloud region of your origin. This is a free-form field with the name of the region your cloud uses to host your origin.\nFor example, \"us-east-1\" for AWS or \"us-ashburn-1\" for OCI."]
    pub fn origin_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_region", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_SecuritySettingsDynamic {
    aws_v4_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeBackendService_SecuritySettings_AwsV4Authentication>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_SecuritySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alt_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_v4_authentication: std::option::Option<std::vec::Vec<ComputeBackendService_SecuritySettings_AwsV4Authentication>>,
    dynamic: ComputeBackendService_SecuritySettingsDynamic,
}

impl ComputeBackendService_SecuritySettings {
    #[doc =
        "Set the field `client_tls_policy`.\nClientTlsPolicy is a resource that specifies how a client should authenticate\nconnections to backends of a service. This resource itself does not affect\nconfiguration unless it is attached to a backend service resource."]
    pub fn set_client_tls_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_tls_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subject_alt_names`.\nA list of alternate names to verify the subject identity in the certificate.\nIf specified, the client will verify that the server certificate's subject\nalt name matches one of the specified values."]
    pub fn set_subject_alt_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subject_alt_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_v4_authentication`.\n"]
    pub fn set_aws_v4_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeBackendService_SecuritySettings_AwsV4Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_v4_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_v4_authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_SecuritySettings {
    type O = terrars::BlockAssignable<ComputeBackendService_SecuritySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_SecuritySettings {}

impl BuildComputeBackendService_SecuritySettings {
    pub fn build(self) -> ComputeBackendService_SecuritySettings {
        ComputeBackendService_SecuritySettings {
            client_tls_policy: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
            aws_v4_authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_SecuritySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_SecuritySettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_SecuritySettingsRef {
        ComputeBackendService_SecuritySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_SecuritySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_tls_policy` after provisioning.\nClientTlsPolicy is a resource that specifies how a client should authenticate\nconnections to backends of a service. This resource itself does not affect\nconfiguration unless it is attached to a backend service resource."]
    pub fn client_tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_tls_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject_alt_names` after provisioning.\nA list of alternate names to verify the subject identity in the certificate.\nIf specified, the client will verify that the server certificate's subject\nalt name matches one of the specified values."]
    pub fn subject_alt_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alt_names", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(
        &self,
    ) -> terrars::ListRef<ComputeBackendService_SecuritySettings_AwsV4AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_v4_authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_StrongSessionAffinityCookieDynamic {
    ttl: std::option::Option<terrars::DynamicBlock<ComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_StrongSessionAffinityCookie {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<std::vec::Vec<ComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
    dynamic: ComputeBackendService_StrongSessionAffinityCookieDynamic,
}

impl ComputeBackendService_StrongSessionAffinityCookie {
    #[doc = "Set the field `name`.\nName of the cookie."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nPath to set for the cookie."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_ConsistentHash_HttpCookie_Ttl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ttl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_StrongSessionAffinityCookie {
    type O = terrars::BlockAssignable<ComputeBackendService_StrongSessionAffinityCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_StrongSessionAffinityCookie {}

impl BuildComputeBackendService_StrongSessionAffinityCookie {
    pub fn build(self) -> ComputeBackendService_StrongSessionAffinityCookie {
        ComputeBackendService_StrongSessionAffinityCookie {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_StrongSessionAffinityCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_StrongSessionAffinityCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_StrongSessionAffinityCookieRef {
        ComputeBackendService_StrongSessionAffinityCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_StrongSessionAffinityCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the cookie."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to set for the cookie."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<ComputeBackendService_ConsistentHash_HttpCookie_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeBackendService_Timeouts {
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

impl terrars::ToListMappable for ComputeBackendService_Timeouts {
    type O = terrars::BlockAssignable<ComputeBackendService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_Timeouts {}

impl BuildComputeBackendService_Timeouts {
    pub fn build(self) -> ComputeBackendService_Timeouts {
        ComputeBackendService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_TimeoutsRef {
        ComputeBackendService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_TimeoutsRef {
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
pub struct ComputeBackendService_TlsSettings_SubjectAltNames {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uniform_resource_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeBackendService_TlsSettings_SubjectAltNames {
    #[doc = "Set the field `dns_name`.\nThe SAN specified as a DNS Name."]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uniform_resource_identifier`.\nThe SAN specified as a URI."]
    pub fn set_uniform_resource_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.uniform_resource_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_TlsSettings_SubjectAltNames {
    type O = terrars::BlockAssignable<ComputeBackendService_TlsSettings_SubjectAltNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_TlsSettings_SubjectAltNames {}

impl BuildComputeBackendService_TlsSettings_SubjectAltNames {
    pub fn build(self) -> ComputeBackendService_TlsSettings_SubjectAltNames {
        ComputeBackendService_TlsSettings_SubjectAltNames {
            dns_name: core::default::Default::default(),
            uniform_resource_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_TlsSettings_SubjectAltNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_TlsSettings_SubjectAltNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendService_TlsSettings_SubjectAltNamesRef {
        ComputeBackendService_TlsSettings_SubjectAltNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_TlsSettings_SubjectAltNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe SAN specified as a DNS Name."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uniform_resource_identifier` after provisioning.\nThe SAN specified as a URI."]
    pub fn uniform_resource_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uniform_resource_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendService_TlsSettingsDynamic {
    subject_alt_names: std::option::Option<terrars::DynamicBlock<ComputeBackendService_TlsSettings_SubjectAltNames>>,
}

#[derive(Serialize)]
pub struct ComputeBackendService_TlsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sni: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alt_names: std::option::Option<std::vec::Vec<ComputeBackendService_TlsSettings_SubjectAltNames>>,
    dynamic: ComputeBackendService_TlsSettingsDynamic,
}

impl ComputeBackendService_TlsSettings {
    #[doc =
        "Set the field `authentication_config`.\nReference to the BackendAuthenticationConfig resource from the networksecurity.googleapis.com namespace.\nCan be used in authenticating TLS connections to the backend, as specified by the authenticationMode field.\nCan only be specified if authenticationMode is not NONE."]
    pub fn set_authentication_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity`.\nThe fully-specified SPIFFE ID without the spiffe:// scheme. Must be in the format //<trust_domain>/ns/<namespace>/sa/<subject>.\nThe load balancer uses certificates and roots of trust provisioned by the Managed Workload Identity system for this identity.\nThe Trust Domain within the identity must refer to a valid Workload Identity Pool, from which the TrustConfig and CertificateIssuanceConfig are inherited.\nIf set, you cannot configure sni, subjectAltNames, or authenticationConfig manually."]
    pub fn set_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sni`.\nServer Name Indication - see RFC3546 section 3.1. If set, the load balancer sends this string as the SNI hostname in the\nTLS connection to the backend, and requires that this string match a Subject Alternative Name (SAN) in the backend's\nserver certificate. With a Regional Internet NEG backend, if the SNI is specified here, the load balancer uses it\nregardless of whether the Regional Internet NEG is specified with FQDN or IP address and port."]
    pub fn set_sni(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sni = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alt_names`.\n"]
    pub fn set_subject_alt_names(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendService_TlsSettings_SubjectAltNames>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subject_alt_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alt_names = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendService_TlsSettings {
    type O = terrars::BlockAssignable<ComputeBackendService_TlsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendService_TlsSettings {}

impl BuildComputeBackendService_TlsSettings {
    pub fn build(self) -> ComputeBackendService_TlsSettings {
        ComputeBackendService_TlsSettings {
            authentication_config: core::default::Default::default(),
            identity: core::default::Default::default(),
            sni: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendService_TlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendService_TlsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendService_TlsSettingsRef {
        ComputeBackendService_TlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendService_TlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_config` after provisioning.\nReference to the BackendAuthenticationConfig resource from the networksecurity.googleapis.com namespace.\nCan be used in authenticating TLS connections to the backend, as specified by the authenticationMode field.\nCan only be specified if authenticationMode is not NONE."]
    pub fn authentication_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nThe fully-specified SPIFFE ID without the spiffe:// scheme. Must be in the format //<trust_domain>/ns/<namespace>/sa/<subject>.\nThe load balancer uses certificates and roots of trust provisioned by the Managed Workload Identity system for this identity.\nThe Trust Domain within the identity must refer to a valid Workload Identity Pool, from which the TrustConfig and CertificateIssuanceConfig are inherited.\nIf set, you cannot configure sni, subjectAltNames, or authenticationConfig manually."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sni` after provisioning.\nServer Name Indication - see RFC3546 section 3.1. If set, the load balancer sends this string as the SNI hostname in the\nTLS connection to the backend, and requires that this string match a Subject Alternative Name (SAN) in the backend's\nserver certificate. With a Regional Internet NEG backend, if the SNI is specified here, the load balancer uses it\nregardless of whether the Regional Internet NEG is specified with FQDN or IP address and port."]
    pub fn sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_alt_names` after provisioning.\n"]
    pub fn subject_alt_names(&self) -> terrars::ListRef<ComputeBackendService_TlsSettings_SubjectAltNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alt_names", self.base))
    }
}
