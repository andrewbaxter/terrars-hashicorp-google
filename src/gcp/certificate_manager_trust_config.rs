#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CertificateManagerTrustConfigData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowlisted_certificates: std::option::Option<std::vec::Vec<CertificateManagerTrustConfig_AllowlistedCertificates>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CertificateManagerTrustConfig_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_stores: std::option::Option<std::vec::Vec<CertificateManagerTrustConfig_TrustStores>>,
    dynamic: CertificateManagerTrustConfigDynamic,
}

struct CertificateManagerTrustConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CertificateManagerTrustConfigData_>,
}

#[derive(Clone)]
pub struct CertificateManagerTrustConfig(std::rc::Rc<CertificateManagerTrustConfig_>);

impl CertificateManagerTrustConfig {
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

    #[doc = "Set the field `description`.\nOne or more paragraphs of text description of a trust config."]
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
        "Set the field `labels`.\nSet of label tags associated with the trust config.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowlisted_certificates`.\n"]
    pub fn set_allowlisted_certificates(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificateManagerTrustConfig_AllowlistedCertificates>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowlisted_certificates = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowlisted_certificates = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CertificateManagerTrustConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_stores`.\n"]
    pub fn set_trust_stores(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificateManagerTrustConfig_TrustStores>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trust_stores = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trust_stores = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of a TrustConfig.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOne or more paragraphs of text description of a trust config."]
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
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the trust config.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe trust config location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the trust config. Trust config names must be unique globally."]
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
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a TrustConfig.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowlisted_certificates` after provisioning.\n"]
    pub fn allowlisted_certificates(&self) -> terrars::ListRef<CertificateManagerTrustConfig_AllowlistedCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlisted_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerTrustConfig_TimeoutsRef {
        CertificateManagerTrustConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trust_stores` after provisioning.\n"]
    pub fn trust_stores(&self) -> terrars::ListRef<CertificateManagerTrustConfig_TrustStoresRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_stores", self.extract_ref()))
    }
}

impl terrars::Referable for CertificateManagerTrustConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CertificateManagerTrustConfig { }

impl terrars::ToListMappable for CertificateManagerTrustConfig {
    type O = terrars::ListRef<CertificateManagerTrustConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CertificateManagerTrustConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_certificate_manager_trust_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCertificateManagerTrustConfig {
    pub tf_id: std::string::String,
    #[doc = "The trust config location."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "A user-defined name of the trust config. Trust config names must be unique globally."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCertificateManagerTrustConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> CertificateManagerTrustConfig {
        let out = CertificateManagerTrustConfig(std::rc::Rc::new(CertificateManagerTrustConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CertificateManagerTrustConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                allowlisted_certificates: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                trust_stores: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CertificateManagerTrustConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CertificateManagerTrustConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of a TrustConfig.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOne or more paragraphs of text description of a trust config."]
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
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the trust config.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe trust config location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the trust config. Trust config names must be unique globally."]
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
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a TrustConfig.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowlisted_certificates` after provisioning.\n"]
    pub fn allowlisted_certificates(&self) -> terrars::ListRef<CertificateManagerTrustConfig_AllowlistedCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlisted_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerTrustConfig_TimeoutsRef {
        CertificateManagerTrustConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trust_stores` after provisioning.\n"]
    pub fn trust_stores(&self) -> terrars::ListRef<CertificateManagerTrustConfig_TrustStoresRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_stores", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CertificateManagerTrustConfigDynamic {
    allowlisted_certificates: std
    ::option
    ::Option<terrars::DynamicBlock<CertificateManagerTrustConfig_AllowlistedCertificates>>,
    trust_stores: std::option::Option<terrars::DynamicBlock<CertificateManagerTrustConfig_TrustStores>>,
}

#[derive(Serialize)]
pub struct CertificateManagerTrustConfig_AllowlistedCertificates {
    pem_certificate: terrars::PrimField<std::string::String>,
}

impl CertificateManagerTrustConfig_AllowlistedCertificates { }

impl terrars::ToListMappable for CertificateManagerTrustConfig_AllowlistedCertificates {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfig_AllowlistedCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfig_AllowlistedCertificates {
    #[doc =
        "PEM certificate that is allowlisted. The certificate can be up to 5k bytes, and must be a parseable X.509 certificate."]
    pub pem_certificate: terrars::PrimField<std::string::String>,
}

impl BuildCertificateManagerTrustConfig_AllowlistedCertificates {
    pub fn build(self) -> CertificateManagerTrustConfig_AllowlistedCertificates {
        CertificateManagerTrustConfig_AllowlistedCertificates { pem_certificate: self.pem_certificate }
    }
}

#[allow(dead_code)]
pub struct CertificateManagerTrustConfig_AllowlistedCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfig_AllowlistedCertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerTrustConfig_AllowlistedCertificatesRef {
        CertificateManagerTrustConfig_AllowlistedCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfig_AllowlistedCertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM certificate that is allowlisted. The certificate can be up to 5k bytes, and must be a parseable X.509 certificate."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct CertificateManagerTrustConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerTrustConfig_Timeouts {
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

impl terrars::ToListMappable for CertificateManagerTrustConfig_Timeouts {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfig_Timeouts {}

impl BuildCertificateManagerTrustConfig_Timeouts {
    pub fn build(self) -> CertificateManagerTrustConfig_Timeouts {
        CertificateManagerTrustConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CertificateManagerTrustConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificateManagerTrustConfig_TimeoutsRef {
        CertificateManagerTrustConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfig_TimeoutsRef {
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

#[derive(Serialize)]
pub struct CertificateManagerTrustConfig_TrustStores_IntermediateCas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerTrustConfig_TrustStores_IntermediateCas {
    #[doc =
        "Set the field `pem_certificate`.\nPEM intermediate certificate used for building up paths for validation.\nEach certificate provided in PEM format may occupy up to 5kB."]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CertificateManagerTrustConfig_TrustStores_IntermediateCas {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfig_TrustStores_IntermediateCas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfig_TrustStores_IntermediateCas {}

impl BuildCertificateManagerTrustConfig_TrustStores_IntermediateCas {
    pub fn build(self) -> CertificateManagerTrustConfig_TrustStores_IntermediateCas {
        CertificateManagerTrustConfig_TrustStores_IntermediateCas {
            pem_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CertificateManagerTrustConfig_TrustStores_IntermediateCasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfig_TrustStores_IntermediateCasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerTrustConfig_TrustStores_IntermediateCasRef {
        CertificateManagerTrustConfig_TrustStores_IntermediateCasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfig_TrustStores_IntermediateCasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM intermediate certificate used for building up paths for validation.\nEach certificate provided in PEM format may occupy up to 5kB."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct CertificateManagerTrustConfig_TrustStores_TrustAnchors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerTrustConfig_TrustStores_TrustAnchors {
    #[doc =
        "Set the field `pem_certificate`.\nPEM root certificate of the PKI used for validation.\nEach certificate provided in PEM format may occupy up to 5kB."]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CertificateManagerTrustConfig_TrustStores_TrustAnchors {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfig_TrustStores_TrustAnchors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfig_TrustStores_TrustAnchors {}

impl BuildCertificateManagerTrustConfig_TrustStores_TrustAnchors {
    pub fn build(self) -> CertificateManagerTrustConfig_TrustStores_TrustAnchors {
        CertificateManagerTrustConfig_TrustStores_TrustAnchors { pem_certificate: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CertificateManagerTrustConfig_TrustStores_TrustAnchorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfig_TrustStores_TrustAnchorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerTrustConfig_TrustStores_TrustAnchorsRef {
        CertificateManagerTrustConfig_TrustStores_TrustAnchorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfig_TrustStores_TrustAnchorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM root certificate of the PKI used for validation.\nEach certificate provided in PEM format may occupy up to 5kB."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct CertificateManagerTrustConfig_TrustStoresDynamic {
    intermediate_cas: std
    ::option
    ::Option<terrars::DynamicBlock<CertificateManagerTrustConfig_TrustStores_IntermediateCas>>,
    trust_anchors: std
    ::option
    ::Option<terrars::DynamicBlock<CertificateManagerTrustConfig_TrustStores_TrustAnchors>>,
}

#[derive(Serialize)]
pub struct CertificateManagerTrustConfig_TrustStores {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intermediate_cas: std::option::Option<std::vec::Vec<CertificateManagerTrustConfig_TrustStores_IntermediateCas>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_anchors: std::option::Option<std::vec::Vec<CertificateManagerTrustConfig_TrustStores_TrustAnchors>>,
    dynamic: CertificateManagerTrustConfig_TrustStoresDynamic,
}

impl CertificateManagerTrustConfig_TrustStores {
    #[doc = "Set the field `intermediate_cas`.\n"]
    pub fn set_intermediate_cas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CertificateManagerTrustConfig_TrustStores_IntermediateCas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intermediate_cas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intermediate_cas = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificateManagerTrustConfig_TrustStores_TrustAnchors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust_anchors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_anchors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CertificateManagerTrustConfig_TrustStores {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfig_TrustStores>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfig_TrustStores {}

impl BuildCertificateManagerTrustConfig_TrustStores {
    pub fn build(self) -> CertificateManagerTrustConfig_TrustStores {
        CertificateManagerTrustConfig_TrustStores {
            intermediate_cas: core::default::Default::default(),
            trust_anchors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CertificateManagerTrustConfig_TrustStoresRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfig_TrustStoresRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificateManagerTrustConfig_TrustStoresRef {
        CertificateManagerTrustConfig_TrustStoresRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfig_TrustStoresRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intermediate_cas` after provisioning.\n"]
    pub fn intermediate_cas(&self) -> terrars::ListRef<CertificateManagerTrustConfig_TrustStores_IntermediateCasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intermediate_cas", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(&self) -> terrars::ListRef<CertificateManagerTrustConfig_TrustStores_TrustAnchorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_anchors", self.base))
    }
}
