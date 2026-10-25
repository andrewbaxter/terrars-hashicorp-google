#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeRoutersData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeRouters_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeRoutersData_>,
}

#[derive(Clone)]
pub struct DataComputeRouters(std::rc::Rc<DataComputeRouters_>);

impl DataComputeRouters {
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

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routers` after provisioning.\n"]
    pub fn routers(&self) -> terrars::ListRef<DataComputeRouters_RoutersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routers", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeRouters {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeRouters { }

impl terrars::ToListMappable for DataComputeRouters {
    type O = terrars::ListRef<DataComputeRoutersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeRouters_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_routers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeRouters {
    pub tf_id: std::string::String,
}

impl BuildDataComputeRouters {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeRouters {
        let out = DataComputeRouters(std::rc::Rc::new(DataComputeRouters_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeRoutersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeRoutersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRoutersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeRoutersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routers` after provisioning.\n"]
    pub fn routers(&self) -> terrars::ListRef<DataComputeRouters_RoutersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouters_Routers_Bgp_AdvertisedIpRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouters_Routers_Bgp_AdvertisedIpRanges {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouters_Routers_Bgp_AdvertisedIpRanges {
    type O = terrars::BlockAssignable<DataComputeRouters_Routers_Bgp_AdvertisedIpRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouters_Routers_Bgp_AdvertisedIpRanges {}

impl BuildDataComputeRouters_Routers_Bgp_AdvertisedIpRanges {
    pub fn build(self) -> DataComputeRouters_Routers_Bgp_AdvertisedIpRanges {
        DataComputeRouters_Routers_Bgp_AdvertisedIpRanges {
            description: core::default::Default::default(),
            range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouters_Routers_Bgp_AdvertisedIpRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouters_Routers_Bgp_AdvertisedIpRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRouters_Routers_Bgp_AdvertisedIpRangesRef {
        DataComputeRouters_Routers_Bgp_AdvertisedIpRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouters_Routers_Bgp_AdvertisedIpRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouters_Routers_Bgp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertise_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertised_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertised_ip_ranges: std::option::Option<terrars::ListField<DataComputeRouters_Routers_Bgp_AdvertisedIpRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keepalive_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeRouters_Routers_Bgp {
    #[doc = "Set the field `advertise_mode`.\n"]
    pub fn set_advertise_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.advertise_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advertised_groups`.\n"]
    pub fn set_advertised_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.advertised_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advertised_ip_ranges`.\n"]
    pub fn set_advertised_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouters_Routers_Bgp_AdvertisedIpRanges>>,
    ) -> Self {
        self.advertised_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keepalive_interval`.\n"]
    pub fn set_keepalive_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keepalive_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouters_Routers_Bgp {
    type O = terrars::BlockAssignable<DataComputeRouters_Routers_Bgp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouters_Routers_Bgp {}

impl BuildDataComputeRouters_Routers_Bgp {
    pub fn build(self) -> DataComputeRouters_Routers_Bgp {
        DataComputeRouters_Routers_Bgp {
            advertise_mode: core::default::Default::default(),
            advertised_groups: core::default::Default::default(),
            advertised_ip_ranges: core::default::Default::default(),
            asn: core::default::Default::default(),
            keepalive_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouters_Routers_BgpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouters_Routers_BgpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouters_Routers_BgpRef {
        DataComputeRouters_Routers_BgpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouters_Routers_BgpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\n"]
    pub fn advertise_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertise_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `advertised_groups` after provisioning.\n"]
    pub fn advertised_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advertised_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `advertised_ip_ranges` after provisioning.\n"]
    pub fn advertised_ip_ranges(&self) -> terrars::ListRef<DataComputeRouters_Routers_Bgp_AdvertisedIpRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advertised_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `keepalive_interval` after provisioning.\n"]
    pub fn keepalive_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keepalive_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouters_Routers_BgpPeers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertise_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertised_route_priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ipv6: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouters_Routers_BgpPeers {
    #[doc = "Set the field `advertise_mode`.\n"]
    pub fn set_advertise_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.advertise_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advertised_route_priority`.\n"]
    pub fn set_advertised_route_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.advertised_route_priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ipv6`.\n"]
    pub fn set_enable_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_name`.\n"]
    pub fn set_interface_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management_type`.\n"]
    pub fn set_management_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.management_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_asn`.\n"]
    pub fn set_peer_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.peer_asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_ip_address`.\n"]
    pub fn set_peer_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_ip_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouters_Routers_BgpPeers {
    type O = terrars::BlockAssignable<DataComputeRouters_Routers_BgpPeers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouters_Routers_BgpPeers {}

impl BuildDataComputeRouters_Routers_BgpPeers {
    pub fn build(self) -> DataComputeRouters_Routers_BgpPeers {
        DataComputeRouters_Routers_BgpPeers {
            advertise_mode: core::default::Default::default(),
            advertised_route_priority: core::default::Default::default(),
            enable: core::default::Default::default(),
            enable_ipv6: core::default::Default::default(),
            interface_name: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            management_type: core::default::Default::default(),
            name: core::default::Default::default(),
            peer_asn: core::default::Default::default(),
            peer_ip_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouters_Routers_BgpPeersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouters_Routers_BgpPeersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouters_Routers_BgpPeersRef {
        DataComputeRouters_Routers_BgpPeersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouters_Routers_BgpPeersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\n"]
    pub fn advertise_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertise_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `advertised_route_priority` after provisioning.\n"]
    pub fn advertised_route_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertised_route_priority", self.base))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_ipv6` after provisioning.\n"]
    pub fn enable_ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `management_type` after provisioning.\n"]
    pub fn management_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\n"]
    pub fn peer_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_ip_address` after provisioning.\n"]
    pub fn peer_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouters_Routers_Interfaces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_interconnect_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_vpn_tunnel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redundant_interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouters_Routers_Interfaces {
    #[doc = "Set the field `ip_range`.\n"]
    pub fn set_ip_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_interconnect_attachment`.\n"]
    pub fn set_linked_interconnect_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.linked_interconnect_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_vpn_tunnel`.\n"]
    pub fn set_linked_vpn_tunnel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.linked_vpn_tunnel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redundant_interface`.\n"]
    pub fn set_redundant_interface(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redundant_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouters_Routers_Interfaces {
    type O = terrars::BlockAssignable<DataComputeRouters_Routers_Interfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouters_Routers_Interfaces {}

impl BuildDataComputeRouters_Routers_Interfaces {
    pub fn build(self) -> DataComputeRouters_Routers_Interfaces {
        DataComputeRouters_Routers_Interfaces {
            ip_range: core::default::Default::default(),
            linked_interconnect_attachment: core::default::Default::default(),
            linked_vpn_tunnel: core::default::Default::default(),
            name: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            redundant_interface: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouters_Routers_InterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouters_Routers_InterfacesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouters_Routers_InterfacesRef {
        DataComputeRouters_Routers_InterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouters_Routers_InterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\n"]
    pub fn ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_interconnect_attachment` after provisioning.\n"]
    pub fn linked_interconnect_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_interconnect_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_vpn_tunnel` after provisioning.\n"]
    pub fn linked_vpn_tunnel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_vpn_tunnel", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `redundant_interface` after provisioning.\n"]
    pub fn redundant_interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redundant_interface", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouters_Routers_Md5AuthenticationKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouters_Routers_Md5AuthenticationKeys {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouters_Routers_Md5AuthenticationKeys {
    type O = terrars::BlockAssignable<DataComputeRouters_Routers_Md5AuthenticationKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouters_Routers_Md5AuthenticationKeys {}

impl BuildDataComputeRouters_Routers_Md5AuthenticationKeys {
    pub fn build(self) -> DataComputeRouters_Routers_Md5AuthenticationKeys {
        DataComputeRouters_Routers_Md5AuthenticationKeys {
            key: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouters_Routers_Md5AuthenticationKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouters_Routers_Md5AuthenticationKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRouters_Routers_Md5AuthenticationKeysRef {
        DataComputeRouters_Routers_Md5AuthenticationKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouters_Routers_Md5AuthenticationKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouters_Routers_Nats {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_endpoint_independent_mapping: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_ports_per_vm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ip_allocate_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_subnetwork_ip_ranges_to_nat: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_established_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_transitory_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeRouters_Routers_Nats {
    #[doc = "Set the field `enable_endpoint_independent_mapping`.\n"]
    pub fn set_enable_endpoint_independent_mapping(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_endpoint_independent_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_idle_timeout_sec`.\n"]
    pub fn set_icmp_idle_timeout_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.icmp_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_ports_per_vm`.\n"]
    pub fn set_min_ports_per_vm(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_ports_per_vm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_ip_allocate_option`.\n"]
    pub fn set_nat_ip_allocate_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nat_ip_allocate_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_ips`.\n"]
    pub fn set_nat_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.nat_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_subnetwork_ip_ranges_to_nat`.\n"]
    pub fn set_source_subnetwork_ip_ranges_to_nat(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_subnetwork_ip_ranges_to_nat = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_established_idle_timeout_sec`.\n"]
    pub fn set_tcp_established_idle_timeout_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tcp_established_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_transitory_idle_timeout_sec`.\n"]
    pub fn set_tcp_transitory_idle_timeout_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tcp_transitory_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `udp_idle_timeout_sec`.\n"]
    pub fn set_udp_idle_timeout_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.udp_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouters_Routers_Nats {
    type O = terrars::BlockAssignable<DataComputeRouters_Routers_Nats>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouters_Routers_Nats {}

impl BuildDataComputeRouters_Routers_Nats {
    pub fn build(self) -> DataComputeRouters_Routers_Nats {
        DataComputeRouters_Routers_Nats {
            enable_endpoint_independent_mapping: core::default::Default::default(),
            icmp_idle_timeout_sec: core::default::Default::default(),
            min_ports_per_vm: core::default::Default::default(),
            name: core::default::Default::default(),
            nat_ip_allocate_option: core::default::Default::default(),
            nat_ips: core::default::Default::default(),
            source_subnetwork_ip_ranges_to_nat: core::default::Default::default(),
            tcp_established_idle_timeout_sec: core::default::Default::default(),
            tcp_transitory_idle_timeout_sec: core::default::Default::default(),
            udp_idle_timeout_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouters_Routers_NatsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouters_Routers_NatsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouters_Routers_NatsRef {
        DataComputeRouters_Routers_NatsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouters_Routers_NatsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_endpoint_independent_mapping` after provisioning.\n"]
    pub fn enable_endpoint_independent_mapping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_endpoint_independent_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_idle_timeout_sec` after provisioning.\n"]
    pub fn icmp_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_idle_timeout_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `min_ports_per_vm` after provisioning.\n"]
    pub fn min_ports_per_vm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ports_per_vm", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_ip_allocate_option` after provisioning.\n"]
    pub fn nat_ip_allocate_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ip_allocate_option", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_ips` after provisioning.\n"]
    pub fn nat_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nat_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat` after provisioning.\n"]
    pub fn source_subnetwork_ip_ranges_to_nat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_subnetwork_ip_ranges_to_nat", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_established_idle_timeout_sec` after provisioning.\n"]
    pub fn tcp_established_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_established_idle_timeout_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_transitory_idle_timeout_sec` after provisioning.\n"]
    pub fn tcp_transitory_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_transitory_idle_timeout_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_idle_timeout_sec` after provisioning.\n"]
    pub fn udp_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_idle_timeout_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouters_Routers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp: std::option::Option<terrars::ListField<DataComputeRouters_Routers_Bgp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_peers: std::option::Option<terrars::ListField<DataComputeRouters_Routers_BgpPeers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted_interconnect_router: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interfaces: std::option::Option<terrars::ListField<DataComputeRouters_Routers_Interfaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    md5_authentication_keys: std::option::Option<terrars::ListField<DataComputeRouters_Routers_Md5AuthenticationKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nats: std::option::Option<terrars::ListField<DataComputeRouters_Routers_Nats>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouters_Routers {
    #[doc = "Set the field `bgp`.\n"]
    pub fn set_bgp(mut self, v: impl std::convert::Into<terrars::ListField<DataComputeRouters_Routers_Bgp>>) -> Self {
        self.bgp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgp_peers`.\n"]
    pub fn set_bgp_peers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouters_Routers_BgpPeers>>,
    ) -> Self {
        self.bgp_peers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.creation_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted_interconnect_router`.\n"]
    pub fn set_encrypted_interconnect_router(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted_interconnect_router = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interfaces`.\n"]
    pub fn set_interfaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouters_Routers_Interfaces>>,
    ) -> Self {
        self.interfaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `md5_authentication_keys`.\n"]
    pub fn set_md5_authentication_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouters_Routers_Md5AuthenticationKeys>>,
    ) -> Self {
        self.md5_authentication_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nats`.\n"]
    pub fn set_nats(mut self, v: impl std::convert::Into<terrars::ListField<DataComputeRouters_Routers_Nats>>) -> Self {
        self.nats = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.self_link = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouters_Routers {
    type O = terrars::BlockAssignable<DataComputeRouters_Routers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouters_Routers {}

impl BuildDataComputeRouters_Routers {
    pub fn build(self) -> DataComputeRouters_Routers {
        DataComputeRouters_Routers {
            bgp: core::default::Default::default(),
            bgp_peers: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            description: core::default::Default::default(),
            encrypted_interconnect_router: core::default::Default::default(),
            interfaces: core::default::Default::default(),
            md5_authentication_keys: core::default::Default::default(),
            name: core::default::Default::default(),
            nats: core::default::Default::default(),
            network: core::default::Default::default(),
            self_link: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouters_RoutersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouters_RoutersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouters_RoutersRef {
        DataComputeRouters_RoutersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouters_RoutersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> terrars::ListRef<DataComputeRouters_Routers_BgpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_peers` after provisioning.\n"]
    pub fn bgp_peers(&self) -> terrars::ListRef<DataComputeRouters_Routers_BgpPeersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_peers", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted_interconnect_router` after provisioning.\n"]
    pub fn encrypted_interconnect_router(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted_interconnect_router", self.base))
    }

    #[doc = "Get a reference to the value of field `interfaces` after provisioning.\n"]
    pub fn interfaces(&self) -> terrars::ListRef<DataComputeRouters_Routers_InterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interfaces", self.base))
    }

    #[doc = "Get a reference to the value of field `md5_authentication_keys` after provisioning.\n"]
    pub fn md5_authentication_keys(&self) -> terrars::ListRef<DataComputeRouters_Routers_Md5AuthenticationKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.md5_authentication_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `nats` after provisioning.\n"]
    pub fn nats(&self) -> terrars::ListRef<DataComputeRouters_Routers_NatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nats", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
}
