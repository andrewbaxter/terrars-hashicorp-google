#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DnsPolicyData_ {
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
    enable_inbound_forwarding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternative_name_server_config: std::option::Option<std::vec::Vec<DnsPolicy_AlternativeNameServerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns64_config: std::option::Option<std::vec::Vec<DnsPolicy_Dns64Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<DnsPolicy_Networks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DnsPolicy_Timeouts>,
    dynamic: DnsPolicyDynamic,
}

struct DnsPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsPolicyData_>,
}

#[derive(Clone)]
pub struct DnsPolicy(std::rc::Rc<DnsPolicy_>);

impl DnsPolicy {
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

    #[doc = "Set the field `description`.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_inbound_forwarding`.\nAllows networks bound to this policy to receive DNS queries sent\nby VMs or applications over VPN connections. When enabled, a\nvirtual IP address will be allocated from each of the sub-networks\nthat are bound to this policy."]
    pub fn set_enable_inbound_forwarding(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_inbound_forwarding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_logging`.\nControls whether logging is enabled for the networks bound to this policy.\nDefaults to no logging if not set."]
    pub fn set_enable_logging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `alternative_name_server_config`.\n"]
    pub fn set_alternative_name_server_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsPolicy_AlternativeNameServerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alternative_name_server_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alternative_name_server_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns64_config`.\n"]
    pub fn set_dns64_config(self, v: impl std::convert::Into<terrars::BlockAssignable<DnsPolicy_Dns64Config>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns64_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns64_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl std::convert::Into<terrars::BlockAssignable<DnsPolicy_Networks>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DnsPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_inbound_forwarding` after provisioning.\nAllows networks bound to this policy to receive DNS queries sent\nby VMs or applications over VPN connections. When enabled, a\nvirtual IP address will be allocated from each of the sub-networks\nthat are bound to this policy."]
    pub fn enable_inbound_forwarding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_inbound_forwarding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_logging` after provisioning.\nControls whether logging is enabled for the networks bound to this policy.\nDefaults to no logging if not set."]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alternative_name_server_config` after provisioning.\n"]
    pub fn alternative_name_server_config(&self) -> terrars::ListRef<DnsPolicy_AlternativeNameServerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_name_server_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns64_config` after provisioning.\n"]
    pub fn dns64_config(&self) -> terrars::ListRef<DnsPolicy_Dns64ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns64_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsPolicy_TimeoutsRef {
        DnsPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DnsPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DnsPolicy { }

impl terrars::ToListMappable for DnsPolicy {
    type O = terrars::ListRef<DnsPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DnsPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dns_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDnsPolicy {
    pub tf_id: std::string::String,
    #[doc = "User assigned name for this policy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDnsPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DnsPolicy {
        let out = DnsPolicy(std::rc::Rc::new(DnsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DnsPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_inbound_forwarding: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                alternative_name_server_config: core::default::Default::default(),
                dns64_config: core::default::Default::default(),
                networks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DnsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DnsPolicyRef {
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
        "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_inbound_forwarding` after provisioning.\nAllows networks bound to this policy to receive DNS queries sent\nby VMs or applications over VPN connections. When enabled, a\nvirtual IP address will be allocated from each of the sub-networks\nthat are bound to this policy."]
    pub fn enable_inbound_forwarding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_inbound_forwarding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_logging` after provisioning.\nControls whether logging is enabled for the networks bound to this policy.\nDefaults to no logging if not set."]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alternative_name_server_config` after provisioning.\n"]
    pub fn alternative_name_server_config(&self) -> terrars::ListRef<DnsPolicy_AlternativeNameServerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_name_server_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns64_config` after provisioning.\n"]
    pub fn dns64_config(&self) -> terrars::ListRef<DnsPolicy_Dns64ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns64_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsPolicy_TimeoutsRef {
        DnsPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DnsPolicyDynamic {
    alternative_name_server_config: std::option::Option<terrars::DynamicBlock<DnsPolicy_AlternativeNameServerConfig>>,
    dns64_config: std::option::Option<terrars::DynamicBlock<DnsPolicy_Dns64Config>>,
    networks: std::option::Option<terrars::DynamicBlock<DnsPolicy_Networks>>,
}

#[derive(Serialize)]
pub struct DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints {
    name: terrars::PrimField<std::string::String>,
}

impl DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints { }

impl terrars::ToListMappable for DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints {
    type O = terrars::BlockAssignable<DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicy_AlternativeNameServerConfig_OutboundEndpoints {
    #[doc =
        "Specified as a resource path including project and region.\nThis should be formatted like\n'projects/{project}/locations/{location}/outboundEndpoints/{endpoint}'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDnsPolicy_AlternativeNameServerConfig_OutboundEndpoints {
    pub fn build(self) -> DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints {
        DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DnsPolicy_AlternativeNameServerConfig_OutboundEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicy_AlternativeNameServerConfig_OutboundEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsPolicy_AlternativeNameServerConfig_OutboundEndpointsRef {
        DnsPolicy_AlternativeNameServerConfig_OutboundEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicy_AlternativeNameServerConfig_OutboundEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecified as a resource path including project and region.\nThis should be formatted like\n'projects/{project}/locations/{location}/outboundEndpoints/{endpoint}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsPolicy_AlternativeNameServerConfig_TargetNameServers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_path: std::option::Option<terrars::PrimField<std::string::String>>,
    ipv4_address: terrars::PrimField<std::string::String>,
}

impl DnsPolicy_AlternativeNameServerConfig_TargetNameServers {
    #[doc =
        "Set the field `forwarding_path`.\nForwarding path for this TargetNameServer. If unset or 'default' Cloud DNS will make forwarding\ndecision based on address ranges, i.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always send queries through VPC for this target Possible values: [\"default\", \"private\"]"]
    pub fn set_forwarding_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsPolicy_AlternativeNameServerConfig_TargetNameServers {
    type O = terrars::BlockAssignable<DnsPolicy_AlternativeNameServerConfig_TargetNameServers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicy_AlternativeNameServerConfig_TargetNameServers {
    #[doc = "IPv4 address to forward to."]
    pub ipv4_address: terrars::PrimField<std::string::String>,
}

impl BuildDnsPolicy_AlternativeNameServerConfig_TargetNameServers {
    pub fn build(self) -> DnsPolicy_AlternativeNameServerConfig_TargetNameServers {
        DnsPolicy_AlternativeNameServerConfig_TargetNameServers {
            forwarding_path: core::default::Default::default(),
            ipv4_address: self.ipv4_address,
        }
    }
}

#[allow(dead_code)]
pub struct DnsPolicy_AlternativeNameServerConfig_TargetNameServersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicy_AlternativeNameServerConfig_TargetNameServersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsPolicy_AlternativeNameServerConfig_TargetNameServersRef {
        DnsPolicy_AlternativeNameServerConfig_TargetNameServersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicy_AlternativeNameServerConfig_TargetNameServersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `forwarding_path` after provisioning.\nForwarding path for this TargetNameServer. If unset or 'default' Cloud DNS will make forwarding\ndecision based on address ranges, i.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always send queries through VPC for this target Possible values: [\"default\", \"private\"]"]
    pub fn forwarding_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_path", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\nIPv4 address to forward to."]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsPolicy_AlternativeNameServerConfigDynamic {
    outbound_endpoints: std
    ::option
    ::Option<terrars::DynamicBlock<DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints>>,
    target_name_servers: std
    ::option
    ::Option<terrars::DynamicBlock<DnsPolicy_AlternativeNameServerConfig_TargetNameServers>>,
}

#[derive(Serialize)]
pub struct DnsPolicy_AlternativeNameServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_endpoints: std::option::Option<std::vec::Vec<DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_name_servers: std::option::Option<std::vec::Vec<DnsPolicy_AlternativeNameServerConfig_TargetNameServers>>,
    dynamic: DnsPolicy_AlternativeNameServerConfigDynamic,
}

impl DnsPolicy_AlternativeNameServerConfig {
    #[doc = "Set the field `outbound_endpoints`.\n"]
    pub fn set_outbound_endpoints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DnsPolicy_AlternativeNameServerConfig_OutboundEndpoints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.outbound_endpoints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.outbound_endpoints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_name_servers`.\n"]
    pub fn set_target_name_servers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DnsPolicy_AlternativeNameServerConfig_TargetNameServers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_name_servers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_name_servers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsPolicy_AlternativeNameServerConfig {
    type O = terrars::BlockAssignable<DnsPolicy_AlternativeNameServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicy_AlternativeNameServerConfig {}

impl BuildDnsPolicy_AlternativeNameServerConfig {
    pub fn build(self) -> DnsPolicy_AlternativeNameServerConfig {
        DnsPolicy_AlternativeNameServerConfig {
            outbound_endpoints: core::default::Default::default(),
            target_name_servers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsPolicy_AlternativeNameServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicy_AlternativeNameServerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicy_AlternativeNameServerConfigRef {
        DnsPolicy_AlternativeNameServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicy_AlternativeNameServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `outbound_endpoints` after provisioning.\n"]
    pub fn outbound_endpoints(&self) -> terrars::ListRef<DnsPolicy_AlternativeNameServerConfig_OutboundEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_endpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsPolicy_Dns64Config_Scope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_queries: std::option::Option<terrars::PrimField<bool>>,
}

impl DnsPolicy_Dns64Config_Scope {
    #[doc = "Set the field `all_queries`.\nControls whether DNS64 is enabled globally at the network level."]
    pub fn set_all_queries(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_queries = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsPolicy_Dns64Config_Scope {
    type O = terrars::BlockAssignable<DnsPolicy_Dns64Config_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicy_Dns64Config_Scope {}

impl BuildDnsPolicy_Dns64Config_Scope {
    pub fn build(self) -> DnsPolicy_Dns64Config_Scope {
        DnsPolicy_Dns64Config_Scope { all_queries: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DnsPolicy_Dns64Config_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicy_Dns64Config_ScopeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicy_Dns64Config_ScopeRef {
        DnsPolicy_Dns64Config_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicy_Dns64Config_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all_queries` after provisioning.\nControls whether DNS64 is enabled globally at the network level."]
    pub fn all_queries(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsPolicy_Dns64ConfigDynamic {
    scope: std::option::Option<terrars::DynamicBlock<DnsPolicy_Dns64Config_Scope>>,
}

#[derive(Serialize)]
pub struct DnsPolicy_Dns64Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<std::vec::Vec<DnsPolicy_Dns64Config_Scope>>,
    dynamic: DnsPolicy_Dns64ConfigDynamic,
}

impl DnsPolicy_Dns64Config {
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsPolicy_Dns64Config_Scope>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsPolicy_Dns64Config {
    type O = terrars::BlockAssignable<DnsPolicy_Dns64Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicy_Dns64Config {}

impl BuildDnsPolicy_Dns64Config {
    pub fn build(self) -> DnsPolicy_Dns64Config {
        DnsPolicy_Dns64Config {
            scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsPolicy_Dns64ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicy_Dns64ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicy_Dns64ConfigRef {
        DnsPolicy_Dns64ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicy_Dns64ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<DnsPolicy_Dns64Config_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsPolicy_Networks {
    network_url: terrars::PrimField<std::string::String>,
}

impl DnsPolicy_Networks { }

impl terrars::ToListMappable for DnsPolicy_Networks {
    type O = terrars::BlockAssignable<DnsPolicy_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicy_Networks {
    #[doc =
        "The id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: terrars::PrimField<std::string::String>,
}

impl BuildDnsPolicy_Networks {
    pub fn build(self) -> DnsPolicy_Networks {
        DnsPolicy_Networks { network_url: self.network_url }
    }
}

#[allow(dead_code)]
pub struct DnsPolicy_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicy_NetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicy_NetworksRef {
        DnsPolicy_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicy_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsPolicy_Timeouts {
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

impl terrars::ToListMappable for DnsPolicy_Timeouts {
    type O = terrars::BlockAssignable<DnsPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicy_Timeouts {}

impl BuildDnsPolicy_Timeouts {
    pub fn build(self) -> DnsPolicy_Timeouts {
        DnsPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicy_TimeoutsRef {
        DnsPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicy_TimeoutsRef {
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
