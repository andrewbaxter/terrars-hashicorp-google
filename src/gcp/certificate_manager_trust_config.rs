use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CertificateManagerTrustConfigData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CertificateManagerTrustConfigTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_stores: std::option::Option<std::vec::Vec<CertificateManagerTrustConfigTrustStoresEl>>,
    dynamic: CertificateManagerTrustConfigDynamic,
}

struct CertificateManagerTrustConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CertificateManagerTrustConfigData>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CertificateManagerTrustConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_stores`.\n"]
    pub fn set_trust_stores(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificateManagerTrustConfigTrustStoresEl>>,
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerTrustConfigTimeoutsElRef {
        CertificateManagerTrustConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trust_stores` after provisioning.\n"]
    pub fn trust_stores(&self) -> terrars::ListRef<CertificateManagerTrustConfigTrustStoresElRef> {
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
            data: std::cell::RefCell::new(CertificateManagerTrustConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerTrustConfigTimeoutsElRef {
        CertificateManagerTrustConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trust_stores` after provisioning.\n"]
    pub fn trust_stores(&self) -> terrars::ListRef<CertificateManagerTrustConfigTrustStoresElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_stores", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CertificateManagerTrustConfigTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerTrustConfigTimeoutsEl {
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

impl terrars::ToListMappable for CertificateManagerTrustConfigTimeoutsEl {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfigTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfigTimeoutsEl {}

impl BuildCertificateManagerTrustConfigTimeoutsEl {
    pub fn build(self) -> CertificateManagerTrustConfigTimeoutsEl {
        CertificateManagerTrustConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerTrustConfigTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfigTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificateManagerTrustConfigTimeoutsElRef {
        CertificateManagerTrustConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfigTimeoutsElRef {
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
pub struct CertificateManagerTrustConfigTrustStoresElIntermediateCasEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerTrustConfigTrustStoresElIntermediateCasEl {
    #[doc =
        "Set the field `pem_certificate`.\nPEM intermediate certificate used for building up paths for validation.\nEach certificate provided in PEM format may occupy up to 5kB."]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CertificateManagerTrustConfigTrustStoresElIntermediateCasEl {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfigTrustStoresElIntermediateCasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfigTrustStoresElIntermediateCasEl {}

impl BuildCertificateManagerTrustConfigTrustStoresElIntermediateCasEl {
    pub fn build(self) -> CertificateManagerTrustConfigTrustStoresElIntermediateCasEl {
        CertificateManagerTrustConfigTrustStoresElIntermediateCasEl {
            pem_certificate: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerTrustConfigTrustStoresElIntermediateCasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfigTrustStoresElIntermediateCasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerTrustConfigTrustStoresElIntermediateCasElRef {
        CertificateManagerTrustConfigTrustStoresElIntermediateCasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfigTrustStoresElIntermediateCasElRef {
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
pub struct CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl {
    #[doc =
        "Set the field `pem_certificate`.\nPEM root certificate of the PKI used for validation.\nEach certificate provided in PEM format may occupy up to 5kB."]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfigTrustStoresElTrustAnchorsEl {}

impl BuildCertificateManagerTrustConfigTrustStoresElTrustAnchorsEl {
    pub fn build(self) -> CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl {
        CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl {
            pem_certificate: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerTrustConfigTrustStoresElTrustAnchorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfigTrustStoresElTrustAnchorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerTrustConfigTrustStoresElTrustAnchorsElRef {
        CertificateManagerTrustConfigTrustStoresElTrustAnchorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfigTrustStoresElTrustAnchorsElRef {
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
struct CertificateManagerTrustConfigTrustStoresElDynamic {
    intermediate_cas: std
    ::option
    ::Option<terrars::DynamicBlock<CertificateManagerTrustConfigTrustStoresElIntermediateCasEl>>,
    trust_anchors: std
    ::option
    ::Option<terrars::DynamicBlock<CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl>>,
}

#[derive(Serialize)]
pub struct CertificateManagerTrustConfigTrustStoresEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intermediate_cas: std::option::Option<std::vec::Vec<CertificateManagerTrustConfigTrustStoresElIntermediateCasEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_anchors: std::option::Option<std::vec::Vec<CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl>>,
    dynamic: CertificateManagerTrustConfigTrustStoresElDynamic,
}

impl CertificateManagerTrustConfigTrustStoresEl {
    #[doc = "Set the field `intermediate_cas`.\n"]
    pub fn set_intermediate_cas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CertificateManagerTrustConfigTrustStoresElIntermediateCasEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CertificateManagerTrustConfigTrustStoresElTrustAnchorsEl>>,
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

impl terrars::ToListMappable for CertificateManagerTrustConfigTrustStoresEl {
    type O = terrars::BlockAssignable<CertificateManagerTrustConfigTrustStoresEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerTrustConfigTrustStoresEl {}

impl BuildCertificateManagerTrustConfigTrustStoresEl {
    pub fn build(self) -> CertificateManagerTrustConfigTrustStoresEl {
        CertificateManagerTrustConfigTrustStoresEl {
            intermediate_cas: core::default::Default::default(),
            trust_anchors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerTrustConfigTrustStoresElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerTrustConfigTrustStoresElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificateManagerTrustConfigTrustStoresElRef {
        CertificateManagerTrustConfigTrustStoresElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerTrustConfigTrustStoresElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intermediate_cas` after provisioning.\n"]
    pub fn intermediate_cas(&self) -> terrars::ListRef<CertificateManagerTrustConfigTrustStoresElIntermediateCasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intermediate_cas", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(&self) -> terrars::ListRef<CertificateManagerTrustConfigTrustStoresElTrustAnchorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_anchors", self.base))
    }
}

#[derive(Serialize, Default)]
struct CertificateManagerTrustConfigDynamic {
    trust_stores: std::option::Option<terrars::DynamicBlock<CertificateManagerTrustConfigTrustStoresEl>>,
}
