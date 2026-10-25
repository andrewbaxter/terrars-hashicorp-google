#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PrivatecaCertificateTemplateData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_lifetime: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_constraints: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_IdentityConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passthrough_extensions: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_PassthroughExtensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_values: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrivatecaCertificateTemplate_Timeouts>,
    dynamic: PrivatecaCertificateTemplateDynamic,
}

struct PrivatecaCertificateTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrivatecaCertificateTemplateData_>,
}

#[derive(Clone)]
pub struct PrivatecaCertificateTemplate(std::rc::Rc<PrivatecaCertificateTemplate_>);

impl PrivatecaCertificateTemplate {
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

    #[doc =
        "Set the field `description`.\nOptional. A human-readable description of scenarios this template is intended for."]
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
        "Set the field `labels`.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maximum_lifetime`.\nOptional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn set_maximum_lifetime(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().maximum_lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_constraints`.\n"]
    pub fn set_identity_constraints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_IdentityConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `passthrough_extensions`.\n"]
    pub fn set_passthrough_extensions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_PassthroughExtensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().passthrough_extensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.passthrough_extensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_values`.\n"]
    pub fn set_predefined_values(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predefined_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predefined_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrivatecaCertificateTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. A human-readable description of scenarios this template is intended for."]
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maximum_lifetime` after provisioning.\nOptional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn maximum_lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateTemplate in the format 'projects/*/locations/*/certificateTemplates/*'."]
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
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_IdentityConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_extensions` after provisioning.\n"]
    pub fn passthrough_extensions(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PassthroughExtensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.passthrough_extensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predefined_values` after provisioning.\n"]
    pub fn predefined_values(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTemplate_TimeoutsRef {
        PrivatecaCertificateTemplate_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for PrivatecaCertificateTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrivatecaCertificateTemplate { }

impl terrars::ToListMappable for PrivatecaCertificateTemplate {
    type O = terrars::ListRef<PrivatecaCertificateTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrivatecaCertificateTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_privateca_certificate_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrivatecaCertificateTemplate {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name for this CertificateTemplate in the format 'projects/*/locations/*/certificateTemplates/*'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificateTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> PrivatecaCertificateTemplate {
        let out = PrivatecaCertificateTemplate(std::rc::Rc::new(PrivatecaCertificateTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrivatecaCertificateTemplateData_ {
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
                maximum_lifetime: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                identity_constraints: core::default::Default::default(),
                passthrough_extensions: core::default::Default::default(),
                predefined_values: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrivatecaCertificateTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrivatecaCertificateTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. A human-readable description of scenarios this template is intended for."]
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maximum_lifetime` after provisioning.\nOptional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn maximum_lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateTemplate in the format 'projects/*/locations/*/certificateTemplates/*'."]
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
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_IdentityConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_extensions` after provisioning.\n"]
    pub fn passthrough_extensions(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PassthroughExtensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.passthrough_extensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predefined_values` after provisioning.\n"]
    pub fn predefined_values(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTemplate_TimeoutsRef {
        PrivatecaCertificateTemplate_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplateDynamic {
    identity_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_IdentityConstraints>>,
    passthrough_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PassthroughExtensions>>,
    predefined_values: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_IdentityConstraints_CelExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateTemplate_IdentityConstraints_CelExpression {
    #[doc =
        "Set the field `description`.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expression`.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `title`.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateTemplate_IdentityConstraints_CelExpression {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_IdentityConstraints_CelExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_IdentityConstraints_CelExpression {}

impl BuildPrivatecaCertificateTemplate_IdentityConstraints_CelExpression {
    pub fn build(self) -> PrivatecaCertificateTemplate_IdentityConstraints_CelExpression {
        PrivatecaCertificateTemplate_IdentityConstraints_CelExpression {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_IdentityConstraints_CelExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_IdentityConstraints_CelExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_IdentityConstraints_CelExpressionRef {
        PrivatecaCertificateTemplate_IdentityConstraints_CelExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_IdentityConstraints_CelExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplate_IdentityConstraintsDynamic {
    cel_expression: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_IdentityConstraints_CelExpression>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_IdentityConstraints {
    allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    allow_subject_passthrough: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cel_expression: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_IdentityConstraints_CelExpression>>,
    dynamic: PrivatecaCertificateTemplate_IdentityConstraintsDynamic,
}

impl PrivatecaCertificateTemplate_IdentityConstraints {
    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_IdentityConstraints_CelExpression>>,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_IdentityConstraints {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_IdentityConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_IdentityConstraints {
    #[doc =
        "Required. If this is true, the SubjectAltNames extension may be copied from a certificate request into the signed certificate. Otherwise, the requested SubjectAltNames will be discarded."]
    pub allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    #[doc =
        "Required. If this is true, the Subject field may be copied from a certificate request into the signed certificate. Otherwise, the requested Subject will be discarded."]
    pub allow_subject_passthrough: terrars::PrimField<bool>,
}

impl BuildPrivatecaCertificateTemplate_IdentityConstraints {
    pub fn build(self) -> PrivatecaCertificateTemplate_IdentityConstraints {
        PrivatecaCertificateTemplate_IdentityConstraints {
            allow_subject_alt_names_passthrough: self.allow_subject_alt_names_passthrough,
            allow_subject_passthrough: self.allow_subject_passthrough,
            cel_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_IdentityConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_IdentityConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_IdentityConstraintsRef {
        PrivatecaCertificateTemplate_IdentityConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_IdentityConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_subject_alt_names_passthrough` after provisioning.\nRequired. If this is true, the SubjectAltNames extension may be copied from a certificate request into the signed certificate. Otherwise, the requested SubjectAltNames will be discarded."]
    pub fn allow_subject_alt_names_passthrough(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_subject_alt_names_passthrough", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_subject_passthrough` after provisioning.\nRequired. If this is true, the Subject field may be copied from a certificate request into the signed certificate. Otherwise, the requested Subject will be discarded."]
    pub fn allow_subject_passthrough(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_subject_passthrough", self.base))
    }

    #[doc = "Get a reference to the value of field `cel_expression` after provisioning.\n"]
    pub fn cel_expression(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_IdentityConstraints_CelExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cel_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions { }

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions {
    pub fn build(self) -> PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions {
        PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions {
            object_id_path: self.object_id_path,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensionsRef {
        PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplate_PassthroughExtensionsDynamic {
    additional_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PassthroughExtensions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    known_extensions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions>>,
    dynamic: PrivatecaCertificateTemplate_PassthroughExtensionsDynamic,
}

impl PrivatecaCertificateTemplate_PassthroughExtensions {
    #[doc =
        "Set the field `known_extensions`.\nOptional. A set of named X.509 extensions. Will be combined with additional_extensions to determine the full set of X.509 extensions."]
    pub fn set_known_extensions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.known_extensions = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensions>,
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
}

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PassthroughExtensions {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PassthroughExtensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PassthroughExtensions {}

impl BuildPrivatecaCertificateTemplate_PassthroughExtensions {
    pub fn build(self) -> PrivatecaCertificateTemplate_PassthroughExtensions {
        PrivatecaCertificateTemplate_PassthroughExtensions {
            known_extensions: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PassthroughExtensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PassthroughExtensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PassthroughExtensionsRef {
        PrivatecaCertificateTemplate_PassthroughExtensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PassthroughExtensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `known_extensions` after provisioning.\nOptional. A set of named X.509 extensions. Will be combined with additional_extensions to determine the full set of X.509 extensions."]
    pub fn known_extensions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.known_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplate_PassthroughExtensions_AdditionalExtensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId { }

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId {
        PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId {
            object_id_path: self.object_id_path,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectIdRef {
        PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsDynamic {
    object_id: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    critical: std::option::Option<terrars::PrimField<bool>>,
    value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId>>,
    dynamic: PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsDynamic,
}

impl PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions {
    #[doc =
        "Set the field `critical`.\nOptional. Indicates whether or not this extension is critical (i.e., if the client does not know how to handle this extension, the client should consider this to be an error)."]
    pub fn set_critical(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.critical = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectId>,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions {
    #[doc = "Required. The value of this X.509 extension."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions {
        PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions {
            critical: core::default::Default::default(),
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsRef {
        PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `critical` after provisioning.\nOptional. Indicates whether or not this extension is critical (i.e., if the client does not know how to handle this extension, the client should consider this to be an error)."]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nRequired. The value of this X.509 extension."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `object_id` after provisioning.\n"]
    pub fn object_id(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions_ObjectIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_CaOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_issuer_path_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    null_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_max_issuer_path_length: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCertificateTemplate_PredefinedValues_CaOptions {
    #[doc =
        "Set the field `is_ca`.\nOptional. Refers to the \"CA\" X.509 extension, which is a boolean value. When this value is true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn set_is_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_issuer_path_length`.\nOptional. Refers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn set_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `null_ca`.\nOptional. When true, the \"CA\" in Basic Constraints extension will be set to null and omitted from the CA certificate.\nIf both 'is_ca' and 'null_ca' are unset, the \"CA\" in Basic Constraints extension will be set to false.\nNote that the behavior when 'is_ca = false' for this resource is different from the behavior in the Certificate Authority, Certificate and CaPool resources."]
    pub fn set_null_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.null_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zero_max_issuer_path_length`.\nOptional. When true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn set_zero_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.zero_max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_CaOptions {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_CaOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_CaOptions {}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_CaOptions {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_CaOptions {
        PrivatecaCertificateTemplate_PredefinedValues_CaOptions {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
            null_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_CaOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_CaOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_CaOptionsRef {
        PrivatecaCertificateTemplate_PredefinedValues_CaOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_CaOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_ca` after provisioning.\nOptional. Refers to the \"CA\" X.509 extension, which is a boolean value. When this value is true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn is_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_issuer_path_length` after provisioning.\nOptional. Refers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn max_issuer_path_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_issuer_path_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `null_ca` after provisioning.\nOptional. When true, the \"CA\" in Basic Constraints extension will be set to null and omitted from the CA certificate.\nIf both 'is_ca' and 'null_ca' are unset, the \"CA\" in Basic Constraints extension will be set to false.\nNote that the behavior when 'is_ca = false' for this resource is different from the behavior in the Certificate Authority, Certificate and CaPool resources."]
    pub fn null_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.null_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zero_max_issuer_path_length` after provisioning.\nOptional. When true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn zero_max_issuer_path_length(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_max_issuer_path_length", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage {
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

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage {}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage {
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
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsageRef {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsageRef {
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
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage {
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

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage {}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage {
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
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsageRef {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsageRef {
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
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages { }

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages {
    type O =
        terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages {
            object_id_path: self.object_id_path,
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsagesRef {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsageDynamic {
    base_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage>>,
    extended_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage>>,
    unknown_extended_key_usages: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_extended_key_usages: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages>>,
    dynamic: PrivatecaCertificateTemplate_PredefinedValues_KeyUsageDynamic,
}

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsage {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsage>,
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
                        ::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsage>,
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
                            PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsages,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_KeyUsage {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage {}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_KeyUsage {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsage {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsage {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_KeyUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_KeyUsageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_KeyUsageRef {
        PrivatecaCertificateTemplate_PredefinedValues_KeyUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_KeyUsageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_BaseKeyUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_ExtendedKeyUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage_UnknownExtendedKeyUsagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unknown_extended_key_usages", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_NameConstraints {
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

impl PrivatecaCertificateTemplate_PredefinedValues_NameConstraints {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_NameConstraints {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_NameConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_NameConstraints {
    #[doc = "Indicates whether or not the name constraints are marked critical."]
    pub critical: terrars::PrimField<bool>,
}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_NameConstraints {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_NameConstraints {
        PrivatecaCertificateTemplate_PredefinedValues_NameConstraints {
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
pub struct PrivatecaCertificateTemplate_PredefinedValues_NameConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_NameConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_NameConstraintsRef {
        PrivatecaCertificateTemplate_PredefinedValues_NameConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_NameConstraintsRef {
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
pub struct PrivatecaCertificateTemplate_PredefinedValues_PolicyIds {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplate_PredefinedValues_PolicyIds { }

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues_PolicyIds {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_PolicyIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues_PolicyIds {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplate_PredefinedValues_PolicyIds {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues_PolicyIds {
        PrivatecaCertificateTemplate_PredefinedValues_PolicyIds { object_id_path: self.object_id_path }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_PredefinedValues_PolicyIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValues_PolicyIdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValues_PolicyIdsRef {
        PrivatecaCertificateTemplate_PredefinedValues_PolicyIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValues_PolicyIdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplate_PredefinedValuesDynamic {
    additional_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions>>,
    ca_options: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_CaOptions>>,
    key_usage: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage>>,
    name_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_NameConstraints>>,
    policy_ids: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateTemplate_PredefinedValues_PolicyIds>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_PredefinedValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_ocsp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_options: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_CaOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_usage: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_constraints: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_NameConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_ids: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplate_PredefinedValues_PolicyIds>>,
    dynamic: PrivatecaCertificateTemplate_PredefinedValuesDynamic,
}

impl PrivatecaCertificateTemplate_PredefinedValues {
    #[doc =
        "Set the field `aia_ocsp_servers`.\nOptional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the \"Authority Information Access\" extension in the certificate."]
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
                        terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensions>,
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
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_CaOptions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_KeyUsage>>,
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
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_NameConstraints>>,
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
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues_PolicyIds>>,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_PredefinedValues {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_PredefinedValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_PredefinedValues {}

impl BuildPrivatecaCertificateTemplate_PredefinedValues {
    pub fn build(self) -> PrivatecaCertificateTemplate_PredefinedValues {
        PrivatecaCertificateTemplate_PredefinedValues {
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
pub struct PrivatecaCertificateTemplate_PredefinedValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_PredefinedValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplate_PredefinedValuesRef {
        PrivatecaCertificateTemplate_PredefinedValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_PredefinedValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aia_ocsp_servers` after provisioning.\nOptional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the \"Authority Information Access\" extension in the certificate."]
    pub fn aia_ocsp_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aia_ocsp_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_AdditionalExtensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_CaOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_KeyUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_NameConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> terrars::ListRef<PrivatecaCertificateTemplate_PredefinedValues_PolicyIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateTemplate_Timeouts {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplate_Timeouts {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplate_Timeouts {}

impl BuildPrivatecaCertificateTemplate_Timeouts {
    pub fn build(self) -> PrivatecaCertificateTemplate_Timeouts {
        PrivatecaCertificateTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivatecaCertificateTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCertificateTemplate_TimeoutsRef {
        PrivatecaCertificateTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplate_TimeoutsRef {
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
