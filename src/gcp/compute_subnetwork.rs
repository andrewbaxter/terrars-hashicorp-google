use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeSubnetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ip_cidr_range: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_type: Option<PrimField<String>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_google_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ipv6_google_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_ip_range: Option<ListField<ComputeSubnetworkSecondaryIpRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<ComputeSubnetworkLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeSubnetworkTimeoutsEl>,
    dynamic: ComputeSubnetworkDynamic,
}

struct ComputeSubnetwork_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeSubnetworkData>,
}

#[derive(Clone)]
pub struct ComputeSubnetwork(Rc<ComputeSubnetwork_>);

impl ComputeSubnetwork {
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

    #[doc= "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource. This field can be set only at resource\ncreation time."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `external_ipv6_prefix`.\nThe range of external IPv6 addresses that are owned by this subnetwork."]
    pub fn set_external_ipv6_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_ipv6_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_access_type`.\nThe access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation\nor the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet\ncannot enable direct path. Possible values: [\"EXTERNAL\", \"INTERNAL\"]"]
    pub fn set_ipv6_access_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv6_access_type = Some(v.into());
        self
    }

    #[doc= "Set the field `private_ip_google_access`.\nWhen enabled, VMs in this subnetwork without external IP addresses can\naccess Google APIs and services by using Private Google Access."]
    pub fn set_private_ip_google_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_ip_google_access = Some(v.into());
        self
    }

    #[doc= "Set the field `private_ipv6_google_access`.\nThe private IPv6 google access type for the VMs in this subnet."]
    pub fn set_private_ipv6_google_access(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_ipv6_google_access = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `purpose`.\nThe purpose of the resource. This field can be either 'PRIVATE_RFC_1918', 'REGIONAL_MANAGED_PROXY', 'GLOBAL_MANAGED_PROXY', 'PRIVATE_SERVICE_CONNECT' or 'PRIVATE_NAT'([Beta](https://terraform.io/docs/providers/google/guides/provider_versions.html)).\nA subnet with purpose set to 'REGIONAL_MANAGED_PROXY' is a user-created subnetwork that is reserved for regional Envoy-based load balancers.\nA subnetwork in a given region with purpose set to 'GLOBAL_MANAGED_PROXY' is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.\nA subnetwork with purpose set to 'PRIVATE_SERVICE_CONNECT' reserves the subnet for hosting a Private Service Connect published service.\nA subnetwork with purpose set to 'PRIVATE_NAT' is used as source range for Private NAT gateways.\nNote that 'REGIONAL_MANAGED_PROXY' is the preferred setting for all regional Envoy load balancers.\nIf unspecified, the purpose defaults to 'PRIVATE_RFC_1918'."]
    pub fn set_purpose(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().purpose = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe GCP region for this subnetwork."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `role`.\nThe role of subnetwork.\nCurrently, this field is only used when 'purpose' is 'REGIONAL_MANAGED_PROXY'.\nThe value can be set to 'ACTIVE' or 'BACKUP'.\nAn 'ACTIVE' subnetwork is one that is currently being used for Envoy-based load balancers in a region.\nA 'BACKUP' subnetwork is one that is ready to be promoted to 'ACTIVE' or is currently draining. Possible values: [\"ACTIVE\", \"BACKUP\"]"]
    pub fn set_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role = Some(v.into());
        self
    }

    #[doc= "Set the field `secondary_ip_range`.\nAn array of configurations for secondary IP ranges for VM instances\ncontained in this subnetwork. The primary IP of such VM must belong\nto the primary ipCidrRange of the subnetwork. The alias IPs may belong\nto either primary or secondary ranges.\n\n**Note**: This field uses [attr-as-block mode](https://www.terraform.io/docs/configuration/attr-as-blocks.html) to avoid\nbreaking users during the 0.12 upgrade. To explicitly send a list\nof zero objects you must use the following syntax:\n'example=[]'\nFor more details about this behavior, see [this section](https://www.terraform.io/docs/configuration/attr-as-blocks.html#defining-a-fixed-object-collection-value)."]
    pub fn set_secondary_ip_range(self, v: impl Into<ListField<ComputeSubnetworkSecondaryIpRangeEl>>) -> Self {
        self.0.data.borrow_mut().secondary_ip_range = Some(v.into());
        self
    }

    #[doc= "Set the field `stack_type`.\nThe stack type for this subnet to identify whether the IPv6 feature is enabled or not.\nIf not specified IPV4_ONLY will be used. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn set_stack_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stack_type = Some(v.into());
        self
    }

    #[doc= "Set the field `log_config`.\n"]
    pub fn set_log_config(self, v: impl Into<BlockAssignable<ComputeSubnetworkLogConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeSubnetworkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource. This field can be set only at resource\ncreation time."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_ipv6_prefix` after provisioning.\nThe range of external IPv6 addresses that are owned by this subnetwork."]
    pub fn external_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_address` after provisioning.\nThe gateway address for default routes to reach destination addresses\noutside this subnetwork."]
    pub fn gateway_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal_ipv6_prefix` after provisioning.\nThe internal IPv6 address range that is assigned to this subnetwork."]
    pub fn internal_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that are owned by this subnetwork.\nProvide this property when you create the subnetwork. For example,\n10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and\nnon-overlapping within a network. Only IPv4 is supported."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_access_type` after provisioning.\nThe access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation\nor the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet\ncannot enable direct path. Possible values: [\"EXTERNAL\", \"INTERNAL\"]"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr_range` after provisioning.\nThe range of internal IPv6 addresses that are owned by this subnetwork."]
    pub fn ipv6_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially\ncreating the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe network this subnet belongs to.\nOnly networks that are in the distributed mode can have subnetworks."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip_google_access` after provisioning.\nWhen enabled, VMs in this subnetwork without external IP addresses can\naccess Google APIs and services by using Private Google Access."]
    pub fn private_ip_google_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_google_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe private IPv6 google access type for the VMs in this subnet."]
    pub fn private_ipv6_google_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose` after provisioning.\nThe purpose of the resource. This field can be either 'PRIVATE_RFC_1918', 'REGIONAL_MANAGED_PROXY', 'GLOBAL_MANAGED_PROXY', 'PRIVATE_SERVICE_CONNECT' or 'PRIVATE_NAT'([Beta](https://terraform.io/docs/providers/google/guides/provider_versions.html)).\nA subnet with purpose set to 'REGIONAL_MANAGED_PROXY' is a user-created subnetwork that is reserved for regional Envoy-based load balancers.\nA subnetwork in a given region with purpose set to 'GLOBAL_MANAGED_PROXY' is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.\nA subnetwork with purpose set to 'PRIVATE_SERVICE_CONNECT' reserves the subnet for hosting a Private Service Connect published service.\nA subnetwork with purpose set to 'PRIVATE_NAT' is used as source range for Private NAT gateways.\nNote that 'REGIONAL_MANAGED_PROXY' is the preferred setting for all regional Envoy load balancers.\nIf unspecified, the purpose defaults to 'PRIVATE_RFC_1918'."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe GCP region for this subnetwork."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\nThe role of subnetwork.\nCurrently, this field is only used when 'purpose' is 'REGIONAL_MANAGED_PROXY'.\nThe value can be set to 'ACTIVE' or 'BACKUP'.\nAn 'ACTIVE' subnetwork is one that is currently being used for Envoy-based load balancers in a region.\nA 'BACKUP' subnetwork is one that is ready to be promoted to 'ACTIVE' or is currently draining. Possible values: [\"ACTIVE\", \"BACKUP\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secondary_ip_range` after provisioning.\nAn array of configurations for secondary IP ranges for VM instances\ncontained in this subnetwork. The primary IP of such VM must belong\nto the primary ipCidrRange of the subnetwork. The alias IPs may belong\nto either primary or secondary ranges.\n\n**Note**: This field uses [attr-as-block mode](https://www.terraform.io/docs/configuration/attr-as-blocks.html) to avoid\nbreaking users during the 0.12 upgrade. To explicitly send a list\nof zero objects you must use the following syntax:\n'example=[]'\nFor more details about this behavior, see [this section](https://www.terraform.io/docs/configuration/attr-as-blocks.html#defining-a-fixed-object-collection-value)."]
    pub fn secondary_ip_range(&self) -> ListRef<ComputeSubnetworkSecondaryIpRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secondary_ip_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this subnet to identify whether the IPv6 feature is enabled or not.\nIf not specified IPV4_ONLY will be used. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeSubnetworkLogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSubnetworkTimeoutsElRef {
        ComputeSubnetworkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeSubnetwork {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeSubnetwork { }

impl ToListMappable for ComputeSubnetwork {
    type O = ListRef<ComputeSubnetworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeSubnetwork_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_subnetwork".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeSubnetwork {
    pub tf_id: String,
    #[doc= "The range of internal addresses that are owned by this subnetwork.\nProvide this property when you create the subnetwork. For example,\n10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and\nnon-overlapping within a network. Only IPv4 is supported."]
    pub ip_cidr_range: PrimField<String>,
    #[doc= "The name of the resource, provided by the client when initially\ncreating the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "The network this subnet belongs to.\nOnly networks that are in the distributed mode can have subnetworks."]
    pub network: PrimField<String>,
}

impl BuildComputeSubnetwork {
    pub fn build(self, stack: &mut Stack) -> ComputeSubnetwork {
        let out = ComputeSubnetwork(Rc::new(ComputeSubnetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeSubnetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                external_ipv6_prefix: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_cidr_range: self.ip_cidr_range,
                ipv6_access_type: core::default::Default::default(),
                name: self.name,
                network: self.network,
                private_ip_google_access: core::default::Default::default(),
                private_ipv6_google_access: core::default::Default::default(),
                project: core::default::Default::default(),
                purpose: core::default::Default::default(),
                region: core::default::Default::default(),
                role: core::default::Default::default(),
                secondary_ip_range: core::default::Default::default(),
                stack_type: core::default::Default::default(),
                log_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeSubnetworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSubnetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeSubnetworkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource. This field can be set only at resource\ncreation time."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_ipv6_prefix` after provisioning.\nThe range of external IPv6 addresses that are owned by this subnetwork."]
    pub fn external_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_address` after provisioning.\nThe gateway address for default routes to reach destination addresses\noutside this subnetwork."]
    pub fn gateway_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal_ipv6_prefix` after provisioning.\nThe internal IPv6 address range that is assigned to this subnetwork."]
    pub fn internal_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that are owned by this subnetwork.\nProvide this property when you create the subnetwork. For example,\n10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and\nnon-overlapping within a network. Only IPv4 is supported."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_access_type` after provisioning.\nThe access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation\nor the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet\ncannot enable direct path. Possible values: [\"EXTERNAL\", \"INTERNAL\"]"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr_range` after provisioning.\nThe range of internal IPv6 addresses that are owned by this subnetwork."]
    pub fn ipv6_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially\ncreating the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe network this subnet belongs to.\nOnly networks that are in the distributed mode can have subnetworks."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip_google_access` after provisioning.\nWhen enabled, VMs in this subnetwork without external IP addresses can\naccess Google APIs and services by using Private Google Access."]
    pub fn private_ip_google_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_google_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe private IPv6 google access type for the VMs in this subnet."]
    pub fn private_ipv6_google_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose` after provisioning.\nThe purpose of the resource. This field can be either 'PRIVATE_RFC_1918', 'REGIONAL_MANAGED_PROXY', 'GLOBAL_MANAGED_PROXY', 'PRIVATE_SERVICE_CONNECT' or 'PRIVATE_NAT'([Beta](https://terraform.io/docs/providers/google/guides/provider_versions.html)).\nA subnet with purpose set to 'REGIONAL_MANAGED_PROXY' is a user-created subnetwork that is reserved for regional Envoy-based load balancers.\nA subnetwork in a given region with purpose set to 'GLOBAL_MANAGED_PROXY' is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.\nA subnetwork with purpose set to 'PRIVATE_SERVICE_CONNECT' reserves the subnet for hosting a Private Service Connect published service.\nA subnetwork with purpose set to 'PRIVATE_NAT' is used as source range for Private NAT gateways.\nNote that 'REGIONAL_MANAGED_PROXY' is the preferred setting for all regional Envoy load balancers.\nIf unspecified, the purpose defaults to 'PRIVATE_RFC_1918'."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe GCP region for this subnetwork."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\nThe role of subnetwork.\nCurrently, this field is only used when 'purpose' is 'REGIONAL_MANAGED_PROXY'.\nThe value can be set to 'ACTIVE' or 'BACKUP'.\nAn 'ACTIVE' subnetwork is one that is currently being used for Envoy-based load balancers in a region.\nA 'BACKUP' subnetwork is one that is ready to be promoted to 'ACTIVE' or is currently draining. Possible values: [\"ACTIVE\", \"BACKUP\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secondary_ip_range` after provisioning.\nAn array of configurations for secondary IP ranges for VM instances\ncontained in this subnetwork. The primary IP of such VM must belong\nto the primary ipCidrRange of the subnetwork. The alias IPs may belong\nto either primary or secondary ranges.\n\n**Note**: This field uses [attr-as-block mode](https://www.terraform.io/docs/configuration/attr-as-blocks.html) to avoid\nbreaking users during the 0.12 upgrade. To explicitly send a list\nof zero objects you must use the following syntax:\n'example=[]'\nFor more details about this behavior, see [this section](https://www.terraform.io/docs/configuration/attr-as-blocks.html#defining-a-fixed-object-collection-value)."]
    pub fn secondary_ip_range(&self) -> ListRef<ComputeSubnetworkSecondaryIpRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secondary_ip_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this subnet to identify whether the IPv6 feature is enabled or not.\nIf not specified IPV4_ONLY will be used. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeSubnetworkLogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSubnetworkTimeoutsElRef {
        ComputeSubnetworkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeSubnetworkSecondaryIpRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_name: Option<PrimField<String>>,
}

impl ComputeSubnetworkSecondaryIpRangeEl {
    #[doc= "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_cidr_range = Some(v.into());
        self
    }

    #[doc= "Set the field `range_name`.\n"]
    pub fn set_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_name = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeSubnetworkSecondaryIpRangeEl {
    type O = BlockAssignable<ComputeSubnetworkSecondaryIpRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSubnetworkSecondaryIpRangeEl {}

impl BuildComputeSubnetworkSecondaryIpRangeEl {
    pub fn build(self) -> ComputeSubnetworkSecondaryIpRangeEl {
        ComputeSubnetworkSecondaryIpRangeEl {
            ip_cidr_range: core::default::Default::default(),
            range_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeSubnetworkSecondaryIpRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSubnetworkSecondaryIpRangeElRef {
    fn new(shared: StackShared, base: String) -> ComputeSubnetworkSecondaryIpRangeElRef {
        ComputeSubnetworkSecondaryIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSubnetworkSecondaryIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.base))
    }

    #[doc= "Get a reference to the value of field `range_name` after provisioning.\n"]
    pub fn range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSubnetworkLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_expr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_sampling: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_fields: Option<SetField<PrimField<String>>>,
}

impl ComputeSubnetworkLogConfigEl {
    #[doc= "Set the field `aggregation_interval`.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nToggles the aggregation interval for collecting flow logs. Increasing the\ninterval time will reduce the amount of generated flow logs for long\nlasting connections. Default is an interval of 5 seconds per connection. Default value: \"INTERVAL_5_SEC\" Possible values: [\"INTERVAL_5_SEC\", \"INTERVAL_30_SEC\", \"INTERVAL_1_MIN\", \"INTERVAL_5_MIN\", \"INTERVAL_10_MIN\", \"INTERVAL_15_MIN\"]"]
    pub fn set_aggregation_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aggregation_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `filter_expr`.\nExport filter used to define which VPC flow logs should be logged, as as CEL expression. See\nhttps://cloud.google.com/vpc/docs/flow-logs#filtering for details on how to format this field.\nThe default value is 'true', which evaluates to include everything."]
    pub fn set_filter_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_expr = Some(v.into());
        self
    }

    #[doc= "Set the field `flow_sampling`.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nThe value of the field must be in [0, 1]. Set the sampling rate of VPC\nflow logs within the subnetwork where 1.0 means all collected logs are\nreported and 0.0 means no logs are reported. Default is 0.5 which means\nhalf of all collected logs are reported."]
    pub fn set_flow_sampling(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.flow_sampling = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata`.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nConfigures whether metadata fields should be added to the reported VPC\nflow logs. Default value: \"INCLUDE_ALL_METADATA\" Possible values: [\"EXCLUDE_ALL_METADATA\", \"INCLUDE_ALL_METADATA\", \"CUSTOM_METADATA\"]"]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata_fields`.\nList of metadata fields that should be added to reported logs.\nCan only be specified if VPC flow logs for this subnetwork is enabled and \"metadata\" is set to CUSTOM_METADATA."]
    pub fn set_metadata_fields(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.metadata_fields = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeSubnetworkLogConfigEl {
    type O = BlockAssignable<ComputeSubnetworkLogConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSubnetworkLogConfigEl {}

impl BuildComputeSubnetworkLogConfigEl {
    pub fn build(self) -> ComputeSubnetworkLogConfigEl {
        ComputeSubnetworkLogConfigEl {
            aggregation_interval: core::default::Default::default(),
            filter_expr: core::default::Default::default(),
            flow_sampling: core::default::Default::default(),
            metadata: core::default::Default::default(),
            metadata_fields: core::default::Default::default(),
        }
    }
}

pub struct ComputeSubnetworkLogConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSubnetworkLogConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeSubnetworkLogConfigElRef {
        ComputeSubnetworkLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSubnetworkLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `aggregation_interval` after provisioning.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nToggles the aggregation interval for collecting flow logs. Increasing the\ninterval time will reduce the amount of generated flow logs for long\nlasting connections. Default is an interval of 5 seconds per connection. Default value: \"INTERVAL_5_SEC\" Possible values: [\"INTERVAL_5_SEC\", \"INTERVAL_30_SEC\", \"INTERVAL_1_MIN\", \"INTERVAL_5_MIN\", \"INTERVAL_10_MIN\", \"INTERVAL_15_MIN\"]"]
    pub fn aggregation_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aggregation_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `filter_expr` after provisioning.\nExport filter used to define which VPC flow logs should be logged, as as CEL expression. See\nhttps://cloud.google.com/vpc/docs/flow-logs#filtering for details on how to format this field.\nThe default value is 'true', which evaluates to include everything."]
    pub fn filter_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_expr", self.base))
    }

    #[doc= "Get a reference to the value of field `flow_sampling` after provisioning.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nThe value of the field must be in [0, 1]. Set the sampling rate of VPC\nflow logs within the subnetwork where 1.0 means all collected logs are\nreported and 0.0 means no logs are reported. Default is 0.5 which means\nhalf of all collected logs are reported."]
    pub fn flow_sampling(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_sampling", self.base))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nCan only be specified if VPC flow logging for this subnetwork is enabled.\nConfigures whether metadata fields should be added to the reported VPC\nflow logs. Default value: \"INCLUDE_ALL_METADATA\" Possible values: [\"EXCLUDE_ALL_METADATA\", \"INCLUDE_ALL_METADATA\", \"CUSTOM_METADATA\"]"]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc= "Get a reference to the value of field `metadata_fields` after provisioning.\nList of metadata fields that should be added to reported logs.\nCan only be specified if VPC flow logs for this subnetwork is enabled and \"metadata\" is set to CUSTOM_METADATA."]
    pub fn metadata_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.metadata_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSubnetworkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeSubnetworkTimeoutsEl {
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

impl ToListMappable for ComputeSubnetworkTimeoutsEl {
    type O = BlockAssignable<ComputeSubnetworkTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSubnetworkTimeoutsEl {}

impl BuildComputeSubnetworkTimeoutsEl {
    pub fn build(self) -> ComputeSubnetworkTimeoutsEl {
        ComputeSubnetworkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeSubnetworkTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSubnetworkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSubnetworkTimeoutsElRef {
        ComputeSubnetworkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSubnetworkTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct ComputeSubnetworkDynamic {
    log_config: Option<DynamicBlock<ComputeSubnetworkLogConfigEl>>,
}
