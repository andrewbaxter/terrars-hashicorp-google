use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PrivatecaCaPoolData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuance_policy: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publishing_options: std::option::Option<std::vec::Vec<PrivatecaCaPoolPublishingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrivatecaCaPoolTimeoutsEl>,
    dynamic: PrivatecaCaPoolDynamic,
}

struct PrivatecaCaPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrivatecaCaPoolData>,
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

    #[doc = "Set the field `issuance_policy`.\n"]
    pub fn set_issuance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPoolPublishingOptionsEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<PrivatecaCaPoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `issuance_policy` after provisioning.\n"]
    pub fn issuance_policy(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issuance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publishing_options` after provisioning.\n"]
    pub fn publishing_options(&self) -> terrars::ListRef<PrivatecaCaPoolPublishingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCaPoolTimeoutsElRef {
        PrivatecaCaPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(PrivatecaCaPoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                tier: self.tier,
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

    #[doc = "Get a reference to the value of field `issuance_policy` after provisioning.\n"]
    pub fn issuance_policy(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issuance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publishing_options` after provisioning.\n"]
    pub fn publishing_options(&self) -> terrars::ListRef<PrivatecaCaPoolPublishingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCaPoolTimeoutsElRef {
        PrivatecaCaPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    allow_config_based_issuance: terrars::PrimField<bool>,
    allow_csr_based_issuance: terrars::PrimField<bool>,
}

impl PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl { }

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    #[doc = "When true, allows callers to create Certificates by specifying a CertificateConfig."]
    pub allow_config_based_issuance: terrars::PrimField<bool>,
    #[doc = "When true, allows callers to create Certificates by specifying a CSR."]
    pub allow_csr_based_issuance: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
        PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl {
            allow_config_based_issuance: self.allow_config_based_issuance,
            allow_csr_based_issuance: self.allow_csr_based_issuance,
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    signature_algorithm: terrars::PrimField<std::string::String>,
}

impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl { }

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    #[doc = "The algorithm used. Possible values: [\"ECDSA_P256\", \"ECDSA_P384\", \"EDDSA_25519\"]"]
    pub signature_algorithm: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl {
            signature_algorithm: self.signature_algorithm,
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_modulus_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_modulus_size: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {}

impl BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl {
            max_modulus_size: core::default::Default::default(),
            min_modulus_size: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef {
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
struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElDynamic {
    elliptic_curve: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>>,
    rsa: std::option::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elliptic_curve: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElDynamic,
}

impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    #[doc = "Set the field `elliptic_curve`.\n"]
    pub fn set_elliptic_curve(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaEl>>,
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {}

impl BuildPrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl {
            elliptic_curve: core::default::Default::default(),
            rsa: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
        PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `elliptic_curve` after provisioning.\n"]
    pub fn elliptic_curve(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElEllipticCurveElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elliptic_curve", self.base))
    }

    #[doc = "Get a reference to the value of field `rsa` after provisioning.\n"]
    pub fn rsa(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRsaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rsa", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl { }

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
    type O =
        terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl {
            object_id_path: self.object_id_path,
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef {
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
struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElDynamic {
    object_id: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    critical: terrars::PrimField<bool>,
    value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElDynamic,
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdEl,
                        >,
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    #[doc =
        "Indicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub critical: terrars::PrimField<bool>,
    #[doc = "The value of this X.509 extension. A base64-encoded string."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl {
            critical: self.critical,
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef {
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
    ) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElObjectIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_issuer_path_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_max_issuer_path_length: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
            non_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
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

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl {
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

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
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

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl { }

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    type O =
        terrars
        ::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: self.object_id_path,
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
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
struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElDynamic {
    base_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl>>,
    extended_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl>>,
    unknown_extended_key_usages: std
    ::option
    ::Option<
        terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl>,
    >,
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_extended_key_usages: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElDynamic,
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageEl>,
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
                        ::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageEl>,
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
                            PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElBaseKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElExtendedKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElUnknownExtendedKeyUsagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unknown_extended_key_usages", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
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

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
    #[doc = "Indicates whether or not the name constraints are marked critical."]
    pub critical: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl {
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

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef {
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
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl { }

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl { object_id_path: self.object_id_path }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef {
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
struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElDynamic {
    additional_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>>,
    ca_options: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>>,
    key_usage: std::option::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>>,
    name_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>>,
    policy_ids: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_ocsp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_options: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_usage: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_constraints: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_ids: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElBaselineValuesElDynamic,
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
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
                        terrars
                        ::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsEl>,
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
                    ::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsEl>,
                    >,
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
                    ::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsEl>>,
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesEl {}

impl BuildPrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesEl {
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

pub struct PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
        PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef {
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
    ) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElAdditionalExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElCaOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(
        &self,
    ) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElNameConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElPolicyIdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef {
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
struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElDynamic {
    cel_expression: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    allow_subject_passthrough: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cel_expression: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElDynamic,
}

impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionEl>,
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    #[doc =
        "If this is set, the SubjectAltNames extension may be copied from a certificate request into the signed certificate.\nOtherwise, the requested SubjectAltNames will be discarded."]
    pub allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    #[doc =
        "If this is set, the Subject field may be copied from a certificate request into the signed certificate.\nOtherwise, the requested Subject will be discarded."]
    pub allow_subject_passthrough: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl {
            allow_subject_alt_names_passthrough: self.allow_subject_alt_names_passthrough,
            allow_subject_passthrough: self.allow_subject_passthrough,
            cel_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
        PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef {
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
    ) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElCelExpressionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cel_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCaPoolIssuancePolicyElDynamic {
    allowed_issuance_modes: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>>,
    allowed_key_types: std::option::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>>,
    baseline_values: std::option::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>>,
    identity_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolIssuancePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_lifetime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_issuance_modes: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_key_types: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_values: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_constraints: std::option::Option<std::vec::Vec<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>>,
    dynamic: PrivatecaCaPoolIssuancePolicyElDynamic,
}

impl PrivatecaCaPoolIssuancePolicyEl {
    #[doc =
        "Set the field `maximum_lifetime`.\nThe maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority\nexpires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn set_maximum_lifetime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.maximum_lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_issuance_modes`.\n"]
    pub fn set_allowed_issuance_modes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElBaselineValuesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsEl>>,
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

impl terrars::ToListMappable for PrivatecaCaPoolIssuancePolicyEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolIssuancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolIssuancePolicyEl {}

impl BuildPrivatecaCaPoolIssuancePolicyEl {
    pub fn build(self) -> PrivatecaCaPoolIssuancePolicyEl {
        PrivatecaCaPoolIssuancePolicyEl {
            maximum_lifetime: core::default::Default::default(),
            allowed_issuance_modes: core::default::Default::default(),
            allowed_key_types: core::default::Default::default(),
            baseline_values: core::default::Default::default(),
            identity_constraints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolIssuancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolIssuancePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCaPoolIssuancePolicyElRef {
        PrivatecaCaPoolIssuancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolIssuancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `maximum_lifetime` after provisioning.\nThe maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority\nexpires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn maximum_lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_lifetime", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_issuance_modes` after provisioning.\n"]
    pub fn allowed_issuance_modes(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElAllowedIssuanceModesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_issuance_modes", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_key_types` after provisioning.\n"]
    pub fn allowed_key_types(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElAllowedKeyTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_key_types", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_values` after provisioning.\n"]
    pub fn baseline_values(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElBaselineValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_values", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(&self) -> terrars::ListRef<PrivatecaCaPoolIssuancePolicyElIdentityConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_constraints", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCaPoolPublishingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding_format: std::option::Option<terrars::PrimField<std::string::String>>,
    publish_ca_cert: terrars::PrimField<bool>,
    publish_crl: terrars::PrimField<bool>,
}

impl PrivatecaCaPoolPublishingOptionsEl {
    #[doc =
        "Set the field `encoding_format`.\nSpecifies the encoding format of each CertificateAuthority's CA\ncertificate and CRLs. If this is omitted, CA certificates and CRLs\nwill be published in PEM. Possible values: [\"PEM\", \"DER\"]"]
    pub fn set_encoding_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCaPoolPublishingOptionsEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolPublishingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolPublishingOptionsEl {
    #[doc =
        "When true, publishes each CertificateAuthority's CA certificate and includes its URL in the \"Authority Information Access\"\nX.509 extension in all issued Certificates. If this is false, the CA certificate will not be published and the corresponding\nX.509 extension will not be written in issued certificates."]
    pub publish_ca_cert: terrars::PrimField<bool>,
    #[doc =
        "When true, publishes each CertificateAuthority's CRL and includes its URL in the \"CRL Distribution Points\" X.509 extension\nin all issued Certificates. If this is false, CRLs will not be published and the corresponding X.509 extension will not\nbe written in issued certificates. CRLs will expire 7 days from their creation. However, we will rebuild daily. CRLs are\nalso rebuilt shortly after a certificate is revoked."]
    pub publish_crl: terrars::PrimField<bool>,
}

impl BuildPrivatecaCaPoolPublishingOptionsEl {
    pub fn build(self) -> PrivatecaCaPoolPublishingOptionsEl {
        PrivatecaCaPoolPublishingOptionsEl {
            encoding_format: core::default::Default::default(),
            publish_ca_cert: self.publish_ca_cert,
            publish_crl: self.publish_crl,
        }
    }
}

pub struct PrivatecaCaPoolPublishingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolPublishingOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCaPoolPublishingOptionsElRef {
        PrivatecaCaPoolPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolPublishingOptionsElRef {
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
pub struct PrivatecaCaPoolTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCaPoolTimeoutsEl {
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

impl terrars::ToListMappable for PrivatecaCaPoolTimeoutsEl {
    type O = terrars::BlockAssignable<PrivatecaCaPoolTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCaPoolTimeoutsEl {}

impl BuildPrivatecaCaPoolTimeoutsEl {
    pub fn build(self) -> PrivatecaCaPoolTimeoutsEl {
        PrivatecaCaPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCaPoolTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCaPoolTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCaPoolTimeoutsElRef {
        PrivatecaCaPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCaPoolTimeoutsElRef {
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
struct PrivatecaCaPoolDynamic {
    issuance_policy: std::option::Option<terrars::DynamicBlock<PrivatecaCaPoolIssuancePolicyEl>>,
    publishing_options: std::option::Option<terrars::DynamicBlock<PrivatecaCaPoolPublishingOptionsEl>>,
}
