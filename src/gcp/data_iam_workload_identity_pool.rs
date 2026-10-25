#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataIamWorkloadIdentityPoolData_ {
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
    workload_identity_pool_id: terrars::PrimField<std::string::String>,
}

struct DataIamWorkloadIdentityPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamWorkloadIdentityPoolData_>,
}

#[derive(Clone)]
pub struct DataIamWorkloadIdentityPool(std::rc::Rc<DataIamWorkloadIdentityPool_>);

impl DataIamWorkloadIdentityPool {
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

    #[doc =
        "Get a reference to the value of field `attestation_rules` after provisioning.\nDefines which workloads can receive an identity within a pool. When an AttestationRule is\ndefined under a managed identity, matching workloads may receive that identity. A maximum of\n50 AttestationRules can be set."]
    pub fn attestation_rules(&self) -> terrars::SetRef<DataIamWorkloadIdentityPool_AttestationRulesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attestation_rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\nRepresents configuration for generating mutual TLS (mTLS) certificates for the identities\nwithin this pool. Defines the Certificate Authority (CA) pool resources and configurations\nrequired for issuance and rotation of mTLS workload certificates."]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> terrars::ListRef<DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `inline_trust_config` after provisioning.\nRepresents config to add additional trusted trust domains. Defines configuration for extending\ntrust to additional trust domains. By establishing trust with another domain, the current\ndomain will recognize and accept certificates issued by entities within the trusted domains.\nNote that a trust domain automatically trusts itself, eliminating the need for explicit\nconfiguration."]
    pub fn inline_trust_config(&self) -> terrars::ListRef<DataIamWorkloadIdentityPool_InlineTrustConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_trust_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode for the pool is operating in. Pools with an unspecified mode will operate as if they\nare in 'FEDERATION_ONLY' mode.\n\n~> **Note** This field cannot be changed after the Workload Identity Pool is created. While\n'terraform plan' may show an update if you change this field's value, 'terraform apply'\n**will fail with an API error** (such as 'Error 400: Attempted to update an immutable field.').\nTo specify a different 'mode', please create a new Workload Identity Pool resource.\n\n* 'FEDERATION_ONLY': Pools can only be used for federating external workload identities into\nGoogle Cloud. Unless otherwise noted, no structure or format constraints are applied to\nworkload identities in a 'FEDERATION_ONLY' mode pool, and you may not create any resources\nwithin the pool besides providers.\n* 'TRUST_DOMAIN': Pools can be used to assign identities to Google Cloud workloads. All\nidentities within a 'TRUST_DOMAIN' mode pool must consist of a single namespace and individual\nworkload identifier. The subject identifier for all identities must conform to the following\nformat: 'ns/<namespace>/sa/<workload_identifier>'.\n'google_iam_workload_identity_pool_provider's cannot be created within 'TRUST_DOMAIN'\nmode pools.\n* 'SYSTEM_TRUST_DOMAIN': Pools are managed by Google Cloud services. Neither\n'google_iam_workload_identity_pool_namespace's nor 'google_iam_workload_identity_pool_provider's\ncan be created within 'SYSTEM_TRUST_DOMAIN' mode pools. All identities within a\n'SYSTEM_TRUST_DOMAIN' mode pool are in one of the following formats:\n\n    * 'spiffe://<trust-domain>/ns/<kubernetes-namespace>/sa/<kubernetes-service-account>'\n    * 'spiffe://<trust-domain>/resources/<resource-scope>/<resource-name>' Possible values: [\"FEDERATION_ONLY\", \"TRUST_DOMAIN\", \"SYSTEM_TRUST_DOMAIN\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the pool as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the pool.\n* 'STATE_UNSPECIFIED': State unspecified.\n* 'ACTIVE': The pool is active, and may be used in Google Cloud policies.\n* 'DELETED': The pool is soft-deleted. Soft-deleted pools are permanently deleted after\n  approximately 30 days. You can restore a soft-deleted pool using\n  'UndeleteWorkloadIdentityPool'. You cannot reuse the ID of a soft-deleted pool until it is\n  permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or\n  use existing tokens to access resources. If the pool is undeleted, existing tokens grant\n  access again."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamWorkloadIdentityPool {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamWorkloadIdentityPool { }

