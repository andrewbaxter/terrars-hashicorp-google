#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeSubnetworkData_ {
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
    allow_subnet_cidr_routes_overlap: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ipv6_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_cidr_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_google_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ipv6_google_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    purpose: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_internal_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolve_subnet_mask: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_secondary_ip_range_if_empty: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<ComputeSubnetwork_LogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeSubnetwork_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_ip_range: std::option::Option<std::vec::Vec<ComputeSubnetwork_SecondaryIpRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeSubnetwork_Timeouts>,
    dynamic: ComputeSubnetworkDynamic,
}

struct ComputeSubnetwork_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeSubnetworkData_>,
}

#[derive(Clone)]
pub struct ComputeSubnetwork(std::rc::Rc<ComputeSubnetwork_>);

impl ComputeSubnetwork {
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
        "Set the field `allow_subnet_cidr_routes_overlap`.\nTypically packets destined to IPs within the subnetwork range that do not match\nexisting resources are dropped and prevented from leaving the VPC.\nSetting this field to true will allow these packets to match dynamic routes injected\nvia BGP even if their destinations match existing subnet ranges."]
    pub fn set_allow_subnet_cidr_routes_overlap(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_subnet_cidr_routes_overlap = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource. This field can be set only at resource\ncreation time."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_ipv6_prefix`.\nThe range of external IPv6 addresses that are owned by this subnetwork. It's immutable and can only be\nspecified during creation or the first time the subnet is updated into IPV4_IPV6 dual stack. Changing this\nfield at any other time forces recreation of the subnetwork."]
    pub fn set_external_ipv6_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().external_ipv6_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `internal_ipv6_prefix`.\nThe internal IPv6 address range that is assigned to this subnetwork."]
    pub fn set_internal_ipv6_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().internal_ipv6_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_cidr_range`.\nThe range of internal addresses that are owned by this subnetwork.\nProvide this property when you create the subnetwork. For example,\n10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and\nnon-overlapping within a network. Only IPv4 is supported.\nField is optional when 'reserved_internal_range' is defined, otherwise required."]
    pub fn set_ip_cidr_range(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_cidr_range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_collection`.\nResource reference of a PublicDelegatedPrefix. The PDP must be a sub-PDP\nin EXTERNAL_IPV6_SUBNETWORK_CREATION or INTERNAL_IPV6_SUBNETWORK_CREATION\nmode. It's immutable and can only be specified during creation or the first\ntime the subnet is updated into IPV4_IPV6 dual stack. Changing this field at\nany other time forces recreation of the subnetwork.\nUse one of the following formats to specify a sub-PDP when creating\na dual stack or IPv6-only subnetwork using BYOIP:\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{sub-pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'"]
    pub fn set_ip_collection(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_collection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_access_type`.\nThe access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation\nor the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet\ncannot enable direct path. Changing this field at any other time forces recreation of the subnetwork. Possible values: [\"EXTERNAL\", \"INTERNAL\"]"]
    pub fn set_ipv6_access_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_ip_google_access`.\nWhen enabled, VMs in this subnetwork without external IP addresses can\naccess Google APIs and services by using Private Google Access."]
    pub fn set_private_ip_google_access(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_ip_google_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_ipv6_google_access`.\nThe private IPv6 google access type for the VMs in this subnet."]
    pub fn set_private_ipv6_google_access(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().private_ipv6_google_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `purpose`.\nThe purpose of the resource. This field can be either 'PRIVATE', 'REGIONAL_MANAGED_PROXY', 'GLOBAL_MANAGED_PROXY', 'PRIVATE_SERVICE_CONNECT', 'PEER_MIGRATION' or 'PRIVATE_NAT'([Beta](../guides/provider_versions.html.markdown)).\nA subnet with purpose set to 'REGIONAL_MANAGED_PROXY' is a user-created subnetwork that is reserved for regional Envoy-based load balancers.\nA subnetwork in a given region with purpose set to 'GLOBAL_MANAGED_PROXY' is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.\nA subnetwork with purpose set to 'PRIVATE_SERVICE_CONNECT' reserves the subnet for hosting a Private Service Connect published service.\nA subnetwork with purpose set to 'PEER_MIGRATION' is a user created subnetwork that is reserved for migrating resources from one peered network to another.\nA subnetwork with purpose set to 'PRIVATE_NAT' is used as source range for Private NAT gateways.\nNote that 'REGIONAL_MANAGED_PROXY' is the preferred setting for all regional Envoy load balancers.\nIf unspecified, the purpose defaults to 'PRIVATE'."]
    pub fn set_purpose(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().purpose = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe GCP region for this subnetwork."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_internal_range`.\nThe ID of the reserved internal range. Must be prefixed with 'networkconnectivity.googleapis.com'\nE.g. 'networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}'"]
    pub fn set_reserved_internal_range(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().reserved_internal_range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resolve_subnet_mask`.\n'Configures subnet mask resolution for this subnetwork.' Possible values: [\"ARP_ALL_RANGES\", \"ARP_PRIMARY_RANGE\", \"ARP_BROADCAST_PRIMARY_RANGE\", \"ARP_BROADCAST_PRIMARY_RANGE_WITH_LEARNING\"]"]
    pub fn set_resolve_subnet_mask(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().resolve_subnet_mask = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `role`.\nThe role of subnetwork.\nCurrently, this field is only used when 'purpose' is 'REGIONAL_MANAGED_PROXY' or 'GLOBAL_MANAGED_PROXY'.\nThe value can be set to 'ACTIVE' or 'BACKUP'.\nAn 'ACTIVE' subnetwork is one that is currently being used for Envoy-based load balancers in a region.\nA 'BACKUP' subnetwork is one that is ready to be promoted to 'ACTIVE' or is currently draining. Possible values: [\"ACTIVE\", \"BACKUP\"]"]
    pub fn set_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `send_secondary_ip_range_if_empty`.\nControls the removal behavior of secondary_ip_range.\nWhen false, removing secondary_ip_range from config will not produce a diff as\nthe provider will default to the API's value.\nWhen true, the provider will treat removing secondary_ip_range as sending an\nempty list of secondary IP ranges to the API.\nDefaults to false."]
    pub fn set_send_secondary_ip_range_if_empty(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().send_secondary_ip_range_if_empty = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stack_type`.\nThe stack type for this subnet to identify whether the IPv6 feature is enabled or not.\nIf not specified IPV4_ONLY will be used. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\", \"IPV6_ONLY\"]"]
    pub fn set_stack_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().stack_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSubnetwork_LogConfig>>,
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

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeSubnetwork_Params>>) -> Self {
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

    #[doc = "Set the field `secondary_ip_range`.\n"]
    pub fn set_secondary_ip_range(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSubnetwork_SecondaryIpRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_ip_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_ip_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeSubnetwork_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_subnet_cidr_routes_overlap` after provisioning.\nTypically packets destined to IPs within the subnetwork range that do not match\nexisting resources are dropped and prevented from leaving the VPC.\nSetting this field to true will allow these packets to match dynamic routes injected\nvia BGP even if their destinations match existing subnet ranges."]
    pub fn allow_subnet_cidr_routes_overlap(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_subnet_cidr_routes_overlap", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource. This field can be set only at resource\ncreation time."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_ipv6_prefix` after provisioning.\nThe range of external IPv6 addresses that are owned by this subnetwork. It's immutable and can only be\nspecified during creation or the first time the subnet is updated into IPV4_IPV6 dual stack. Changing this\nfield at any other time forces recreation of the subnetwork."]
    pub fn external_ipv6_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateway_address` after provisioning.\nThe gateway address for default routes to reach destination addresses\noutside this subnetwork."]
    pub fn gateway_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `internal_ipv6_prefix` after provisioning.\nThe internal IPv6 address range that is assigned to this subnetwork."]
    pub fn internal_ipv6_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that are owned by this subnetwork.\nProvide this property when you create the subnetwork. For example,\n10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and\nnon-overlapping within a network. Only IPv4 is supported.\nField is optional when 'reserved_internal_range' is defined, otherwise required."]
    pub fn ip_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_collection` after provisioning.\nResource reference of a PublicDelegatedPrefix. The PDP must be a sub-PDP\nin EXTERNAL_IPV6_SUBNETWORK_CREATION or INTERNAL_IPV6_SUBNETWORK_CREATION\nmode. It's immutable and can only be specified during creation or the first\ntime the subnet is updated into IPV4_IPV6 dual stack. Changing this field at\nany other time forces recreation of the subnetwork.\nUse one of the following formats to specify a sub-PDP when creating\na dual stack or IPv6-only subnetwork using BYOIP:\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{sub-pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'"]
    pub fn ip_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_access_type` after provisioning.\nThe access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation\nor the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet\ncannot enable direct path. Changing this field at any other time forces recreation of the subnetwork. Possible values: [\"EXTERNAL\", \"INTERNAL\"]"]
    pub fn ipv6_access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_cidr_range` after provisioning.\nThe range of internal IPv6 addresses that are owned by this subnetwork."]
    pub fn ipv6_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_gce_endpoint` after provisioning.\nPossible endpoints of this subnetwork. It can be one of the following:\n  * VM_ONLY: The subnetwork can be used for creating instances and IPv6 addresses with VM endpoint type. Such a subnetwork\n  gets external IPv6 ranges from a public delegated prefix and cannot be used to create NetLb.\n  * VM_AND_FR: The subnetwork can be used for creating both VM instances and Forwarding Rules. It can also be used to reserve\n  IPv6 addresses with both VM and FR endpoint types. Such a subnetwork gets its IPv6 range from Google IP Pool directly."]
    pub fn ipv6_gce_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_gce_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially\ncreating the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe network this subnet belongs to.\nOnly networks that are in the distributed mode can have subnetworks."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_google_access` after provisioning.\nWhen enabled, VMs in this subnetwork without external IP addresses can\naccess Google APIs and services by using Private Google Access."]
    pub fn private_ip_google_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_google_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe private IPv6 google access type for the VMs in this subnet."]
    pub fn private_ipv6_google_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose` after provisioning.\nThe purpose of the resource. This field can be either 'PRIVATE', 'REGIONAL_MANAGED_PROXY', 'GLOBAL_MANAGED_PROXY', 'PRIVATE_SERVICE_CONNECT', 'PEER_MIGRATION' or 'PRIVATE_NAT'([Beta](../guides/provider_versions.html.markdown)).\nA subnet with purpose set to 'REGIONAL_MANAGED_PROXY' is a user-created subnetwork that is reserved for regional Envoy-based load balancers.\nA subnetwork in a given region with purpose set to 'GLOBAL_MANAGED_PROXY' is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.\nA subnetwork with purpose set to 'PRIVATE_SERVICE_CONNECT' reserves the subnet for hosting a Private Service Connect published service.\nA subnetwork with purpose set to 'PEER_MIGRATION' is a user created subnetwork that is reserved for migrating resources from one peered network to another.\nA subnetwork with purpose set to 'PRIVATE_NAT' is used as source range for Private NAT gateways.\nNote that 'REGIONAL_MANAGED_PROXY' is the preferred setting for all regional Envoy load balancers.\nIf unspecified, the purpose defaults to 'PRIVATE'."]
    pub fn purpose(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe GCP region for this subnetwork."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_internal_range` after provisioning.\nThe ID of the reserved internal range. Must be prefixed with 'networkconnectivity.googleapis.com'\nE.g. 'networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}'"]
    pub fn reserved_internal_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_internal_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resolve_subnet_mask` after provisioning.\n'Configures subnet mask resolution for this subnetwork.' Possible values: [\"ARP_ALL_RANGES\", \"ARP_PRIMARY_RANGE\", \"ARP_BROADCAST_PRIMARY_RANGE\", \"ARP_BROADCAST_PRIMARY_RANGE_WITH_LEARNING\"]"]
    pub fn resolve_subnet_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolve_subnet_mask", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe role of subnetwork.\nCurrently, this field is only used when 'purpose' is 'REGIONAL_MANAGED_PROXY' or 'GLOBAL_MANAGED_PROXY'.\nThe value can be set to 'ACTIVE' or 'BACKUP'.\nAn 'ACTIVE' subnetwork is one that is currently being used for Envoy-based load balancers in a region.\nA 'BACKUP' subnetwork is one that is ready to be promoted to 'ACTIVE' or is currently draining. Possible values: [\"ACTIVE\", \"BACKUP\"]"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `send_secondary_ip_range_if_empty` after provisioning.\nControls the removal behavior of secondary_ip_range.\nWhen false, removing secondary_ip_range from config will not produce a diff as\nthe provider will default to the API's value.\nWhen true, the provider will treat removing secondary_ip_range as sending an\nempty list of secondary IP ranges to the API.\nDefaults to false."]
    pub fn send_secondary_ip_range_if_empty(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_secondary_ip_range_if_empty", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this subnet to identify whether the IPv6 feature is enabled or not.\nIf not specified IPV4_ONLY will be used. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\", \"IPV6_ONLY\"]"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n'The state of the subnetwork, which can be one of the following values:\n READY: Subnetwork is created and ready to use DRAINING: only applicable to subnetworks that have the purpose\n set to INTERNAL_HTTPS_LOAD_BALANCER and indicates that connections to the load balancer are being drained.\n A subnetwork that is draining cannot be used or modified until it reaches a status of READY'"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn subnetwork_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeSubnetwork_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeSubnetwork_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_ip_range` after provisioning.\n"]
    pub fn secondary_ip_range(&self) -> terrars::ListRef<ComputeSubnetwork_SecondaryIpRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_ip_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSubnetwork_TimeoutsRef {
        ComputeSubnetwork_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeSubnetwork {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeSubnetwork { }

impl terrars::ToListMappable for ComputeSubnetwork {
    type O = terrars::ListRef<ComputeSubnetworkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeSubnetwork_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_subnetwork".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeSubnetwork {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the resource, provided by the client when initially\ncreating the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The network this subnet belongs to.\nOnly networks that are in the distributed mode can have subnetworks."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildComputeSubnetwork {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeSubnetwork {
        let out = ComputeSubnetwork(std::rc::Rc::new(ComputeSubnetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeSubnetworkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_subnet_cidr_routes_overlap: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                external_ipv6_prefix: core::default::Default::default(),
                id: core::default::Default::default(),
                internal_ipv6_prefix: core::default::Default::default(),
                ip_cidr_range: core::default::Default::default(),
                ip_collection: core::default::Default::default(),
                ipv6_access_type: core::default::Default::default(),
                name: self.name,
                network: self.network,
                private_ip_google_access: core::default::Default::default(),
                private_ipv6_google_access: core::default::Default::default(),
                project: core::default::Default::default(),
                purpose: core::default::Default::default(),
                region: core::default::Default::default(),
                reserved_internal_range: core::default::Default::default(),
                resolve_subnet_mask: core::default::Default::default(),
                role: core::default::Default::default(),
                send_secondary_ip_range_if_empty: core::default::Default::default(),
                stack_type: core::default::Default::default(),
                log_config: core::default::Default::default(),
                params: core::default::Default::default(),
                secondary_ip_range: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeSubnetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSubnetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeSubnetworkRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_subnet_cidr_routes_overlap` after provisioning.\nTypically packets destined to IPs within the subnetwork range that do not match\nexisting resources are dropped and prevented from leaving the VPC.\nSetting this field to true will allow these packets to match dynamic routes injected\nvia BGP even if their destinations match existing subnet ranges."]
    pub fn allow_subnet_cidr_routes_overlap(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_subnet_cidr_routes_overlap", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource. This field can be set only at resource\ncreation time."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_ipv6_prefix` after provisioning.\nThe range of external IPv6 addresses that are owned by this subnetwork. It's immutable and can only be\nspecified during creation or the first time the subnet is updated into IPV4_IPV6 dual stack. Changing this\nfield at any other time forces recreation of the subnetwork."]
    pub fn external_ipv6_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateway_address` after provisioning.\nThe gateway address for default routes to reach destination addresses\noutside this subnetwork."]
    pub fn gateway_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `internal_ipv6_prefix` after provisioning.\nThe internal IPv6 address range that is assigned to this subnetwork."]
    pub fn internal_ipv6_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that are owned by this subnetwork.\nProvide this property when you create the subnetwork. For example,\n10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and\nnon-overlapping within a network. Only IPv4 is supported.\nField is optional when 'reserved_internal_range' is defined, otherwise required."]
    pub fn ip_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_collection` after provisioning.\nResource reference of a PublicDelegatedPrefix. The PDP must be a sub-PDP\nin EXTERNAL_IPV6_SUBNETWORK_CREATION or INTERNAL_IPV6_SUBNETWORK_CREATION\nmode. It's immutable and can only be specified during creation or the first\ntime the subnet is updated into IPV4_IPV6 dual stack. Changing this field at\nany other time forces recreation of the subnetwork.\nUse one of the following formats to specify a sub-PDP when creating\na dual stack or IPv6-only subnetwork using BYOIP:\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{sub-pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{sub-pdp-name}}'"]
    pub fn ip_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_access_type` after provisioning.\nThe access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation\nor the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet\ncannot enable direct path. Changing this field at any other time forces recreation of the subnetwork. Possible values: [\"EXTERNAL\", \"INTERNAL\"]"]
    pub fn ipv6_access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_cidr_range` after provisioning.\nThe range of internal IPv6 addresses that are owned by this subnetwork."]
    pub fn ipv6_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_gce_endpoint` after provisioning.\nPossible endpoints of this subnetwork. It can be one of the following:\n  * VM_ONLY: The subnetwork can be used for creating instances and IPv6 addresses with VM endpoint type. Such a subnetwork\n  gets external IPv6 ranges from a public delegated prefix and cannot be used to create NetLb.\n  * VM_AND_FR: The subnetwork can be used for creating both VM instances and Forwarding Rules. It can also be used to reserve\n  IPv6 addresses with both VM and FR endpoint types. Such a subnetwork gets its IPv6 range from Google IP Pool directly."]
    pub fn ipv6_gce_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_gce_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially\ncreating the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe network this subnet belongs to.\nOnly networks that are in the distributed mode can have subnetworks."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_google_access` after provisioning.\nWhen enabled, VMs in this subnetwork without external IP addresses can\naccess Google APIs and services by using Private Google Access."]
    pub fn private_ip_google_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_google_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe private IPv6 google access type for the VMs in this subnet."]
    pub fn private_ipv6_google_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose` after provisioning.\nThe purpose of the resource. This field can be either 'PRIVATE', 'REGIONAL_MANAGED_PROXY', 'GLOBAL_MANAGED_PROXY', 'PRIVATE_SERVICE_CONNECT', 'PEER_MIGRATION' or 'PRIVATE_NAT'([Beta](../guides/provider_versions.html.markdown)).\nA subnet with purpose set to 'REGIONAL_MANAGED_PROXY' is a user-created subnetwork that is reserved for regional Envoy-based load balancers.\nA subnetwork in a given region with purpose set to 'GLOBAL_MANAGED_PROXY' is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.\nA subnetwork with purpose set to 'PRIVATE_SERVICE_CONNECT' reserves the subnet for hosting a Private Service Connect published service.\nA subnetwork with purpose set to 'PEER_MIGRATION' is a user created subnetwork that is reserved for migrating resources from one peered network to another.\nA subnetwork with purpose set to 'PRIVATE_NAT' is used as source range for Private NAT gateways.\nNote that 'REGIONAL_MANAGED_PROXY' is the preferred setting for all regional Envoy load balancers.\nIf unspecified, the purpose defaults to 'PRIVATE'."]
    pub fn purpose(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe GCP region for this subnetwork."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_internal_range` after provisioning.\nThe ID of the reserved internal range. Must be prefixed with 'networkconnectivity.googleapis.com'\nE.g. 'networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}'"]
    pub fn reserved_internal_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_internal_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resolve_subnet_mask` after provisioning.\n'Configures subnet mask resolution for this subnetwork.' Possible values: [\"ARP_ALL_RANGES\", \"ARP_PRIMARY_RANGE\", \"ARP_BROADCAST_PRIMARY_RANGE\", \"ARP_BROADCAST_PRIMARY_RANGE_WITH_LEARNING\"]"]
    pub fn resolve_subnet_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolve_subnet_mask", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe role of subnetwork.\nCurrently, this field is only used when 'purpose' is 'REGIONAL_MANAGED_PROXY' or 'GLOBAL_MANAGED_PROXY'.\nThe value can be set to 'ACTIVE' or 'BACKUP'.\nAn 'ACTIVE' subnetwork is one that is currently being used for Envoy-based load balancers in a region.\nA 'BACKUP' subnetwork is one that is ready to be promoted to 'ACTIVE' or is currently draining. Possible values: [\"ACTIVE\", \"BACKUP\"]"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `send_secondary_ip_range_if_empty` after provisioning.\nControls the removal behavior of secondary_ip_range.\nWhen false, removing secondary_ip_range from config will not produce a diff as\nthe provider will default to the API's value.\nWhen true, the provider will treat removing secondary_ip_range as sending an\nempty list of secondary IP ranges to the API.\nDefaults to false."]
    pub fn send_secondary_ip_range_if_empty(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_secondary_ip_range_if_empty", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this subnet to identify whether the IPv6 feature is enabled or not.\nIf not specified IPV4_ONLY will be used. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\", \"IPV6_ONLY\"]"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n'The state of the subnetwork, which can be one of the following values:\n READY: Subnetwork is created and ready to use DRAINING: only applicable to subnetworks that have the purpose\n set to INTERNAL_HTTPS_LOAD_BALANCER and indicates that connections to the load balancer are being drained.\n A subnetwork that is draining cannot be used or modified until it reaches a status of READY'"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn subnetwork_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeSubnetwork_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeSubnetwork_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_ip_range` after provisioning.\n"]
    pub fn secondary_ip_range(&self) -> terrars::ListRef<ComputeSubnetwork_SecondaryIpRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_ip_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSubnetwork_TimeoutsRef {
        ComputeSubnetwork_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeSubnetworkDynamic {
    log_config: std::option::Option<terrars::DynamicBlock<ComputeSubnetwork_LogConfig>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeSubnetwork_Params>>,
    secondary_ip_range: std::option::Option<terrars::DynamicBlock<ComputeSubnetwork_SecondaryIpRange>>,
}

#[derive(Serialize)]
pub struct ComputeSubnetwork_LogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_expr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_sampling: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ComputeSubnetwork_LogConfig {
    #[doc =
        "Set the field `aggregation_interval`.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nToggles the aggregation interval for collecting flow logs. Increasing the\ninterval time will reduce the amount of generated flow logs for long\nlasting connections. Default is an interval of 5 seconds per connection. Default value: \"INTERVAL_5_SEC\" Possible values: [\"INTERVAL_5_SEC\", \"INTERVAL_30_SEC\", \"INTERVAL_1_MIN\", \"INTERVAL_5_MIN\", \"INTERVAL_10_MIN\", \"INTERVAL_15_MIN\"]"]
    pub fn set_aggregation_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.aggregation_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter_expr`.\nExport filter used to define which VPC flow logs should be logged, as as CEL expression. See\nhttps://cloud.google.com/vpc/docs/flow-logs#filtering for details on how to format this field.\nThe default value is 'true', which evaluates to include everything."]
    pub fn set_filter_expr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_expr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `flow_sampling`.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nThe value of the field must be in [0, 1]. Set the sampling rate of VPC\nflow logs within the subnetwork where 1.0 means all collected logs are\nreported and 0.0 means no logs are reported. Default is 0.5 which means\nhalf of all collected logs are reported."]
    pub fn set_flow_sampling(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.flow_sampling = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata`.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nConfigures whether metadata fields should be added to the reported VPC\nflow logs. Default value: \"INCLUDE_ALL_METADATA\" Possible values: [\"EXCLUDE_ALL_METADATA\", \"INCLUDE_ALL_METADATA\", \"CUSTOM_METADATA\"]"]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata_fields`.\nList of metadata fields that should be added to reported logs.\nCan only be specified if VPC flow logs for this subnetwork is enabled and \"metadata\" is set to CUSTOM_METADATA."]
    pub fn set_metadata_fields(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata_fields = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSubnetwork_LogConfig {
    type O = terrars::BlockAssignable<ComputeSubnetwork_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSubnetwork_LogConfig {}

impl BuildComputeSubnetwork_LogConfig {
    pub fn build(self) -> ComputeSubnetwork_LogConfig {
        ComputeSubnetwork_LogConfig {
            aggregation_interval: core::default::Default::default(),
            filter_expr: core::default::Default::default(),
            flow_sampling: core::default::Default::default(),
            metadata: core::default::Default::default(),
            metadata_fields: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSubnetwork_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSubnetwork_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSubnetwork_LogConfigRef {
        ComputeSubnetwork_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSubnetwork_LogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aggregation_interval` after provisioning.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nToggles the aggregation interval for collecting flow logs. Increasing the\ninterval time will reduce the amount of generated flow logs for long\nlasting connections. Default is an interval of 5 seconds per connection. Default value: \"INTERVAL_5_SEC\" Possible values: [\"INTERVAL_5_SEC\", \"INTERVAL_30_SEC\", \"INTERVAL_1_MIN\", \"INTERVAL_5_MIN\", \"INTERVAL_10_MIN\", \"INTERVAL_15_MIN\"]"]
    pub fn aggregation_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_expr` after provisioning.\nExport filter used to define which VPC flow logs should be logged, as as CEL expression. See\nhttps://cloud.google.com/vpc/docs/flow-logs#filtering for details on how to format this field.\nThe default value is 'true', which evaluates to include everything."]
    pub fn filter_expr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_expr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flow_sampling` after provisioning.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nThe value of the field must be in [0, 1]. Set the sampling rate of VPC\nflow logs within the subnetwork where 1.0 means all collected logs are\nreported and 0.0 means no logs are reported. Default is 0.5 which means\nhalf of all collected logs are reported."]
    pub fn flow_sampling(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_sampling", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nConfigures whether metadata fields should be added to the reported VPC\nflow logs. Default value: \"INCLUDE_ALL_METADATA\" Possible values: [\"EXCLUDE_ALL_METADATA\", \"INCLUDE_ALL_METADATA\", \"CUSTOM_METADATA\"]"]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata_fields` after provisioning.\nList of metadata fields that should be added to reported logs.\nCan only be specified if VPC flow logs for this subnetwork is enabled and \"metadata\" is set to CUSTOM_METADATA."]
    pub fn metadata_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.metadata_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSubnetwork_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeSubnetwork_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the subnetwork. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSubnetwork_Params {
    type O = terrars::BlockAssignable<ComputeSubnetwork_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSubnetwork_Params {}

impl BuildComputeSubnetwork_Params {
    pub fn build(self) -> ComputeSubnetwork_Params {
        ComputeSubnetwork_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeSubnetwork_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSubnetwork_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSubnetwork_ParamsRef {
        ComputeSubnetwork_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSubnetwork_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the subnetwork. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSubnetwork_SecondaryIpRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_cidr_range: std::option::Option<terrars::PrimField<std::string::String>>,
    range_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_internal_range: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSubnetwork_SecondaryIpRange {
    #[doc =
        "Set the field `ip_cidr_range`.\nThe range of IP addresses belonging to this subnetwork secondary\nrange. Provide this property when you create the subnetwork.\nRanges must be unique and non-overlapping with all primary and\nsecondary IP ranges within a network. Only IPv4 is supported.\nField is optional when 'reserved_internal_range' is defined, otherwise required."]
    pub fn set_ip_cidr_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_cidr_range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_internal_range`.\nThe ID of the reserved internal range. Must be prefixed with 'networkconnectivity.googleapis.com'\nE.g. 'networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}'"]
    pub fn set_reserved_internal_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.reserved_internal_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSubnetwork_SecondaryIpRange {
    type O = terrars::BlockAssignable<ComputeSubnetwork_SecondaryIpRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSubnetwork_SecondaryIpRange {
    #[doc =
        "The name associated with this subnetwork secondary range, used\nwhen adding an alias IP range to a VM instance. The name must\nbe 1-63 characters long, and comply with RFC1035. The name\nmust be unique within the subnetwork."]
    pub range_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeSubnetwork_SecondaryIpRange {
    pub fn build(self) -> ComputeSubnetwork_SecondaryIpRange {
        ComputeSubnetwork_SecondaryIpRange {
            ip_cidr_range: core::default::Default::default(),
            range_name: self.range_name,
            reserved_internal_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSubnetwork_SecondaryIpRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSubnetwork_SecondaryIpRangeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSubnetwork_SecondaryIpRangeRef {
        ComputeSubnetwork_SecondaryIpRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSubnetwork_SecondaryIpRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of IP addresses belonging to this subnetwork secondary\nrange. Provide this property when you create the subnetwork.\nRanges must be unique and non-overlapping with all primary and\nsecondary IP ranges within a network. Only IPv4 is supported.\nField is optional when 'reserved_internal_range' is defined, otherwise required."]
    pub fn ip_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `range_name` after provisioning.\nThe name associated with this subnetwork secondary range, used\nwhen adding an alias IP range to a VM instance. The name must\nbe 1-63 characters long, and comply with RFC1035. The name\nmust be unique within the subnetwork."]
    pub fn range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reserved_internal_range` after provisioning.\nThe ID of the reserved internal range. Must be prefixed with 'networkconnectivity.googleapis.com'\nE.g. 'networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}'"]
    pub fn reserved_internal_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_internal_range", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSubnetwork_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSubnetwork_Timeouts {
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

impl terrars::ToListMappable for ComputeSubnetwork_Timeouts {
    type O = terrars::BlockAssignable<ComputeSubnetwork_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSubnetwork_Timeouts {}

impl BuildComputeSubnetwork_Timeouts {
    pub fn build(self) -> ComputeSubnetwork_Timeouts {
        ComputeSubnetwork_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSubnetwork_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSubnetwork_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSubnetwork_TimeoutsRef {
        ComputeSubnetwork_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSubnetwork_TimeoutsRef {
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
