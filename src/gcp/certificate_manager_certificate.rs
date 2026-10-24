use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CertificateManagerCertificateData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed: std::option::Option<std::vec::Vec<CertificateManagerCertificateManagedEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed: std::option::Option<std::vec::Vec<CertificateManagerCertificateSelfManagedEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CertificateManagerCertificateTimeoutsEl>,
    dynamic: CertificateManagerCertificateDynamic,
}

struct CertificateManagerCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CertificateManagerCertificateData>,
}

#[derive(Clone)]
pub struct CertificateManagerCertificate(std::rc::Rc<CertificateManagerCertificate_>);

impl CertificateManagerCertificate {
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

    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
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
        "Set the field `labels`.\nSet of label tags associated with the Certificate resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scope`.\nThe scope of the certificate.\n\nDEFAULT: Certificates with default scope are served from core Google data centers.\nIf unsure, choose this option.\n\nEDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.\nSee https://cloud.google.com/vpc/docs/edge-locations.\n\nALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).\nSee https://cloud.google.com/compute/docs/regions-zones"]
    pub fn set_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed`.\n"]
    pub fn set_managed(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificateManagerCertificateManagedEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed`.\n"]
    pub fn set_self_managed(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificateManagerCertificateSelfManagedEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CertificateManagerCertificateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
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
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Certificate resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate. Certificate names must be unique\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope of the certificate.\n\nDEFAULT: Certificates with default scope are served from core Google data centers.\nIf unsure, choose this option.\n\nEDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.\nSee https://cloud.google.com/vpc/docs/edge-locations.\n\nALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).\nSee https://cloud.google.com/compute/docs/regions-zones"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed` after provisioning.\n"]
    pub fn managed(&self) -> terrars::ListRef<CertificateManagerCertificateManagedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed` after provisioning.\n"]
    pub fn self_managed(&self) -> terrars::ListRef<CertificateManagerCertificateSelfManagedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateTimeoutsElRef {
        CertificateManagerCertificateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CertificateManagerCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CertificateManagerCertificate { }

impl terrars::ToListMappable for CertificateManagerCertificate {
    type O = terrars::ListRef<CertificateManagerCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CertificateManagerCertificate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_certificate_manager_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCertificateManagerCertificate {
    pub tf_id: std::string::String,
    #[doc =
        "A user-defined name of the certificate. Certificate names must be unique\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCertificateManagerCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> CertificateManagerCertificate {
        let out = CertificateManagerCertificate(std::rc::Rc::new(CertificateManagerCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CertificateManagerCertificateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                scope: core::default::Default::default(),
                managed: core::default::Default::default(),
                self_managed: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CertificateManagerCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CertificateManagerCertificateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
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
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Certificate resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate. Certificate names must be unique\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope of the certificate.\n\nDEFAULT: Certificates with default scope are served from core Google data centers.\nIf unsure, choose this option.\n\nEDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.\nSee https://cloud.google.com/vpc/docs/edge-locations.\n\nALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).\nSee https://cloud.google.com/compute/docs/regions-zones"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed` after provisioning.\n"]
    pub fn managed(&self) -> terrars::ListRef<CertificateManagerCertificateManagedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed` after provisioning.\n"]
    pub fn self_managed(&self) -> terrars::ListRef<CertificateManagerCertificateSelfManagedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateTimeoutsElRef {
        CertificateManagerCertificateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
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

impl terrars::ToListMappable for CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
    type O = terrars::BlockAssignable<CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {}

impl BuildCertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
    pub fn build(self) -> CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
        CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
            details: core::default::Default::default(),
            domain: core::default::Default::default(),
            failure_reason: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
        CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
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
pub struct CertificateManagerCertificateManagedElProvisioningIssueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerCertificateManagedElProvisioningIssueEl {
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

impl terrars::ToListMappable for CertificateManagerCertificateManagedElProvisioningIssueEl {
    type O = terrars::BlockAssignable<CertificateManagerCertificateManagedElProvisioningIssueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateManagedElProvisioningIssueEl {}

impl BuildCertificateManagerCertificateManagedElProvisioningIssueEl {
    pub fn build(self) -> CertificateManagerCertificateManagedElProvisioningIssueEl {
        CertificateManagerCertificateManagedElProvisioningIssueEl {
            details: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerCertificateManagedElProvisioningIssueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateManagedElProvisioningIssueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerCertificateManagedElProvisioningIssueElRef {
        CertificateManagerCertificateManagedElProvisioningIssueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateManagedElProvisioningIssueElRef {
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
pub struct CertificateManagerCertificateManagedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_authorizations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuance_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerCertificateManagedEl {
    #[doc =
        "Set the field `dns_authorizations`.\nAuthorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn set_dns_authorizations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_authorizations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `domains`.\nThe domains for which a managed SSL certificate will be generated.\nWildcard domains are only supported with DNS challenge resolution"]
    pub fn set_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `issuance_config`.\nThe resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.\nIf this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.\nEither issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn set_issuance_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuance_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CertificateManagerCertificateManagedEl {
    type O = terrars::BlockAssignable<CertificateManagerCertificateManagedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateManagedEl {}

impl BuildCertificateManagerCertificateManagedEl {
    pub fn build(self) -> CertificateManagerCertificateManagedEl {
        CertificateManagerCertificateManagedEl {
            dns_authorizations: core::default::Default::default(),
            domains: core::default::Default::default(),
            issuance_config: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerCertificateManagedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateManagedElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificateManagerCertificateManagedElRef {
        CertificateManagerCertificateManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateManagedElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authorization_attempt_info` after provisioning.\nDetailed state of the latest authorization attempt for each domain\nspecified for this Managed Certificate."]
    pub fn authorization_attempt_info(
        &self,
    ) -> terrars::ListRef<CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_attempt_info", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_authorizations` after provisioning.\nAuthorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn dns_authorizations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_authorizations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nThe domains for which a managed SSL certificate will be generated.\nWildcard domains are only supported with DNS challenge resolution"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issuance_config` after provisioning.\nThe resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.\nIf this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.\nEither issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn issuance_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuance_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioning_issue` after provisioning.\nInformation about issues with provisioning this Managed Certificate."]
    pub fn provisioning_issue(&self) -> terrars::ListRef<CertificateManagerCertificateManagedElProvisioningIssueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioning_issue", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nA state of this Managed Certificate."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CertificateManagerCertificateSelfManagedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_pem: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_pem: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerCertificateSelfManagedEl {
    #[doc =
        "Set the field `certificate_pem`.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn set_certificate_pem(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_pem = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pem_certificate`.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pem_private_key`.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn set_pem_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_pem`.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn set_private_key_pem(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key_pem = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CertificateManagerCertificateSelfManagedEl {
    type O = terrars::BlockAssignable<CertificateManagerCertificateSelfManagedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateSelfManagedEl {}

impl BuildCertificateManagerCertificateSelfManagedEl {
    pub fn build(self) -> CertificateManagerCertificateSelfManagedEl {
        CertificateManagerCertificateSelfManagedEl {
            certificate_pem: core::default::Default::default(),
            pem_certificate: core::default::Default::default(),
            pem_private_key: core::default::Default::default(),
            private_key_pem: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerCertificateSelfManagedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateSelfManagedElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificateManagerCertificateSelfManagedElRef {
        CertificateManagerCertificateSelfManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateSelfManagedElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `certificate_pem` after provisioning.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn certificate_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_pem", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pem_private_key` after provisioning.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn pem_private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_pem` after provisioning.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn private_key_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_pem", self.base))
    }
}

#[derive(Serialize)]
pub struct CertificateManagerCertificateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerCertificateTimeoutsEl {
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

impl terrars::ToListMappable for CertificateManagerCertificateTimeoutsEl {
    type O = terrars::BlockAssignable<CertificateManagerCertificateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateTimeoutsEl {}

impl BuildCertificateManagerCertificateTimeoutsEl {
    pub fn build(self) -> CertificateManagerCertificateTimeoutsEl {
        CertificateManagerCertificateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerCertificateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificateManagerCertificateTimeoutsElRef {
        CertificateManagerCertificateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateTimeoutsElRef {
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
struct CertificateManagerCertificateDynamic {
    managed: std::option::Option<terrars::DynamicBlock<CertificateManagerCertificateManagedEl>>,
    self_managed: std::option::Option<terrars::DynamicBlock<CertificateManagerCertificateSelfManagedEl>>,
}
