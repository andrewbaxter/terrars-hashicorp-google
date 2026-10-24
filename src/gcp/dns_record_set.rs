use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DnsRecordSetData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    managed_zone: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rrdatas: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policy: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyEl>>,
    dynamic: DnsRecordSetDynamic,
}

struct DnsRecordSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsRecordSetData>,
}

#[derive(Clone)]
pub struct DnsRecordSet(std::rc::Rc<DnsRecordSet_>);

impl DnsRecordSet {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rrdatas`.\nThe string data for the records in this record set whose meaning depends on the DNS type. For TXT record, if the string data contains spaces, add surrounding \\\" if you don't want your string to get split on spaces. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add \\\"\\\" inside the Terraform configuration string (e.g. \"first255characters\\\"\\\"morecharacters\")."]
    pub fn set_rrdatas(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().rrdatas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\nThe time-to-live of this record set (seconds)."]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_policy`.\n"]
    pub fn set_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_zone` after provisioning.\nThe name of the zone in which this record set will reside."]
    pub fn managed_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe DNS name this record set will apply to."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rrdatas` after provisioning.\nThe string data for the records in this record set whose meaning depends on the DNS type. For TXT record, if the string data contains spaces, add surrounding \\\" if you don't want your string to get split on spaces. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add \\\"\\\" inside the Terraform configuration string (e.g. \"first255characters\\\"\\\"morecharacters\")."]
    pub fn rrdatas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe time-to-live of this record set (seconds)."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe DNS record set type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_policy", self.extract_ref()))
    }
}

impl terrars::Referable for DnsRecordSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DnsRecordSet { }

