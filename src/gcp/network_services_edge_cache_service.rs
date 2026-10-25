#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesEdgeCacheServiceData_ {
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
    log_config: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheService_LogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesEdgeCacheService_Timeouts>,
    dynamic: NetworkServicesEdgeCacheServiceDynamic,
}

struct NetworkServicesEdgeCacheService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesEdgeCacheServiceData_>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheService_LogConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesEdgeCacheService_Timeouts>) -> Self {
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
    pub fn log_config(&self) -> terrars::ListRef<NetworkServicesEdgeCacheService_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing` after provisioning.\n"]
    pub fn routing(&self) -> terrars::ListRef<NetworkServicesEdgeCacheService_RoutingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheService_TimeoutsRef {
        NetworkServicesEdgeCacheService_TimeoutsRef::new(
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
            data: std::cell::RefCell::new(NetworkServicesEdgeCacheServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
    pub fn log_config(&self) -> terrars::ListRef<NetworkServicesEdgeCacheService_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing` after provisioning.\n"]
    pub fn routing(&self) -> terrars::ListRef<NetworkServicesEdgeCacheService_RoutingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheService_TimeoutsRef {
        NetworkServicesEdgeCacheService_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceDynamic {
    log_config: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_LogConfig>>,
    routing: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_LogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesEdgeCacheService_LogConfig {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_LogConfig {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_LogConfig {}

impl BuildNetworkServicesEdgeCacheService_LogConfig {
    pub fn build(self) -> NetworkServicesEdgeCacheService_LogConfig {
        NetworkServicesEdgeCacheService_LogConfig {
            enable: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheService_LogConfigRef {
        NetworkServicesEdgeCacheService_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_LogConfigRef {
    #[allow(dead_code)]
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
pub struct NetworkServicesEdgeCacheService_Routing_HostRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hosts: terrars::ListField<terrars::PrimField<std::string::String>>,
    path_matcher: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheService_Routing_HostRule {
    #[doc = "Set the field `description`.\nA human-readable description of the hostRule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_HostRule {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_HostRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_HostRule {
    #[doc =
        "The list of host patterns to match.\n\nHost patterns must be valid hostnames. Ports are not allowed. Wildcard hosts are supported in the suffix or prefix form. * matches any string of ([a-z0-9-.]*). It does not match the empty string.\n\nWhen multiple hosts are specified, hosts are matched in the following priority:\n\n  1. Exact domain names: ''www.foo.com''.\n  2. Suffix domain wildcards: ''*.foo.com'' or ''*-bar.foo.com''.\n  3. Prefix domain wildcards: ''foo.*'' or ''foo-*''.\n  4. Special wildcard ''*'' matching any domain.\n\n  Notes:\n\n    The wildcard will not match the empty string. e.g. ''*-bar.foo.com'' will match ''baz-bar.foo.com'' but not ''-bar.foo.com''. The longest wildcards match first. Only a single host in the entire service can match on ''*''. A domain must be unique across all configured hosts within a service.\n\n    Hosts are matched against the HTTP Host header, or for HTTP/2 and HTTP/3, the \":authority\" header, from the incoming request.\n\n    You may specify up to 10 hosts."]
    pub hosts: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The name of the pathMatcher associated with this hostRule."]
    pub path_matcher: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_HostRule {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_HostRule {
        NetworkServicesEdgeCacheService_Routing_HostRule {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_HostRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_HostRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_HostRuleRef {
        NetworkServicesEdgeCacheService_Routing_HostRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_HostRuleRef {
    #[allow(dead_code)]
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
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd {
    #[doc = "Set the field `replace`.\nWhether to replace all existing headers with the same name."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd {
    #[doc = "The name of the header to add."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAddRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAddRef {
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
        "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove {
    header_name: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove { }

impl terrars
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove {
    #[doc = "The name of the header to remove."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove {
            header_name: self.header_name,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemoveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemoveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemoveRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemoveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemoveRef {
    #[allow(dead_code)]
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
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd {
    #[doc = "Set the field `replace`.\nWhether to replace all existing headers with the same name."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd {
    #[doc = "The name of the header to add."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAddRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAddRef {
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
        "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove {
    header_name: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove { }

impl terrars
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove {
    #[doc =
        "Headers to remove from the response prior to sending it back to the client.\n\nResponse headers are only sent to the client, and do not have an effect on the cache serving the response."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove {
            header_name: self.header_name,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemoveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemoveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemoveRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemoveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemoveRef {
    #[allow(dead_code)]
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
struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionDynamic {
    request_header_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd>,
    >,
    request_header_to_remove: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove,
        >,
    >,
    response_header_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd>,
    >,
    response_header_to_remove: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header_to_add: std
    ::option
    ::Option<
        std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header_to_remove: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header_to_add: std
    ::option
    ::Option<
        std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header_to_remove: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove>,
    >,
    dynamic: NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionDynamic,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction {
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAdd,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemove,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAdd,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemove,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction {
            request_header_to_add: core::default::Default::default(),
            request_header_to_remove: core::default::Default::default(),
            response_header_to_add: core::default::Default::default(),
            response_header_to_remove: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_header_to_add` after provisioning.\n"]
    pub fn request_header_to_add(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header_to_remove` after provisioning.\n"]
    pub fn request_header_to_remove(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_RequestHeaderToRemoveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header_to_remove", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header_to_add` after provisioning.\n"]
    pub fn response_header_to_add(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header_to_remove` after provisioning.\n"]
    pub fn response_header_to_remove(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction_ResponseHeaderToRemoveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header_to_remove", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch {
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

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch {
    #[doc = "The header name to match on."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch {
            exact_match: core::default::Default::default(),
            header_name: self.header_name,
            invert_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            suffix_match: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatchRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatchRef {
    #[allow(dead_code)]
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
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch {
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
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch {
    #[doc =
        "The name of the query parameter to match. The query parameter must exist in the request, in the absence of which the request match fails."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch {
            exact_match: core::default::Default::default(),
            name: self.name,
            present_match: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatchRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatchRef {
    #[allow(dead_code)]
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
struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleDynamic {
    header_match: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch>,
    >,
    query_parameter_match: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch>,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule {
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
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter_match: std
    ::option
    ::Option<
        std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch>,
    >,
    dynamic: NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleDynamic,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule {
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatch,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatch,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule {
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

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_HeaderMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_match", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameter_match` after provisioning.\n"]
    pub fn query_parameter_match(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule_QueryParameterMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameter_match", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures {
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

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures {
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
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures {
    #[doc =
        "The actions to take to add signatures to responses. Possible values: [\"GENERATE_COOKIE\", \"GENERATE_TOKEN_HLS_COOKIELESS\", \"PROPAGATE_TOKEN_HLS_COOKIELESS\"]"]
    pub actions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures {
            actions: self.actions,
            copied_parameters: core::default::Default::default(),
            keyset: core::default::Default::default(),
            token_query_parameter: core::default::Default::default(),
            token_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignaturesRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignaturesRef {
    #[allow(dead_code)]
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
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy {
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

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy {
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
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy {
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

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicyRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicyRef {
    #[allow(dead_code)]
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
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_signature_algorithms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_query_parameter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions {
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
::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions {
    type O =
        terrars
        ::BlockAssignable<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions {
            allowed_signature_algorithms: core::default::Default::default(),
            token_query_parameter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptionsRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptionsRef {
    #[allow(dead_code)]
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
struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyDynamic {
    add_signatures: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures,
        >,
    >,
    cache_key_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy,
        >,
    >,
    signed_token_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy {
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
        ::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_token_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions>,
    >,
    dynamic: NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyDynamic,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy {
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignatures,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicy,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptions,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy {
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

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyRef {
    #[allow(dead_code)]
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
    ::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_AddSignaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_signatures", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(
        &self,
    ) -> terrars
    ::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_CacheKeyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `signed_token_options` after provisioning.\n"]
    pub fn signed_token_options(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy_SignedTokenOptionsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_token_options", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy {
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

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy {
    #[doc =
        "Specifies how long results of a preflight request can be cached by a client in seconds. Note that many browser clients enforce a maximum TTL of 600s (10 minutes).\n\n- Setting the value to -1 forces a pre-flight check for all requests (not recommended)\n- A maximum TTL of 86400s can be set, but note that (as above) some clients may force pre-flight checks at a more regular interval.\n- This translates to the Access-Control-Max-Age header.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub max_age: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy {
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

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicyRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicyRef {
    #[allow(dead_code)]
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
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_template_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewriteRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewriteRef {
    #[allow(dead_code)]
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
struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionDynamic {
    cdn_policy: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy>,
    >,
    cors_policy: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy>,
    >,
    url_rewrite: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite>,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdn_policy: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite>>,
    dynamic: NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionDynamic,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction {
    #[doc =
        "Set the field `compression_mode`.\nSetting the compression mode to automatic enables dynamic compression for every eligible response.\n\nWhen dynamic compression is enabled, it is recommended to also set a cache policy to maximize efficiency. Possible values: [\"DISABLED\", \"AUTOMATIC\"]"]
    pub fn set_compression_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_mode = std::option::Option::Some(v.into());
        self
    }

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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicy,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicy,
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
                            NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewrite,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction {
            compression_mode: core::default::Default::default(),
            cdn_policy: core::default::Default::default(),
            cors_policy: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compression_mode` after provisioning.\nSetting the compression mode to automatic enables dynamic compression for every eligible response.\n\nWhen dynamic compression is enabled, it is recommended to also set a cache policy to maximize efficiency. Possible values: [\"DISABLED\", \"AUTOMATIC\"]"]
    pub fn compression_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CdnPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods {
    #[doc =
        "Set the field `allowed_methods`.\nThe non-empty set of HTTP methods that are allowed for this route.\n\nAny combination of \"GET\", \"HEAD\", \"OPTIONS\", \"PUT\", \"POST\", \"DELETE\", and \"PATCH\"."]
    pub fn set_allowed_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_methods = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods {
            allowed_methods: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethodsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethodsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethodsRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethodsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethodsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_methods` after provisioning.\nThe non-empty set of HTTP methods that are allowed for this route.\n\nAny combination of \"GET\", \"HEAD\", \"OPTIONS\", \"PUT\", \"POST\", \"DELETE\", and \"PATCH\"."]
    pub fn allowed_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect {
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

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect {}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirectRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirectRef {
    #[allow(dead_code)]
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
struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction>>,
    match_rule: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule>>,
    route_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction>>,
    route_methods: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods>>,
    url_redirect: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<std::string::String>>,
    priority: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_rule: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_methods: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect>>,
    dynamic: NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleDynamic,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule {
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
                        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderAction>,
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
                        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRule>,
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
                        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteAction>,
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

    #[doc = "Set the field `route_methods`.\n"]
    pub fn set_route_methods(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethods>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_methods = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_methods = std::option::Option::Some(d);
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
                        ::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirect>,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule {
    #[doc =
        "The priority of this route rule, where 1 is the highest priority.\n\nYou cannot configure two or more routeRules with the same priority. Priority for each rule must be set to a number between 1 and 999 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules in the future without affecting the rest of the rules. For example, 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers\nto which you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the future without any impact on existing rules."]
    pub priority: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule {
            description: core::default::Default::default(),
            origin: core::default::Default::default(),
            priority: self.priority,
            header_action: core::default::Default::default(),
            match_rule: core::default::Default::default(),
            route_action: core::default::Default::default(),
            route_methods: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_rule` after provisioning.\n"]
    pub fn match_rule(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_MatchRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `route_methods` after provisioning.\n"]
    pub fn route_methods(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_RouteMethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule_UrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheService_Routing_PathMatcherDynamic {
    route_rule: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcher {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_rule: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule>>,
    dynamic: NetworkServicesEdgeCacheService_Routing_PathMatcherDynamic,
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcher {
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
                    ::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRule>>,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing_PathMatcher {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing_PathMatcher {
    #[doc = "The name to which this PathMatcher is referred by the HostRule."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheService_Routing_PathMatcher {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing_PathMatcher {
        NetworkServicesEdgeCacheService_Routing_PathMatcher {
            description: core::default::Default::default(),
            name: self.name,
            route_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_Routing_PathMatcherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_Routing_PathMatcherRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheService_Routing_PathMatcherRef {
        NetworkServicesEdgeCacheService_Routing_PathMatcherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_Routing_PathMatcherRef {
    #[allow(dead_code)]
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
    pub fn route_rule(&self) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcher_RouteRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheService_RoutingDynamic {
    host_rule: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_HostRule>>,
    path_matcher: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheService_Routing_PathMatcher>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Routing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rule: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_HostRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_matcher: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheService_Routing_PathMatcher>>,
    dynamic: NetworkServicesEdgeCacheService_RoutingDynamic,
}

impl NetworkServicesEdgeCacheService_Routing {
    #[doc = "Set the field `host_rule`.\n"]
    pub fn set_host_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_HostRule>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing_PathMatcher>>,
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Routing {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Routing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Routing {}

impl BuildNetworkServicesEdgeCacheService_Routing {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Routing {
        NetworkServicesEdgeCacheService_Routing {
            host_rule: core::default::Default::default(),
            path_matcher: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_RoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_RoutingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheService_RoutingRef {
        NetworkServicesEdgeCacheService_RoutingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_RoutingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_rule` after provisioning.\n"]
    pub fn host_rule(&self) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_HostRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<NetworkServicesEdgeCacheService_Routing_PathMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheService_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheService_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheService_Timeouts {}

impl BuildNetworkServicesEdgeCacheService_Timeouts {
    pub fn build(self) -> NetworkServicesEdgeCacheService_Timeouts {
        NetworkServicesEdgeCacheService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheService_TimeoutsRef {
        NetworkServicesEdgeCacheService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheService_TimeoutsRef {
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
