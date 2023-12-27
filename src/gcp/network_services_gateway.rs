use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_swg_autogen_router_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    ports: ListField<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_tls_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesGatewayTimeoutsEl>,
}

struct NetworkServicesGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesGatewayData>,
}

#[derive(Clone)]
pub struct NetworkServicesGateway(Rc<NetworkServicesGateway_>);

impl NetworkServicesGateway {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `addresses`.\nZero or one IPv4-address on which the Gateway will receive the traffic. When no address is provided,\nan IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'.\nGateways of type 'OPEN_MESH' listen on 0.0.0.0."]
    pub fn set_addresses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().addresses = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate_urls`.\nA fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.\nThis feature only applies to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn set_certificate_urls(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().certificate_urls = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_swg_autogen_router_on_destroy`.\nWhen deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.\nIf there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted."]
    pub fn set_delete_swg_autogen_router_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_swg_autogen_router_on_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `gateway_security_policy`.\nA fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.\nFor example: 'projects/*/locations/*/gatewaySecurityPolicies/swg-policy'.\nThis policy is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn set_gateway_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gateway_security_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nSet of label tags associated with the Gateway resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe location of the gateway.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\nThe relative resource name identifying the VPC network that is using this configuration.\nFor example: 'projects/*/global/networks/network-1'.\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nImmutable. Scope determines how configuration across multiple Gateway instances are merged.\nThe configuration for multiple Gateway instances with the same scope will be merged as presented as\na single coniguration to the proxy/load balancer.\nMax length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens."]
    pub fn set_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scope = Some(v.into());
        self
    }

    #[doc= "Set the field `server_tls_policy`.\nA fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated.\nIf empty, TLS termination is disabled."]
    pub fn set_server_tls_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_tls_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `subnetwork`.\nThe relative resource name identifying the subnetwork in which this SWG is allocated.\nFor example: 'projects/*/regions/us-central1/subnetworks/network-1'.\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY."]
    pub fn set_subnetwork(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `addresses` after provisioning.\nZero or one IPv4-address on which the Gateway will receive the traffic. When no address is provided,\nan IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'.\nGateways of type 'OPEN_MESH' listen on 0.0.0.0."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_urls` after provisioning.\nA fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.\nThis feature only applies to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn certificate_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_swg_autogen_router_on_destroy` after provisioning.\nWhen deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.\nIf there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted."]
    pub fn delete_swg_autogen_router_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_swg_autogen_router_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_security_policy` after provisioning.\nA fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.\nFor example: 'projects/*/locations/*/gatewaySecurityPolicies/swg-policy'.\nThis policy is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn gateway_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_security_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Gateway resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the gateway.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nShort name of the Gateway resource to be created."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe relative resource name identifying the VPC network that is using this configuration.\nFor example: 'projects/*/global/networks/network-1'.\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\nOne or more port numbers (1-65535), on which the Gateway will receive traffic.\nThe proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are\nlimited to 1 port. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 and support multiple ports."]
    pub fn ports(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nImmutable. Scope determines how configuration across multiple Gateway instances are merged.\nThe configuration for multiple Gateway instances with the same scope will be merged as presented as\na single coniguration to the proxy/load balancer.\nMax length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_tls_policy` after provisioning.\nA fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated.\nIf empty, TLS termination is disabled."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_tls_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnetwork` after provisioning.\nThe relative resource name identifying the subnetwork in which this SWG is allocated.\nFor example: 'projects/*/regions/us-central1/subnetworks/network-1'.\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nImmutable. The type of the customer-managed gateway. Possible values are: * OPEN_MESH * SECURE_WEB_GATEWAY. Possible values: [\"TYPE_UNSPECIFIED\", \"OPEN_MESH\", \"SECURE_WEB_GATEWAY\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGatewayTimeoutsElRef {
        NetworkServicesGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for NetworkServicesGateway {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkServicesGateway { }

impl ToListMappable for NetworkServicesGateway {
    type O = ListRef<NetworkServicesGatewayRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkServicesGateway_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_gateway".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesGateway {
    pub tf_id: String,
    #[doc= "Short name of the Gateway resource to be created."]
    pub name: PrimField<String>,
    #[doc= "One or more port numbers (1-65535), on which the Gateway will receive traffic.\nThe proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are\nlimited to 1 port. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 and support multiple ports."]
    pub ports: ListField<PrimField<f64>>,
    #[doc= "Immutable. The type of the customer-managed gateway. Possible values are: * OPEN_MESH * SECURE_WEB_GATEWAY. Possible values: [\"TYPE_UNSPECIFIED\", \"OPEN_MESH\", \"SECURE_WEB_GATEWAY\"]"]
    pub type_: PrimField<String>,
}

impl BuildNetworkServicesGateway {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesGateway {
        let out = NetworkServicesGateway(Rc::new(NetworkServicesGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                addresses: core::default::Default::default(),
                certificate_urls: core::default::Default::default(),
                delete_swg_autogen_router_on_destroy: core::default::Default::default(),
                description: core::default::Default::default(),
                gateway_security_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                ports: self.ports,
                project: core::default::Default::default(),
                scope: core::default::Default::default(),
                server_tls_policy: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                type_: self.type_,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesGatewayRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkServicesGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesGatewayRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `addresses` after provisioning.\nZero or one IPv4-address on which the Gateway will receive the traffic. When no address is provided,\nan IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'.\nGateways of type 'OPEN_MESH' listen on 0.0.0.0."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_urls` after provisioning.\nA fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.\nThis feature only applies to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn certificate_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_swg_autogen_router_on_destroy` after provisioning.\nWhen deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.\nIf there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted."]
    pub fn delete_swg_autogen_router_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_swg_autogen_router_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_security_policy` after provisioning.\nA fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.\nFor example: 'projects/*/locations/*/gatewaySecurityPolicies/swg-policy'.\nThis policy is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn gateway_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_security_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Gateway resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the gateway.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nShort name of the Gateway resource to be created."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe relative resource name identifying the VPC network that is using this configuration.\nFor example: 'projects/*/global/networks/network-1'.\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\nOne or more port numbers (1-65535), on which the Gateway will receive traffic.\nThe proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are\nlimited to 1 port. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 and support multiple ports."]
    pub fn ports(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nImmutable. Scope determines how configuration across multiple Gateway instances are merged.\nThe configuration for multiple Gateway instances with the same scope will be merged as presented as\na single coniguration to the proxy/load balancer.\nMax length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_tls_policy` after provisioning.\nA fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated.\nIf empty, TLS termination is disabled."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_tls_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnetwork` after provisioning.\nThe relative resource name identifying the subnetwork in which this SWG is allocated.\nFor example: 'projects/*/regions/us-central1/subnetworks/network-1'.\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nImmutable. The type of the customer-managed gateway. Possible values are: * OPEN_MESH * SECURE_WEB_GATEWAY. Possible values: [\"TYPE_UNSPECIFIED\", \"OPEN_MESH\", \"SECURE_WEB_GATEWAY\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGatewayTimeoutsElRef {
        NetworkServicesGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NetworkServicesGatewayTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkServicesGatewayTimeoutsEl {
    type O = BlockAssignable<NetworkServicesGatewayTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGatewayTimeoutsEl {}

impl BuildNetworkServicesGatewayTimeoutsEl {
    pub fn build(self) -> NetworkServicesGatewayTimeoutsEl {
        NetworkServicesGatewayTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkServicesGatewayTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkServicesGatewayTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesGatewayTimeoutsElRef {
        NetworkServicesGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGatewayTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
