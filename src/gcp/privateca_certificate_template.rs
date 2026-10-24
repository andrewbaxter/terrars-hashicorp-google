use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PrivatecaCertificateTemplateData {
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
    identity_constraints: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplateIdentityConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passthrough_extensions: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplatePassthroughExtensionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_values: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrivatecaCertificateTemplateTimeoutsEl>,
    dynamic: PrivatecaCertificateTemplateDynamic,
}

struct PrivatecaCertificateTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrivatecaCertificateTemplateData>,
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
        "Set the field `labels`.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_constraints`.\n"]
    pub fn set_identity_constraints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateTemplatePassthroughExtensionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<PrivatecaCertificateTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(&self) -> terrars::ListRef<PrivatecaCertificateTemplateIdentityConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_extensions` after provisioning.\n"]
    pub fn passthrough_extensions(&self) -> terrars::ListRef<PrivatecaCertificateTemplatePassthroughExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.passthrough_extensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predefined_values` after provisioning.\n"]
    pub fn predefined_values(&self) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTemplateTimeoutsElRef {
        PrivatecaCertificateTemplateTimeoutsElRef::new(
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
        "The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificateTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> PrivatecaCertificateTemplate {
        let out = PrivatecaCertificateTemplate(std::rc::Rc::new(PrivatecaCertificateTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrivatecaCertificateTemplateData {
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(&self) -> terrars::ListRef<PrivatecaCertificateTemplateIdentityConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_extensions` after provisioning.\n"]
    pub fn passthrough_extensions(&self) -> terrars::ListRef<PrivatecaCertificateTemplatePassthroughExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.passthrough_extensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predefined_values` after provisioning.\n"]
    pub fn predefined_values(&self) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTemplateTimeoutsElRef {
        PrivatecaCertificateTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {}

impl BuildPrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
    pub fn build(self) -> PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
        PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
        PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
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
struct PrivatecaCertificateTemplateIdentityConstraintsElDynamic {
    cel_expression: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplateIdentityConstraintsEl {
    allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    allow_subject_passthrough: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cel_expression: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>>,
    dynamic: PrivatecaCertificateTemplateIdentityConstraintsElDynamic,
}

impl PrivatecaCertificateTemplateIdentityConstraintsEl {
    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplateIdentityConstraintsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplateIdentityConstraintsEl {
    #[doc =
        "Required. If this is true, the SubjectAltNames extension may be copied from a certificate request into the signed certificate. Otherwise, the requested SubjectAltNames will be discarded."]
    pub allow_subject_alt_names_passthrough: terrars::PrimField<bool>,
    #[doc =
        "Required. If this is true, the Subject field may be copied from a certificate request into the signed certificate. Otherwise, the requested Subject will be discarded."]
    pub allow_subject_passthrough: terrars::PrimField<bool>,
}

impl BuildPrivatecaCertificateTemplateIdentityConstraintsEl {
    pub fn build(self) -> PrivatecaCertificateTemplateIdentityConstraintsEl {
        PrivatecaCertificateTemplateIdentityConstraintsEl {
            allow_subject_alt_names_passthrough: self.allow_subject_alt_names_passthrough,
            allow_subject_passthrough: self.allow_subject_passthrough,
            cel_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplateIdentityConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplateIdentityConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplateIdentityConstraintsElRef {
        PrivatecaCertificateTemplateIdentityConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplateIdentityConstraintsElRef {
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
    pub fn cel_expression(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cel_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl { }

impl terrars::ToListMappable for PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
        PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
            object_id_path: self.object_id_path,
        }
    }
}

pub struct PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
        PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
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
struct PrivatecaCertificateTemplatePassthroughExtensionsElDynamic {
    additional_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePassthroughExtensionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    known_extensions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl>>,
    dynamic: PrivatecaCertificateTemplatePassthroughExtensionsElDynamic,
}

impl PrivatecaCertificateTemplatePassthroughExtensionsEl {
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
                        ::BlockAssignable<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl>,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplatePassthroughExtensionsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePassthroughExtensionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePassthroughExtensionsEl {}

impl BuildPrivatecaCertificateTemplatePassthroughExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePassthroughExtensionsEl {
        PrivatecaCertificateTemplatePassthroughExtensionsEl {
            known_extensions: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplatePassthroughExtensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePassthroughExtensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePassthroughExtensionsElRef {
        PrivatecaCertificateTemplatePassthroughExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePassthroughExtensionsElRef {
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
    ) -> terrars::ListRef<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl { }

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
    type O =
        terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
            object_id_path: self.object_id_path,
        }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
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
struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElDynamic {
    object_id: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    critical: std::option::Option<terrars::PrimField<bool>>,
    value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl>>,
    dynamic: PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElDynamic,
}

impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
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
                        ::BlockAssignable<
                            PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    #[doc = "Required. The value of this X.509 extension."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
            critical: core::default::Default::default(),
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
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
    ) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_issuer_path_length: std::option::Option<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    #[doc =
        "Set the field `is_ca`.\nOptional. Refers to the \"CA\" X.509 extension, which is a boolean value. When this value is missing, the extension will be omitted from the CA certificate."]
    pub fn set_is_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_issuer_path_length`.\nOptional. Refers to the path length restriction X.509 extension. For a CA certificate, this value describes the depth of subordinate CA certificates that are allowed. If this value is less than 0, the request will fail. If this value is missing, the max path length will be omitted from the CA certificate."]
    pub fn set_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
        PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
        PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_ca` after provisioning.\nOptional. Refers to the \"CA\" X.509 extension, which is a boolean value. When this value is missing, the extension will be omitted from the CA certificate."]
    pub fn is_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_issuer_path_length` after provisioning.\nOptional. Refers to the path length restriction X.509 extension. For a CA certificate, this value describes the depth of subordinate CA certificates that are allowed. If this value is less than 0, the request will fail. If this value is missing, the max path length will be omitted from the CA certificate."]
    pub fn max_issuer_path_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_issuer_path_length", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
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

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
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

pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
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
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
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

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
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
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl { }

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    type O =
        terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: self.object_id_path,
        }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
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
struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElDynamic {
    base_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>>,
    extended_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl>>,
    unknown_extended_key_usages: std
    ::option
    ::Option<
        terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl>,
    >,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_extended_key_usages: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl>>,
    dynamic: PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElDynamic,
}

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>,
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
                        ::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl>,
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
                            PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unknown_extended_key_usages", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl { }

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
        PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl { object_id_path: self.object_id_path }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
        PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
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
struct PrivatecaCertificateTemplatePredefinedValuesElDynamic {
    additional_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>>,
    ca_options: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>>,
    key_usage: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>>,
    policy_ids: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_ocsp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_options: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_usage: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_ids: std::option::Option<std::vec::Vec<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>>,
    dynamic: PrivatecaCertificateTemplatePredefinedValuesElDynamic,
}

impl PrivatecaCertificateTemplatePredefinedValuesEl {
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
                        terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>,
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
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>>,
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
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>>,
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

    #[doc = "Set the field `policy_ids`.\n"]
    pub fn set_policy_ids(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>>,
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

impl terrars::ToListMappable for PrivatecaCertificateTemplatePredefinedValuesEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplatePredefinedValuesEl {}

impl BuildPrivatecaCertificateTemplatePredefinedValuesEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesEl {
        PrivatecaCertificateTemplatePredefinedValuesEl {
            aia_ocsp_servers: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            ca_options: core::default::Default::default(),
            key_usage: core::default::Default::default(),
            policy_ids: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplatePredefinedValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplatePredefinedValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElRef {
        PrivatecaCertificateTemplatePredefinedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplatePredefinedValuesElRef {
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
    ) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(&self) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> terrars::ListRef<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateTemplateTimeoutsEl {
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

impl terrars::ToListMappable for PrivatecaCertificateTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTemplateTimeoutsEl {}

impl BuildPrivatecaCertificateTemplateTimeoutsEl {
    pub fn build(self) -> PrivatecaCertificateTemplateTimeoutsEl {
        PrivatecaCertificateTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTemplateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCertificateTemplateTimeoutsElRef {
        PrivatecaCertificateTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTemplateTimeoutsElRef {
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
struct PrivatecaCertificateTemplateDynamic {
    identity_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplateIdentityConstraintsEl>>,
    passthrough_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePassthroughExtensionsEl>>,
    predefined_values: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesEl>>,
}
