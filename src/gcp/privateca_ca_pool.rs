#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PrivatecaCaPoolData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<PrivatecaCaPool_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuance_policy: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publishing_options: std::option::Option<std::vec::Vec<PrivatecaCaPool_PublishingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrivatecaCaPool_Timeouts>,
    dynamic: PrivatecaCaPoolDynamic,
}

struct PrivatecaCaPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrivatecaCaPoolData_>,
}

#[derive(Clone)]
pub struct PrivatecaCaPool(std::rc::Rc<PrivatecaCaPool_>);

impl PrivatecaCaPool {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_EncryptionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `issuance_policy`.\n"]
    pub fn set_issuance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().issuance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.issuance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publishing_options`.\n"]
    pub fn set_publishing_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_PublishingOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().publishing_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.publishing_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrivatecaCaPool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the CaPool. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this CaPool."]
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
        "Get a reference to the value of field `tier` after provisioning.\nThe Tier of this CaPool. Possible values: [\"ENTERPRISE\", \"DEVOPS\"]"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<PrivatecaCaPool_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `issuance_policy` after provisioning.\n"]
    pub fn issuance_policy(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issuance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publishing_options` after provisioning.\n"]
    pub fn publishing_options(&self) -> terrars::ListRef<PrivatecaCaPool_PublishingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCaPool_TimeoutsRef {
        PrivatecaCaPool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PrivatecaCaPool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrivatecaCaPool { }

impl terrars::ToListMappable for PrivatecaCaPool {
    type O = terrars::ListRef<PrivatecaCaPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrivatecaCaPool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_privateca_ca_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrivatecaCaPool {
    pub tf_id: std::string::String,
    #[doc =
        "Location of the CaPool. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name for this CaPool."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The Tier of this CaPool. Possible values: [\"ENTERPRISE\", \"DEVOPS\"]"]
    pub tier: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCaPool {
    pub fn build(self, stack: &mut terrars::Stack) -> PrivatecaCaPool {
        let out = PrivatecaCaPool(std::rc::Rc::new(PrivatecaCaPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrivatecaCaPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                tier: self.tier,
                encryption_spec: core::default::Default::default(),
                issuance_policy: core::default::Default::default(),
                publishing_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrivatecaCaPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrivatecaCaPoolRef {
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
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the CaPool. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this CaPool."]
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
        "Get a reference to the value of field `tier` after provisioning.\nThe Tier of this CaPool. Possible values: [\"ENTERPRISE\", \"DEVOPS\"]"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<PrivatecaCaPool_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `issuance_policy` after provisioning.\n"]
    pub fn issuance_policy(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issuance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publishing_options` after provisioning.\n"]
    pub fn publishing_options(&self) -> terrars::ListRef<PrivatecaCaPool_PublishingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCaPool_TimeoutsRef {
        PrivatecaCaPool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPoolDynamic {
    encryption_spec: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_EncryptionSpec>>,
    issuance_policy: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy>>,
    publishing_options: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_PublishingOptions>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_EncryptionSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCaPool_EncryptionSpec {
    #[doc =
        "Set the field `cloud_kms_key`.\nThe resource name for an existing Cloud KMS key in the format\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub fn set_cloud_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_EncryptionSpec {
    type O = terrars::BlockAssignable<PrivatecaCaPool_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_EncryptionSpec {}

impl BuildPrivatecaCaPool_EncryptionSpec {
    pub fn build(self) -> PrivatecaCaPool_EncryptionSpec {
        PrivatecaCaPool_EncryptionSpec { cloud_kms_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCaPool_EncryptionSpecRef {
        PrivatecaCaPool_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_kms_key` after provisioning.\nThe resource name for an existing Cloud KMS key in the format\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub fn cloud_kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes {
    allow_config_based_issuance: terrars::PrimField<bool>,
    allow_csr_based_issuance: terrars::PrimField<bool>,
}

impl PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes { }

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes {
    #[doc = "When true, allows callers to create Certificates by specifying a CertificateConfig."]
    pub allow_config_based_issuance: terrars::PrimField<bool>,
    #[doc = "When true, allows callers to create Certificates by specifying a CSR."]
    pub allow_csr_based_issuance: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes {
        PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes {
            allow_config_based_issuance: self.allow_config_based_issuance,
            allow_csr_based_issuance: self.allow_csr_based_issuance,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModesRef {
        PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_config_based_issuance` after provisioning.\nWhen true, allows callers to create Certificates by specifying a CertificateConfig."]
    pub fn allow_config_based_issuance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_config_based_issuance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_csr_based_issuance` after provisioning.\nWhen true, allows callers to create Certificates by specifying a CSR."]
    pub fn allow_csr_based_issuance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_csr_based_issuance", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve {
    signature_algorithm: terrars::PrimField<std::string::String>,
}

impl PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve { }

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve {
    #[doc = "The algorithm used. Possible values: [\"ECDSA_P256\", \"ECDSA_P384\", \"EDDSA_25519\"]"]
    pub signature_algorithm: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve {
        PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve {
            signature_algorithm: self.signature_algorithm,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurveRef {
        PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `signature_algorithm` after provisioning.\nThe algorithm used. Possible values: [\"ECDSA_P256\", \"ECDSA_P384\", \"EDDSA_25519\"]"]
    pub fn signature_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_algorithm", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_modulus_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_modulus_size: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa {
    #[doc =
        "Set the field `max_modulus_size`.\nThe maximum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice will not enforce an explicit upper bound on RSA modulus sizes."]
    pub fn set_max_modulus_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_modulus_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_modulus_size`.\nThe minimum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice-level min RSA modulus size will continue to apply."]
    pub fn set_min_modulus_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_modulus_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa {}

impl BuildPrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa {
        PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa {
            max_modulus_size: core::default::Default::default(),
            min_modulus_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_RsaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_RsaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_RsaRef {
        PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_RsaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_RsaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_modulus_size` after provisioning.\nThe maximum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice will not enforce an explicit upper bound on RSA modulus sizes."]
    pub fn max_modulus_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_modulus_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_modulus_size` after provisioning.\nThe minimum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the\nservice-level min RSA modulus size will continue to apply."]
    pub fn min_modulus_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_modulus_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesDynamic {
    elliptic_curve: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve>>,
    rsa: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elliptic_curve: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa>>,
    dynamic: PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesDynamic,
}

impl PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes {
    #[doc = "Set the field `elliptic_curve`.\n"]
    pub fn set_elliptic_curve(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurve>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elliptic_curve = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elliptic_curve = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rsa`.\n"]
    pub fn set_rsa(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_Rsa>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rsa = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rsa = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_AllowedKeyTypes {}

impl BuildPrivatecaCaPool_IssuancePolicy_AllowedKeyTypes {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes {
        PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes {
            elliptic_curve: core::default::Default::default(),
            rsa: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesRef {
        PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `elliptic_curve` after provisioning.\n"]
    pub fn elliptic_curve(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_EllipticCurveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elliptic_curve", self.base))
    }

    #[doc = "Get a reference to the value of field `rsa` after provisioning.\n"]
    pub fn rsa(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes_RsaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rsa", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId { }

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId {
            object_id_path: self.object_id_path,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectIdRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsDynamic {
    object_id: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions {
    critical: terrars::PrimField<bool>,
    value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId>>,
    dynamic: PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsDynamic,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions {
    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectId>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.object_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.object_id = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions {
    #[doc =
        "Indicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub critical: terrars::PrimField<bool>,
    #[doc = "The value of this X.509 extension. A base64-encoded string."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions {
            critical: self.critical,
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `critical` after provisioning.\nIndicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value of this X.509 extension. A base64-encoded string."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `object_id` after provisioning.\n"]
    pub fn object_id(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions_ObjectIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_issuer_path_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_max_issuer_path_length: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions {
    #[doc = "Set the field `is_ca`.\nWhen true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn set_is_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_issuer_path_length`.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn set_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `non_ca`.\nWhen true, the \"CA\" in Basic Constraints extension will be set to false.\nIf both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate."]
    pub fn set_non_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.non_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zero_max_issuer_path_length`.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn set_zero_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.zero_max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions {}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
            non_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptionsRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_ca` after provisioning.\nWhen true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn is_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_issuer_path_length` after provisioning.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn max_issuer_path_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_issuer_path_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `non_ca` after provisioning.\nWhen true, the \"CA\" in Basic Constraints extension will be set to false.\nIf both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate."]
    pub fn non_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.non_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zero_max_issuer_path_length` after provisioning.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn zero_max_issuer_path_length(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_max_issuer_path_length", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_sign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_commitment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_sign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_encipherment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decipher_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digital_signature: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encipher_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_agreement: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_encipherment: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage {
    #[doc = "Set the field `cert_sign`.\nThe key may be used to sign certificates."]
    pub fn set_cert_sign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cert_sign = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_commitment`.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn set_content_commitment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.content_commitment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crl_sign`.\nThe key may be used sign certificate revocation lists."]
    pub fn set_crl_sign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.crl_sign = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_encipherment`.\nThe key may be used to encipher data."]
    pub fn set_data_encipherment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_encipherment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `decipher_only`.\nThe key may be used to decipher only."]
    pub fn set_decipher_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.decipher_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digital_signature`.\nThe key may be used for digital signatures."]
    pub fn set_digital_signature(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.digital_signature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encipher_only`.\nThe key may be used to encipher only."]
    pub fn set_encipher_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encipher_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_agreement`.\nThe key may be used in a key agreement protocol."]
    pub fn set_key_agreement(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.key_agreement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_encipherment`.\nThe key may be used to encipher other keys."]
    pub fn set_key_encipherment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.key_encipherment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage {}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage {
            cert_sign: core::default::Default::default(),
            content_commitment: core::default::Default::default(),
            crl_sign: core::default::Default::default(),
            data_encipherment: core::default::Default::default(),
            decipher_only: core::default::Default::default(),
            digital_signature: core::default::Default::default(),
            encipher_only: core::default::Default::default(),
            key_agreement: core::default::Default::default(),
            key_encipherment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsageRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cert_sign` after provisioning.\nThe key may be used to sign certificates."]
    pub fn cert_sign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_sign", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_commitment` after provisioning.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn content_commitment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_commitment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `crl_sign` after provisioning.\nThe key may be used sign certificate revocation lists."]
    pub fn crl_sign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crl_sign", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_encipherment` after provisioning.\nThe key may be used to encipher data."]
    pub fn data_encipherment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_encipherment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `decipher_only` after provisioning.\nThe key may be used to decipher only."]
    pub fn decipher_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decipher_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `digital_signature` after provisioning.\nThe key may be used for digital signatures."]
    pub fn digital_signature(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digital_signature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encipher_only` after provisioning.\nThe key may be used to encipher only."]
    pub fn encipher_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encipher_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_agreement` after provisioning.\nThe key may be used in a key agreement protocol."]
    pub fn key_agreement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_agreement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_encipherment` after provisioning.\nThe key may be used to encipher other keys."]
    pub fn key_encipherment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_encipherment", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_signing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocsp_signing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_stamping: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage {
    #[doc =
        "Set the field `client_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn set_client_auth(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `code_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn set_code_signing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.code_signing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `email_protection`.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn set_email_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.email_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ocsp_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn set_ocsp_signing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ocsp_signing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn set_server_auth(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_stamping`.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn set_time_stamping(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.time_stamping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage {}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsageRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn client_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_auth", self.base))
    }

    #[doc =
        "Get a reference to the value of field `code_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn code_signing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_signing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_protection` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn email_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_protection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ocsp_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn ocsp_signing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocsp_signing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn server_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_auth", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_stamping` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn time_stamping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_stamping", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages { }

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages {
    type O =
        terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages {
            object_id_path: self.object_id_path,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsagesRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageDynamic {
    base_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage>>,
    extended_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage>>,
    unknown_extended_key_usages: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_extended_key_usages: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages>>,
    dynamic: PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageDynamic,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.base_key_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.base_key_usage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `extended_key_usage`.\n"]
    pub fn set_extended_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extended_key_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extended_key_usage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unknown_extended_key_usages`.\n"]
    pub fn set_unknown_extended_key_usages(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsages,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unknown_extended_key_usages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unknown_extended_key_usages = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage {}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_BaseKeyUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_ExtendedKeyUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage_UnknownExtendedKeyUsagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unknown_extended_key_usages", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints {
    critical: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints {
    #[doc =
        "Set the field `excluded_dns_names`.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_excluded_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_email_addresses`.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_excluded_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_ip_ranges`.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_excluded_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_uris`.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_excluded_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_dns_names`.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_permitted_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_email_addresses`.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_permitted_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_ip_ranges`.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_permitted_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_uris`.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_permitted_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints {
    #[doc = "Indicates whether or not the name constraints are marked critical."]
    pub critical: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints {
            critical: self.critical,
            excluded_dns_names: core::default::Default::default(),
            excluded_email_addresses: core::default::Default::default(),
            excluded_ip_ranges: core::default::Default::default(),
            excluded_uris: core::default::Default::default(),
            permitted_dns_names: core::default::Default::default(),
            permitted_email_addresses: core::default::Default::default(),
            permitted_ip_ranges: core::default::Default::default(),
            permitted_uris: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraintsRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `critical` after provisioning.\nIndicates whether or not the name constraints are marked critical."]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_dns_names` after provisioning.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn excluded_dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_dns_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_email_addresses` after provisioning.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn excluded_email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_email_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_ip_ranges` after provisioning.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn excluded_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_uris` after provisioning.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn excluded_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_dns_names` after provisioning.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn permitted_dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_dns_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_email_addresses` after provisioning.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn permitted_email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_email_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_ip_ranges` after provisioning.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn permitted_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_uris` after provisioning.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn permitted_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds { }

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds { object_id_path: self.object_id_path }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIdsRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPool_IssuancePolicy_BaselineValuesDynamic {
    additional_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions>>,
    ca_options: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions>>,
    key_usage: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage>>,
    name_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints>>,
    policy_ids: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_ocsp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_options: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_usage: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_constraints: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_ids: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds>>,
    dynamic: PrivatecaCaPool_IssuancePolicy_BaselineValuesDynamic,
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValues {
    #[doc =
        "Set the field `aia_ocsp_servers`.\nDescribes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the\n\"Authority Information Access\" extension in the certificate."]
    pub fn set_aia_ocsp_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aia_ocsp_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_extensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_extensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ca_options`.\n"]
    pub fn set_ca_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ca_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ca_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `key_usage`.\n"]
    pub fn set_key_usage(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key_usage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `name_constraints`.\n"]
    pub fn set_name_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.name_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.name_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy_ids`.\n"]
    pub fn set_policy_ids(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_ids = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_ids = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_BaselineValues {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_BaselineValues {}

impl BuildPrivatecaCaPool_IssuancePolicy_BaselineValues {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_BaselineValues {
        PrivatecaCaPool_IssuancePolicy_BaselineValues {
            aia_ocsp_servers: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            ca_options: core::default::Default::default(),
            key_usage: core::default::Default::default(),
            name_constraints: core::default::Default::default(),
            policy_ids: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_BaselineValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_BaselineValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_BaselineValuesRef {
        PrivatecaCaPool_IssuancePolicy_BaselineValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_BaselineValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aia_ocsp_servers` after provisioning.\nDescribes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the\n\"Authority Information Access\" extension in the certificate."]
    pub fn aia_ocsp_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aia_ocsp_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_AdditionalExtensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_CaOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_KeyUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_NameConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValues_PolicyIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression {
    #[doc =
        "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression {
        PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpressionRef {
        PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPool_IssuancePolicy_IdentityConstraintsDynamic {
    cel_expression: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy_IdentityConstraints {
    allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    allow_subject_passthrough: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cel_expression: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression>>,
    dynamic: PrivatecaCaPool_IssuancePolicy_IdentityConstraintsDynamic,
}

impl PrivatecaCaPool_IssuancePolicy_IdentityConstraints {
    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpression>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cel_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cel_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy_IdentityConstraints {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_IdentityConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy_IdentityConstraints {
    #[doc =
        "If this is set, the SubjectAltNames extension may be copied from a certificate request into the signed certificate.\nOtherwise, the requested SubjectAltNames will be discarded."]
    pub allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    #[doc =
        "If this is set, the Subject field may be copied from a certificate request into the signed certificate.\nOtherwise, the requested Subject will be discarded."]
    pub allow_subject_passthrough: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPool_IssuancePolicy_IdentityConstraints {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy_IdentityConstraints {
        PrivatecaCaPool_IssuancePolicy_IdentityConstraints {
            allow_subject_alt_names_passthrough: self.allow_subject_alt_names_passthrough,
            allow_subject_passthrough: self.allow_subject_passthrough,
            cel_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicy_IdentityConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicy_IdentityConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPool_IssuancePolicy_IdentityConstraintsRef {
        PrivatecaCaPool_IssuancePolicy_IdentityConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicy_IdentityConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_subject_alt_names_passthrough` after provisioning.\nIf this is set, the SubjectAltNames extension may be copied from a certificate request into the signed certificate.\nOtherwise, the requested SubjectAltNames will be discarded."]
    pub fn allow_subject_alt_names_passthrough(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_subject_alt_names_passthrough", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_subject_passthrough` after provisioning.\nIf this is set, the Subject field may be copied from a certificate request into the signed certificate.\nOtherwise, the requested Subject will be discarded."]
    pub fn allow_subject_passthrough(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_subject_passthrough", self.base))
    }

    #[doc = "Get a reference to the value of field `cel_expression` after provisioning.\n"]
    pub fn cel_expression(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_IdentityConstraints_CelExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cel_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPool_IssuancePolicyDynamic {
    allowed_issuance_modes: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes>>,
    allowed_key_types: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes>>,
    baseline_values: std::option::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_BaselineValues>>,
    identity_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPool_IssuancePolicy_IdentityConstraints>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_IssuancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_requester_specified_not_before_time: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backdate_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_lifetime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_issuance_modes: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_key_types: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_values: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_BaselineValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_constraints: std::option::Option<std::vec::Vec<PrivatecaCaPool_IssuancePolicy_IdentityConstraints>>,
    dynamic: PrivatecaCaPool_IssuancePolicyDynamic,
}

impl PrivatecaCaPool_IssuancePolicy {
    #[doc =
        "Set the field `allow_requester_specified_not_before_time`.\nIf set to true, allows requesters to specify the requested_not_before_time\nfield when creating a Certificate. Certificates requested with this\noption enabled will have a 'not_before_time' equal to the value\nspecified in the request."]
    pub fn set_allow_requester_specified_not_before_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.allow_requester_specified_not_before_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `backdate_duration`.\nThe duration to backdate all certificates issued from this CaPool. If not set, the\ncertificates will be issued with a not_before_time of the issuance time (i.e. the current\ntime). If set, the certificates will be issued with a not_before_time of the issuance\ntime minus the backdate_duration. The not_after_time will be adjusted to preserve the\nrequested lifetime. The backdate_duration must be less than or equal to 48 hours."]
    pub fn set_backdate_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backdate_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maximum_lifetime`.\nThe maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority\nexpires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn set_maximum_lifetime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.maximum_lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_issuance_modes`.\n"]
    pub fn set_allowed_issuance_modes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_issuance_modes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_issuance_modes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `allowed_key_types`.\n"]
    pub fn set_allowed_key_types(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_key_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_key_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `baseline_values`.\n"]
    pub fn set_baseline_values(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_BaselineValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.baseline_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_constraints`.\n"]
    pub fn set_identity_constraints(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy_IdentityConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_constraints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_IssuancePolicy {
    type O = terrars::BlockAssignable<PrivatecaCaPool_IssuancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_IssuancePolicy {}

impl BuildPrivatecaCaPool_IssuancePolicy {
    pub fn build(self) -> PrivatecaCaPool_IssuancePolicy {
        PrivatecaCaPool_IssuancePolicy {
            allow_requester_specified_not_before_time: core::default::Default::default(),
            backdate_duration: core::default::Default::default(),
            maximum_lifetime: core::default::Default::default(),
            allowed_issuance_modes: core::default::Default::default(),
            allowed_key_types: core::default::Default::default(),
            baseline_values: core::default::Default::default(),
            identity_constraints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_IssuancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_IssuancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCaPool_IssuancePolicyRef {
        PrivatecaCaPool_IssuancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_IssuancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_requester_specified_not_before_time` after provisioning.\nIf set to true, allows requesters to specify the requested_not_before_time\nfield when creating a Certificate. Certificates requested with this\noption enabled will have a 'not_before_time' equal to the value\nspecified in the request."]
    pub fn allow_requester_specified_not_before_time(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_requester_specified_not_before_time", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `backdate_duration` after provisioning.\nThe duration to backdate all certificates issued from this CaPool. If not set, the\ncertificates will be issued with a not_before_time of the issuance time (i.e. the current\ntime). If set, the certificates will be issued with a not_before_time of the issuance\ntime minus the backdate_duration. The not_after_time will be adjusted to preserve the\nrequested lifetime. The backdate_duration must be less than or equal to 48 hours."]
    pub fn backdate_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backdate_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maximum_lifetime` after provisioning.\nThe maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority\nexpires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn maximum_lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_lifetime", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_issuance_modes` after provisioning.\n"]
    pub fn allowed_issuance_modes(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_AllowedIssuanceModesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_issuance_modes", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_key_types` after provisioning.\n"]
    pub fn allowed_key_types(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_AllowedKeyTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_key_types", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_values` after provisioning.\n"]
    pub fn baseline_values(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_BaselineValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_values", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(&self) -> terrars::ListRef<PrivatecaCaPool_IssuancePolicy_IdentityConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_constraints", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_PublishingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding_format: std::option::Option<terrars::PrimField<std::string::String>>,
    publish_ca_cert: terrars::PrimField<bool>,
    publish_crl: terrars::PrimField<bool>,
}

impl PrivatecaCaPool_PublishingOptions {
    #[doc =
        "Set the field `encoding_format`.\nSpecifies the encoding format of each CertificateAuthority's CA\ncertificate and CRLs. If this is omitted, CA certificates and CRLs\nwill be published in PEM. Possible values: [\"PEM\", \"DER\"]"]
    pub fn set_encoding_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPool_PublishingOptions {
    type O = terrars::BlockAssignable<PrivatecaCaPool_PublishingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_PublishingOptions {
    #[doc =
        "When true, publishes each CertificateAuthority's CA certificate and includes its URL in the \"Authority Information Access\"\nX.509 extension in all issued Certificates. If this is false, the CA certificate will not be published and the corresponding\nX.509 extension will not be written in issued certificates."]
    pub publish_ca_cert: terrars::PrimField<bool>,
    #[doc =
        "When true, publishes each CertificateAuthority's CRL and includes its URL in the \"CRL Distribution Points\" X.509 extension\nin all issued Certificates. If this is false, CRLs will not be published and the corresponding X.509 extension will not\nbe written in issued certificates. CRLs will expire 7 days from their creation. However, we will rebuild daily. CRLs are\nalso rebuilt shortly after a certificate is revoked."]
    pub publish_crl: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPool_PublishingOptions {
    pub fn build(self) -> PrivatecaCaPool_PublishingOptions {
        PrivatecaCaPool_PublishingOptions {
            encoding_format: core::default::Default::default(),
            publish_ca_cert: self.publish_ca_cert,
            publish_crl: self.publish_crl,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_PublishingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_PublishingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCaPool_PublishingOptionsRef {
        PrivatecaCaPool_PublishingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_PublishingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encoding_format` after provisioning.\nSpecifies the encoding format of each CertificateAuthority's CA\ncertificate and CRLs. If this is omitted, CA certificates and CRLs\nwill be published in PEM. Possible values: [\"PEM\", \"DER\"]"]
    pub fn encoding_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `publish_ca_cert` after provisioning.\nWhen true, publishes each CertificateAuthority's CA certificate and includes its URL in the \"Authority Information Access\"\nX.509 extension in all issued Certificates. If this is false, the CA certificate will not be published and the corresponding\nX.509 extension will not be written in issued certificates."]
    pub fn publish_ca_cert(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_ca_cert", self.base))
    }

    #[doc =
        "Get a reference to the value of field `publish_crl` after provisioning.\nWhen true, publishes each CertificateAuthority's CRL and includes its URL in the \"CRL Distribution Points\" X.509 extension\nin all issued Certificates. If this is false, CRLs will not be published and the corresponding X.509 extension will not\nbe written in issued certificates. CRLs will expire 7 days from their creation. However, we will rebuild daily. CRLs are\nalso rebuilt shortly after a certificate is revoked."]
    pub fn publish_crl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_crl", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCaPool_Timeouts {
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

impl terrars::ToListMappable for PrivatecaCaPool_Timeouts {
    type O = terrars::BlockAssignable<PrivatecaCaPool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPool_Timeouts {}

impl BuildPrivatecaCaPool_Timeouts {
    pub fn build(self) -> PrivatecaCaPool_Timeouts {
        PrivatecaCaPool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCaPool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCaPool_TimeoutsRef {
        PrivatecaCaPool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPool_TimeoutsRef {
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
