#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowToolData_ {
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
    action_confirmation_requirement: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    tool_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_spec: std::option::Option<std::vec::Vec<DialogflowTool_ConnectorSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_spec: std::option::Option<std::vec::Vec<DialogflowTool_FunctionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_spec: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowTool_Timeouts>,
    dynamic: DialogflowToolDynamic,
}

struct DialogflowTool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowToolData_>,
}

#[derive(Clone)]
pub struct DialogflowTool(std::rc::Rc<DialogflowTool_>);

impl DialogflowTool {
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
        "Set the field `action_confirmation_requirement`.\nOptional. Confirmation requirement for the actions. Each key is an action name in the 'action_schemas'.\nIf an action's confirmation requirement is not specified (either the key is not present, or its value is 'CONFIRMATION_REQUIREMENT_UNSPECIFIED'), a default value will be populated depending on the action's method type: GET actions default to 'NOT_REQUIRED', and other actions default to 'REQUIRED'.\nValid values are 'REQUIRED' and 'NOT_REQUIRED'."]
    pub fn set_action_confirmation_requirement(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().action_confirmation_requirement = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. Human readable description of the tool."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nOptional. The human-readable name of the tool, unique within the project and location."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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
        "Set the field `tool_id`.\nOptional. The ID to use for the tool, which will become the final component of the tool's resource name.\nThe tool ID must be compliant with the regression formula '[a-zA-Z][a-zA-Z0-9_-]*' with the characters length in range of [3,64].\nIf the field is not provided, an Id will be auto-generated."]
    pub fn set_tool_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_spec`.\n"]
    pub fn set_connector_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_ConnectorSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connector_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connector_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function_spec`.\n"]
    pub fn set_function_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_FunctionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().function_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.function_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `open_api_spec`.\n"]
    pub fn set_open_api_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_api_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_api_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowTool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action_confirmation_requirement` after provisioning.\nOptional. Confirmation requirement for the actions. Each key is an action name in the 'action_schemas'.\nIf an action's confirmation requirement is not specified (either the key is not present, or its value is 'CONFIRMATION_REQUIREMENT_UNSPECIFIED'), a default value will be populated depending on the action's method type: GET actions default to 'NOT_REQUIRED', and other actions default to 'REQUIRED'.\nValid values are 'REQUIRED' and 'NOT_REQUIRED'."]
    pub fn action_confirmation_requirement(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(
            self.shared().clone(),
            format!("{}.action_confirmation_requirement", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation time of this tool."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Human readable description of the tool."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. The human-readable name of the tool, unique within the project and location."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the tool."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tool. Format: 'projects/<Project ID>/locations/<Location ID>/tools/<Tool ID>'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzi` after provisioning.\nOutput only. A read only boolean field reflecting Zone Isolation status of the tool."]
    pub fn satisfies_pzi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzi", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzs` after provisioning.\nOutput only. A read only boolean field reflecting Zone Separation status of the tool."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_id` after provisioning.\nOptional. The ID to use for the tool, which will become the final component of the tool's resource name.\nThe tool ID must be compliant with the regression formula '[a-zA-Z][a-zA-Z0-9_-]*' with the characters length in range of [3,64].\nIf the field is not provided, an Id will be auto-generated."]
    pub fn tool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_key` after provisioning.\nRequired. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool."]
    pub fn tool_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time of this tool."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_spec` after provisioning.\n"]
    pub fn connector_spec(&self) -> terrars::ListRef<DialogflowTool_ConnectorSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_spec` after provisioning.\n"]
    pub fn function_spec(&self) -> terrars::ListRef<DialogflowTool_FunctionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_api_spec` after provisioning.\n"]
    pub fn open_api_spec(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowTool_TimeoutsRef {
        DialogflowTool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowTool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowTool { }

impl terrars::ToListMappable for DialogflowTool {
    type O = terrars::ListRef<DialogflowToolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowTool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_tool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowTool {
    pub tf_id: std::string::String,
    #[doc = "The location of the tool."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool."]
    pub tool_key: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowTool {
        let out = DialogflowTool(std::rc::Rc::new(DialogflowTool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowToolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action_confirmation_requirement: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                tool_id: core::default::Default::default(),
                tool_key: self.tool_key,
                connector_spec: core::default::Default::default(),
                function_spec: core::default::Default::default(),
                open_api_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowToolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action_confirmation_requirement` after provisioning.\nOptional. Confirmation requirement for the actions. Each key is an action name in the 'action_schemas'.\nIf an action's confirmation requirement is not specified (either the key is not present, or its value is 'CONFIRMATION_REQUIREMENT_UNSPECIFIED'), a default value will be populated depending on the action's method type: GET actions default to 'NOT_REQUIRED', and other actions default to 'REQUIRED'.\nValid values are 'REQUIRED' and 'NOT_REQUIRED'."]
    pub fn action_confirmation_requirement(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(
            self.shared().clone(),
            format!("{}.action_confirmation_requirement", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation time of this tool."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Human readable description of the tool."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. The human-readable name of the tool, unique within the project and location."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the tool."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tool. Format: 'projects/<Project ID>/locations/<Location ID>/tools/<Tool ID>'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzi` after provisioning.\nOutput only. A read only boolean field reflecting Zone Isolation status of the tool."]
    pub fn satisfies_pzi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzi", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzs` after provisioning.\nOutput only. A read only boolean field reflecting Zone Separation status of the tool."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_id` after provisioning.\nOptional. The ID to use for the tool, which will become the final component of the tool's resource name.\nThe tool ID must be compliant with the regression formula '[a-zA-Z][a-zA-Z0-9_-]*' with the characters length in range of [3,64].\nIf the field is not provided, an Id will be auto-generated."]
    pub fn tool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_key` after provisioning.\nRequired. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool."]
    pub fn tool_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time of this tool."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_spec` after provisioning.\n"]
    pub fn connector_spec(&self) -> terrars::ListRef<DialogflowTool_ConnectorSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_spec` after provisioning.\n"]
    pub fn function_spec(&self) -> terrars::ListRef<DialogflowTool_FunctionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_api_spec` after provisioning.\n"]
    pub fn open_api_spec(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowTool_TimeoutsRef {
        DialogflowTool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowToolDynamic {
    connector_spec: std::option::Option<terrars::DynamicBlock<DialogflowTool_ConnectorSpec>>,
    function_spec: std::option::Option<terrars::DynamicBlock<DialogflowTool_FunctionSpec>>,
    open_api_spec: std::option::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec>>,
}

#[derive(Serialize)]
pub struct DialogflowTool_ConnectorSpec_Actions_EntityOperation {
    entity_id: terrars::PrimField<std::string::String>,
    operation: terrars::PrimField<std::string::String>,
}

impl DialogflowTool_ConnectorSpec_Actions_EntityOperation { }

impl terrars::ToListMappable for DialogflowTool_ConnectorSpec_Actions_EntityOperation {
    type O = terrars::BlockAssignable<DialogflowTool_ConnectorSpec_Actions_EntityOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_ConnectorSpec_Actions_EntityOperation {
    #[doc = "ID of the entity."]
    pub entity_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The operation to perform on the entity. Possible values: [\"LIST\", \"GET\", \"CREATE\", \"UPDATE\", \"DELETE\"]"]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool_ConnectorSpec_Actions_EntityOperation {
    pub fn build(self) -> DialogflowTool_ConnectorSpec_Actions_EntityOperation {
        DialogflowTool_ConnectorSpec_Actions_EntityOperation {
            entity_id: self.entity_id,
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_ConnectorSpec_Actions_EntityOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_ConnectorSpec_Actions_EntityOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowTool_ConnectorSpec_Actions_EntityOperationRef {
        DialogflowTool_ConnectorSpec_Actions_EntityOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_ConnectorSpec_Actions_EntityOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\nID of the entity."]
    pub fn entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform on the entity. Possible values: [\"LIST\", \"GET\", \"CREATE\", \"UPDATE\", \"DELETE\"]"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowTool_ConnectorSpec_ActionsDynamic {
    entity_operation: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowTool_ConnectorSpec_Actions_EntityOperation>>,
}

#[derive(Serialize)]
pub struct DialogflowTool_ConnectorSpec_Actions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_action_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_operation: std::option::Option<std::vec::Vec<DialogflowTool_ConnectorSpec_Actions_EntityOperation>>,
    dynamic: DialogflowTool_ConnectorSpec_ActionsDynamic,
}

impl DialogflowTool_ConnectorSpec_Actions {
    #[doc =
        "Set the field `connection_action_id`.\nID of a Connection action for the tool to use. This field is part of a required union field 'action_spec'."]
    pub fn set_connection_action_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connection_action_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `input_fields`.\nEntity fields to use as inputs for the operation.\nIf no fields are specified, all fields of the Entity will be used."]
    pub fn set_input_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_fields`.\nEntity fields to return from the operation.\nIf no fields are specified, all fields of the Entity will be returned."]
    pub fn set_output_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entity_operation`.\n"]
    pub fn set_entity_operation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_ConnectorSpec_Actions_EntityOperation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entity_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entity_operation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_ConnectorSpec_Actions {
    type O = terrars::BlockAssignable<DialogflowTool_ConnectorSpec_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_ConnectorSpec_Actions {}

impl BuildDialogflowTool_ConnectorSpec_Actions {
    pub fn build(self) -> DialogflowTool_ConnectorSpec_Actions {
        DialogflowTool_ConnectorSpec_Actions {
            connection_action_id: core::default::Default::default(),
            input_fields: core::default::Default::default(),
            output_fields: core::default::Default::default(),
            entity_operation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_ConnectorSpec_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_ConnectorSpec_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowTool_ConnectorSpec_ActionsRef {
        DialogflowTool_ConnectorSpec_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_ConnectorSpec_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_action_id` after provisioning.\nID of a Connection action for the tool to use. This field is part of a required union field 'action_spec'."]
    pub fn connection_action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_action_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `input_fields` after provisioning.\nEntity fields to use as inputs for the operation.\nIf no fields are specified, all fields of the Entity will be used."]
    pub fn input_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_fields` after provisioning.\nEntity fields to return from the operation.\nIf no fields are specified, all fields of the Entity will be returned."]
    pub fn output_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_operation` after provisioning.\n"]
    pub fn entity_operation(&self) -> terrars::ListRef<DialogflowTool_ConnectorSpec_Actions_EntityOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_operation", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowTool_ConnectorSpecDynamic {
    actions: std::option::Option<terrars::DynamicBlock<DialogflowTool_ConnectorSpec_Actions>>,
}

#[derive(Serialize)]
pub struct DialogflowTool_ConnectorSpec {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<DialogflowTool_ConnectorSpec_Actions>>,
    dynamic: DialogflowTool_ConnectorSpecDynamic,
}

impl DialogflowTool_ConnectorSpec {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_ConnectorSpec_Actions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_ConnectorSpec {
    type O = terrars::BlockAssignable<DialogflowTool_ConnectorSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_ConnectorSpec {
    #[doc =
        "The full resource name of the referenced Integration Connectors Connection.\nFormat: 'projects/*/locations/*/connections/*'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool_ConnectorSpec {
    pub fn build(self) -> DialogflowTool_ConnectorSpec {
        DialogflowTool_ConnectorSpec {
            name: self.name,
            actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_ConnectorSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_ConnectorSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowTool_ConnectorSpecRef {
        DialogflowTool_ConnectorSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_ConnectorSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the referenced Integration Connectors Connection.\nFormat: 'projects/*/locations/*/connections/*'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<DialogflowTool_ConnectorSpec_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_FunctionSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_schema: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowTool_FunctionSpec {
    #[doc =
        "Set the field `input_schema`.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the input of the function.\nThis input is a JSON object that contains the function's parameters as properties of the object."]
    pub fn set_input_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method_type`.\nOptional. The method type of the function. If not specified, the default value is GET. Possible values: [\"GET\", \"POST\", \"PUT\", \"DELETE\", \"PATCH\"]"]
    pub fn set_method_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_schema`.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the output of the function.\nThis output is a JSON object that contains the function's parameters as properties of the object."]
    pub fn set_output_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_FunctionSpec {
    type O = terrars::BlockAssignable<DialogflowTool_FunctionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_FunctionSpec {}

impl BuildDialogflowTool_FunctionSpec {
    pub fn build(self) -> DialogflowTool_FunctionSpec {
        DialogflowTool_FunctionSpec {
            input_schema: core::default::Default::default(),
            method_type: core::default::Default::default(),
            output_schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_FunctionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_FunctionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowTool_FunctionSpecRef {
        DialogflowTool_FunctionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_FunctionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `input_schema` after provisioning.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the input of the function.\nThis input is a JSON object that contains the function's parameters as properties of the object."]
    pub fn input_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method_type` after provisioning.\nOptional. The method type of the function. If not specified, the default value is GET. Possible values: [\"GET\", \"POST\", \"PUT\", \"DELETE\", \"PATCH\"]"]
    pub fn method_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_schema` after provisioning.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the output of the function.\nThis output is a JSON object that contains the function's parameters as properties of the object."]
    pub fn output_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_schema", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    key_name: terrars::PrimField<std::string::String>,
    request_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_version_for_api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig {
    #[doc =
        "Set the field `api_key`.\nOptional. The API key. If the 'secretVersionForApiKey' field is set, this field will be ignored."]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_version_for_api_key`.\nOptional. The name of the SecretManager secret version resource storing the API key.\nIf this field is set, the 'apiKey' field will be ignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_api_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig {
    #[doc =
        "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: terrars::PrimField<std::string::String>,
    #[doc = "Key location in the request. Possible values: [\"HEADER\", \"QUERY_STRING\"]"]
    pub request_location: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig {
        DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig {
            api_key: core::default::Default::default(),
            key_name: self.key_name,
            request_location: self.request_location,
            secret_version_for_api_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
        DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key` after provisioning.\nOptional. The API key. If the 'secretVersionForApiKey' field is set, this field will be ignored."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request. Possible values: [\"HEADER\", \"QUERY_STRING\"]"]
    pub fn request_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_api_key` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the API key.\nIf this field is set, the 'apiKey' field will be ignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version_for_api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_version_for_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig {
    #[doc =
        "Set the field `secret_version_for_token`.\nOptional. The name of the SecretManager secret version resource storing the Bearer token.\nIf this field is set, the 'token' field will be ignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_token(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token`.\nOptional. The text token appended to the text 'Bearer' to the request Authorization header."]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig {}

impl BuildDialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig {
        DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig {
            secret_version_for_token: core::default::Default::default(),
            token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
        DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_token` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the Bearer token.\nIf this field is set, the 'token' field will be ignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version_for_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token` after provisioning.\nOptional. The text token appended to the text 'Bearer' to the request Authorization header."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_Authentication_OauthConfig {
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    oauth_grant_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_version_for_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    token_endpoint: terrars::PrimField<std::string::String>,
}

impl DialogflowTool_OpenApiSpec_Authentication_OauthConfig {
    #[doc =
        "Set the field `client_secret`.\nOptional. The client secret from the OAuth provider. If the 'secretVersionForClientSecret' field is set, this field will be ignored."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\nOptional. The OAuth scopes to grant."]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_version_for_client_secret`.\nOptional. The name of the SecretManager secret version resource storing the client secret.\nIf this field is set, the 'clientSecret' field will be ignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_Authentication_OauthConfig {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_Authentication_OauthConfig {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "OAuth grant types. Possible values: [\"CLIENT_CREDENTIAL\"]"]
    pub oauth_grant_type: terrars::PrimField<std::string::String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool_OpenApiSpec_Authentication_OauthConfig {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_Authentication_OauthConfig {
        DialogflowTool_OpenApiSpec_Authentication_OauthConfig {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            oauth_grant_type: self.oauth_grant_type,
            scopes: core::default::Default::default(),
            secret_version_for_client_secret: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_Authentication_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_Authentication_OauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowTool_OpenApiSpec_Authentication_OauthConfigRef {
        DialogflowTool_OpenApiSpec_Authentication_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_Authentication_OauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client ID from the OAuth provider."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nOptional. The client secret from the OAuth provider. If the 'secretVersionForClientSecret' field is set, this field will be ignored."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_grant_type` after provisioning.\nOAuth grant types. Possible values: [\"CLIENT_CREDENTIAL\"]"]
    pub fn oauth_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nOptional. The OAuth scopes to grant."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_client_secret` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the client secret.\nIf this field is set, the 'clientSecret' field will be ignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version_for_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint in the OAuth provider to exchange for an access token."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_auth: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    #[doc =
        "Set the field `service_agent_auth`.\nOptional. Indicate the auth token type generated from the Dialogflow service agent.\nThe generated token is sent in the Authorization header. Possible values: [\"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn set_service_agent_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_agent_auth = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {}

impl BuildDialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
        DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
            service_agent_auth: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
        DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_agent_auth` after provisioning.\nOptional. Indicate the auth token type generated from the Dialogflow service agent.\nThe generated token is sent in the Authorization header. Possible values: [\"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn service_agent_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_agent_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowTool_OpenApiSpec_AuthenticationDynamic {
    api_key_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig>>,
    bearer_token_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig>>,
    oauth_config: std::option::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_Authentication_OauthConfig>>,
    service_agent_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_config: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_Authentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_auth_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>>,
    dynamic: DialogflowTool_OpenApiSpec_AuthenticationDynamic,
}

impl DialogflowTool_OpenApiSpec_Authentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bearer_token_config`.\n"]
    pub fn set_bearer_token_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bearer_token_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bearer_token_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_OauthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_agent_auth_config`.\n"]
    pub fn set_service_agent_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_agent_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_Authentication {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_Authentication {}

impl BuildDialogflowTool_OpenApiSpec_Authentication {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_Authentication {
        DialogflowTool_OpenApiSpec_Authentication {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_agent_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_AuthenticationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowTool_OpenApiSpec_AuthenticationRef {
        DialogflowTool_OpenApiSpec_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpec_Authentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> terrars::ListRef<DialogflowTool_OpenApiSpec_Authentication_BearerTokenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpec_Authentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_auth_config` after provisioning.\n"]
    pub fn service_agent_auth_config(
        &self,
    ) -> terrars::ListRef<DialogflowTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_auth_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl DialogflowTool_OpenApiSpec_ServiceDirectoryConfig { }

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_ServiceDirectoryConfig {
    #[doc =
        "The name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: 'projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>'.\n'LocationID' of the service directory must be the same as the location of the tool."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool_OpenApiSpec_ServiceDirectoryConfig {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_ServiceDirectoryConfig {
        DialogflowTool_OpenApiSpec_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowTool_OpenApiSpec_ServiceDirectoryConfigRef {
        DialogflowTool_OpenApiSpec_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: 'projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>'.\n'LocationID' of the service directory must be the same as the location of the tool."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_TlsConfig_CaCerts {
    cert: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
}

impl DialogflowTool_OpenApiSpec_TlsConfig_CaCerts { }

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_TlsConfig_CaCerts {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_TlsConfig_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_TlsConfig_CaCerts {
    #[doc =
        "The allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store.\nIf this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt name\".\nA base64-encoded string."]
    pub cert: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the allowed custom CA certificates. This can be used to disambiguate the custom CA certificates."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool_OpenApiSpec_TlsConfig_CaCerts {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_TlsConfig_CaCerts {
        DialogflowTool_OpenApiSpec_TlsConfig_CaCerts {
            cert: self.cert,
            display_name: self.display_name,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_TlsConfig_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_TlsConfig_CaCertsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowTool_OpenApiSpec_TlsConfig_CaCertsRef {
        DialogflowTool_OpenApiSpec_TlsConfig_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_TlsConfig_CaCertsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cert` after provisioning.\nThe allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store.\nIf this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt name\".\nA base64-encoded string."]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe name of the allowed custom CA certificates. This can be used to disambiguate the custom CA certificates."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowTool_OpenApiSpec_TlsConfigDynamic {
    ca_certs: std::option::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_TlsConfig_CaCerts>>,
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec_TlsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_TlsConfig_CaCerts>>,
    dynamic: DialogflowTool_OpenApiSpec_TlsConfigDynamic,
}

impl DialogflowTool_OpenApiSpec_TlsConfig {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec_TlsConfig_CaCerts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ca_certs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ca_certs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec_TlsConfig {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec_TlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec_TlsConfig {}

impl BuildDialogflowTool_OpenApiSpec_TlsConfig {
    pub fn build(self) -> DialogflowTool_OpenApiSpec_TlsConfig {
        DialogflowTool_OpenApiSpec_TlsConfig {
            ca_certs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpec_TlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpec_TlsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowTool_OpenApiSpec_TlsConfigRef {
        DialogflowTool_OpenApiSpec_TlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpec_TlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpec_TlsConfig_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowTool_OpenApiSpecDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_Authentication>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_ServiceDirectoryConfig>>,
    tls_config: std::option::Option<terrars::DynamicBlock<DialogflowTool_OpenApiSpec_TlsConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowTool_OpenApiSpec {
    text_schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_Authentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<std::vec::Vec<DialogflowTool_OpenApiSpec_TlsConfig>>,
    dynamic: DialogflowTool_OpenApiSpecDynamic,
}

impl DialogflowTool_OpenApiSpec {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec_ServiceDirectoryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_directory_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowTool_OpenApiSpec_TlsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowTool_OpenApiSpec {
    type O = terrars::BlockAssignable<DialogflowTool_OpenApiSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_OpenApiSpec {
    #[doc =
        "Required. The OpenAPI schema specified as a text.\nNote: Plays a role in linking the OpenAPI spec with the tool. The 'info.title' field in the OpenAPI schema must match the 'toolKey' of the tool, otherwise the API will overwrite 'info.title' with 'toolKey'."]
    pub text_schema: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowTool_OpenApiSpec {
    pub fn build(self) -> DialogflowTool_OpenApiSpec {
        DialogflowTool_OpenApiSpec {
            text_schema: self.text_schema,
            authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_OpenApiSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_OpenApiSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowTool_OpenApiSpecRef {
        DialogflowTool_OpenApiSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_OpenApiSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `text_schema` after provisioning.\nRequired. The OpenAPI schema specified as a text.\nNote: Plays a role in linking the OpenAPI spec with the tool. The 'info.title' field in the OpenAPI schema must match the 'toolKey' of the tool, otherwise the API will overwrite 'info.title' with 'toolKey'."]
    pub fn text_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpec_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpec_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<DialogflowTool_OpenApiSpec_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowTool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowTool_Timeouts {
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

impl terrars::ToListMappable for DialogflowTool_Timeouts {
    type O = terrars::BlockAssignable<DialogflowTool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowTool_Timeouts {}

impl BuildDialogflowTool_Timeouts {
    pub fn build(self) -> DialogflowTool_Timeouts {
        DialogflowTool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowTool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowTool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowTool_TimeoutsRef {
        DialogflowTool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowTool_TimeoutsRef {
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
