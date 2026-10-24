use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ActiveDirectoryDomainTrustData {
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
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selective_authentication: std::option::Option<terrars::PrimField<bool>>,
    target_dns_ip_addresses: terrars::SetField<terrars::PrimField<std::string::String>>,
    target_domain_name: terrars::PrimField<std::string::String>,
    trust_direction: terrars::PrimField<std::string::String>,
    trust_handshake_secret: terrars::PrimField<std::string::String>,
    trust_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ActiveDirectoryDomainTrustTimeoutsEl>,
}

struct ActiveDirectoryDomainTrust_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ActiveDirectoryDomainTrustData>,
}

#[derive(Clone)]
pub struct ActiveDirectoryDomainTrust(std::rc::Rc<ActiveDirectoryDomainTrust_>);

impl ActiveDirectoryDomainTrust {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `selective_authentication`.\nWhether the trusted side has forest/domain wide access or selective access to an approved set of resources."]
    pub fn set_selective_authentication(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().selective_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ActiveDirectoryDomainTrustTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions,\nhttps://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `selective_authentication` after provisioning.\nWhether the trusted side has forest/domain wide access or selective access to an approved set of resources."]
    pub fn selective_authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selective_authentication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_dns_ip_addresses` after provisioning.\nThe target DNS server IP addresses which can resolve the remote domain involved in the trust."]
    pub fn target_dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_dns_ip_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_domain_name` after provisioning.\nThe fully qualified target domain name which will be in trust with the current domain."]
    pub fn target_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trust_direction` after provisioning.\nThe trust direction, which decides if the current domain is trusted, trusting, or both. Possible values: [\"INBOUND\", \"OUTBOUND\", \"BIDIRECTIONAL\"]"]
    pub fn trust_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trust_handshake_secret` after provisioning.\nThe trust secret used for the handshake with the target domain. This will not be stored."]
    pub fn trust_handshake_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_handshake_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trust_type` after provisioning.\nThe type of trust represented by the trust resource. Possible values: [\"FOREST\", \"EXTERNAL\"]"]
    pub fn trust_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ActiveDirectoryDomainTrustTimeoutsElRef {
        ActiveDirectoryDomainTrustTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ActiveDirectoryDomainTrust {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ActiveDirectoryDomainTrust { }

impl terrars::ToListMappable for ActiveDirectoryDomainTrust {
    type O = terrars::ListRef<ActiveDirectoryDomainTrustRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ActiveDirectoryDomainTrust_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_active_directory_domain_trust".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildActiveDirectoryDomainTrust {
    pub tf_id: std::string::String,
    #[doc =
        "The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions,\nhttps://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc = "The target DNS server IP addresses which can resolve the remote domain involved in the trust."]
    pub target_dns_ip_addresses: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "The fully qualified target domain name which will be in trust with the current domain."]
    pub target_domain_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The trust direction, which decides if the current domain is trusted, trusting, or both. Possible values: [\"INBOUND\", \"OUTBOUND\", \"BIDIRECTIONAL\"]"]
    pub trust_direction: terrars::PrimField<std::string::String>,
    #[doc = "The trust secret used for the handshake with the target domain. This will not be stored."]
    pub trust_handshake_secret: terrars::PrimField<std::string::String>,
    #[doc = "The type of trust represented by the trust resource. Possible values: [\"FOREST\", \"EXTERNAL\"]"]
    pub trust_type: terrars::PrimField<std::string::String>,
}

impl BuildActiveDirectoryDomainTrust {
    pub fn build(self, stack: &mut terrars::Stack) -> ActiveDirectoryDomainTrust {
        let out = ActiveDirectoryDomainTrust(std::rc::Rc::new(ActiveDirectoryDomainTrust_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ActiveDirectoryDomainTrustData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain: self.domain,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                selective_authentication: core::default::Default::default(),
                target_dns_ip_addresses: self.target_dns_ip_addresses,
                target_domain_name: self.target_domain_name,
                trust_direction: self.trust_direction,
                trust_handshake_secret: self.trust_handshake_secret,
                trust_type: self.trust_type,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ActiveDirectoryDomainTrustRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ActiveDirectoryDomainTrustRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ActiveDirectoryDomainTrustRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions,\nhttps://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `selective_authentication` after provisioning.\nWhether the trusted side has forest/domain wide access or selective access to an approved set of resources."]
    pub fn selective_authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selective_authentication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_dns_ip_addresses` after provisioning.\nThe target DNS server IP addresses which can resolve the remote domain involved in the trust."]
    pub fn target_dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_dns_ip_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_domain_name` after provisioning.\nThe fully qualified target domain name which will be in trust with the current domain."]
    pub fn target_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trust_direction` after provisioning.\nThe trust direction, which decides if the current domain is trusted, trusting, or both. Possible values: [\"INBOUND\", \"OUTBOUND\", \"BIDIRECTIONAL\"]"]
    pub fn trust_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trust_handshake_secret` after provisioning.\nThe trust secret used for the handshake with the target domain. This will not be stored."]
    pub fn trust_handshake_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_handshake_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trust_type` after provisioning.\nThe type of trust represented by the trust resource. Possible values: [\"FOREST\", \"EXTERNAL\"]"]
    pub fn trust_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ActiveDirectoryDomainTrustTimeoutsElRef {
        ActiveDirectoryDomainTrustTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ActiveDirectoryDomainTrustTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ActiveDirectoryDomainTrustTimeoutsEl {
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

impl terrars::ToListMappable for ActiveDirectoryDomainTrustTimeoutsEl {
    type O = terrars::BlockAssignable<ActiveDirectoryDomainTrustTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildActiveDirectoryDomainTrustTimeoutsEl {}

impl BuildActiveDirectoryDomainTrustTimeoutsEl {
    pub fn build(self) -> ActiveDirectoryDomainTrustTimeoutsEl {
        ActiveDirectoryDomainTrustTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ActiveDirectoryDomainTrustTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ActiveDirectoryDomainTrustTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ActiveDirectoryDomainTrustTimeoutsElRef {
        ActiveDirectoryDomainTrustTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ActiveDirectoryDomainTrustTimeoutsElRef {
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
