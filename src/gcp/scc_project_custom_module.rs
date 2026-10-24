use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SccProjectCustomModuleData {
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
    display_name: terrars::PrimField<std::string::String>,
    enablement_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_config: std::option::Option<std::vec::Vec<SccProjectCustomModuleCustomConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SccProjectCustomModuleTimeoutsEl>,
    dynamic: SccProjectCustomModuleDynamic,
}

struct SccProjectCustomModule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SccProjectCustomModuleData>,
}

#[derive(Clone)]
pub struct SccProjectCustomModule(std::rc::Rc<SccProjectCustomModule_>);

impl SccProjectCustomModule {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_config`.\n"]
    pub fn set_custom_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SccProjectCustomModuleCustomConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<SccProjectCustomModuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ancestor_module` after provisioning.\nIf empty, indicates that the custom module was created in the organization,folder,\nor project in which you are viewing the custom module. Otherwise, ancestor_module\nspecifies the organization or folder from which the custom module is inherited."]
    pub fn ancestor_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ancestor_module", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}\".\nThe id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the custom module was last updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_config` after provisioning.\n"]
    pub fn custom_config(&self) -> terrars::ListRef<SccProjectCustomModuleCustomConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccProjectCustomModuleTimeoutsElRef {
        SccProjectCustomModuleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SccProjectCustomModule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SccProjectCustomModule { }

impl terrars::ToListMappable for SccProjectCustomModule {
    type O = terrars::ListRef<SccProjectCustomModuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SccProjectCustomModule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_scc_project_custom_module".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSccProjectCustomModule {
    pub tf_id: std::string::String,
    #[doc =
        "The display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub enablement_state: terrars::PrimField<std::string::String>,
}

impl BuildSccProjectCustomModule {
    pub fn build(self, stack: &mut terrars::Stack) -> SccProjectCustomModule {
        let out = SccProjectCustomModule(std::rc::Rc::new(SccProjectCustomModule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SccProjectCustomModuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: self.display_name,
                enablement_state: self.enablement_state,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                custom_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SccProjectCustomModuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SccProjectCustomModuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ancestor_module` after provisioning.\nIf empty, indicates that the custom module was created in the organization,folder,\nor project in which you are viewing the custom module. Otherwise, ancestor_module\nspecifies the organization or folder from which the custom module is inherited."]
    pub fn ancestor_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ancestor_module", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}\".\nThe id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the custom module was last updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_config` after provisioning.\n"]
    pub fn custom_config(&self) -> terrars::ListRef<SccProjectCustomModuleCustomConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccProjectCustomModuleTimeoutsElRef {
        SccProjectCustomModuleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
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

impl terrars::ToListMappable for SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
    type O =
        terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
    pub fn build(self) -> SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
        SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
        SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
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
struct SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElDynamic {
    value_expression: std
    ::option
    ::Option<terrars::DynamicBlock<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl>>,
}

#[derive(Serialize)]
pub struct SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_expression: std
    ::option
    ::Option<std::vec::Vec<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl>>,
    dynamic: SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElDynamic,
}

impl SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl {
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
                            SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl,
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

impl terrars::ToListMappable for SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl {
    type O = terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl {}

impl BuildSccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl {
    pub fn build(self) -> SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl {
        SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl {
            name: core::default::Default::default(),
            value_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
        SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
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
    ) -> terrars::ListRef<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct SccProjectCustomModuleCustomConfigElCustomOutputElDynamic {
    properties: std
    ::option
    ::Option<terrars::DynamicBlock<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl>>,
}

#[derive(Serialize)]
pub struct SccProjectCustomModuleCustomConfigElCustomOutputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl>>,
    dynamic: SccProjectCustomModuleCustomConfigElCustomOutputElDynamic,
}

impl SccProjectCustomModuleCustomConfigElCustomOutputEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesEl>>,
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

impl terrars::ToListMappable for SccProjectCustomModuleCustomConfigElCustomOutputEl {
    type O = terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElCustomOutputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccProjectCustomModuleCustomConfigElCustomOutputEl {}

impl BuildSccProjectCustomModuleCustomConfigElCustomOutputEl {
    pub fn build(self) -> SccProjectCustomModuleCustomConfigElCustomOutputEl {
        SccProjectCustomModuleCustomConfigElCustomOutputEl {
            properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SccProjectCustomModuleCustomConfigElCustomOutputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleCustomConfigElCustomOutputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccProjectCustomModuleCustomConfigElCustomOutputElRef {
        SccProjectCustomModuleCustomConfigElCustomOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccProjectCustomModuleCustomConfigElCustomOutputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<SccProjectCustomModuleCustomConfigElCustomOutputElPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct SccProjectCustomModuleCustomConfigElPredicateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SccProjectCustomModuleCustomConfigElPredicateEl {
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

impl terrars::ToListMappable for SccProjectCustomModuleCustomConfigElPredicateEl {
    type O = terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElPredicateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccProjectCustomModuleCustomConfigElPredicateEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSccProjectCustomModuleCustomConfigElPredicateEl {
    pub fn build(self) -> SccProjectCustomModuleCustomConfigElPredicateEl {
        SccProjectCustomModuleCustomConfigElPredicateEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct SccProjectCustomModuleCustomConfigElPredicateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleCustomConfigElPredicateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccProjectCustomModuleCustomConfigElPredicateElRef {
        SccProjectCustomModuleCustomConfigElPredicateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccProjectCustomModuleCustomConfigElPredicateElRef {
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
pub struct SccProjectCustomModuleCustomConfigElResourceSelectorEl {
    resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SccProjectCustomModuleCustomConfigElResourceSelectorEl { }

impl terrars::ToListMappable for SccProjectCustomModuleCustomConfigElResourceSelectorEl {
    type O = terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElResourceSelectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccProjectCustomModuleCustomConfigElResourceSelectorEl {
    #[doc = "The resource types to run the detector on."]
    pub resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSccProjectCustomModuleCustomConfigElResourceSelectorEl {
    pub fn build(self) -> SccProjectCustomModuleCustomConfigElResourceSelectorEl {
        SccProjectCustomModuleCustomConfigElResourceSelectorEl { resource_types: self.resource_types }
    }
}

pub struct SccProjectCustomModuleCustomConfigElResourceSelectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleCustomConfigElResourceSelectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SccProjectCustomModuleCustomConfigElResourceSelectorElRef {
        SccProjectCustomModuleCustomConfigElResourceSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccProjectCustomModuleCustomConfigElResourceSelectorElRef {
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
struct SccProjectCustomModuleCustomConfigElDynamic {
    custom_output: std::option::Option<terrars::DynamicBlock<SccProjectCustomModuleCustomConfigElCustomOutputEl>>,
    predicate: std::option::Option<terrars::DynamicBlock<SccProjectCustomModuleCustomConfigElPredicateEl>>,
    resource_selector: std
    ::option
    ::Option<terrars::DynamicBlock<SccProjectCustomModuleCustomConfigElResourceSelectorEl>>,
}

#[derive(Serialize)]
pub struct SccProjectCustomModuleCustomConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    recommendation: terrars::PrimField<std::string::String>,
    severity: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_output: std::option::Option<std::vec::Vec<SccProjectCustomModuleCustomConfigElCustomOutputEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predicate: std::option::Option<std::vec::Vec<SccProjectCustomModuleCustomConfigElPredicateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_selector: std::option::Option<std::vec::Vec<SccProjectCustomModuleCustomConfigElResourceSelectorEl>>,
    dynamic: SccProjectCustomModuleCustomConfigElDynamic,
}

impl SccProjectCustomModuleCustomConfigEl {
    #[doc =
        "Set the field `description`.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects. This explanation is returned with each finding instance to\nhelp investigators understand the detected issue. The text must be enclosed in quotation marks."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_output`.\n"]
    pub fn set_custom_output(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElCustomOutputEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElPredicateEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SccProjectCustomModuleCustomConfigElResourceSelectorEl>>,
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

impl terrars::ToListMappable for SccProjectCustomModuleCustomConfigEl {
    type O = terrars::BlockAssignable<SccProjectCustomModuleCustomConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccProjectCustomModuleCustomConfigEl {
    #[doc =
        "An explanation of the recommended steps that security teams can take to resolve\nthe detected issue. This explanation is returned with each finding generated by\nthis module in the nextSteps property of the finding JSON."]
    pub recommendation: terrars::PrimField<std::string::String>,
    #[doc =
        "The severity to assign to findings generated by the module. Possible values: [\"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub severity: terrars::PrimField<std::string::String>,
}

impl BuildSccProjectCustomModuleCustomConfigEl {
    pub fn build(self) -> SccProjectCustomModuleCustomConfigEl {
        SccProjectCustomModuleCustomConfigEl {
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

pub struct SccProjectCustomModuleCustomConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleCustomConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SccProjectCustomModuleCustomConfigElRef {
        SccProjectCustomModuleCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccProjectCustomModuleCustomConfigElRef {
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
    pub fn custom_output(&self) -> terrars::ListRef<SccProjectCustomModuleCustomConfigElCustomOutputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_output", self.base))
    }

    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(&self) -> terrars::ListRef<SccProjectCustomModuleCustomConfigElPredicateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predicate", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_selector` after provisioning.\n"]
    pub fn resource_selector(&self) -> terrars::ListRef<SccProjectCustomModuleCustomConfigElResourceSelectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct SccProjectCustomModuleTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SccProjectCustomModuleTimeoutsEl {
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

impl terrars::ToListMappable for SccProjectCustomModuleTimeoutsEl {
    type O = terrars::BlockAssignable<SccProjectCustomModuleTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSccProjectCustomModuleTimeoutsEl {}

impl BuildSccProjectCustomModuleTimeoutsEl {
    pub fn build(self) -> SccProjectCustomModuleTimeoutsEl {
        SccProjectCustomModuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SccProjectCustomModuleTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SccProjectCustomModuleTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SccProjectCustomModuleTimeoutsElRef {
        SccProjectCustomModuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SccProjectCustomModuleTimeoutsElRef {
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
struct SccProjectCustomModuleDynamic {
    custom_config: std::option::Option<terrars::DynamicBlock<SccProjectCustomModuleCustomConfigEl>>,
}
