#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamWorkloadIdentityPoolData_ {
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
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    workload_identity_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attestation_rules: std::option::Option<std::vec::Vec<IamWorkloadIdentityPool_AttestationRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_certificate_issuance_config: std
    ::option
    ::Option<std::vec::Vec<IamWorkloadIdentityPool_InlineCertificateIssuanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_trust_config: std::option::Option<std::vec::Vec<IamWorkloadIdentityPool_InlineTrustConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamWorkloadIdentityPool_Timeouts>,
    dynamic: IamWorkloadIdentityPoolDynamic,
}

struct IamWorkloadIdentityPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamWorkloadIdentityPoolData_>,
}

#[derive(Clone)]
pub struct IamWorkloadIdentityPool(std::rc::Rc<IamWorkloadIdentityPool_>);

impl IamWorkloadIdentityPool {
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

    #[doc = "Set the field `description`.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nThe mode for the pool is operating in. Pools with an unspecified mode will operate as if they\nare in 'FEDERATION_ONLY' mode.\n\n~> **Note** This field cannot be changed after the Workload Identity Pool is created. While\n'terraform plan' may show an update if you change this field's value, 'terraform apply'\n**will fail with an API error** (such as 'Error 400: Attempted to update an immutable field.').\nTo specify a different 'mode', please create a new Workload Identity Pool resource.\n\n* 'FEDERATION_ONLY': Pools can only be used for federating external workload identities into\nGoogle Cloud. Unless otherwise noted, no structure or format constraints are applied to\nworkload identities in a 'FEDERATION_ONLY' mode pool, and you may not create any resources\nwithin the pool besides providers.\n* 'TRUST_DOMAIN': Pools can be used to assign identities to Google Cloud workloads. All\nidentities within a 'TRUST_DOMAIN' mode pool must consist of a single namespace and individual\nworkload identifier. The subject identifier for all identities must conform to the following\nformat: 'ns/<namespace>/sa/<workload_identifier>'.\n'google_iam_workload_identity_pool_provider's cannot be created within 'TRUST_DOMAIN'\nmode pools.\n* 'SYSTEM_TRUST_DOMAIN': Pools are managed by Google Cloud services. Neither\n'google_iam_workload_identity_pool_namespace's nor 'google_iam_workload_identity_pool_provider's\ncan be created within 'SYSTEM_TRUST_DOMAIN' mode pools. All identities within a\n'SYSTEM_TRUST_DOMAIN' mode pool are in one of the following formats:\n\n    * 'spiffe://<trust-domain>/ns/<kubernetes-namespace>/sa/<kubernetes-service-account>'\n    * 'spiffe://<trust-domain>/resources/<resource-scope>/<resource-name>' Possible values: [\"FEDERATION_ONLY\", \"TRUST_DOMAIN\", \"SYSTEM_TRUST_DOMAIN\"]"]
    pub fn set_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attestation_rules`.\n"]
    pub fn set_attestation_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkloadIdentityPool_AttestationRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inline_certificate_issuance_config`.\n"]
    pub fn set_inline_certificate_issuance_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IamWorkloadIdentityPool_InlineCertificateIssuanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inline_certificate_issuance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inline_certificate_issuance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inline_trust_config`.\n"]
    pub fn set_inline_trust_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkloadIdentityPool_InlineTrustConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inline_trust_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inline_trust_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IamWorkloadIdentityPool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\n"]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> terrars::ListRef<IamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inline_trust_config` after provisioning.\n"]
    pub fn inline_trust_config(&self) -> terrars::ListRef<IamWorkloadIdentityPool_InlineTrustConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_trust_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPool_TimeoutsRef {
        IamWorkloadIdentityPool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IamWorkloadIdentityPool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamWorkloadIdentityPool { }

impl terrars::ToListMappable for IamWorkloadIdentityPool {
    type O = terrars::ListRef<IamWorkloadIdentityPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamWorkloadIdentityPool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_workload_identity_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamWorkloadIdentityPool {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPool {
    pub fn build(self, stack: &mut terrars::Stack) -> IamWorkloadIdentityPool {
        let out = IamWorkloadIdentityPool(std::rc::Rc::new(IamWorkloadIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamWorkloadIdentityPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                mode: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                attestation_rules: core::default::Default::default(),
                inline_certificate_issuance_config: core::default::Default::default(),
                inline_trust_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamWorkloadIdentityPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamWorkloadIdentityPoolRef {
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

    #[doc = "Get a reference to the value of field `inline_certificate_issuance_config` after provisioning.\n"]
    pub fn inline_certificate_issuance_config(
        &self,
    ) -> terrars::ListRef<IamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.inline_certificate_issuance_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inline_trust_config` after provisioning.\n"]
    pub fn inline_trust_config(&self) -> terrars::ListRef<IamWorkloadIdentityPool_InlineTrustConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_trust_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPool_TimeoutsRef {
        IamWorkloadIdentityPool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolDynamic {
    attestation_rules: std::option::Option<terrars::DynamicBlock<IamWorkloadIdentityPool_AttestationRules>>,
    inline_certificate_issuance_config: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkloadIdentityPool_InlineCertificateIssuanceConfig>>,
    inline_trust_config: std::option::Option<terrars::DynamicBlock<IamWorkloadIdentityPool_InlineTrustConfig>>,
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPool_AttestationRules {
    google_cloud_resource: terrars::PrimField<std::string::String>,
}

impl IamWorkloadIdentityPool_AttestationRules { }

impl terrars::ToListMappable for IamWorkloadIdentityPool_AttestationRules {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPool_AttestationRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPool_AttestationRules {
    #[doc =
        "A single workload operating on Google Cloud. For example:\n'//run.googleapis.com/projects/123/type/Service/*'."]
    pub google_cloud_resource: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPool_AttestationRules {
    pub fn build(self) -> IamWorkloadIdentityPool_AttestationRules {
        IamWorkloadIdentityPool_AttestationRules { google_cloud_resource: self.google_cloud_resource }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPool_AttestationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPool_AttestationRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPool_AttestationRulesRef {
        IamWorkloadIdentityPool_AttestationRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPool_AttestationRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `google_cloud_resource` after provisioning.\nA single workload operating on Google Cloud. For example:\n'//run.googleapis.com/projects/123/type/Service/*'."]
    pub fn google_cloud_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_cloud_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
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

impl IamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
    #[doc =
        "Set the field `ca_pools`.\nA required mapping of a cloud region to the CA pool resource located in that region used\nfor certificate issuance, adhering to these constraints:\n\n* **Key format:** A supported cloud region name equivalent to the location identifier in\nthe corresponding map entry's value.\n* **Value format:** A valid CA pool resource path format like:\n'projects/{project}/locations/{location}/caPools/{ca_pool}'\n* **Region Matching:** Workloads are ONLY issued certificates from CA pools within the\nsame region. Also the CA pool region (in value) must match the workload's region (key)."]
    pub fn set_ca_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ca_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_algorithm`.\nKey algorithm to use when generating the key pair. This key pair will be used to create\nthe certificate. If unspecified, this will default to 'ECDSA_P256'.\n\n* 'RSA_2048': Specifies RSA with a 2048-bit modulus.\n* 'RSA_3072': Specifies RSA with a 3072-bit modulus.\n* 'RSA_4096': Specifies RSA with a 4096-bit modulus.\n* 'ECDSA_P256': Specifies ECDSA with curve P256.\n* 'ECDSA_P384': Specifies ECDSA with curve P384. Possible values: [\"RSA_2048\", \"RSA_3072\", \"RSA_4096\", \"ECDSA_P256\", \"ECDSA_P384\"]"]
    pub fn set_key_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lifetime`.\nLifetime of the workload certificates issued by the CA pool in seconds. Must be between\n'86400s' (24 hours) to '2592000s' (30 days), ends in the suffix \"'s'\" (indicating seconds)\nand is preceded by the number of seconds. If unspecified, this will be defaulted to\n'86400s' (24 hours)."]
    pub fn set_lifetime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rotation_window_percentage`.\nRotation window percentage indicating when certificate rotation should be initiated based\non remaining lifetime. Must be between '50' - '80'. If unspecified, this will be defaulted\nto '50'."]
    pub fn set_rotation_window_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rotation_window_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_default_shared_ca`.\nIf set to true, the trust domain will utilize the GCP-provisioned default CA. A default\nCA in the same region as the workload will be selected to issue the certificate. Enabling\nthis will clear any existing 'ca_pools' configuration to provision the certificates.\n\n~> **Note** This field is mutually exclusive with 'ca_pools'. If this flag is enabled,\ncertificates will be automatically provisioned from the default shared CAs. This flag should\nnot be set if you want to use your own CA pools to provision the certificates."]
    pub fn set_use_default_shared_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_default_shared_ca = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPool_InlineCertificateIssuanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {}

impl BuildIamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
    pub fn build(self) -> IamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
        IamWorkloadIdentityPool_InlineCertificateIssuanceConfig {
            ca_pools: core::default::Default::default(),
            key_algorithm: core::default::Default::default(),
            lifetime: core::default::Default::default(),
            rotation_window_percentage: core::default::Default::default(),
            use_default_shared_ca: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
        IamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPool_InlineCertificateIssuanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_pools` after provisioning.\nA required mapping of a cloud region to the CA pool resource located in that region used\nfor certificate issuance, adhering to these constraints:\n\n* **Key format:** A supported cloud region name equivalent to the location identifier in\nthe corresponding map entry's value.\n* **Value format:** A valid CA pool resource path format like:\n'projects/{project}/locations/{location}/caPools/{ca_pool}'\n* **Region Matching:** Workloads are ONLY issued certificates from CA pools within the\nsame region. Also the CA pool region (in value) must match the workload's region (key)."]
    pub fn ca_pools(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ca_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_algorithm` after provisioning.\nKey algorithm to use when generating the key pair. This key pair will be used to create\nthe certificate. If unspecified, this will default to 'ECDSA_P256'.\n\n* 'RSA_2048': Specifies RSA with a 2048-bit modulus.\n* 'RSA_3072': Specifies RSA with a 3072-bit modulus.\n* 'RSA_4096': Specifies RSA with a 4096-bit modulus.\n* 'ECDSA_P256': Specifies ECDSA with curve P256.\n* 'ECDSA_P384': Specifies ECDSA with curve P384. Possible values: [\"RSA_2048\", \"RSA_3072\", \"RSA_4096\", \"ECDSA_P256\", \"ECDSA_P384\"]"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nLifetime of the workload certificates issued by the CA pool in seconds. Must be between\n'86400s' (24 hours) to '2592000s' (30 days), ends in the suffix \"'s'\" (indicating seconds)\nand is preceded by the number of seconds. If unspecified, this will be defaulted to\n'86400s' (24 hours)."]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rotation_window_percentage` after provisioning.\nRotation window percentage indicating when certificate rotation should be initiated based\non remaining lifetime. Must be between '50' - '80'. If unspecified, this will be defaulted\nto '50'."]
    pub fn rotation_window_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_window_percentage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_default_shared_ca` after provisioning.\nIf set to true, the trust domain will utilize the GCP-provisioned default CA. A default\nCA in the same region as the workload will be selected to issue the certificate. Enabling\nthis will clear any existing 'ca_pools' configuration to provision the certificates.\n\n~> **Note** This field is mutually exclusive with 'ca_pools'. If this flag is enabled,\ncertificates will be automatically provisioned from the default shared CAs. This flag should\nnot be set if you want to use your own CA pools to provision the certificates."]
    pub fn use_default_shared_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_default_shared_ca", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    pem_certificate: terrars::PrimField<std::string::String>,
}

impl IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors { }

impl terrars::ToListMappable for IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    #[doc =
        "PEM certificate of the PKI used for validation. Must only contain one ca\ncertificate(either root or intermediate cert)."]
    pub pem_certificate: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
    pub fn build(self) -> IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
        IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors {
            pem_certificate: self.pem_certificate,
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
        IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM certificate of the PKI used for validation. Must only contain one ca\ncertificate(either root or intermediate cert)."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesDynamic {
    trust_anchors: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors>>,
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_default_shared_ca: std::option::Option<terrars::PrimField<bool>>,
    trust_domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_anchors: std
    ::option
    ::Option<std::vec::Vec<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors>>,
    dynamic: IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesDynamic,
}

impl IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    #[doc =
        "Set the field `trust_default_shared_ca`.\nIf set to True, the trust bundle will include the private ca managed identity regional root\npublic certificates.\n\n~> **Note** 'trust_default_shared_ca' is only supported for managed identity trust domain\nresource."]
    pub fn set_trust_default_shared_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.trust_default_shared_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchors,
                        >,
                    >,
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

impl terrars::ToListMappable for IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    #[doc = ""]
    pub trust_domain: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
    pub fn build(self) -> IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
        IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles {
            trust_default_shared_ca: core::default::Default::default(),
            trust_domain: self.trust_domain,
            trust_anchors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
        IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundlesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `trust_default_shared_ca` after provisioning.\nIf set to True, the trust bundle will include the private ca managed identity regional root\npublic certificates.\n\n~> **Note** 'trust_default_shared_ca' is only supported for managed identity trust domain\nresource."]
    pub fn trust_default_shared_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_default_shared_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_domain` after provisioning.\n"]
    pub fn trust_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(
        &self,
    ) -> terrars::ListRef<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles_TrustAnchorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_anchors", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkloadIdentityPool_InlineTrustConfigDynamic {
    additional_trust_bundles: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles>>,
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPool_InlineTrustConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_trust_bundles: std
    ::option
    ::Option<std::vec::Vec<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles>>,
    dynamic: IamWorkloadIdentityPool_InlineTrustConfigDynamic,
}

impl IamWorkloadIdentityPool_InlineTrustConfig {
    #[doc = "Set the field `additional_trust_bundles`.\n"]
    pub fn set_additional_trust_bundles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IamWorkloadIdentityPool_InlineTrustConfig_AdditionalTrustBundles>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_trust_bundles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_trust_bundles = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamWorkloadIdentityPool_InlineTrustConfig {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPool_InlineTrustConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPool_InlineTrustConfig {}

impl BuildIamWorkloadIdentityPool_InlineTrustConfig {
    pub fn build(self) -> IamWorkloadIdentityPool_InlineTrustConfig {
        IamWorkloadIdentityPool_InlineTrustConfig {
            additional_trust_bundles: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPool_InlineTrustConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPool_InlineTrustConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPool_InlineTrustConfigRef {
        IamWorkloadIdentityPool_InlineTrustConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPool_InlineTrustConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkloadIdentityPool_Timeouts {
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

impl terrars::ToListMappable for IamWorkloadIdentityPool_Timeouts {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPool_Timeouts {}

impl BuildIamWorkloadIdentityPool_Timeouts {
    pub fn build(self) -> IamWorkloadIdentityPool_Timeouts {
        IamWorkloadIdentityPool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPool_TimeoutsRef {
        IamWorkloadIdentityPool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPool_TimeoutsRef {
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
