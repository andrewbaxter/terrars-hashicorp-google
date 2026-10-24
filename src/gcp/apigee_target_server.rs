use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeTargetServerData {
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
    env_id: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s_sl_info: std::option::Option<std::vec::Vec<ApigeeTargetServerSSlInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeTargetServerTimeoutsEl>,
    dynamic: ApigeeTargetServerDynamic,
}

struct ApigeeTargetServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeTargetServerData>,
}

#[derive(Clone)]
pub struct ApigeeTargetServer(std::rc::Rc<ApigeeTargetServer_>);

impl ApigeeTargetServer {
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

    #[doc = "Set the field `description`.\nA human-readable description of this TargetServer."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_enabled`.\nEnabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
    pub fn set_is_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protocol`.\nImmutable. The protocol used by this TargetServer. Possible values: [\"HTTP\", \"HTTP2\", \"GRPC_TARGET\", \"GRPC\", \"EXTERNAL_CALLOUT\"]"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s_sl_info`.\n"]
    pub fn set_s_sl_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeTargetServerSSlInfoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s_sl_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s_sl_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeTargetServerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of this TargetServer."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub fn env_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_enabled` after provisioning.\nEnabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource id of this reference. Values must match the regular expression [\\w\\s-.]+."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nImmutable. The protocol used by this TargetServer. Possible values: [\"HTTP\", \"HTTP2\", \"GRPC_TARGET\", \"GRPC\", \"EXTERNAL_CALLOUT\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s_sl_info` after provisioning.\n"]
    pub fn s_sl_info(&self) -> terrars::ListRef<ApigeeTargetServerSSlInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s_sl_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeTargetServerTimeoutsElRef {
        ApigeeTargetServerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApigeeTargetServer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeTargetServer { }

impl terrars::ToListMappable for ApigeeTargetServer {
    type O = terrars::ListRef<ApigeeTargetServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeTargetServer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_target_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeTargetServer {
    pub tf_id: std::string::String,
    #[doc =
        "The Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub env_id: terrars::PrimField<std::string::String>,
    #[doc = "The host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "The resource id of this reference. Values must match the regular expression [\\w\\s-.]+."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
    pub port: terrars::PrimField<f64>,
}

impl BuildApigeeTargetServer {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeTargetServer {
        let out = ApigeeTargetServer(std::rc::Rc::new(ApigeeTargetServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeTargetServerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                env_id: self.env_id,
                host: self.host,
                id: core::default::Default::default(),
                is_enabled: core::default::Default::default(),
                name: self.name,
                port: self.port,
                protocol: core::default::Default::default(),
                s_sl_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeTargetServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeTargetServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeTargetServerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of this TargetServer."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub fn env_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_enabled` after provisioning.\nEnabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource id of this reference. Values must match the regular expression [\\w\\s-.]+."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nImmutable. The protocol used by this TargetServer. Possible values: [\"HTTP\", \"HTTP2\", \"GRPC_TARGET\", \"GRPC\", \"EXTERNAL_CALLOUT\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s_sl_info` after provisioning.\n"]
    pub fn s_sl_info(&self) -> terrars::ListRef<ApigeeTargetServerSSlInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s_sl_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeTargetServerTimeoutsElRef {
        ApigeeTargetServerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApigeeTargetServerSSlInfoElCommonNameEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard_match: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeTargetServerSSlInfoElCommonNameEl {
    #[doc = "Set the field `value`.\nThe TLS Common Name string of the certificate."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard_match`.\nIndicates whether the cert should be matched against as a wildcard cert."]
    pub fn set_wildcard_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard_match = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeTargetServerSSlInfoElCommonNameEl {
    type O = terrars::BlockAssignable<ApigeeTargetServerSSlInfoElCommonNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeTargetServerSSlInfoElCommonNameEl {}

impl BuildApigeeTargetServerSSlInfoElCommonNameEl {
    pub fn build(self) -> ApigeeTargetServerSSlInfoElCommonNameEl {
        ApigeeTargetServerSSlInfoElCommonNameEl {
            value: core::default::Default::default(),
            wildcard_match: core::default::Default::default(),
        }
    }
}

pub struct ApigeeTargetServerSSlInfoElCommonNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeTargetServerSSlInfoElCommonNameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeTargetServerSSlInfoElCommonNameElRef {
        ApigeeTargetServerSSlInfoElCommonNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeTargetServerSSlInfoElCommonNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe TLS Common Name string of the certificate."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wildcard_match` after provisioning.\nIndicates whether the cert should be matched against as a wildcard cert."]
    pub fn wildcard_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeTargetServerSSlInfoElDynamic {
    common_name: std::option::Option<terrars::DynamicBlock<ApigeeTargetServerSSlInfoElCommonNameEl>>,
}

#[derive(Serialize)]
pub struct ApigeeTargetServerSSlInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ciphers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_auth_enabled: std::option::Option<terrars::PrimField<bool>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_validation_errors: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<std::vec::Vec<ApigeeTargetServerSSlInfoElCommonNameEl>>,
    dynamic: ApigeeTargetServerSSlInfoElDynamic,
}

impl ApigeeTargetServerSSlInfoEl {
    #[doc =
        "Set the field `ciphers`.\nThe SSL/TLS cipher suites to be used. For programmable proxies, it must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites. For configurable proxies, it must follow the configuration specified in: https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration. This setting has no effect for configurable proxies when negotiating TLS 1.3."]
    pub fn set_ciphers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ciphers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_auth_enabled`.\nEnables two-way TLS."]
    pub fn set_client_auth_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_auth_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_validation_errors`.\nIf true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails."]
    pub fn set_ignore_validation_errors(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_validation_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_alias`.\nRequired if clientAuthEnabled is true. The resource ID for the alias containing the private key and cert."]
    pub fn set_key_alias(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store`.\nRequired if clientAuthEnabled is true. The resource ID of the keystore."]
    pub fn set_key_store(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_store = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\nThe TLS versioins to be used."]
    pub fn set_protocols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store`.\nThe resource ID of the truststore."]
    pub fn set_trust_store(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_store = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeTargetServerSSlInfoElCommonNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.common_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.common_name = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeTargetServerSSlInfoEl {
    type O = terrars::BlockAssignable<ApigeeTargetServerSSlInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeTargetServerSSlInfoEl {
    #[doc = "Enables TLS. If false, neither one-way nor two-way TLS will be enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildApigeeTargetServerSSlInfoEl {
    pub fn build(self) -> ApigeeTargetServerSSlInfoEl {
        ApigeeTargetServerSSlInfoEl {
            ciphers: core::default::Default::default(),
            client_auth_enabled: core::default::Default::default(),
            enabled: self.enabled,
            ignore_validation_errors: core::default::Default::default(),
            key_alias: core::default::Default::default(),
            key_store: core::default::Default::default(),
            protocols: core::default::Default::default(),
            trust_store: core::default::Default::default(),
            common_name: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApigeeTargetServerSSlInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeTargetServerSSlInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeTargetServerSSlInfoElRef {
        ApigeeTargetServerSSlInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeTargetServerSSlInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ciphers` after provisioning.\nThe SSL/TLS cipher suites to be used. For programmable proxies, it must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites. For configurable proxies, it must follow the configuration specified in: https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration. This setting has no effect for configurable proxies when negotiating TLS 1.3."]
    pub fn ciphers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc = "Get a reference to the value of field `client_auth_enabled` after provisioning.\nEnables two-way TLS."]
    pub fn client_auth_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_auth_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables TLS. If false, neither one-way nor two-way TLS will be enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_validation_errors` after provisioning.\nIf true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails."]
    pub fn ignore_validation_errors(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_validation_errors", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_alias` after provisioning.\nRequired if clientAuthEnabled is true. The resource ID for the alias containing the private key and cert."]
    pub fn key_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_alias", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store` after provisioning.\nRequired if clientAuthEnabled is true. The resource ID of the keystore."]
    pub fn key_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\nThe TLS versioins to be used."]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store` after provisioning.\nThe resource ID of the truststore."]
    pub fn trust_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store", self.base))
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> terrars::ListRef<ApigeeTargetServerSSlInfoElCommonNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeTargetServerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeTargetServerTimeoutsEl {
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

impl terrars::ToListMappable for ApigeeTargetServerTimeoutsEl {
    type O = terrars::BlockAssignable<ApigeeTargetServerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeTargetServerTimeoutsEl {}

impl BuildApigeeTargetServerTimeoutsEl {
    pub fn build(self) -> ApigeeTargetServerTimeoutsEl {
        ApigeeTargetServerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ApigeeTargetServerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeTargetServerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeTargetServerTimeoutsElRef {
        ApigeeTargetServerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeTargetServerTimeoutsElRef {
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
struct ApigeeTargetServerDynamic {
    s_sl_info: std::option::Option<terrars::DynamicBlock<ApigeeTargetServerSSlInfoEl>>,
}
