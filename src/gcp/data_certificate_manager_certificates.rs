#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCertificateManagerCertificatesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCertificateManagerCertificates_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCertificateManagerCertificatesData_>,
}

#[derive(Clone)]
pub struct DataCertificateManagerCertificates(std::rc::Rc<DataCertificateManagerCertificates_>);

impl DataCertificateManagerCertificates {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> terrars::ListRef<DataCertificateManagerCertificates_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataCertificateManagerCertificates {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCertificateManagerCertificates { }

impl terrars::ToListMappable for DataCertificateManagerCertificates {
    type O = terrars::ListRef<DataCertificateManagerCertificatesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCertificateManagerCertificates_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_certificate_manager_certificates".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCertificateManagerCertificates {
    pub tf_id: std::string::String,
}

impl BuildDataCertificateManagerCertificates {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCertificateManagerCertificates {
        let out = DataCertificateManagerCertificates(std::rc::Rc::new(DataCertificateManagerCertificates_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCertificateManagerCertificatesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCertificateManagerCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificateManagerCertificatesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCertificateManagerCertificatesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> terrars::ListRef<DataCertificateManagerCertificates_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_reason`.\n"]
    pub fn set_failure_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failure_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo {
    type O =
        terrars::BlockAssignable<DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo {}

impl BuildDataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo {
    pub fn build(self) -> DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo {
        DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo {
            details: core::default::Default::default(),
            domain: core::default::Default::default(),
            failure_reason: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfoRef {
        DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue {
    type O = terrars::BlockAssignable<DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue {}

impl BuildDataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue {
    pub fn build(self) -> DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue {
        DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue {
            details: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssueRef {
        DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCertificateManagerCertificates_Certificates_Managed {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_attempt_info: std
    ::option
    ::Option<terrars::ListField<DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_authorizations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuance_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_issue: std
    ::option
    ::Option<terrars::ListField<DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCertificateManagerCertificates_Certificates_Managed {
    #[doc = "Set the field `authorization_attempt_info`.\n"]
    pub fn set_authorization_attempt_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfo>,
                    >,
    ) -> Self {
        self.authorization_attempt_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_authorizations`.\n"]
    pub fn set_dns_authorizations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_authorizations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuance_config`.\n"]
    pub fn set_issuance_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuance_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_issue`.\n"]
    pub fn set_provisioning_issue(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssue>,
                    >,
    ) -> Self {
        self.provisioning_issue = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCertificateManagerCertificates_Certificates_Managed {
    type O = terrars::BlockAssignable<DataCertificateManagerCertificates_Certificates_Managed>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificateManagerCertificates_Certificates_Managed {}

impl BuildDataCertificateManagerCertificates_Certificates_Managed {
    pub fn build(self) -> DataCertificateManagerCertificates_Certificates_Managed {
        DataCertificateManagerCertificates_Certificates_Managed {
            authorization_attempt_info: core::default::Default::default(),
            dns_authorizations: core::default::Default::default(),
            domains: core::default::Default::default(),
            issuance_config: core::default::Default::default(),
            provisioning_issue: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCertificateManagerCertificates_Certificates_ManagedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificateManagerCertificates_Certificates_ManagedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCertificateManagerCertificates_Certificates_ManagedRef {
        DataCertificateManagerCertificates_Certificates_ManagedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificateManagerCertificates_Certificates_ManagedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_attempt_info` after provisioning.\n"]
    pub fn authorization_attempt_info(
        &self,
    ) -> terrars::ListRef<DataCertificateManagerCertificates_Certificates_Managed_AuthorizationAttemptInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_attempt_info", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_authorizations` after provisioning.\n"]
    pub fn dns_authorizations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_authorizations", self.base))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc = "Get a reference to the value of field `issuance_config` after provisioning.\n"]
    pub fn issuance_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioning_issue` after provisioning.\n"]
    pub fn provisioning_issue(
        &self,
    ) -> terrars::ListRef<DataCertificateManagerCertificates_Certificates_Managed_ProvisioningIssueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioning_issue", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCertificateManagerCertificates_Certificates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed: std::option::Option<terrars::ListField<DataCertificateManagerCertificates_Certificates_Managed>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    san_dnsnames: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terraform_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataCertificateManagerCertificates_Certificates {
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.effective_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed`.\n"]
    pub fn set_managed(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCertificateManagerCertificates_Certificates_Managed>>,
    ) -> Self {
        self.managed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `san_dnsnames`.\n"]
    pub fn set_san_dnsnames(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.san_dnsnames = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.terraform_labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCertificateManagerCertificates_Certificates {
    type O = terrars::BlockAssignable<DataCertificateManagerCertificates_Certificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificateManagerCertificates_Certificates {}

impl BuildDataCertificateManagerCertificates_Certificates {
    pub fn build(self) -> DataCertificateManagerCertificates_Certificates {
        DataCertificateManagerCertificates_Certificates {
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            managed: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            san_dnsnames: core::default::Default::default(),
            scope: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCertificateManagerCertificates_CertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificateManagerCertificates_CertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCertificateManagerCertificates_CertificatesRef {
        DataCertificateManagerCertificates_CertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificateManagerCertificates_CertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `managed` after provisioning.\n"]
    pub fn managed(&self) -> terrars::ListRef<DataCertificateManagerCertificates_Certificates_ManagedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `san_dnsnames` after provisioning.\n"]
    pub fn san_dnsnames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.san_dnsnames", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.base))
    }
}
