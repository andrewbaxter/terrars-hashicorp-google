use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesEdgeCacheServiceData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_http2: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_quic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_ssl_certificates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_tls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceLogConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesEdgeCacheServiceTimeoutsEl>,
    dynamic: NetworkServicesEdgeCacheServiceDynamic,
}

struct NetworkServicesEdgeCacheService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesEdgeCacheServiceData>,
}

#[derive(Clone)]
pub struct NetworkServicesEdgeCacheService(std::rc::Rc<NetworkServicesEdgeCacheService_>);

impl NetworkServicesEdgeCacheService {
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

    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_http2`.\nDisables HTTP/2.\n\nHTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use and reduces connection setup overhead by sending multiple streams over the same connection.\n\nSome legacy HTTP clients may have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from being advertised and negotiated."]
    pub fn set_disable_http2(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_http2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_quic`.\nHTTP/3 (IETF QUIC) and Google QUIC are enabled by default."]
    pub fn set_disable_quic(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_quic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `edge_security_policy`.\nResource URL that points at the Cloud Armor edge security policy that is applied on each request against the EdgeCacheService."]
    pub fn set_edge_security_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().edge_security_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `edge_ssl_certificates`.\nURLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService.\n\nNote that only \"global\" certificates with a \"scope\" of \"EDGE_CACHE\" can be attached to an EdgeCacheService."]
    pub fn set_edge_ssl_certificates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().edge_ssl_certificates = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_tls`.\nRequire TLS (HTTPS) for all clients connecting to this service.\n\nClients who connect over HTTP (port 80) will receive a HTTP 301 to the same URL over HTTPS (port 443).\nYou must have at least one (1) edgeSslCertificate specified to enable this."]
    pub fn set_require_tls(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_tls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_policy`.\nURL of the SslPolicy resource that will be associated with the EdgeCacheService.\n\nIf not set, the EdgeCacheService has no SSL policy configured, and will default to the \"COMPATIBLE\" policy."]
    pub fn set_ssl_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheServiceLogConfigEl>>,
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

    #[doc = "Set the field `routing`.\n"]
    pub fn set_routing(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesEdgeCacheServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_http2` after provisioning.\nDisables HTTP/2.\n\nHTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use and reduces connection setup overhead by sending multiple streams over the same connection.\n\nSome legacy HTTP clients may have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from being advertised and negotiated."]
    pub fn disable_http2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_http2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_quic` after provisioning.\nHTTP/3 (IETF QUIC) and Google QUIC are enabled by default."]
    pub fn disable_quic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_quic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_security_policy` after provisioning.\nResource URL that points at the Cloud Armor edge security policy that is applied on each request against the EdgeCacheService."]
    pub fn edge_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_ssl_certificates` after provisioning.\nURLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService.\n\nNote that only \"global\" certificates with a \"scope\" of \"EDGE_CACHE\" can be attached to an EdgeCacheService."]
    pub fn edge_ssl_certificates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_ssl_certificates", self.extract_ref()))
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
        "Get a reference to the value of field `ipv4_addresses` after provisioning.\nThe IPv4 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_addresses` after provisioning.\nThe IPv6 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv6_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_tls` after provisioning.\nRequire TLS (HTTPS) for all clients connecting to this service.\n\nClients who connect over HTTP (port 80) will receive a HTTP 301 to the same URL over HTTPS (port 443).\nYou must have at least one (1) edgeSslCertificate specified to enable this."]
    pub fn require_tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl_policy` after provisioning.\nURL of the SslPolicy resource that will be associated with the EdgeCacheService.\n\nIf not set, the EdgeCacheService has no SSL policy configured, and will default to the \"COMPATIBLE\" policy."]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<NetworkServicesEdgeCacheServiceLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing` after provisioning.\n"]
    pub fn routing(&self) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheServiceTimeoutsElRef {
        NetworkServicesEdgeCacheServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkServicesEdgeCacheService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesEdgeCacheService { }

impl terrars::ToListMappable for NetworkServicesEdgeCacheService {
    type O = terrars::ListRef<NetworkServicesEdgeCacheServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesEdgeCacheService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_edge_cache_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesEdgeCacheService {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesEdgeCacheService {
        let out = NetworkServicesEdgeCacheService(std::rc::Rc::new(NetworkServicesEdgeCacheService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesEdgeCacheServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_http2: core::default::Default::default(),
                disable_quic: core::default::Default::default(),
                edge_security_policy: core::default::Default::default(),
                edge_ssl_certificates: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                require_tls: core::default::Default::default(),
                ssl_policy: core::default::Default::default(),
                log_config: core::default::Default::default(),
                routing: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesEdgeCacheServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesEdgeCacheServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_http2` after provisioning.\nDisables HTTP/2.\n\nHTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use and reduces connection setup overhead by sending multiple streams over the same connection.\n\nSome legacy HTTP clients may have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from being advertised and negotiated."]
    pub fn disable_http2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_http2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_quic` after provisioning.\nHTTP/3 (IETF QUIC) and Google QUIC are enabled by default."]
    pub fn disable_quic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_quic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_security_policy` after provisioning.\nResource URL that points at the Cloud Armor edge security policy that is applied on each request against the EdgeCacheService."]
    pub fn edge_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_ssl_certificates` after provisioning.\nURLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService.\n\nNote that only \"global\" certificates with a \"scope\" of \"EDGE_CACHE\" can be attached to an EdgeCacheService."]
    pub fn edge_ssl_certificates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_ssl_certificates", self.extract_ref()))
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
        "Get a reference to the value of field `ipv4_addresses` after provisioning.\nThe IPv4 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_addresses` after provisioning.\nThe IPv6 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv6_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_tls` after provisioning.\nRequire TLS (HTTPS) for all clients connecting to this service.\n\nClients who connect over HTTP (port 80) will receive a HTTP 301 to the same URL over HTTPS (port 443).\nYou must have at least one (1) edgeSslCertificate specified to enable this."]
    pub fn require_tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl_policy` after provisioning.\nURL of the SslPolicy resource that will be associated with the EdgeCacheService.\n\nIf not set, the EdgeCacheService has no SSL policy configured, and will default to the \"COMPATIBLE\" policy."]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<NetworkServicesEdgeCacheServiceLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing` after provisioning.\n"]
    pub fn routing(&self) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheServiceTimeoutsElRef {
        NetworkServicesEdgeCacheServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceLogConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesEdgeCacheServiceLogConfigEl {
    #[doc = "Set the field `enable`.\nSpecifies whether to enable logging for traffic served by this service."]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sample_rate`.\nConfigures the sampling rate of requests, where 1.0 means all logged requests are reported and 0.0 means no logged requests are reported. The default value is 1.0, and the value of the field must be in [0, 1].\n\nThis field can only be specified if logging is enabled for this service."]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceLogConfigEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceLogConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceLogConfigEl {}

impl BuildNetworkServicesEdgeCacheServiceLogConfigEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceLogConfigEl {
        NetworkServicesEdgeCacheServiceLogConfigEl {
            enable: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceLogConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceLogConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheServiceLogConfigElRef {
        NetworkServicesEdgeCacheServiceLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceLogConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nSpecifies whether to enable logging for traffic served by this service."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_rate` after provisioning.\nConfigures the sampling rate of requests, where 1.0 means all logged requests are reported and 0.0 means no logged requests are reported. The default value is 1.0, and the value of the field must be in [0, 1].\n\nThis field can only be specified if logging is enabled for this service."]
    pub fn sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hosts: terrars::ListField<terrars::PrimField<std::string::String>>,
    path_matcher: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    #[doc = "Set the field `description`.\nA human-readable description of the hostRule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    #[doc =
        "The list of host patterns to match.\n\nHost patterns must be valid hostnames. Ports are not allowed. Wildcard hosts are supported in the suffix or prefix form. * matches any string of ([a-z0-9-.]*). It does not match the empty string.\n\nWhen multiple hosts are specified, hosts are matched in the following priority:\n\n  1. Exact domain names: ''www.foo.com''.\n  2. Suffix domain wildcards: ''*.foo.com'' or ''*-bar.foo.com''.\n  3. Prefix domain wildcards: ''foo.*'' or ''foo-*''.\n  4. Special wildcard ''*'' matching any domain.\n\n  Notes:\n\n    The wildcard will not match the empty string. e.g. ''*-bar.foo.com'' will match ''baz-bar.foo.com'' but not ''-bar.foo.com''. The longest wildcards match first. Only a single host in the entire service can match on ''*''. A domain must be unique across all configured hosts within a service.\n\n    Hosts are matched against the HTTP Host header, or for HTTP/2 and HTTP/3, the \":authority\" header, from the incoming request.\n\n    You may specify up to 10 hosts."]
    pub hosts: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The name of the pathMatcher associated with this hostRule."]
    pub path_matcher: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
        NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
        NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the hostRule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nThe list of host patterns to match.\n\nHost patterns must be valid hostnames. Ports are not allowed. Wildcard hosts are supported in the suffix or prefix form. * matches any string of ([a-z0-9-.]*). It does not match the empty string.\n\nWhen multiple hosts are specified, hosts are matched in the following priority:\n\n  1. Exact domain names: ''www.foo.com''.\n  2. Suffix domain wildcards: ''*.foo.com'' or ''*-bar.foo.com''.\n  3. Prefix domain wildcards: ''foo.*'' or ''foo-*''.\n  4. Special wildcard ''*'' matching any domain.\n\n  Notes:\n\n    The wildcard will not match the empty string. e.g. ''*-bar.foo.com'' will match ''baz-bar.foo.com'' but not ''-bar.foo.com''. The longest wildcards match first. Only a single host in the entire service can match on ''*''. A domain must be unique across all configured hosts within a service.\n\n    Hosts are matched against the HTTP Host header, or for HTTP/2 and HTTP/3, the \":authority\" header, from the incoming request.\n\n    You may specify up to 10 hosts."]
    pub fn hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_matcher` after provisioning.\nThe name of the pathMatcher associated with this hostRule."]
    pub fn path_matcher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl {
    #[doc = "Set the field `replace`.\nWhether to replace all existing headers with the same name."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl {
    #[doc = "The name of the header to add."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef {
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
        "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl {
    header_name: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl { }

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl {
    #[doc = "The name of the header to remove."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl {
            header_name: self.header_name,
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to remove."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl {
    #[doc = "Set the field `replace`.\nWhether to replace all existing headers with the same name."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl {
    #[doc = "The name of the header to add."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef {
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
        "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl {
    header_name: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl { }

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl {
    #[doc =
        "Headers to remove from the response prior to sending it back to the client.\n\nResponse headers are only sent to the client, and do not have an effect on the cache serving the response."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl {
            header_name: self.header_name,
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `header_name` after provisioning.\nHeaders to remove from the response prior to sending it back to the client.\n\nResponse headers are only sent to the client, and do not have an effect on the cache serving the response."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElDynamic {
    request_header_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl,
        >,
    >,
    request_header_to_remove: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl,
        >,
    >,
    response_header_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl,
        >,
    >,
    response_header_to_remove: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header_to_remove: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header_to_remove: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl>,
    >,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElDynamic,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
    #[doc = "Set the field `request_header_to_add`.\n"]
    pub fn set_request_header_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_header_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header_to_add = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_header_to_remove`.\n"]
    pub fn set_request_header_to_remove(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_header_to_remove = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header_to_remove = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_header_to_add`.\n"]
    pub fn set_response_header_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_header_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header_to_add = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_header_to_remove`.\n"]
    pub fn set_response_header_to_remove(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_header_to_remove = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header_to_remove = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
    type O =
        terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
            request_header_to_add: core::default::Default::default(),
            request_header_to_remove: core::default::Default::default(),
            response_header_to_add: core::default::Default::default(),
            response_header_to_remove: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_header_to_add` after provisioning.\n"]
    pub fn request_header_to_add(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header_to_remove` after provisioning.\n"]
    pub fn request_header_to_remove(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header_to_remove", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header_to_add` after provisioning.\n"]
    pub fn response_header_to_add(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header_to_remove` after provisioning.\n"]
    pub fn response_header_to_remove(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header_to_remove", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    header_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix_match: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
    #[doc = "Set the field `exact_match`.\nThe value of the header should exactly match contents of exactMatch."]
    pub fn set_exact_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_match`.\nIf set to false (default), the headerMatch is considered a match if the match criteria above are met.\nIf set to true, the headerMatch is considered a match if the match criteria above are NOT met."]
    pub fn set_invert_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_match`.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn set_prefix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `present_match`.\nA header with the contents of headerName must exist. The match takes place whether or not the request's header has a value."]
    pub fn set_present_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.present_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix_match`.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn set_suffix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix_match = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
    #[doc = "The header name to match on."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
            exact_match: core::default::Default::default(),
            header_name: self.header_name,
            invert_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            suffix_match: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nThe value of the header should exactly match contents of exactMatch."]
    pub fn exact_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe header name to match on."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_match` after provisioning.\nIf set to false (default), the headerMatch is considered a match if the match criteria above are met.\nIf set to true, the headerMatch is considered a match if the match criteria above are NOT met."]
    pub fn invert_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present_match` after provisioning.\nA header with the contents of headerName must exist. The match takes place whether or not the request's header has a value."]
    pub fn present_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix_match` after provisioning.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn suffix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix_match", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl {
    #[doc =
        "Set the field `exact_match`.\nThe queryParameterMatch matches if the value of the parameter exactly matches the contents of exactMatch."]
    pub fn set_exact_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `present_match`.\nSpecifies that the queryParameterMatch matches if the request contains the query parameter, irrespective of whether the parameter has a value or not."]
    pub fn set_present_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.present_match = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl {
    #[doc =
        "The name of the query parameter to match. The query parameter must exist in the request, in the absence of which the request match fails."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl {
            exact_match: core::default::Default::default(),
            name: self.name,
            present_match: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter exactly matches the contents of exactMatch."]
    pub fn exact_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the query parameter to match. The query parameter must exist in the request, in the absence of which the request match fails."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present_match` after provisioning.\nSpecifies that the queryParameterMatch matches if the request contains the query parameter, irrespective of whether the parameter has a value or not."]
    pub fn present_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElDynamic {
    header_match: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl>,
    >,
    query_parameter_match: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_path_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_template_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_match: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter_match: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl>,
    >,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElDynamic,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
    #[doc =
        "Set the field `full_path_match`.\nFor satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL."]
    pub fn set_full_path_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.full_path_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_case`.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_template_match`.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn set_path_template_match(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_template_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_match`.\nFor satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /."]
    pub fn set_prefix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_match`.\n"]
    pub fn set_header_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_parameter_match`.\n"]
    pub fn set_query_parameter_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_parameter_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            path_template_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            header_match: core::default::Default::default(),
            query_parameter_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `full_path_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL."]
    pub fn full_path_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_path_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_template_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn path_template_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_template_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nFor satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc = "Get a reference to the value of field `header_match` after provisioning.\n"]
    pub fn header_match(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_match", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameter_match` after provisioning.\n"]
    pub fn query_parameter_match(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameter_match", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl {
    actions: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copied_parameters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_query_parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl {
    #[doc =
        "Set the field `copied_parameters`.\nThe parameters to copy from the verified token to the generated token.\n\nOnly the following parameters may be copied:\n\n  * 'PathGlobs'\n  * 'paths'\n  * 'acl'\n  * 'URLPrefix'\n  * 'IPRanges'\n  * 'SessionID'\n  * 'id'\n  * 'Data'\n  * 'data'\n  * 'payload'\n  * 'Headers'\n\nYou may specify up to 6 parameters to copy.  A given parameter is be copied only if the parameter exists in the verified token.  Parameter names are matched exactly as specified.  The order of the parameters does not matter.  Duplicates are not allowed.\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified."]
    pub fn set_copied_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.copied_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keyset`.\nThe keyset to use for signature generation.\n\nThe following are both valid paths to an EdgeCacheKeyset resource:\n\n  * 'projects/project/locations/global/edgeCacheKeysets/yourKeyset'\n  * 'yourKeyset'\n\nThis must be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.  This field may not be specified otherwise."]
    pub fn set_keyset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keyset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token_query_parameter`.\nThe query parameter in which to put the generated token.\n\nIf not specified, defaults to 'edge-cache-token'.\n\nIf specified, the name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nThis field may only be set when the GENERATE_TOKEN_HLS_COOKIELESS or PROPAGATE_TOKEN_HLS_COOKIELESS actions are specified."]
    pub fn set_token_query_parameter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.token_query_parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token_ttl`.\nThe duration the token is valid starting from the moment the token is first generated.\n\nDefaults to '86400s' (1 day).\n\nThe TTL must be >= 0 and <= 604,800 seconds (1 week).\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_token_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl {
    #[doc =
        "The actions to take to add signatures to responses. Possible values: [\"GENERATE_COOKIE\", \"GENERATE_TOKEN_HLS_COOKIELESS\", \"PROPAGATE_TOKEN_HLS_COOKIELESS\"]"]
    pub actions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl {
            actions: self.actions,
            copied_parameters: core::default::Default::default(),
            keyset: core::default::Default::default(),
            token_query_parameter: core::default::Default::default(),
            token_ttl: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nThe actions to take to add signatures to responses. Possible values: [\"GENERATE_COOKIE\", \"GENERATE_TOKEN_HLS_COOKIELESS\", \"PROPAGATE_TOKEN_HLS_COOKIELESS\"]"]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `copied_parameters` after provisioning.\nThe parameters to copy from the verified token to the generated token.\n\nOnly the following parameters may be copied:\n\n  * 'PathGlobs'\n  * 'paths'\n  * 'acl'\n  * 'URLPrefix'\n  * 'IPRanges'\n  * 'SessionID'\n  * 'id'\n  * 'Data'\n  * 'data'\n  * 'payload'\n  * 'Headers'\n\nYou may specify up to 6 parameters to copy.  A given parameter is be copied only if the parameter exists in the verified token.  Parameter names are matched exactly as specified.  The order of the parameters does not matter.  Duplicates are not allowed.\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified."]
    pub fn copied_parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copied_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keyset` after provisioning.\nThe keyset to use for signature generation.\n\nThe following are both valid paths to an EdgeCacheKeyset resource:\n\n  * 'projects/project/locations/global/edgeCacheKeysets/yourKeyset'\n  * 'yourKeyset'\n\nThis must be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.  This field may not be specified otherwise."]
    pub fn keyset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_query_parameter` after provisioning.\nThe query parameter in which to put the generated token.\n\nIf not specified, defaults to 'edge-cache-token'.\n\nIf specified, the name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nThis field may only be set when the GENERATE_TOKEN_HLS_COOKIELESS or PROPAGATE_TOKEN_HLS_COOKIELESS actions are specified."]
    pub fn token_query_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_query_parameter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_ttl` after provisioning.\nThe duration the token is valid starting from the moment the token is first generated.\n\nDefaults to '86400s' (1 day).\n\nThe TTL must be >= 0 and <= 604,800 seconds (1 week).\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn token_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_host: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_query_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_query_parameters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_protocol: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_cookie_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_header_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_query_parameters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl {
    #[doc =
        "Set the field `exclude_host`.\nIf true, requests to different hosts will be cached separately.\n\nNote: this should only be enabled if hosts share the same origin and content. Removing the host from the cache key may inadvertently result in different objects being cached than intended, depending on which route the first user matched."]
    pub fn set_exclude_host(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude_query_string`.\nIf true, exclude query string parameters from the cache key\n\nIf false (the default), include the query string parameters in\nthe cache key according to includeQueryParameters and\nexcludeQueryParameters. If neither includeQueryParameters nor\nexcludeQueryParameters is set, the entire query string will be\nincluded."]
    pub fn set_exclude_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_query_parameters`.\nNames of query string parameters to exclude from cache keys. All other parameters will be included.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."]
    pub fn set_excluded_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_query_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_protocol`.\nIf true, http and https requests will be cached separately."]
    pub fn set_include_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_cookie_names`.\nNames of Cookies to include in cache keys.  The cookie name and cookie value of each cookie named will be used as part of the cache key.\n\nCookie names:\n  - must be valid RFC 6265 \"cookie-name\" tokens\n  - are case sensitive\n  - cannot start with \"Edge-Cache-\" (case insensitive)\n\n  Note that specifying several cookies, and/or cookies that have a large range of values (e.g., per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.\n\n  You may specify up to three cookie names."]
    pub fn set_included_cookie_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_cookie_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_header_names`.\nNames of HTTP request headers to include in cache keys. The value of the header field will be used as part of the cache key.\n\n- Header names must be valid HTTP RFC 7230 header field values.\n- Header field names are case insensitive\n- To include the HTTP method, use \":method\"\n\nNote that specifying several headers, and/or headers that have a large range of values (e.g. per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance."]
    pub fn set_included_header_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_header_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_query_parameters`.\nNames of query string parameters to include in cache keys. All other parameters will be excluded.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."]
    pub fn set_included_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_query_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl {
            exclude_host: core::default::Default::default(),
            exclude_query_string: core::default::Default::default(),
            excluded_query_parameters: core::default::Default::default(),
            include_protocol: core::default::Default::default(),
            included_cookie_names: core::default::Default::default(),
            included_header_names: core::default::Default::default(),
            included_query_parameters: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_host` after provisioning.\nIf true, requests to different hosts will be cached separately.\n\nNote: this should only be enabled if hosts share the same origin and content. Removing the host from the cache key may inadvertently result in different objects being cached than intended, depending on which route the first user matched."]
    pub fn exclude_host(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_query_string` after provisioning.\nIf true, exclude query string parameters from the cache key\n\nIf false (the default), include the query string parameters in\nthe cache key according to includeQueryParameters and\nexcludeQueryParameters. If neither includeQueryParameters nor\nexcludeQueryParameters is set, the entire query string will be\nincluded."]
    pub fn exclude_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_query_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_query_parameters` after provisioning.\nNames of query string parameters to exclude from cache keys. All other parameters will be included.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."]
    pub fn excluded_query_parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_query_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_protocol` after provisioning.\nIf true, http and https requests will be cached separately."]
    pub fn include_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_cookie_names` after provisioning.\nNames of Cookies to include in cache keys.  The cookie name and cookie value of each cookie named will be used as part of the cache key.\n\nCookie names:\n  - must be valid RFC 6265 \"cookie-name\" tokens\n  - are case sensitive\n  - cannot start with \"Edge-Cache-\" (case insensitive)\n\n  Note that specifying several cookies, and/or cookies that have a large range of values (e.g., per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.\n\n  You may specify up to three cookie names."]
    pub fn included_cookie_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_cookie_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_header_names` after provisioning.\nNames of HTTP request headers to include in cache keys. The value of the header field will be used as part of the cache key.\n\n- Header names must be valid HTTP RFC 7230 header field values.\n- Header field names are case insensitive\n- To include the HTTP method, use \":method\"\n\nNote that specifying several headers, and/or headers that have a large range of values (e.g. per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance."]
    pub fn included_header_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_header_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_query_parameters` after provisioning.\nNames of query string parameters to include in cache keys. All other parameters will be excluded.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."]
    pub fn included_query_parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_query_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_signature_algorithms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_query_parameter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl {
    #[doc =
        "Set the field `allowed_signature_algorithms`.\nThe allowed signature algorithms to use.\n\nDefaults to using only ED25519.\n\nYou may specify up to 3 signature algorithms to use. Possible values: [\"ED25519\", \"HMAC_SHA_256\", \"HMAC_SHA1\"]"]
    pub fn set_allowed_signature_algorithms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_signature_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token_query_parameter`.\nThe query parameter in which to find the token.\n\nThe name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nDefaults to 'edge-cache-token'."]
    pub fn set_token_query_parameter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.token_query_parameter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl {
            allowed_signature_algorithms: core::default::Default::default(),
            token_query_parameter: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_signature_algorithms` after provisioning.\nThe allowed signature algorithms to use.\n\nDefaults to using only ED25519.\n\nYou may specify up to 3 signature algorithms to use. Possible values: [\"ED25519\", \"HMAC_SHA_256\", \"HMAC_SHA1\"]"]
    pub fn allowed_signature_algorithms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_signature_algorithms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_query_parameter` after provisioning.\nThe query parameter in which to find the token.\n\nThe name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nDefaults to 'edge-cache-token'."]
    pub fn token_query_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_query_parameter", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElDynamic {
    add_signatures: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl,
        >,
    >,
    cache_key_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl,
        >,
    >,
    signed_token_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching_policy: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_request_keyset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_request_maximum_expiration_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_request_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_signatures: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_token_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl,
        >,
    >,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElDynamic,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
    #[doc =
        "Set the field `cache_mode`.\nCache modes allow users to control the behaviour of the cache, what content it should cache automatically, whether to respect origin headers, or whether to unconditionally cache all responses.\n\nFor all cache modes, Cache-Control headers will be passed to the client. Use clientTtl to override what is sent to the client. Possible values: [\"CACHE_ALL_STATIC\", \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"BYPASS_CACHE\"]"]
    pub fn set_cache_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_ttl`.\nSpecifies a separate client (e.g. browser client) TTL, separate from the TTL used by the edge caches. Leaving this empty will use the same cache TTL for both the CDN and the client-facing response.\n\n- The TTL must be > 0 and <= 86400s (1 day)\n- The clientTtl cannot be larger than the defaultTtl (if set)\n- Fractions of a second are not allowed.\n\nOmit this field to use the defaultTtl, or the max-age set by the origin, as the client-facing TTL.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn set_client_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_ttl`.\nSpecifies the default TTL for cached content served by this origin for responses that do not have an existing valid TTL (max-age or s-max-age).\n\nDefaults to 3600s (1 hour).\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\" (equivalent to must-revalidate)\n- The value of defaultTTL cannot be set to a value greater than that of maxTTL.\n- Fractions of a second are not allowed.\n- When the cacheMode is set to FORCE_CACHE_ALL, the defaultTTL will overwrite the TTL set in all responses.\n\nNote that infrequently accessed objects may be evicted from the cache before the defined TTL. Objects that expire will be revalidated with the origin.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin.\n\nDefaults to 86400s (1 day).\n\nCache directives that attempt to set a max-age or s-maxage higher than this, or an Expires header more than maxTtl seconds in the future will be capped at the value of maxTTL, as if it were the value of an s-maxage Cache-Control directive.\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\"\n- The value of maxTtl must be equal to or greater than defaultTtl.\n- Fractions of a second are not allowed.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn set_max_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `negative_caching`.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects. This can reduce the load on your origin and improve end-user experience by reducing response latency.\n\nBy default, the CDNPolicy will apply the following default TTLs to these status codes:\n\n- HTTP 300 (Multiple Choice), 301, 308 (Permanent Redirects): 10m\n- HTTP 404 (Not Found), 410 (Gone), 451 (Unavailable For Legal Reasons): 120s\n- HTTP 405 (Method Not Found), 414 (URI Too Long), 501 (Not Implemented): 60s\n\nThese defaults can be overridden in negativeCachingPolicy"]
    pub fn set_negative_caching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.negative_caching = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `negative_caching_policy`.\nSets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.\n\n- Omitting the policy and leaving negativeCaching enabled will use the default TTLs for each status code, defined in negativeCaching.\n- TTLs must be >= 0 (where 0 is \"always revalidate\") and <= 86400s (1 day)\n\nNote that when specifying an explicit negativeCachingPolicy, you should take care to specify a cache TTL for all response codes that you wish to cache. The CDNPolicy will not apply any default negative caching when a policy exists."]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.negative_caching_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `signed_request_keyset`.\nThe EdgeCacheKeyset containing the set of public keys used to validate signed requests at the edge."]
    pub fn set_signed_request_keyset(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.signed_request_keyset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `signed_request_maximum_expiration_ttl`.\nLimit how far into the future the expiration time of a signed request may be.\n\nWhen set, a signed request is rejected if its expiration time is later than now + signedRequestMaximumExpirationTtl, where now is the time at which the signed request is first handled by the CDN.\n\n- The TTL must be > 0.\n- Fractions of a second are not allowed.\n\nBy default, signedRequestMaximumExpirationTtl is not set and the expiration time of a signed request may be arbitrarily far into future."]
    pub fn set_signed_request_maximum_expiration_ttl(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.signed_request_maximum_expiration_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `signed_request_mode`.\nWhether to enforce signed requests. The default value is DISABLED, which means all content is public, and does not authorize access.\n\nYou must also set a signedRequestKeyset to enable signed requests.\n\nWhen set to REQUIRE_SIGNATURES, all matching requests will have their signature validated. Requests that were not signed with the corresponding private key, or that are otherwise invalid (expired, do not match the signature, IP address, or header) will be rejected with a HTTP 403 and (if enabled) logged. Possible values: [\"DISABLED\", \"REQUIRE_SIGNATURES\", \"REQUIRE_TOKENS\"]"]
    pub fn set_signed_request_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.signed_request_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `add_signatures`.\n"]
    pub fn set_add_signatures(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.add_signatures = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.add_signatures = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl,
                        >,
                    >,
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

    #[doc = "Set the field `signed_token_options`.\n"]
    pub fn set_signed_token_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.signed_token_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.signed_token_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            signed_request_keyset: core::default::Default::default(),
            signed_request_maximum_expiration_ttl: core::default::Default::default(),
            signed_request_mode: core::default::Default::default(),
            add_signatures: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            signed_token_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_mode` after provisioning.\nCache modes allow users to control the behaviour of the cache, what content it should cache automatically, whether to respect origin headers, or whether to unconditionally cache all responses.\n\nFor all cache modes, Cache-Control headers will be passed to the client. Use clientTtl to override what is sent to the client. Possible values: [\"CACHE_ALL_STATIC\", \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"BYPASS_CACHE\"]"]
    pub fn cache_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_ttl` after provisioning.\nSpecifies a separate client (e.g. browser client) TTL, separate from the TTL used by the edge caches. Leaving this empty will use the same cache TTL for both the CDN and the client-facing response.\n\n- The TTL must be > 0 and <= 86400s (1 day)\n- The clientTtl cannot be larger than the defaultTtl (if set)\n- Fractions of a second are not allowed.\n\nOmit this field to use the defaultTtl, or the max-age set by the origin, as the client-facing TTL.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn client_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_ttl` after provisioning.\nSpecifies the default TTL for cached content served by this origin for responses that do not have an existing valid TTL (max-age or s-max-age).\n\nDefaults to 3600s (1 hour).\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\" (equivalent to must-revalidate)\n- The value of defaultTTL cannot be set to a value greater than that of maxTTL.\n- Fractions of a second are not allowed.\n- When the cacheMode is set to FORCE_CACHE_ALL, the defaultTTL will overwrite the TTL set in all responses.\n\nNote that infrequently accessed objects may be evicted from the cache before the defined TTL. Objects that expire will be revalidated with the origin.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn default_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin.\n\nDefaults to 86400s (1 day).\n\nCache directives that attempt to set a max-age or s-maxage higher than this, or an Expires header more than maxTtl seconds in the future will be capped at the value of maxTTL, as if it were the value of an s-maxage Cache-Control directive.\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\"\n- The value of maxTtl must be equal to or greater than defaultTtl.\n- Fractions of a second are not allowed.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn max_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `negative_caching` after provisioning.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects. This can reduce the load on your origin and improve end-user experience by reducing response latency.\n\nBy default, the CDNPolicy will apply the following default TTLs to these status codes:\n\n- HTTP 300 (Multiple Choice), 301, 308 (Permanent Redirects): 10m\n- HTTP 404 (Not Found), 410 (Gone), 451 (Unavailable For Legal Reasons): 120s\n- HTTP 405 (Method Not Found), 414 (URI Too Long), 501 (Not Implemented): 60s\n\nThese defaults can be overridden in negativeCachingPolicy"]
    pub fn negative_caching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negative_caching", self.base))
    }

    #[doc =
        "Get a reference to the value of field `negative_caching_policy` after provisioning.\nSets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.\n\n- Omitting the policy and leaving negativeCaching enabled will use the default TTLs for each status code, defined in negativeCaching.\n- TTLs must be >= 0 (where 0 is \"always revalidate\") and <= 86400s (1 day)\n\nNote that when specifying an explicit negativeCachingPolicy, you should take care to specify a cache TTL for all response codes that you wish to cache. The CDNPolicy will not apply any default negative caching when a policy exists."]
    pub fn negative_caching_policy(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.negative_caching_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signed_request_keyset` after provisioning.\nThe EdgeCacheKeyset containing the set of public keys used to validate signed requests at the edge."]
    pub fn signed_request_keyset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signed_request_keyset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signed_request_maximum_expiration_ttl` after provisioning.\nLimit how far into the future the expiration time of a signed request may be.\n\nWhen set, a signed request is rejected if its expiration time is later than now + signedRequestMaximumExpirationTtl, where now is the time at which the signed request is first handled by the CDN.\n\n- The TTL must be > 0.\n- Fractions of a second are not allowed.\n\nBy default, signedRequestMaximumExpirationTtl is not set and the expiration time of a signed request may be arbitrarily far into future."]
    pub fn signed_request_maximum_expiration_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signed_request_maximum_expiration_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signed_request_mode` after provisioning.\nWhether to enforce signed requests. The default value is DISABLED, which means all content is public, and does not authorize access.\n\nYou must also set a signedRequestKeyset to enable signed requests.\n\nWhen set to REQUIRE_SIGNATURES, all matching requests will have their signature validated. Requests that were not signed with the corresponding private key, or that are otherwise invalid (expired, do not match the signature, IP address, or header) will be rejected with a HTTP 403 and (if enabled) logged. Possible values: [\"DISABLED\", \"REQUIRE_SIGNATURES\", \"REQUIRE_TOKENS\"]"]
    pub fn signed_request_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signed_request_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `add_signatures` after provisioning.\n"]
    pub fn add_signatures(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_signatures", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `signed_token_options` after provisioning.\n"]
    pub fn signed_token_options(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_token_options", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    max_age: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
    #[doc =
        "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\n\nThis translates to the Access-Control-Allow-Credentials response header."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods response header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests.\n\nThis translates to the Access-Control-Allow-Origin response header."]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
    #[doc =
        "Specifies how long results of a preflight request can be cached by a client in seconds. Note that many browser clients enforce a maximum TTL of 600s (10 minutes).\n\n- Setting the value to -1 forces a pre-flight check for all requests (not recommended)\n- A maximum TTL of 86400s can be set, but note that (as above) some clients may force pre-flight checks at a more regular interval.\n- This translates to the Access-Control-Max-Age header.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub max_age: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: self.max_age,
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\n\nThis translates to the Access-Control-Allow-Credentials response header."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn allow_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods response header."]
    pub fn allow_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\n\nThis translates to the Access-Control-Allow-Origin response header."]
    pub fn allow_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn expose_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long results of a preflight request can be cached by a client in seconds. Note that many browser clients enforce a maximum TTL of 600s (10 minutes).\n\n- Setting the value to -1 forces a pre-flight check for all requests (not recommended)\n- A maximum TTL of 86400s can be set, but note that (as above) some clients may force pre-flight checks at a more regular interval.\n- This translates to the Access-Control-Max-Age header.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_template_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
    #[doc =
        "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected origin, the request's host header is replaced with contents of hostRewrite."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected origin, the matching portion of the request's path is replaced by pathPrefixRewrite."]
    pub fn set_path_prefix_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_prefix_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_template_rewrite`.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn set_path_template_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_template_rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, the request's host header is replaced with contents of hostRewrite."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, the matching portion of the request's path is replaced by pathPrefixRewrite."]
    pub fn path_prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_template_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn path_template_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_template_rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElDynamic {
    cdn_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl>,
    >,
    cors_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl>,
    >,
    url_rewrite: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdn_policy: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std
    ::option
    ::Option<
        std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std
    ::option
    ::Option<
        std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl>,
    >,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElDynamic,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
    #[doc = "Set the field `cdn_policy`.\n"]
    pub fn set_cdn_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cdn_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cdn_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = std::option::Option::Some(d);
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl,
                        >,
                    >,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
            cdn_policy: core::default::Default::default(),
            cors_policy: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_redirect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_query: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
    #[doc =
        "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn set_host_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to false, the URL scheme of the redirected request will remain the same as that of the request.\n\nThis can only be set if there is at least one (1) edgeSslCertificate set on the service."]
    pub fn set_https_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.https_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was supplied in the request.\n\npathRedirect cannot be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.\n\nThe path value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the routeRule, retaining the remaining portion of the URL before redirecting the request.\n\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn set_prefix_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction.\n\nThe supported values are:\n\n- 'MOVED_PERMANENTLY_DEFAULT', which is the default value and corresponds to 301.\n- 'FOUND', which corresponds to 302.\n- 'SEE_OTHER' which corresponds to 303.\n- 'TEMPORARY_REDIRECT', which corresponds to 307. in this case, the request method will be retained.\n- 'PERMANENT_REDIRECT', which corresponds to 308. in this case, the request method will be retained. Possible values: [\"MOVED_PERMANENTLY_DEFAULT\", \"FOUND\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\", \"PERMANENT_REDIRECT\"]"]
    pub fn set_redirect_response_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redirect_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request. If set to false, the query portion of the original URL is retained."]
    pub fn set_strip_query(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn host_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to false, the URL scheme of the redirected request will remain the same as that of the request.\n\nThis can only be set if there is at least one (1) edgeSslCertificate set on the service."]
    pub fn https_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was supplied in the request.\n\npathRedirect cannot be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.\n\nThe path value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the routeRule, retaining the remaining portion of the URL before redirecting the request.\n\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn prefix_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction.\n\nThe supported values are:\n\n- 'MOVED_PERMANENTLY_DEFAULT', which is the default value and corresponds to 301.\n- 'FOUND', which corresponds to 302.\n- 'SEE_OTHER' which corresponds to 303.\n- 'TEMPORARY_REDIRECT', which corresponds to 307. in this case, the request method will be retained.\n- 'PERMANENT_REDIRECT', which corresponds to 308. in this case, the request method will be retained. Possible values: [\"MOVED_PERMANENTLY_DEFAULT\", \"FOUND\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\", \"PERMANENT_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request. If set to false, the query portion of the original URL is retained."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl>>,
    match_rule: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl>>,
    route_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl>>,
    url_redirect: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<std::string::String>>,
    priority: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_rule: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl>>,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElDynamic,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    #[doc = "Set the field `description`.\nA human-readable description of the routeRule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin`.\nThe Origin resource that requests to this route should fetch from when a matching response is not in cache. Origins can be defined as short names (\"my-origin\") or fully-qualified resource URLs - e.g. \"networkservices.googleapis.com/projects/my-project/global/edgecacheorigins/my-origin\"\n\nOnly one of origin or urlRedirect can be set."]
    pub fn set_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl,
                        >,
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

    #[doc = "Set the field `match_rule`.\n"]
    pub fn set_match_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    #[doc =
        "The priority of this route rule, where 1 is the highest priority.\n\nYou cannot configure two or more routeRules with the same priority. Priority for each rule must be set to a number between 1 and 999 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules in the future without affecting the rest of the rules. For example, 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers\nto which you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the future without any impact on existing rules."]
    pub priority: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
            description: core::default::Default::default(),
            origin: core::default::Default::default(),
            priority: self.priority,
            header_action: core::default::Default::default(),
            match_rule: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the routeRule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nThe Origin resource that requests to this route should fetch from when a matching response is not in cache. Origins can be defined as short names (\"my-origin\") or fully-qualified resource URLs - e.g. \"networkservices.googleapis.com/projects/my-project/global/edgecacheorigins/my-origin\"\n\nOnly one of origin or urlRedirect can be set."]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe priority of this route rule, where 1 is the highest priority.\n\nYou cannot configure two or more routeRules with the same priority. Priority for each rule must be set to a number between 1 and 999 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules in the future without affecting the rest of the rules. For example, 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers\nto which you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the future without any impact on existing rules."]
    pub fn priority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_rule` after provisioning.\n"]
    pub fn match_rule(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElDynamic {
    route_rule: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_rule: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>>,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElDynamic,
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_rule`.\n"]
    pub fn set_route_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    #[doc = "The name to which this PathMatcher is referred by the HostRule."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
            description: core::default::Default::default(),
            name: self.name,
            route_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name to which this PathMatcher is referred by the HostRule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `route_rule` after provisioning.\n"]
    pub fn route_rule(&self) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElDynamic {
    host_rule: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>>,
    path_matcher: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rule: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_matcher: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>>,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElDynamic,
}

impl NetworkServicesEdgeCacheServiceRoutingEl {
    #[doc = "Set the field `host_rule`.\n"]
    pub fn set_host_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path_matcher`.\n"]
    pub fn set_path_matcher(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path_matcher = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path_matcher = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceRoutingEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceRoutingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceRoutingEl {}

impl BuildNetworkServicesEdgeCacheServiceRoutingEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingEl {
        NetworkServicesEdgeCacheServiceRoutingEl {
            host_rule: core::default::Default::default(),
            path_matcher: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceRoutingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceRoutingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheServiceRoutingElRef {
        NetworkServicesEdgeCacheServiceRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceRoutingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_rule` after provisioning.\n"]
    pub fn host_rule(&self) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheServiceTimeoutsEl {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheServiceTimeoutsEl {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheServiceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheServiceTimeoutsEl {}

impl BuildNetworkServicesEdgeCacheServiceTimeoutsEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceTimeoutsEl {
        NetworkServicesEdgeCacheServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesEdgeCacheServiceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheServiceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheServiceTimeoutsElRef {
        NetworkServicesEdgeCacheServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheServiceTimeoutsElRef {
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
struct NetworkServicesEdgeCacheServiceDynamic {
    log_config: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceLogConfigEl>>,
    routing: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheServiceRoutingEl>>,
}
