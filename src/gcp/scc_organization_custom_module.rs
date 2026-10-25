#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SccOrganizationCustomModuleData_ {
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
    display_name: terrars::PrimField<std::string::String>,
    enablement_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    organization: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_config: std::option::Option<std::vec::Vec<SccOrganizationCustomModule_CustomConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SccOrganizationCustomModule_Timeouts>,
    dynamic: SccOrganizationCustomModuleDynamic,
}

struct SccOrganizationCustomModule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SccOrganizationCustomModuleData_>,
}

#[derive(Clone)]
pub struct SccOrganizationCustomModule(std::rc::Rc<SccOrganizationCustomModule_>);

impl SccOrganizationCustomModule {
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

    #[doc = "Set the field `custom_config`.\n"]
    pub fn set_custom_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SccOrganizationCustomModule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ancestor_module` after provisioning.\nIf empty, indicates that the custom module was created in the organization, folder,\nor project in which you are viewing the custom module. Otherwise, ancestor_module\nspecifies the organization or folder from which the custom module is inherited."]
    pub fn ancestor_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ancestor_module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enablement_state` after provisioning.\nThe enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn enablement_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_editor` after provisioning.\nThe editor that last updated the custom module."]
    pub fn last_editor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_editor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"organizations/{org_id}/securityHealthAnalyticsSettings/customModules/{customModule}\".\nThe id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nNumerical ID of the parent organization."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the custom module was last updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_config` after provisioning.\n"]
    pub fn custom_config(&self) -> terrars::ListRef<SccOrganizationCustomModule_CustomConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccOrganizationCustomModule_TimeoutsRef {
        SccOrganizationCustomModule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SccOrganizationCustomModule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SccOrganizationCustomModule { }

impl terrars::ToListMappable for SccOrganizationCustomModule {
    type O = terrars::ListRef<SccOrganizationCustomModuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SccOrganizationCustomModule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_scc_organization_custom_module".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSccOrganizationCustomModule {
    pub tf_id: std::string::String,
    #[doc =
        "The display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub enablement_state: terrars::PrimField<std::string::String>,
    #[doc = "Numerical ID of the parent organization."]
    pub organization: terrars::PrimField<std::string::String>,
}

impl BuildSccOrganizationCustomModule {
    pub fn build(self, stack: &mut terrars::Stack) -> SccOrganizationCustomModule {
        let out = SccOrganizationCustomModule(std::rc::Rc::new(SccOrganizationCustomModule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SccOrganizationCustomModuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enablement_state: self.enablement_state,
                id: core::default::Default::default(),
                organization: self.organization,
                custom_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SccOrganizationCustomModuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SccOrganizationCustomModuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ancestor_module` after provisioning.\nIf empty, indicates that the custom module was created in the organization, folder,\nor project in which you are viewing the custom module. Otherwise, ancestor_module\nspecifies the organization or folder from which the custom module is inherited."]
    pub fn ancestor_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ancestor_module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enablement_state` after provisioning.\nThe enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn enablement_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_editor` after provisioning.\nThe editor that last updated the custom module."]
    pub fn last_editor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_editor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"organizations/{org_id}/securityHealthAnalyticsSettings/customModules/{customModule}\".\nThe id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nNumerical ID of the parent organization."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the custom module was last updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_config` after provisioning.\n"]
    pub fn custom_config(&self) -> terrars::ListRef<SccOrganizationCustomModule_CustomConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccOrganizationCustomModule_TimeoutsRef {
        SccOrganizationCustomModule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SccOrganizationCustomModuleDynamic {
    custom_config: std::option::Option<terrars::DynamicBlock<SccOrganizationCustomModule_CustomConfig>>,
}

#[derive(Serialize)]
pub struct SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression {
    #[doc =
        "Set the field `description`.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression {
    type O =
        terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression {
    pub fn build(self) -> SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression {
        SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpressionRef {
        SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesDynamic {
    value_expression: std
    ::option
    ::Option<
        terrars::DynamicBlock<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression>,
    >,
}

#[derive(Serialize)]
pub struct SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_expression: std
    ::option
    ::Option<std::vec::Vec<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression>>,
    dynamic: SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesDynamic,
}

impl SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties {
    #[doc = "Set the field `name`.\nName of the property for the custom output."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_expression`.\n"]
    pub fn set_value_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpression,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties {
    type O = terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccOrganizationCustomModule_CustomConfig_CustomOutput_Properties {}

impl BuildSccOrganizationCustomModule_CustomConfig_CustomOutput_Properties {
    pub fn build(self) -> SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties {
        SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties {
            name: core::default::Default::default(),
            value_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesRef {
        SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the property for the custom output."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_expression` after provisioning.\n"]
    pub fn value_expression(
        &self,
    ) -> terrars::ListRef<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties_ValueExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct SccOrganizationCustomModule_CustomConfig_CustomOutputDynamic {
    properties: std
    ::option
    ::Option<terrars::DynamicBlock<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties>>,
}

#[derive(Serialize)]
pub struct SccOrganizationCustomModule_CustomConfig_CustomOutput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties>>,
    dynamic: SccOrganizationCustomModule_CustomConfig_CustomOutputDynamic,
}

impl SccOrganizationCustomModule_CustomConfig_CustomOutput {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_CustomOutput_Properties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SccOrganizationCustomModule_CustomConfig_CustomOutput {
    type O = terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_CustomOutput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccOrganizationCustomModule_CustomConfig_CustomOutput {}

impl BuildSccOrganizationCustomModule_CustomConfig_CustomOutput {
    pub fn build(self) -> SccOrganizationCustomModule_CustomConfig_CustomOutput {
        SccOrganizationCustomModule_CustomConfig_CustomOutput {
            properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SccOrganizationCustomModule_CustomConfig_CustomOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModule_CustomConfig_CustomOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccOrganizationCustomModule_CustomConfig_CustomOutputRef {
        SccOrganizationCustomModule_CustomConfig_CustomOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccOrganizationCustomModule_CustomConfig_CustomOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<SccOrganizationCustomModule_CustomConfig_CustomOutput_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct SccOrganizationCustomModule_CustomConfig_Predicate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SccOrganizationCustomModule_CustomConfig_Predicate {
    #[doc =
        "Set the field `description`.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SccOrganizationCustomModule_CustomConfig_Predicate {
    type O = terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_Predicate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccOrganizationCustomModule_CustomConfig_Predicate {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSccOrganizationCustomModule_CustomConfig_Predicate {
    pub fn build(self) -> SccOrganizationCustomModule_CustomConfig_Predicate {
        SccOrganizationCustomModule_CustomConfig_Predicate {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SccOrganizationCustomModule_CustomConfig_PredicateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModule_CustomConfig_PredicateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccOrganizationCustomModule_CustomConfig_PredicateRef {
        SccOrganizationCustomModule_CustomConfig_PredicateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccOrganizationCustomModule_CustomConfig_PredicateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct SccOrganizationCustomModule_CustomConfig_ResourceSelector {
    resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SccOrganizationCustomModule_CustomConfig_ResourceSelector { }

impl terrars::ToListMappable for SccOrganizationCustomModule_CustomConfig_ResourceSelector {
    type O = terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_ResourceSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccOrganizationCustomModule_CustomConfig_ResourceSelector {
    #[doc = "The resource types to run the detector on."]
    pub resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSccOrganizationCustomModule_CustomConfig_ResourceSelector {
    pub fn build(self) -> SccOrganizationCustomModule_CustomConfig_ResourceSelector {
        SccOrganizationCustomModule_CustomConfig_ResourceSelector { resource_types: self.resource_types }
    }
}

#[allow(dead_code)]
pub struct SccOrganizationCustomModule_CustomConfig_ResourceSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModule_CustomConfig_ResourceSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccOrganizationCustomModule_CustomConfig_ResourceSelectorRef {
        SccOrganizationCustomModule_CustomConfig_ResourceSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccOrganizationCustomModule_CustomConfig_ResourceSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_types` after provisioning.\nThe resource types to run the detector on."]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct SccOrganizationCustomModule_CustomConfigDynamic {
    custom_output: std::option::Option<terrars::DynamicBlock<SccOrganizationCustomModule_CustomConfig_CustomOutput>>,
    predicate: std::option::Option<terrars::DynamicBlock<SccOrganizationCustomModule_CustomConfig_Predicate>>,
    resource_selector: std
    ::option
    ::Option<terrars::DynamicBlock<SccOrganizationCustomModule_CustomConfig_ResourceSelector>>,
}

#[derive(Serialize)]
pub struct SccOrganizationCustomModule_CustomConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    recommendation: terrars::PrimField<std::string::String>,
    severity: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_output: std::option::Option<std::vec::Vec<SccOrganizationCustomModule_CustomConfig_CustomOutput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predicate: std::option::Option<std::vec::Vec<SccOrganizationCustomModule_CustomConfig_Predicate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_selector: std::option::Option<std::vec::Vec<SccOrganizationCustomModule_CustomConfig_ResourceSelector>>,
    dynamic: SccOrganizationCustomModule_CustomConfigDynamic,
}

impl SccOrganizationCustomModule_CustomConfig {
    #[doc =
        "Set the field `description`.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects. This explanation is returned with each finding instance to\nhelp investigators understand the detected issue. The text must be enclosed in quotation marks."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_output`.\n"]
    pub fn set_custom_output(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_CustomOutput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predicate`.\n"]
    pub fn set_predicate(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_Predicate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predicate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predicate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_selector`.\n"]
    pub fn set_resource_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig_ResourceSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SccOrganizationCustomModule_CustomConfig {
    type O = terrars::BlockAssignable<SccOrganizationCustomModule_CustomConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccOrganizationCustomModule_CustomConfig {
    #[doc =
        "An explanation of the recommended steps that security teams can take to resolve\nthe detected issue. This explanation is returned with each finding generated by\nthis module in the nextSteps property of the finding JSON."]
    pub recommendation: terrars::PrimField<std::string::String>,
    #[doc =
        "The severity to assign to findings generated by the module. Possible values: [\"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub severity: terrars::PrimField<std::string::String>,
}

impl BuildSccOrganizationCustomModule_CustomConfig {
    pub fn build(self) -> SccOrganizationCustomModule_CustomConfig {
        SccOrganizationCustomModule_CustomConfig {
            description: core::default::Default::default(),
            recommendation: self.recommendation,
            severity: self.severity,
            custom_output: core::default::Default::default(),
            predicate: core::default::Default::default(),
            resource_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SccOrganizationCustomModule_CustomConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModule_CustomConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SccOrganizationCustomModule_CustomConfigRef {
        SccOrganizationCustomModule_CustomConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccOrganizationCustomModule_CustomConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects. This explanation is returned with each finding instance to\nhelp investigators understand the detected issue. The text must be enclosed in quotation marks."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recommendation` after provisioning.\nAn explanation of the recommended steps that security teams can take to resolve\nthe detected issue. This explanation is returned with each finding generated by\nthis module in the nextSteps property of the finding JSON."]
    pub fn recommendation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recommendation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nThe severity to assign to findings generated by the module. Possible values: [\"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_output` after provisioning.\n"]
    pub fn custom_output(&self) -> terrars::ListRef<SccOrganizationCustomModule_CustomConfig_CustomOutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_output", self.base))
    }

    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(&self) -> terrars::ListRef<SccOrganizationCustomModule_CustomConfig_PredicateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predicate", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_selector` after provisioning.\n"]
    pub fn resource_selector(&self) -> terrars::ListRef<SccOrganizationCustomModule_CustomConfig_ResourceSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct SccOrganizationCustomModule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SccOrganizationCustomModule_Timeouts {
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

impl terrars::ToListMappable for SccOrganizationCustomModule_Timeouts {
    type O = terrars::BlockAssignable<SccOrganizationCustomModule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccOrganizationCustomModule_Timeouts {}

impl BuildSccOrganizationCustomModule_Timeouts {
    pub fn build(self) -> SccOrganizationCustomModule_Timeouts {
        SccOrganizationCustomModule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SccOrganizationCustomModule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccOrganizationCustomModule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SccOrganizationCustomModule_TimeoutsRef {
        SccOrganizationCustomModule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccOrganizationCustomModule_TimeoutsRef {
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
