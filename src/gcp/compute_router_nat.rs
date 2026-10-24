use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRouterNatData {
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
    drain_nat_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dynamic_port_allocation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_endpoint_independent_mapping: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ports_per_vm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_ports_per_vm: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ip_allocate_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    router: terrars::PrimField<std::string::String>,
    source_subnetwork_ip_ranges_to_nat: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_established_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_time_wait_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_transitory_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_idle_timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<ComputeRouterNatLogConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<ComputeRouterNatRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<std::vec::Vec<ComputeRouterNatSubnetworkEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRouterNatTimeoutsEl>,
    dynamic: ComputeRouterNatDynamic,
}

struct ComputeRouterNat_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRouterNatData>,
}

#[derive(Clone)]
pub struct ComputeRouterNat(std::rc::Rc<ComputeRouterNat_>);

impl ComputeRouterNat {
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
        "Set the field `drain_nat_ips`.\nA list of URLs of the IP resources to be drained. These IPs must be\nvalid static external IPs that have been assigned to the NAT."]
    pub fn set_drain_nat_ips(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().drain_nat_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_dynamic_port_allocation`.\nEnable Dynamic Port Allocation.\nIf minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.\nIf minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.\nIf maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.\nIf maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.\n\nMutually exclusive with enableEndpointIndependentMapping."]
    pub fn set_enable_dynamic_port_allocation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dynamic_port_allocation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_endpoint_independent_mapping`.\nEnable endpoint independent mapping.\nFor more information see the [official documentation](https://cloud.google.com/nat/docs/overview#specs-rfcs)."]
    pub fn set_enable_endpoint_independent_mapping(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_endpoint_independent_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `icmp_idle_timeout_sec`.\nTimeout (in seconds) for ICMP connections. Defaults to 30s if not set."]
    pub fn set_icmp_idle_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().icmp_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_ports_per_vm`.\nMaximum number of ports allocated to a VM from this NAT.\nThis field can only be set when enableDynamicPortAllocation is enabled."]
    pub fn set_max_ports_per_vm(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_ports_per_vm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_ports_per_vm`.\nMinimum number of ports allocated to a VM from this NAT."]
    pub fn set_min_ports_per_vm(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_ports_per_vm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nat_ip_allocate_option`.\nHow external IPs should be allocated for this NAT. Valid values are\n'AUTO_ONLY' for only allowing NAT IPs allocated by Google Cloud\nPlatform, or 'MANUAL_ONLY' for only user-allocated NAT IP addresses. Possible values: [\"MANUAL_ONLY\", \"AUTO_ONLY\"]"]
    pub fn set_nat_ip_allocate_option(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().nat_ip_allocate_option = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nat_ips`.\nSelf-links of NAT IPs. Only valid if natIpAllocateOption\nis set to MANUAL_ONLY."]
    pub fn set_nat_ips(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().nat_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where the router and NAT reside."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tcp_established_idle_timeout_sec`.\nTimeout (in seconds) for TCP established connections.\nDefaults to 1200s if not set."]
    pub fn set_tcp_established_idle_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_established_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tcp_time_wait_timeout_sec`.\nTimeout (in seconds) for TCP connections that are in TIME_WAIT state.\nDefaults to 120s if not set."]
    pub fn set_tcp_time_wait_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_time_wait_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tcp_transitory_idle_timeout_sec`.\nTimeout (in seconds) for TCP transitory connections.\nDefaults to 30s if not set."]
    pub fn set_tcp_transitory_idle_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_transitory_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `udp_idle_timeout_sec`.\nTimeout (in seconds) for UDP connections. Defaults to 30s if not set."]
    pub fn set_udp_idle_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().udp_idle_timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRouterNatLogConfigEl>>,
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

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeRouterNatRulesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRouterNatSubnetworkEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnetwork = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnetwork = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRouterNatTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `drain_nat_ips` after provisioning.\nA list of URLs of the IP resources to be drained. These IPs must be\nvalid static external IPs that have been assigned to the NAT."]
    pub fn drain_nat_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drain_nat_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_dynamic_port_allocation` after provisioning.\nEnable Dynamic Port Allocation.\nIf minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.\nIf minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.\nIf maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.\nIf maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.\n\nMutually exclusive with enableEndpointIndependentMapping."]
    pub fn enable_dynamic_port_allocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dynamic_port_allocation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_endpoint_independent_mapping` after provisioning.\nEnable endpoint independent mapping.\nFor more information see the [official documentation](https://cloud.google.com/nat/docs/overview#specs-rfcs)."]
    pub fn enable_endpoint_independent_mapping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_endpoint_independent_mapping", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `icmp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for ICMP connections. Defaults to 30s if not set."]
    pub fn icmp_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_idle_timeout_sec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_ports_per_vm` after provisioning.\nMaximum number of ports allocated to a VM from this NAT.\nThis field can only be set when enableDynamicPortAllocation is enabled."]
    pub fn max_ports_per_vm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ports_per_vm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_ports_per_vm` after provisioning.\nMinimum number of ports allocated to a VM from this NAT."]
    pub fn min_ports_per_vm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ports_per_vm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the NAT service. The name must be 1-63 characters long and\ncomply with RFC1035."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_ip_allocate_option` after provisioning.\nHow external IPs should be allocated for this NAT. Valid values are\n'AUTO_ONLY' for only allowing NAT IPs allocated by Google Cloud\nPlatform, or 'MANUAL_ONLY' for only user-allocated NAT IP addresses. Possible values: [\"MANUAL_ONLY\", \"AUTO_ONLY\"]"]
    pub fn nat_ip_allocate_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ip_allocate_option", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_ips` after provisioning.\nSelf-links of NAT IPs. Only valid if natIpAllocateOption\nis set to MANUAL_ONLY."]
    pub fn nat_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.nat_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the router and NAT reside."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this NAT will be configured."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat` after provisioning.\nHow NAT should be configured per Subnetwork.\nIf 'ALL_SUBNETWORKS_ALL_IP_RANGES', all of the\nIP ranges in every Subnetwork are allowed to Nat.\nIf 'ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES', all of the primary IP\nranges in every Subnetwork are allowed to Nat.\n'LIST_OF_SUBNETWORKS': A list of Subnetworks are allowed to Nat\n(specified in the field subnetwork below). Note that if this field\ncontains ALL_SUBNETWORKS_ALL_IP_RANGES or\nALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any\nother RouterNat section in any Router for this network in this region. Possible values: [\"ALL_SUBNETWORKS_ALL_IP_RANGES\", \"ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES\", \"LIST_OF_SUBNETWORKS\"]"]
    pub fn source_subnetwork_ip_ranges_to_nat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_subnetwork_ip_ranges_to_nat", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tcp_established_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP established connections.\nDefaults to 1200s if not set."]
    pub fn tcp_established_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_established_idle_timeout_sec", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tcp_time_wait_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP connections that are in TIME_WAIT state.\nDefaults to 120s if not set."]
    pub fn tcp_time_wait_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_time_wait_timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tcp_transitory_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP transitory connections.\nDefaults to 30s if not set."]
    pub fn tcp_transitory_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_transitory_idle_timeout_sec", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `udp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for UDP connections. Defaults to 30s if not set."]
    pub fn udp_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_idle_timeout_sec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeRouterNatLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterNatTimeoutsElRef {
        ComputeRouterNatTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRouterNat {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRouterNat { }

impl terrars::ToListMappable for ComputeRouterNat {
    type O = terrars::ListRef<ComputeRouterNatRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRouterNat_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_router_nat".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRouterNat {
    pub tf_id: std::string::String,
    #[doc = "Name of the NAT service. The name must be 1-63 characters long and\ncomply with RFC1035."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Cloud Router in which this NAT will be configured."]
    pub router: terrars::PrimField<std::string::String>,
    #[doc =
        "How NAT should be configured per Subnetwork.\nIf 'ALL_SUBNETWORKS_ALL_IP_RANGES', all of the\nIP ranges in every Subnetwork are allowed to Nat.\nIf 'ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES', all of the primary IP\nranges in every Subnetwork are allowed to Nat.\n'LIST_OF_SUBNETWORKS': A list of Subnetworks are allowed to Nat\n(specified in the field subnetwork below). Note that if this field\ncontains ALL_SUBNETWORKS_ALL_IP_RANGES or\nALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any\nother RouterNat section in any Router for this network in this region. Possible values: [\"ALL_SUBNETWORKS_ALL_IP_RANGES\", \"ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES\", \"LIST_OF_SUBNETWORKS\"]"]
    pub source_subnetwork_ip_ranges_to_nat: terrars::PrimField<std::string::String>,
}

impl BuildComputeRouterNat {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRouterNat {
        let out = ComputeRouterNat(std::rc::Rc::new(ComputeRouterNat_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRouterNatData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                drain_nat_ips: core::default::Default::default(),
                enable_dynamic_port_allocation: core::default::Default::default(),
                enable_endpoint_independent_mapping: core::default::Default::default(),
                icmp_idle_timeout_sec: core::default::Default::default(),
                id: core::default::Default::default(),
                max_ports_per_vm: core::default::Default::default(),
                min_ports_per_vm: core::default::Default::default(),
                name: self.name,
                nat_ip_allocate_option: core::default::Default::default(),
                nat_ips: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: self.router,
                source_subnetwork_ip_ranges_to_nat: self.source_subnetwork_ip_ranges_to_nat,
                tcp_established_idle_timeout_sec: core::default::Default::default(),
                tcp_time_wait_timeout_sec: core::default::Default::default(),
                tcp_transitory_idle_timeout_sec: core::default::Default::default(),
                udp_idle_timeout_sec: core::default::Default::default(),
                log_config: core::default::Default::default(),
                rules: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRouterNatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterNatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRouterNatRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drain_nat_ips` after provisioning.\nA list of URLs of the IP resources to be drained. These IPs must be\nvalid static external IPs that have been assigned to the NAT."]
    pub fn drain_nat_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drain_nat_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_dynamic_port_allocation` after provisioning.\nEnable Dynamic Port Allocation.\nIf minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.\nIf minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.\nIf maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.\nIf maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.\n\nMutually exclusive with enableEndpointIndependentMapping."]
    pub fn enable_dynamic_port_allocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_dynamic_port_allocation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_endpoint_independent_mapping` after provisioning.\nEnable endpoint independent mapping.\nFor more information see the [official documentation](https://cloud.google.com/nat/docs/overview#specs-rfcs)."]
    pub fn enable_endpoint_independent_mapping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_endpoint_independent_mapping", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `icmp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for ICMP connections. Defaults to 30s if not set."]
    pub fn icmp_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_idle_timeout_sec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_ports_per_vm` after provisioning.\nMaximum number of ports allocated to a VM from this NAT.\nThis field can only be set when enableDynamicPortAllocation is enabled."]
    pub fn max_ports_per_vm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ports_per_vm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_ports_per_vm` after provisioning.\nMinimum number of ports allocated to a VM from this NAT."]
    pub fn min_ports_per_vm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ports_per_vm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the NAT service. The name must be 1-63 characters long and\ncomply with RFC1035."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_ip_allocate_option` after provisioning.\nHow external IPs should be allocated for this NAT. Valid values are\n'AUTO_ONLY' for only allowing NAT IPs allocated by Google Cloud\nPlatform, or 'MANUAL_ONLY' for only user-allocated NAT IP addresses. Possible values: [\"MANUAL_ONLY\", \"AUTO_ONLY\"]"]
    pub fn nat_ip_allocate_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ip_allocate_option", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_ips` after provisioning.\nSelf-links of NAT IPs. Only valid if natIpAllocateOption\nis set to MANUAL_ONLY."]
    pub fn nat_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.nat_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the router and NAT reside."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this NAT will be configured."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_subnetwork_ip_ranges_to_nat` after provisioning.\nHow NAT should be configured per Subnetwork.\nIf 'ALL_SUBNETWORKS_ALL_IP_RANGES', all of the\nIP ranges in every Subnetwork are allowed to Nat.\nIf 'ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES', all of the primary IP\nranges in every Subnetwork are allowed to Nat.\n'LIST_OF_SUBNETWORKS': A list of Subnetworks are allowed to Nat\n(specified in the field subnetwork below). Note that if this field\ncontains ALL_SUBNETWORKS_ALL_IP_RANGES or\nALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any\nother RouterNat section in any Router for this network in this region. Possible values: [\"ALL_SUBNETWORKS_ALL_IP_RANGES\", \"ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES\", \"LIST_OF_SUBNETWORKS\"]"]
    pub fn source_subnetwork_ip_ranges_to_nat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_subnetwork_ip_ranges_to_nat", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tcp_established_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP established connections.\nDefaults to 1200s if not set."]
    pub fn tcp_established_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_established_idle_timeout_sec", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tcp_time_wait_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP connections that are in TIME_WAIT state.\nDefaults to 120s if not set."]
    pub fn tcp_time_wait_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_time_wait_timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tcp_transitory_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for TCP transitory connections.\nDefaults to 30s if not set."]
    pub fn tcp_transitory_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_transitory_idle_timeout_sec", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `udp_idle_timeout_sec` after provisioning.\nTimeout (in seconds) for UDP connections. Defaults to 30s if not set."]
    pub fn udp_idle_timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_idle_timeout_sec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeRouterNatLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterNatTimeoutsElRef {
        ComputeRouterNatTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeRouterNatLogConfigEl {
    enable: terrars::PrimField<bool>,
    filter: terrars::PrimField<std::string::String>,
}

impl ComputeRouterNatLogConfigEl { }

impl terrars::ToListMappable for ComputeRouterNatLogConfigEl {
    type O = terrars::BlockAssignable<ComputeRouterNatLogConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterNatLogConfigEl {
    #[doc = "Indicates whether or not to export logs."]
    pub enable: terrars::PrimField<bool>,
    #[doc =
        "Specifies the desired filtering of logs on this NAT. Possible values: [\"ERRORS_ONLY\", \"TRANSLATIONS_ONLY\", \"ALL\"]"]
    pub filter: terrars::PrimField<std::string::String>,
}

impl BuildComputeRouterNatLogConfigEl {
    pub fn build(self) -> ComputeRouterNatLogConfigEl {
        ComputeRouterNatLogConfigEl {
            enable: self.enable,
            filter: self.filter,
        }
    }
}

pub struct ComputeRouterNatLogConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterNatLogConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterNatLogConfigElRef {
        ComputeRouterNatLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterNatLogConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not to export logs."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nSpecifies the desired filtering of logs on this NAT. Possible values: [\"ERRORS_ONLY\", \"TRANSLATIONS_ONLY\", \"ALL\"]"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRouterNatRulesElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_nat_active_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_nat_drain_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ComputeRouterNatRulesElActionEl {
    #[doc =
        "Set the field `source_nat_active_ips`.\nA list of URLs of the IP resources used for this NAT rule.\nThese IP addresses must be valid static external IP addresses assigned to the project.\nThis field is used for public NAT."]
    pub fn set_source_nat_active_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_nat_active_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_nat_drain_ips`.\nA list of URLs of the IP resources to be drained.\nThese IPs must be valid static external IPs that have been assigned to the NAT.\nThese IPs should be used for updating/patching a NAT rule only.\nThis field is used for public NAT."]
    pub fn set_source_nat_drain_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_nat_drain_ips = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRouterNatRulesElActionEl {
    type O = terrars::BlockAssignable<ComputeRouterNatRulesElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterNatRulesElActionEl {}

impl BuildComputeRouterNatRulesElActionEl {
    pub fn build(self) -> ComputeRouterNatRulesElActionEl {
        ComputeRouterNatRulesElActionEl {
            source_nat_active_ips: core::default::Default::default(),
            source_nat_drain_ips: core::default::Default::default(),
        }
    }
}

pub struct ComputeRouterNatRulesElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterNatRulesElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterNatRulesElActionElRef {
        ComputeRouterNatRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterNatRulesElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_nat_active_ips` after provisioning.\nA list of URLs of the IP resources used for this NAT rule.\nThese IP addresses must be valid static external IP addresses assigned to the project.\nThis field is used for public NAT."]
    pub fn source_nat_active_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_nat_active_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_nat_drain_ips` after provisioning.\nA list of URLs of the IP resources to be drained.\nThese IPs must be valid static external IPs that have been assigned to the NAT.\nThese IPs should be used for updating/patching a NAT rule only.\nThis field is used for public NAT."]
    pub fn source_nat_drain_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_nat_drain_ips", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRouterNatRulesElDynamic {
    action: std::option::Option<terrars::DynamicBlock<ComputeRouterNatRulesElActionEl>>,
}

#[derive(Serialize)]
pub struct ComputeRouterNatRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "match")]
    match_: terrars::PrimField<std::string::String>,
    rule_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<ComputeRouterNatRulesElActionEl>>,
    dynamic: ComputeRouterNatRulesElDynamic,
}

impl ComputeRouterNatRulesEl {
    #[doc = "Set the field `description`.\nAn optional description of this rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRouterNatRulesElActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRouterNatRulesEl {
    type O = terrars::BlockAssignable<ComputeRouterNatRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterNatRulesEl {
    #[doc =
        "CEL expression that specifies the match condition that egress traffic from a VM is evaluated against.\nIf it evaluates to true, the corresponding action is enforced.\n\nThe following examples are valid match expressions for public NAT:\n\n\"inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')\"\n\n\"destination.ip == '1.1.0.1' || destination.ip == '8.8.8.8'\"\n\nThe following example is a valid match expression for private NAT:\n\n\"nexthop.hub == 'https://networkconnectivity.googleapis.com/v1alpha1/projects/my-project/global/hub/hub-1'\""]
    pub match_: terrars::PrimField<std::string::String>,
    #[doc =
        "An integer uniquely identifying a rule in the list.\nThe rule number must be a positive value between 0 and 65000, and must be unique among rules within a NAT."]
    pub rule_number: terrars::PrimField<f64>,
}

impl BuildComputeRouterNatRulesEl {
    pub fn build(self) -> ComputeRouterNatRulesEl {
        ComputeRouterNatRulesEl {
            description: core::default::Default::default(),
            match_: self.match_,
            rule_number: self.rule_number,
            action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRouterNatRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterNatRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterNatRulesElRef {
        ComputeRouterNatRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterNatRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nCEL expression that specifies the match condition that egress traffic from a VM is evaluated against.\nIf it evaluates to true, the corresponding action is enforced.\n\nThe following examples are valid match expressions for public NAT:\n\n\"inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')\"\n\n\"destination.ip == '1.1.0.1' || destination.ip == '8.8.8.8'\"\n\nThe following example is a valid match expression for private NAT:\n\n\"nexthop.hub == 'https://networkconnectivity.googleapis.com/v1alpha1/projects/my-project/global/hub/hub-1'\""]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rule_number` after provisioning.\nAn integer uniquely identifying a rule in the list.\nThe rule number must be a positive value between 0 and 65000, and must be unique among rules within a NAT."]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<ComputeRouterNatRulesElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRouterNatSubnetworkEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_ip_range_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    source_ip_ranges_to_nat: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeRouterNatSubnetworkEl {
    #[doc =
        "Set the field `secondary_ip_range_names`.\nList of the secondary ranges of the subnetwork that are allowed\nto use NAT. This can be populated only if\n'LIST_OF_SECONDARY_IP_RANGES' is one of the values in\nsourceIpRangesToNat"]
    pub fn set_secondary_ip_range_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secondary_ip_range_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRouterNatSubnetworkEl {
    type O = terrars::BlockAssignable<ComputeRouterNatSubnetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterNatSubnetworkEl {
    #[doc = "Self-link of subnetwork to NAT"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "List of options for which source IPs in the subnetwork\nshould have NAT enabled. Supported values include:\n'ALL_IP_RANGES', 'LIST_OF_SECONDARY_IP_RANGES',\n'PRIMARY_IP_RANGE'."]
    pub source_ip_ranges_to_nat: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeRouterNatSubnetworkEl {
    pub fn build(self) -> ComputeRouterNatSubnetworkEl {
        ComputeRouterNatSubnetworkEl {
            name: self.name,
            secondary_ip_range_names: core::default::Default::default(),
            source_ip_ranges_to_nat: self.source_ip_ranges_to_nat,
        }
    }
}

pub struct ComputeRouterNatSubnetworkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterNatSubnetworkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterNatSubnetworkElRef {
        ComputeRouterNatSubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterNatSubnetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSelf-link of subnetwork to NAT"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secondary_ip_range_names` after provisioning.\nList of the secondary ranges of the subnetwork that are allowed\nto use NAT. This can be populated only if\n'LIST_OF_SECONDARY_IP_RANGES' is one of the values in\nsourceIpRangesToNat"]
    pub fn secondary_ip_range_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_ip_range_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_ip_ranges_to_nat` after provisioning.\nList of options for which source IPs in the subnetwork\nshould have NAT enabled. Supported values include:\n'ALL_IP_RANGES', 'LIST_OF_SECONDARY_IP_RANGES',\n'PRIMARY_IP_RANGE'."]
    pub fn source_ip_ranges_to_nat(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_ip_ranges_to_nat", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRouterNatTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRouterNatTimeoutsEl {
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

impl terrars::ToListMappable for ComputeRouterNatTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeRouterNatTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterNatTimeoutsEl {}

impl BuildComputeRouterNatTimeoutsEl {
    pub fn build(self) -> ComputeRouterNatTimeoutsEl {
        ComputeRouterNatTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeRouterNatTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterNatTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterNatTimeoutsElRef {
        ComputeRouterNatTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterNatTimeoutsElRef {
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
struct ComputeRouterNatDynamic {
    log_config: std::option::Option<terrars::DynamicBlock<ComputeRouterNatLogConfigEl>>,
    rules: std::option::Option<terrars::DynamicBlock<ComputeRouterNatRulesEl>>,
    subnetwork: std::option::Option<terrars::DynamicBlock<ComputeRouterNatSubnetworkEl>>,
}
