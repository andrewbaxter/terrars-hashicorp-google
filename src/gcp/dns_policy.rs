use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DnsPolicyData {
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
    enable_inbound_forwarding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternative_name_server_config: std::option::Option<std::vec::Vec<DnsPolicyAlternativeNameServerConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<DnsPolicyNetworksEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DnsPolicyTimeoutsEl>,
    dynamic: DnsPolicyDynamic,
}

struct DnsPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsPolicyData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DnsPolicyAlternativeNameServerConfigEl>>,
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

    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl std::convert::Into<terrars::BlockAssignable<DnsPolicyNetworksEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DnsPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
    pub fn alternative_name_server_config(&self) -> terrars::ListRef<DnsPolicyAlternativeNameServerConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_name_server_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsPolicyTimeoutsElRef {
        DnsPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DnsPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_inbound_forwarding: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                alternative_name_server_config: core::default::Default::default(),
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
    pub fn alternative_name_server_config(&self) -> terrars::ListRef<DnsPolicyAlternativeNameServerConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_name_server_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsPolicyTimeoutsElRef {
        DnsPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_path: std::option::Option<terrars::PrimField<std::string::String>>,
    ipv4_address: terrars::PrimField<std::string::String>,
}

impl DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    #[doc =
        "Set the field `forwarding_path`.\nForwarding path for this TargetNameServer. If unset or 'default' Cloud DNS will make forwarding\ndecision based on address ranges, i.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always send queries through VPC for this target Possible values: [\"default\", \"private\"]"]
    pub fn set_forwarding_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    type O = terrars::BlockAssignable<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    #[doc = "IPv4 address to forward to."]
    pub ipv4_address: terrars::PrimField<std::string::String>,
}

impl BuildDnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    pub fn build(self) -> DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
        DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
            forwarding_path: core::default::Default::default(),
            ipv4_address: self.ipv4_address,
        }
    }
}

pub struct DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
        DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
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
struct DnsPolicyAlternativeNameServerConfigElDynamic {
    target_name_servers: std
    ::option
    ::Option<terrars::DynamicBlock<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>>,
}

#[derive(Serialize)]
pub struct DnsPolicyAlternativeNameServerConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_name_servers: std::option::Option<std::vec::Vec<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>>,
    dynamic: DnsPolicyAlternativeNameServerConfigElDynamic,
}

impl DnsPolicyAlternativeNameServerConfigEl {
    #[doc = "Set the field `target_name_servers`.\n"]
    pub fn set_target_name_servers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>>,
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

impl terrars::ToListMappable for DnsPolicyAlternativeNameServerConfigEl {
    type O = terrars::BlockAssignable<DnsPolicyAlternativeNameServerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicyAlternativeNameServerConfigEl {}

impl BuildDnsPolicyAlternativeNameServerConfigEl {
    pub fn build(self) -> DnsPolicyAlternativeNameServerConfigEl {
        DnsPolicyAlternativeNameServerConfigEl {
            target_name_servers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsPolicyAlternativeNameServerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicyAlternativeNameServerConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicyAlternativeNameServerConfigElRef {
        DnsPolicyAlternativeNameServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicyAlternativeNameServerConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DnsPolicyNetworksEl {
    network_url: terrars::PrimField<std::string::String>,
}

impl DnsPolicyNetworksEl { }

impl terrars::ToListMappable for DnsPolicyNetworksEl {
    type O = terrars::BlockAssignable<DnsPolicyNetworksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicyNetworksEl {
    #[doc =
        "The id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: terrars::PrimField<std::string::String>,
}

impl BuildDnsPolicyNetworksEl {
    pub fn build(self) -> DnsPolicyNetworksEl {
        DnsPolicyNetworksEl { network_url: self.network_url }
    }
}

pub struct DnsPolicyNetworksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicyNetworksElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicyNetworksElRef {
        DnsPolicyNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicyNetworksElRef {
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
pub struct DnsPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsPolicyTimeoutsEl {
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

impl terrars::ToListMappable for DnsPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<DnsPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsPolicyTimeoutsEl {}

impl BuildDnsPolicyTimeoutsEl {
    pub fn build(self) -> DnsPolicyTimeoutsEl {
        DnsPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DnsPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsPolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsPolicyTimeoutsElRef {
        DnsPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsPolicyTimeoutsElRef {
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
struct DnsPolicyDynamic {
    alternative_name_server_config: std
    ::option
    ::Option<terrars::DynamicBlock<DnsPolicyAlternativeNameServerConfigEl>>,
    networks: std::option::Option<terrars::DynamicBlock<DnsPolicyNetworksEl>>,
}
