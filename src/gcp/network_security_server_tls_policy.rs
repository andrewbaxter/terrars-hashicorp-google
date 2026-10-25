#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkSecurityServerTlsPolicyData_ {
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
    allow_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls_policy: std::option::Option<std::vec::Vec<NetworkSecurityServerTlsPolicy_MtlsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_certificate: std::option::Option<std::vec::Vec<NetworkSecurityServerTlsPolicy_ServerCertificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkSecurityServerTlsPolicy_Timeouts>,
    dynamic: NetworkSecurityServerTlsPolicyDynamic,
}

struct NetworkSecurityServerTlsPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkSecurityServerTlsPolicyData_>,
}

#[derive(Clone)]
pub struct NetworkSecurityServerTlsPolicy(std::rc::Rc<NetworkSecurityServerTlsPolicy_>);

impl NetworkSecurityServerTlsPolicy {
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
        "Set the field `allow_open`.\nThis field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.\nDetermines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.\nConsider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80."]
    pub fn set_allow_open(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_open = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
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
        "Set the field `labels`.\nSet of label tags associated with the ServerTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location of the server tls policy.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mtls_policy`.\n"]
    pub fn set_mtls_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_MtlsPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mtls_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mtls_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_certificate`.\n"]
    pub fn set_server_certificate(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_ServerCertificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkSecurityServerTlsPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_open` after provisioning.\nThis field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.\nDetermines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.\nConsider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80."]
    pub fn allow_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_open", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the ServerTlsPolicy was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the ServerTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the server tls policy.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ServerTlsPolicy resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the ServerTlsPolicy was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mtls_policy` after provisioning.\n"]
    pub fn mtls_policy(&self) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_MtlsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mtls_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(&self) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_ServerCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityServerTlsPolicy_TimeoutsRef {
        NetworkSecurityServerTlsPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkSecurityServerTlsPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkSecurityServerTlsPolicy { }

impl terrars::ToListMappable for NetworkSecurityServerTlsPolicy {
    type O = terrars::ListRef<NetworkSecurityServerTlsPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkSecurityServerTlsPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_security_server_tls_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkSecurityServerTlsPolicy {
    pub tf_id: std::string::String,
    #[doc = "Name of the ServerTlsPolicy resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecurityServerTlsPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkSecurityServerTlsPolicy {
        let out = NetworkSecurityServerTlsPolicy(std::rc::Rc::new(NetworkSecurityServerTlsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkSecurityServerTlsPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_open: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                mtls_policy: core::default::Default::default(),
                server_certificate: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkSecurityServerTlsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityServerTlsPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkSecurityServerTlsPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_open` after provisioning.\nThis field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.\nDetermines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.\nConsider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80."]
    pub fn allow_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_open", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the ServerTlsPolicy was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the ServerTlsPolicy resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the server tls policy.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ServerTlsPolicy resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the ServerTlsPolicy was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mtls_policy` after provisioning.\n"]
    pub fn mtls_policy(&self) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_MtlsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mtls_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(&self) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_ServerCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityServerTlsPolicy_TimeoutsRef {
        NetworkSecurityServerTlsPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityServerTlsPolicyDynamic {
    mtls_policy: std::option::Option<terrars::DynamicBlock<NetworkSecurityServerTlsPolicy_MtlsPolicy>>,
    server_certificate: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityServerTlsPolicy_ServerCertificate>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance {
    plugin_instance: terrars::PrimField<std::string::String>,
}

impl NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance { }

impl terrars::ToListMappable for NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance {
    type O = terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance {
    #[doc =
        "Plugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub plugin_instance: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance {
    pub fn build(self) -> NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance {
        NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance {
            plugin_instance: self.plugin_instance,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstanceRef {
        NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `plugin_instance` after provisioning.\nPlugin instance name, used to locate and load CertificateProvider instance configuration. Set to \"google_cloud_private_spiffe\" to use Certificate Authority Service certificate provider instance."]
    pub fn plugin_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint {
    target_uri: terrars::PrimField<std::string::String>,
}

impl NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint { }

impl terrars::ToListMappable for NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint {
    type O = terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint {
    #[doc = "The target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub target_uri: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint {
    pub fn build(self) -> NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint {
        NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint { target_uri: self.target_uri }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpointRef {
        NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_uri` after provisioning.\nThe target URI of the gRPC endpoint. Only UDS path is supported, and should start with \"unix:\"."]
    pub fn target_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaDynamic {
    certificate_provider_instance: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance>>,
    grpc_endpoint: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_provider_instance: std
    ::option
    ::Option<std::vec::Vec<NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_endpoint: std::option::Option<std::vec::Vec<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint>>,
    dynamic: NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaDynamic,
}

impl NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa {
    #[doc = "Set the field `certificate_provider_instance`.\n"]
    pub fn set_certificate_provider_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate_provider_instance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_provider_instance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grpc_endpoint`.\n"]
    pub fn set_grpc_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa {
    type O = terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa {}

impl BuildNetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa {
    pub fn build(self) -> NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa {
        NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa {
            certificate_provider_instance: core::default::Default::default(),
            grpc_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaRef {
        NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_provider_instance` after provisioning.\n"]
    pub fn certificate_provider_instance(
        &self,
    ) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_provider_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_endpoint` after provisioning.\n"]
    pub fn grpc_endpoint(&self) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityServerTlsPolicy_MtlsPolicyDynamic {
    client_validation_ca: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicy_MtlsPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_validation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_validation_trust_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_validation_ca: std
    ::option
    ::Option<std::vec::Vec<NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa>>,
    dynamic: NetworkSecurityServerTlsPolicy_MtlsPolicyDynamic,
}

impl NetworkSecurityServerTlsPolicy_MtlsPolicy {
    #[doc =
        "Set the field `client_validation_mode`.\nWhen the client presents an invalid certificate or no certificate to the load balancer, the clientValidationMode specifies how the client connection is handled.\nRequired if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty. Possible values: [\"CLIENT_VALIDATION_MODE_UNSPECIFIED\", \"ALLOW_INVALID_OR_MISSING_CLIENT_CERT\", \"REJECT_INVALID\"]"]
    pub fn set_client_validation_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_validation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_validation_trust_config`.\nReference to the TrustConfig from certificatemanager.googleapis.com namespace.\nIf specified, the chain validation will be performed against certificates configured in the given TrustConfig.\nAllowed only if the policy is to be used with external HTTPS load balancers."]
    pub fn set_client_validation_trust_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_validation_trust_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_validation_ca`.\n"]
    pub fn set_client_validation_ca(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCa>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_validation_ca = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_validation_ca = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityServerTlsPolicy_MtlsPolicy {
    type O = terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_MtlsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityServerTlsPolicy_MtlsPolicy {}

impl BuildNetworkSecurityServerTlsPolicy_MtlsPolicy {
    pub fn build(self) -> NetworkSecurityServerTlsPolicy_MtlsPolicy {
        NetworkSecurityServerTlsPolicy_MtlsPolicy {
            client_validation_mode: core::default::Default::default(),
            client_validation_trust_config: core::default::Default::default(),
            client_validation_ca: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityServerTlsPolicy_MtlsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityServerTlsPolicy_MtlsPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityServerTlsPolicy_MtlsPolicyRef {
        NetworkSecurityServerTlsPolicy_MtlsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityServerTlsPolicy_MtlsPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_validation_mode` after provisioning.\nWhen the client presents an invalid certificate or no certificate to the load balancer, the clientValidationMode specifies how the client connection is handled.\nRequired if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty. Possible values: [\"CLIENT_VALIDATION_MODE_UNSPECIFIED\", \"ALLOW_INVALID_OR_MISSING_CLIENT_CERT\", \"REJECT_INVALID\"]"]
    pub fn client_validation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_validation_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_validation_trust_config` after provisioning.\nReference to the TrustConfig from certificatemanager.googleapis.com namespace.\nIf specified, the chain validation will be performed against certificates configured in the given TrustConfig.\nAllowed only if the policy is to be used with external HTTPS load balancers."]
    pub fn client_validation_trust_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_validation_trust_config", self.base))
    }

    #[doc = "Get a reference to the value of field `client_validation_ca` after provisioning.\n"]
    pub fn client_validation_ca(
        &self,
    ) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_MtlsPolicy_ClientValidationCaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_validation_ca", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityServerTlsPolicy_ServerCertificateDynamic {
    certificate_provider_instance: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance>>,
    grpc_endpoint: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicy_ServerCertificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_provider_instance: std
    ::option
    ::Option<std::vec::Vec<NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_endpoint: std::option::Option<std::vec::Vec<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint>>,
    dynamic: NetworkSecurityServerTlsPolicy_ServerCertificateDynamic,
}

impl NetworkSecurityServerTlsPolicy_ServerCertificate {
    #[doc = "Set the field `certificate_provider_instance`.\n"]
    pub fn set_certificate_provider_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstance,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate_provider_instance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_provider_instance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grpc_endpoint`.\n"]
    pub fn set_grpc_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityServerTlsPolicy_ServerCertificate {
    type O = terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_ServerCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityServerTlsPolicy_ServerCertificate {}

impl BuildNetworkSecurityServerTlsPolicy_ServerCertificate {
    pub fn build(self) -> NetworkSecurityServerTlsPolicy_ServerCertificate {
        NetworkSecurityServerTlsPolicy_ServerCertificate {
            certificate_provider_instance: core::default::Default::default(),
            grpc_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityServerTlsPolicy_ServerCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityServerTlsPolicy_ServerCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityServerTlsPolicy_ServerCertificateRef {
        NetworkSecurityServerTlsPolicy_ServerCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityServerTlsPolicy_ServerCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_provider_instance` after provisioning.\n"]
    pub fn certificate_provider_instance(
        &self,
    ) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_ServerCertificate_CertificateProviderInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_provider_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_endpoint` after provisioning.\n"]
    pub fn grpc_endpoint(&self) -> terrars::ListRef<NetworkSecurityServerTlsPolicy_ServerCertificate_GrpcEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityServerTlsPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityServerTlsPolicy_Timeouts {
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

impl terrars::ToListMappable for NetworkSecurityServerTlsPolicy_Timeouts {
    type O = terrars::BlockAssignable<NetworkSecurityServerTlsPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityServerTlsPolicy_Timeouts {}

impl BuildNetworkSecurityServerTlsPolicy_Timeouts {
    pub fn build(self) -> NetworkSecurityServerTlsPolicy_Timeouts {
        NetworkSecurityServerTlsPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityServerTlsPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityServerTlsPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityServerTlsPolicy_TimeoutsRef {
        NetworkSecurityServerTlsPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityServerTlsPolicy_TimeoutsRef {
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