impl terrars::ToListMappable for DataIamWorkloadIdentityPool {
    type O = terrars::ListRef<DataIamWorkloadIdentityPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamWorkloadIdentityPool_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_iam_workload_identity_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamWorkloadIdentityPool {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildDataIamWorkloadIdentityPool {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamWorkloadIdentityPool {
        let out = DataIamWorkloadIdentityPool(std::rc::Rc::new(DataIamWorkloadIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamWorkloadIdentityPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamWorkloadIdentityPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamWorkloadIdentityPoolRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `attestation_rules` after provisioning.\nDefines which workloads can receive an identity within a pool. When an AttestationRule is\ndefined under a managed identity, matching workloads may receive that identity. A maximum of\n50 AttestationRules can be set."]
    pub fn attestation_rules(&self) -> terrars::SetRef<DataIamWorkloadIdentityPool_AttestationRulesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attestation_rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\nRepresents configuration for generating mutual TLS (mTLS) certificates for the identities\nwithin this pool. Defines the Certificate Authority (CA) pool resources and configurations\nrequired for issuance and rotation of mTLS workload certificates."]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> terrars::ListRef<DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `inline_trust_config` after provisioning.\nRepresents config to add additional trusted trust domains. Defines configuration for extending\ntrust to additional trust domains. By establishing trust with another domain, the current\ndomain will recognize and accept certificates issued by entities within the trusted domains.\nNote that a trust domain automatically trusts itself, eliminating the need for explicit\nconfiguration."]
    pub fn inline_trust_config(&self) -> terrars::ListRef<DataIamWorkloadIdentityPool_InlineTrustConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_trust_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode for the pool is operating in. Pools with an unspecified mode will operate as if they\nare in 'FEDERATION_ONLY' mode.\n\n~> **Note** This field cannot be changed after the Workload Identity Pool is created. While\n'terraform plan' may show an update if you change this field's value, 'terraform apply'\n**will fail with an API error** (such as 'Error 400: Attempted to update an immutable field.').\nTo specify a different 'mode', please create a new Workload Identity Pool resource.\n\n* 'FEDERATION_ONLY': Pools can only be used for federating external workload identities into\nGoogle Cloud. Unless otherwise noted, no structure or format constraints are applied to\nworkload identities in a 'FEDERATION_ONLY' mode pool, and you may not create any resources\nwithin the pool besides providers.\n* 'TRUST_DOMAIN': Pools can be used to assign identities to Google Cloud workloads. All\nidentities within a 'TRUST_DOMAIN' mode pool must consist of a single namespace and individual\nworkload identifier. The subject identifier for all identities must conform to the following\nformat: 'ns/<namespace>/sa/<workload_identifier>'.\n'google_iam_workload_identity_pool_provider's cannot be created within 'TRUST_DOMAIN'\nmode pools.\n* 'SYSTEM_TRUST_DOMAIN': Pools are managed by Google Cloud services. Neither\n'google_iam_workload_identity_pool_namespace's nor 'google_iam_workload_identity_pool_provider's\ncan be created within 'SYSTEM_TRUST_DOMAIN' mode pools. All identities within a\n'SYSTEM_TRUST_DOMAIN' mode pool are in one of the following formats:\n\n    * 'spiffe://<trust-domain>/ns/<kubernetes-namespace>/sa/<kubernetes-service-account>'\n    * 'spiffe://<trust-domain>/resources/<resource-scope>/<resource-name>' Possible values: [\"FEDERATION_ONLY\", \"TRUST_DOMAIN\", \"SYSTEM_TRUST_DOMAIN\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the pool as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the pool.\n* 'STATE_UNSPECIFIED': State unspecified.\n* 'ACTIVE': The pool is active, and may be used in Google Cloud policies.\n* 'DELETED': The pool is soft-deleted. Soft-deleted pools are permanently deleted after\n  approximately 30 days. You can restore a soft-deleted pool using\n  'UndeleteWorkloadIdentityPool'. You cannot reuse the ID of a soft-deleted pool until it is\n  permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or\n  use existing tokens to access resources. If the pool is undeleted, existing tokens grant\n  access again."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPool_AttestationRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPool_AttestationRules {
    #[doc = "Set the field `google_cloud_resource`.\n"]
    pub fn set_google_cloud_resource(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.google_cloud_resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPool_AttestationRules {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPool_AttestationRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPool_AttestationRules {}

impl BuildDataIamWorkloadIdentityPool_AttestationRules {
    pub fn build(self) -> DataIamWorkloadIdentityPool_AttestationRules {
        DataIamWorkloadIdentityPool_AttestationRules { google_cloud_resource: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPool_AttestationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPool_AttestationRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPool_AttestationRulesRef {
        DataIamWorkloadIdentityPool_AttestationRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPool_AttestationRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `google_cloud_resource` after provisioning.\n"]
    pub fn google_cloud_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_cloud_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_pools: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifetime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_window_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_default_shared_ca: std::option::Option<terrars::PrimField<bool>>,
}

impl DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
    #[doc = "Set the field `ca_pools`.\n"]
    pub fn set_ca_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ca_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_algorithm`.\n"]
    pub fn set_key_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifetime`.\n"]
    pub fn set_lifetime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_window_percentage`.\n"]
    pub fn set_rotation_window_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rotation_window_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_default_shared_ca`.\n"]
    pub fn set_use_default_shared_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_default_shared_ca = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {}

impl BuildDataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
    pub fn build(self) -> DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
        DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
            ca_pools: core::default::Default::default(),
            key_algorithm: core::default::Default::default(),
            lifetime: core::default::Default::default(),
            rotation_window_percentage: core::default::Default::default(),
            use_default_shared_ca: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
        DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_pools` after provisioning.\n"]
    pub fn ca_pools(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ca_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\n"]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_window_percentage` after provisioning.\n"]
    pub fn rotation_window_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_window_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `use_default_shared_ca` after provisioning.\n"]
    pub fn use_default_shared_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_default_shared_ca", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    #[doc = "Set the field `pem_certificate`.\n"]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    type O =
        terrars::BlockAssignable<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {}

impl BuildDataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    pub fn build(self) -> DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
        DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
            pem_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
        DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\n"]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_anchors: std
    ::option
    ::Option<terrars::ListField<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_default_shared_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors>,
                    >,
    ) -> Self {
        self.trust_anchors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_default_shared_ca`.\n"]
    pub fn set_trust_default_shared_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.trust_default_shared_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_domain`.\n"]
    pub fn set_trust_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {}

impl BuildDataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    pub fn build(self) -> DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
        DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
            trust_anchors: core::default::Default::default(),
            trust_default_shared_ca: core::default::Default::default(),
            trust_domain: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
        DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(
        &self,
    ) -> terrars::ListRef<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_anchors", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_default_shared_ca` after provisioning.\n"]
    pub fn trust_default_shared_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_default_shared_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_domain` after provisioning.\n"]
    pub fn trust_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPool_InlineTrustConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_trust_bundles: std
    ::option
    ::Option<terrars::SetField<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles>>,
}

impl DataIamWorkloadIdentityPool_InlineTrustConfig {
    #[doc = "Set the field `additional_trust_bundles`.\n"]
    pub fn set_additional_trust_bundles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles>>,
    ) -> Self {
        self.additional_trust_bundles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPool_InlineTrustConfig {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPool_InlineTrustConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPool_InlineTrustConfig {}

impl BuildDataIamWorkloadIdentityPool_InlineTrustConfig {
    pub fn build(self) -> DataIamWorkloadIdentityPool_InlineTrustConfig {
        DataIamWorkloadIdentityPool_InlineTrustConfig { additional_trust_bundles: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPool_InlineTrustConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPool_InlineTrustConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPool_InlineTrustConfigRef {
        DataIamWorkloadIdentityPool_InlineTrustConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPool_InlineTrustConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_trust_bundles` after provisioning.\n"]
    pub fn additional_trust_bundles(
        &self,
    ) -> terrars::SetRef<DataIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_trust_bundles", self.base))
    }
}