impl terrars::ToListMappable for DnsRecordSet {
    type O = terrars::ListRef<DnsRecordSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DnsRecordSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dns_record_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDnsRecordSet {
    pub tf_id: std::string::String,
    #[doc = "The name of the zone in which this record set will reside."]
    pub managed_zone: terrars::PrimField<std::string::String>,
    #[doc = "The DNS name this record set will apply to."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The DNS record set type."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecordSet {
    pub fn build(self, stack: &mut terrars::Stack) -> DnsRecordSet {
        let out = DnsRecordSet(std::rc::Rc::new(DnsRecordSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DnsRecordSetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                managed_zone: self.managed_zone,
                name: self.name,
                project: core::default::Default::default(),
                rrdatas: core::default::Default::default(),
                ttl: core::default::Default::default(),
                type_: self.type_,
                routing_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DnsRecordSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DnsRecordSetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_zone` after provisioning.\nThe name of the zone in which this record set will reside."]
    pub fn managed_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe DNS name this record set will apply to."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rrdatas` after provisioning.\nThe string data for the records in this record set whose meaning depends on the DNS type. For TXT record, if the string data contains spaces, add surrounding \\\" if you don't want your string to get split on spaces. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add \\\"\\\" inside the Terraform configuration string (e.g. \"first255characters\\\"\\\"morecharacters\")."]
    pub fn rrdatas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe time-to-live of this record set (seconds)."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe DNS record set type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    ip_address: terrars::PrimField<std::string::String>,
    ip_protocol: terrars::PrimField<std::string::String>,
    load_balancer_type: terrars::PrimField<std::string::String>,
    network_url: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<std::string::String>,
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: terrars::PrimField<std::string::String>,
    #[doc =
        "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub load_balancer_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: terrars::PrimField<std::string::String>,
    #[doc = "The configured port of the load balancer."]
    pub port: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
            ip_address: self.ip_address,
            ip_protocol: self.ip_protocol,
            load_balancer_type: self.load_balancer_type,
            network_url: self.network_url,
            port: self.port,
            project: self.project,
            region: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn load_balancer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElDynamic {
    internal_load_balancers: std
    ::option
    ::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_load_balancers: std
    ::option
    ::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl>>,
    dynamic: DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElDynamic,
}

impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.internal_load_balancers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {}

impl BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
            internal_load_balancers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]
    pub fn internal_load_balancers(
        &self,
    ) -> terrars::ListRef<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internal_load_balancers", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElGeoElDynamic {
    health_checked_targets: std
    ::option
    ::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElGeoEl {
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rrdatas: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_checked_targets: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>>,
    dynamic: DnsRecordSetRoutingPolicyElGeoElDynamic,
}

impl DnsRecordSetRoutingPolicyElGeoEl {
    #[doc = "Set the field `rrdatas`.\n"]
    pub fn set_rrdatas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rrdatas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_checked_targets`.\n"]
    pub fn set_health_checked_targets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_checked_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_checked_targets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElGeoEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElGeoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElGeoEl {
    #[doc = "The location name defined in Google Cloud."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecordSetRoutingPolicyElGeoEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElGeoEl {
        DnsRecordSetRoutingPolicyElGeoEl {
            location: self.location,
            rrdatas: core::default::Default::default(),
            health_checked_targets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElGeoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElGeoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsRecordSetRoutingPolicyElGeoElRef {
        DnsRecordSetRoutingPolicyElGeoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElGeoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location name defined in Google Cloud."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\n"]
    pub fn rrdatas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }

    #[doc = "Get a reference to the value of field `health_checked_targets` after provisioning.\n"]
    pub fn health_checked_targets(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_checked_targets", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    ip_address: terrars::PrimField<std::string::String>,
    ip_protocol: terrars::PrimField<std::string::String>,
    load_balancer_type: terrars::PrimField<std::string::String>,
    network_url: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<std::string::String>,
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    type O =
        terrars
        ::BlockAssignable<
            DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: terrars::PrimField<std::string::String>,
    #[doc =
        "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub load_balancer_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: terrars::PrimField<std::string::String>,
    #[doc = "The configured port of the load balancer."]
    pub port: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    pub fn build(
        self,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
            ip_address: self.ip_address,
            ip_protocol: self.ip_protocol,
            load_balancer_type: self.load_balancer_type,
            network_url: self.network_url,
            port: self.port,
            project: self.project,
            region: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn load_balancer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElDynamic {
    internal_load_balancers: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_load_balancers: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl>,
    >,
    dynamic: DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElDynamic,
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.internal_load_balancers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {}

impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
            internal_load_balancers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]
    pub fn internal_load_balancers(
        &self,
    ) -> terrars
    ::ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internal_load_balancers", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElDynamic {
    health_checked_targets: std
    ::option
    ::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rrdatas: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_checked_targets: std
    ::option
    ::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl>>,
    dynamic: DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElDynamic,
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    #[doc = "Set the field `rrdatas`.\n"]
    pub fn set_rrdatas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rrdatas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_checked_targets`.\n"]
    pub fn set_health_checked_targets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_checked_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_checked_targets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    #[doc = "The location name defined in Google Cloud."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
            location: self.location,
            rrdatas: core::default::Default::default(),
            health_checked_targets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location name defined in Google Cloud."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\n"]
    pub fn rrdatas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }

    #[doc = "Get a reference to the value of field `health_checked_targets` after provisioning.\n"]
    pub fn health_checked_targets(
        &self,
    ) -> terrars::ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_checked_targets", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    ip_address: terrars::PrimField<std::string::String>,
    ip_protocol: terrars::PrimField<std::string::String>,
    load_balancer_type: terrars::PrimField<std::string::String>,
    network_url: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<std::string::String>,
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    #[doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: terrars::PrimField<std::string::String>,
    #[doc =
        "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub load_balancer_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: terrars::PrimField<std::string::String>,
    #[doc = "The configured port of the load balancer."]
    pub port: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
            ip_address: self.ip_address,
            ip_protocol: self.ip_protocol,
            load_balancer_type: self.load_balancer_type,
            network_url: self.network_url,
            port: self.port,
            project: self.project,
            region: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn load_balancer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElDynamic {
    internal_load_balancers: std
    ::option
    ::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_load_balancers: std
    ::option
    ::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl>>,
    dynamic: DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElDynamic,
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.internal_load_balancers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {}

impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
            internal_load_balancers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]
    pub fn internal_load_balancers(
        &self,
    ) -> terrars::ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internal_load_balancers", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElDynamic {
    backup_geo: std::option::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>>,
    primary: std::option::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_geo_fencing_for_backups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trickle_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_geo: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>>,
    dynamic: DnsRecordSetRoutingPolicyElPrimaryBackupElDynamic,
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupEl {
    #[doc =
        "Set the field `enable_geo_fencing_for_backups`.\nSpecifies whether to enable fencing for backup geo queries."]
    pub fn set_enable_geo_fencing_for_backups(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_geo_fencing_for_backups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trickle_ratio`.\nSpecifies the percentage of traffic to send to the backup targets even when the primary targets are healthy."]
    pub fn set_trickle_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.trickle_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_geo`.\n"]
    pub fn set_backup_geo(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backup_geo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_geo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.primary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.primary = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupEl {}

impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupEl {
            enable_geo_fencing_for_backups: core::default::Default::default(),
            trickle_ratio: core::default::Default::default(),
            backup_geo: core::default::Default::default(),
            primary: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_geo_fencing_for_backups` after provisioning.\nSpecifies whether to enable fencing for backup geo queries."]
    pub fn enable_geo_fencing_for_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_geo_fencing_for_backups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trickle_ratio` after provisioning.\nSpecifies the percentage of traffic to send to the backup targets even when the primary targets are healthy."]
    pub fn trickle_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trickle_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_geo` after provisioning.\n"]
    pub fn backup_geo(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_geo", self.base))
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    ip_address: terrars::PrimField<std::string::String>,
    ip_protocol: terrars::PrimField<std::string::String>,
    load_balancer_type: terrars::PrimField<std::string::String>,
    network_url: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<std::string::String>,
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: terrars::PrimField<std::string::String>,
    #[doc =
        "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub load_balancer_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: terrars::PrimField<std::string::String>,
    #[doc = "The configured port of the load balancer."]
    pub port: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
            ip_address: self.ip_address,
            ip_protocol: self.ip_protocol,
            load_balancer_type: self.load_balancer_type,
            network_url: self.network_url,
            port: self.port,
            project: self.project,
            region: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn load_balancer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElDynamic {
    internal_load_balancers: std
    ::option
    ::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_load_balancers: std
    ::option
    ::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl>>,
    dynamic: DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElDynamic,
}

impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.internal_load_balancers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {}

impl BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
            internal_load_balancers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]
    pub fn internal_load_balancers(
        &self,
    ) -> terrars::ListRef<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internal_load_balancers", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElWrrElDynamic {
    health_checked_targets: std
    ::option
    ::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElWrrEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rrdatas: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    weight: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_checked_targets: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>>,
    dynamic: DnsRecordSetRoutingPolicyElWrrElDynamic,
}

impl DnsRecordSetRoutingPolicyElWrrEl {
    #[doc = "Set the field `rrdatas`.\n"]
    pub fn set_rrdatas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rrdatas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_checked_targets`.\n"]
    pub fn set_health_checked_targets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_checked_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_checked_targets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyElWrrEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyElWrrEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyElWrrEl {
    #[doc = "The ratio of traffic routed to the target."]
    pub weight: terrars::PrimField<f64>,
}

impl BuildDnsRecordSetRoutingPolicyElWrrEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElWrrEl {
        DnsRecordSetRoutingPolicyElWrrEl {
            rrdatas: core::default::Default::default(),
            weight: self.weight,
            health_checked_targets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElWrrElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElWrrElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsRecordSetRoutingPolicyElWrrElRef {
        DnsRecordSetRoutingPolicyElWrrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElWrrElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\n"]
    pub fn rrdatas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nThe ratio of traffic routed to the target."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `health_checked_targets` after provisioning.\n"]
    pub fn health_checked_targets(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_checked_targets", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElDynamic {
    geo: std::option::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElGeoEl>>,
    primary_backup: std::option::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupEl>>,
    wrr: std::option::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyElWrrEl>>,
}

#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_geo_fencing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElGeoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_backup: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElPrimaryBackupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wrr: std::option::Option<std::vec::Vec<DnsRecordSetRoutingPolicyElWrrEl>>,
    dynamic: DnsRecordSetRoutingPolicyElDynamic,
}

impl DnsRecordSetRoutingPolicyEl {
    #[doc = "Set the field `enable_geo_fencing`.\nSpecifies whether to enable fencing for geo queries."]
    pub fn set_enable_geo_fencing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_geo_fencing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyElGeoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `primary_backup`.\n"]
    pub fn set_primary_backup(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.primary_backup = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_backup = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `wrr`.\n"]
    pub fn set_wrr(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsRecordSetRoutingPolicyElWrrEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.wrr = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.wrr = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsRecordSetRoutingPolicyEl {
    type O = terrars::BlockAssignable<DnsRecordSetRoutingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsRecordSetRoutingPolicyEl {}

impl BuildDnsRecordSetRoutingPolicyEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyEl {
        DnsRecordSetRoutingPolicyEl {
            enable_geo_fencing: core::default::Default::default(),
            geo: core::default::Default::default(),
            primary_backup: core::default::Default::default(),
            wrr: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsRecordSetRoutingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsRecordSetRoutingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsRecordSetRoutingPolicyElRef {
        DnsRecordSetRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsRecordSetRoutingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_geo_fencing` after provisioning.\nSpecifies whether to enable fencing for geo queries."]
    pub fn enable_geo_fencing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_geo_fencing", self.base))
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElGeoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_backup` after provisioning.\n"]
    pub fn primary_backup(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `wrr` after provisioning.\n"]
    pub fn wrr(&self) -> terrars::ListRef<DnsRecordSetRoutingPolicyElWrrElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wrr", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsRecordSetDynamic {
    routing_policy: std::option::Option<terrars::DynamicBlock<DnsRecordSetRoutingPolicyEl>>,
}
