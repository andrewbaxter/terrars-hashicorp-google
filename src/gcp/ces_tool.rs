#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesToolData_ {
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
    app: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    tool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_tool: std::option::Option<std::vec::Vec<CesTool_AgentTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_function: std::option::Option<std::vec::Vec<CesTool_ClientFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_tool: std::option::Option<std::vec::Vec<CesTool_DataStoreTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_search_tool: std::option::Option<std::vec::Vec<CesTool_FileSearchTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_search_tool: std::option::Option<std::vec::Vec<CesTool_GoogleSearchTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_function: std::option::Option<std::vec::Vec<CesTool_PythonFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesTool_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_fake_config: std::option::Option<std::vec::Vec<CesTool_ToolFakeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    widget_tool: std::option::Option<std::vec::Vec<CesTool_WidgetTool>>,
    dynamic: CesToolDynamic,
}

struct CesTool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesToolData_>,
}

#[derive(Clone)]
pub struct CesTool(std::rc::Rc<CesTool_>);

impl CesTool {
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

    #[doc = "Set the field `execution_type`.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn set_execution_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_type = std::option::Option::Some(v.into());
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
        "Set the field `timeout`.\nThe timeout for the tool execution. If not set, the default timeout is 30\nseconds for SYNCHRONOUS tools and 60 seconds for ASYNCHRONOUS tools."]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_tool`.\n"]
    pub fn set_agent_tool(self, v: impl std::convert::Into<terrars::BlockAssignable<CesTool_AgentTool>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_tool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_function`.\n"]
    pub fn set_client_function(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_ClientFunction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_store_tool`.\n"]
    pub fn set_data_store_tool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_store_tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_store_tool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_search_tool`.\n"]
    pub fn set_file_search_tool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_FileSearchTool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_search_tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_search_tool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_search_tool`.\n"]
    pub fn set_google_search_tool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_GoogleSearchTool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().google_search_tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.google_search_tool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `python_function`.\n"]
    pub fn set_python_function(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_PythonFunction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().python_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.python_function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesTool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool_fake_config`.\n"]
    pub fn set_tool_fake_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_ToolFakeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tool_fake_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tool_fake_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `widget_tool`.\n"]
    pub fn set_widget_tool(self, v: impl std::convert::Into<terrars::BlockAssignable<CesTool_WidgetTool>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().widget_tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.widget_tool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_tool` after provisioning.\nA ConnectorTool allows connections to different integrations."]
    pub fn connector_tool(&self) -> terrars::ListRef<CesTool_ConnectorToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_tool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the tool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tool, derived based on the tool's type. For\nexample, display name of a ClientFunction is derived\nfrom its 'name' property."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_type` after provisioning.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn execution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_summary` after provisioning.\nIf the tool is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_tool` after provisioning.\nAn MCP tool."]
    pub fn mcp_tool(&self) -> terrars::ListRef<CesTool_McpToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_tool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the tool.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `open_api_tool` after provisioning.\nA remote API tool defined by an OpenAPI schema."]
    pub fn open_api_tool(&self) -> terrars::ListRef<CesTool_OpenApiToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_agent_tool` after provisioning.\nRepresents a tool that allows the agent to call another remote agent."]
    pub fn remote_agent_tool(&self) -> terrars::ListRef<CesTool_RemoteAgentToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_agent_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_tool` after provisioning.\nThe system tool."]
    pub fn system_tool(&self) -> terrars::ListRef<CesTool_SystemToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_tool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout for the tool execution. If not set, the default timeout is 30\nseconds for SYNCHRONOUS tools and 60 seconds for ASYNCHRONOUS tools."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_id` after provisioning.\nThe ID to use for the tool, which will become the final component of\nthe tool's resource name. If not provided, a unique ID will be\nautomatically assigned for the tool."]
    pub fn tool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the tool was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_tool` after provisioning.\n"]
    pub fn agent_tool(&self) -> terrars::ListRef<CesTool_AgentToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_function` after provisioning.\n"]
    pub fn client_function(&self) -> terrars::ListRef<CesTool_ClientFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_store_tool` after provisioning.\n"]
    pub fn data_store_tool(&self) -> terrars::ListRef<CesTool_DataStoreToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_search_tool` after provisioning.\n"]
    pub fn file_search_tool(&self) -> terrars::ListRef<CesTool_FileSearchToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_search_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `google_search_tool` after provisioning.\n"]
    pub fn google_search_tool(&self) -> terrars::ListRef<CesTool_GoogleSearchToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_search_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `python_function` after provisioning.\n"]
    pub fn python_function(&self) -> terrars::ListRef<CesTool_PythonFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesTool_TimeoutsRef {
        CesTool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool_fake_config` after provisioning.\n"]
    pub fn tool_fake_config(&self) -> terrars::ListRef<CesTool_ToolFakeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_fake_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `widget_tool` after provisioning.\n"]
    pub fn widget_tool(&self) -> terrars::ListRef<CesTool_WidgetToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.widget_tool", self.extract_ref()))
    }
}

impl terrars::Referable for CesTool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesTool { }

impl terrars::ToListMappable for CesTool {
    type O = terrars::ListRef<CesToolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesTool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_tool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesTool {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the tool, which will become the final component of\nthe tool's resource name. If not provided, a unique ID will be\nautomatically assigned for the tool."]
    pub tool_id: terrars::PrimField<std::string::String>,
}

impl BuildCesTool {
    pub fn build(self, stack: &mut terrars::Stack) -> CesTool {
        let out = CesTool(std::rc::Rc::new(CesTool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesToolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                deletion_policy: core::default::Default::default(),
                execution_type: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                timeout: core::default::Default::default(),
                tool_id: self.tool_id,
                agent_tool: core::default::Default::default(),
                client_function: core::default::Default::default(),
                data_store_tool: core::default::Default::default(),
                file_search_tool: core::default::Default::default(),
                google_search_tool: core::default::Default::default(),
                python_function: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tool_fake_config: core::default::Default::default(),
                widget_tool: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesToolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_tool` after provisioning.\nA ConnectorTool allows connections to different integrations."]
    pub fn connector_tool(&self) -> terrars::ListRef<CesTool_ConnectorToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_tool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the tool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tool, derived based on the tool's type. For\nexample, display name of a ClientFunction is derived\nfrom its 'name' property."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_type` after provisioning.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn execution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_summary` after provisioning.\nIf the tool is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_tool` after provisioning.\nAn MCP tool."]
    pub fn mcp_tool(&self) -> terrars::ListRef<CesTool_McpToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_tool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the tool.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `open_api_tool` after provisioning.\nA remote API tool defined by an OpenAPI schema."]
    pub fn open_api_tool(&self) -> terrars::ListRef<CesTool_OpenApiToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_agent_tool` after provisioning.\nRepresents a tool that allows the agent to call another remote agent."]
    pub fn remote_agent_tool(&self) -> terrars::ListRef<CesTool_RemoteAgentToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_agent_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_tool` after provisioning.\nThe system tool."]
    pub fn system_tool(&self) -> terrars::ListRef<CesTool_SystemToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_tool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout for the tool execution. If not set, the default timeout is 30\nseconds for SYNCHRONOUS tools and 60 seconds for ASYNCHRONOUS tools."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_id` after provisioning.\nThe ID to use for the tool, which will become the final component of\nthe tool's resource name. If not provided, a unique ID will be\nautomatically assigned for the tool."]
    pub fn tool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the tool was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_tool` after provisioning.\n"]
    pub fn agent_tool(&self) -> terrars::ListRef<CesTool_AgentToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_function` after provisioning.\n"]
    pub fn client_function(&self) -> terrars::ListRef<CesTool_ClientFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_store_tool` after provisioning.\n"]
    pub fn data_store_tool(&self) -> terrars::ListRef<CesTool_DataStoreToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_search_tool` after provisioning.\n"]
    pub fn file_search_tool(&self) -> terrars::ListRef<CesTool_FileSearchToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_search_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `google_search_tool` after provisioning.\n"]
    pub fn google_search_tool(&self) -> terrars::ListRef<CesTool_GoogleSearchToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_search_tool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `python_function` after provisioning.\n"]
    pub fn python_function(&self) -> terrars::ListRef<CesTool_PythonFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesTool_TimeoutsRef {
        CesTool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool_fake_config` after provisioning.\n"]
    pub fn tool_fake_config(&self) -> terrars::ListRef<CesTool_ToolFakeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_fake_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `widget_tool` after provisioning.\n"]
    pub fn widget_tool(&self) -> terrars::ListRef<CesTool_WidgetToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.widget_tool", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CesToolDynamic {
    agent_tool: std::option::Option<terrars::DynamicBlock<CesTool_AgentTool>>,
    client_function: std::option::Option<terrars::DynamicBlock<CesTool_ClientFunction>>,
    data_store_tool: std::option::Option<terrars::DynamicBlock<CesTool_DataStoreTool>>,
    file_search_tool: std::option::Option<terrars::DynamicBlock<CesTool_FileSearchTool>>,
    google_search_tool: std::option::Option<terrars::DynamicBlock<CesTool_GoogleSearchTool>>,
    python_function: std::option::Option<terrars::DynamicBlock<CesTool_PythonFunction>>,
    tool_fake_config: std::option::Option<terrars::DynamicBlock<CesTool_ToolFakeConfig>>,
    widget_tool: std::option::Option<terrars::DynamicBlock<CesTool_WidgetTool>>,
}

#[derive(Serialize)]
pub struct CesTool_ConnectorTool_Action_EntityOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_ConnectorTool_Action_EntityOperation {
    #[doc = "Set the field `entity_id`.\n"]
    pub fn set_entity_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entity_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_ConnectorTool_Action_EntityOperation {
    type O = terrars::BlockAssignable<CesTool_ConnectorTool_Action_EntityOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_ConnectorTool_Action_EntityOperation {}

impl BuildCesTool_ConnectorTool_Action_EntityOperation {
    pub fn build(self) -> CesTool_ConnectorTool_Action_EntityOperation {
        CesTool_ConnectorTool_Action_EntityOperation {
            entity_id: core::default::Default::default(),
            operation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_ConnectorTool_Action_EntityOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_ConnectorTool_Action_EntityOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_ConnectorTool_Action_EntityOperationRef {
        CesTool_ConnectorTool_Action_EntityOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_ConnectorTool_Action_EntityOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\n"]
    pub fn entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_ConnectorTool_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_action_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_operation: std::option::Option<terrars::ListField<CesTool_ConnectorTool_Action_EntityOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CesTool_ConnectorTool_Action {
    #[doc = "Set the field `connection_action_id`.\n"]
    pub fn set_connection_action_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connection_action_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entity_operation`.\n"]
    pub fn set_entity_operation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_ConnectorTool_Action_EntityOperation>>,
    ) -> Self {
        self.entity_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_fields`.\n"]
    pub fn set_input_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_fields`.\n"]
    pub fn set_output_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_fields = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_ConnectorTool_Action {
    type O = terrars::BlockAssignable<CesTool_ConnectorTool_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_ConnectorTool_Action {}

impl BuildCesTool_ConnectorTool_Action {
    pub fn build(self) -> CesTool_ConnectorTool_Action {
        CesTool_ConnectorTool_Action {
            connection_action_id: core::default::Default::default(),
            entity_operation: core::default::Default::default(),
            input_fields: core::default::Default::default(),
            output_fields: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_ConnectorTool_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_ConnectorTool_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_ConnectorTool_ActionRef {
        CesTool_ConnectorTool_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_ConnectorTool_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_action_id` after provisioning.\n"]
    pub fn connection_action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_action_id", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_operation` after provisioning.\n"]
    pub fn entity_operation(&self) -> terrars::ListRef<CesTool_ConnectorTool_Action_EntityOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `input_fields` after provisioning.\n"]
    pub fn input_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `output_fields` after provisioning.\n"]
    pub fn output_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_ConnectorTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<CesTool_ConnectorTool_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_ConnectorTool {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::ListField<CesTool_ConnectorTool_Action>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection`.\n"]
    pub fn set_connection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_ConnectorTool {
    type O = terrars::BlockAssignable<CesTool_ConnectorTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_ConnectorTool {}

impl BuildCesTool_ConnectorTool {
    pub fn build(self) -> CesTool_ConnectorTool {
        CesTool_ConnectorTool {
            action: core::default::Default::default(),
            auth_config: core::default::Default::default(),
            connection: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_ConnectorToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_ConnectorToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_ConnectorToolRef {
        CesTool_ConnectorToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_ConnectorToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<CesTool_ConnectorTool_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `connection` after provisioning.\n"]
    pub fn connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_ApiAuthentication_ApiKeyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_McpTool_ApiAuthentication_ApiKeyConfig {
    #[doc = "Set the field `api_key_secret_version`.\n"]
    pub fn set_api_key_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.api_key_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_location`.\n"]
    pub fn set_request_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_ApiAuthentication_ApiKeyConfig {
    type O = terrars::BlockAssignable<CesTool_McpTool_ApiAuthentication_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_ApiAuthentication_ApiKeyConfig {}

impl BuildCesTool_McpTool_ApiAuthentication_ApiKeyConfig {
    pub fn build(self) -> CesTool_McpTool_ApiAuthentication_ApiKeyConfig {
        CesTool_McpTool_ApiAuthentication_ApiKeyConfig {
            api_key_secret_version: core::default::Default::default(),
            key_name: core::default::Default::default(),
            request_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_ApiAuthentication_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_ApiAuthentication_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_McpTool_ApiAuthentication_ApiKeyConfigRef {
        CesTool_McpTool_ApiAuthentication_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_ApiAuthentication_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_secret_version` after provisioning.\n"]
    pub fn api_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `request_location` after provisioning.\n"]
    pub fn request_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_location", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_ApiAuthentication_BearerTokenConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_McpTool_ApiAuthentication_BearerTokenConfig {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_ApiAuthentication_BearerTokenConfig {
    type O = terrars::BlockAssignable<CesTool_McpTool_ApiAuthentication_BearerTokenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_ApiAuthentication_BearerTokenConfig {}

impl BuildCesTool_McpTool_ApiAuthentication_BearerTokenConfig {
    pub fn build(self) -> CesTool_McpTool_ApiAuthentication_BearerTokenConfig {
        CesTool_McpTool_ApiAuthentication_BearerTokenConfig { token: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_ApiAuthentication_BearerTokenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_ApiAuthentication_BearerTokenConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_McpTool_ApiAuthentication_BearerTokenConfigRef {
        CesTool_McpTool_ApiAuthentication_BearerTokenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_ApiAuthentication_BearerTokenConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_ApiAuthentication_OauthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_grant_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_McpTool_ApiAuthentication_OauthConfig {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_version`.\n"]
    pub fn set_client_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_grant_type`.\n"]
    pub fn set_oauth_grant_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oauth_grant_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_ApiAuthentication_OauthConfig {
    type O = terrars::BlockAssignable<CesTool_McpTool_ApiAuthentication_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_ApiAuthentication_OauthConfig {}

impl BuildCesTool_McpTool_ApiAuthentication_OauthConfig {
    pub fn build(self) -> CesTool_McpTool_ApiAuthentication_OauthConfig {
        CesTool_McpTool_ApiAuthentication_OauthConfig {
            client_id: core::default::Default::default(),
            client_secret_version: core::default::Default::default(),
            oauth_grant_type: core::default::Default::default(),
            scopes: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_ApiAuthentication_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_ApiAuthentication_OauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_McpTool_ApiAuthentication_OauthConfigRef {
        CesTool_McpTool_ApiAuthentication_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_ApiAuthentication_OauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\n"]
    pub fn client_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\n"]
    pub fn oauth_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig {
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig {
    type O = terrars::BlockAssignable<CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig {}

impl BuildCesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig {
    pub fn build(self) -> CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig {
        CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig {
            scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfigRef {
        CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}

impl CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig { }

impl terrars::ToListMappable for CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
    type O = terrars::BlockAssignable<CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}

impl BuildCesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
    pub fn build(self) -> CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
        CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
        CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_ApiAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_config: std::option::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication_BearerTokenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_auth_config: std
    ::option
    ::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_id_token_auth_config: std
    ::option
    ::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig>>,
}

impl CesTool_McpTool_ApiAuthentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication_ApiKeyConfig>>,
    ) -> Self {
        self.api_key_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bearer_token_config`.\n"]
    pub fn set_bearer_token_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication_BearerTokenConfig>>,
    ) -> Self {
        self.bearer_token_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication_OauthConfig>>,
    ) -> Self {
        self.oauth_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfig>>,
    ) -> Self {
        self.service_account_auth_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig>>,
    ) -> Self {
        self.service_agent_id_token_auth_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_ApiAuthentication {
    type O = terrars::BlockAssignable<CesTool_McpTool_ApiAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_ApiAuthentication {}

impl BuildCesTool_McpTool_ApiAuthentication {
    pub fn build(self) -> CesTool_McpTool_ApiAuthentication {
        CesTool_McpTool_ApiAuthentication {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_ApiAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_ApiAuthenticationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_McpTool_ApiAuthenticationRef {
        CesTool_McpTool_ApiAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_ApiAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> terrars::ListRef<CesTool_McpTool_ApiAuthentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(&self) -> terrars::ListRef<CesTool_McpTool_ApiAuthentication_BearerTokenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<CesTool_McpTool_ApiAuthentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> terrars::ListRef<CesTool_McpTool_ApiAuthentication_ServiceAccountAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> terrars::ListRef<CesTool_McpTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_id_token_auth_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_ServiceDirectoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_McpTool_ServiceDirectoryConfig {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<CesTool_McpTool_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_ServiceDirectoryConfig {}

impl BuildCesTool_McpTool_ServiceDirectoryConfig {
    pub fn build(self) -> CesTool_McpTool_ServiceDirectoryConfig {
        CesTool_McpTool_ServiceDirectoryConfig { service: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_ServiceDirectoryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_McpTool_ServiceDirectoryConfigRef {
        CesTool_McpTool_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_TlsConfig_CaCerts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_McpTool_TlsConfig_CaCerts {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_TlsConfig_CaCerts {
    type O = terrars::BlockAssignable<CesTool_McpTool_TlsConfig_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_TlsConfig_CaCerts {}

impl BuildCesTool_McpTool_TlsConfig_CaCerts {
    pub fn build(self) -> CesTool_McpTool_TlsConfig_CaCerts {
        CesTool_McpTool_TlsConfig_CaCerts {
            cert: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_TlsConfig_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_TlsConfig_CaCertsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_McpTool_TlsConfig_CaCertsRef {
        CesTool_McpTool_TlsConfig_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_TlsConfig_CaCertsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool_TlsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<terrars::ListField<CesTool_McpTool_TlsConfig_CaCerts>>,
}

impl CesTool_McpTool_TlsConfig {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_TlsConfig_CaCerts>>,
    ) -> Self {
        self.ca_certs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool_TlsConfig {
    type O = terrars::BlockAssignable<CesTool_McpTool_TlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool_TlsConfig {}

impl BuildCesTool_McpTool_TlsConfig {
    pub fn build(self) -> CesTool_McpTool_TlsConfig {
        CesTool_McpTool_TlsConfig { ca_certs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpTool_TlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpTool_TlsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_McpTool_TlsConfigRef {
        CesTool_McpTool_TlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpTool_TlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<CesTool_McpTool_TlsConfig_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_McpTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std::option::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_override: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std::option::Option<terrars::ListField<CesTool_McpTool_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<terrars::ListField<CesTool_McpTool_TlsConfig>>,
}

impl CesTool_McpTool {
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication>>,
    ) -> Self {
        self.api_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_headers`.\n"]
    pub fn set_custom_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_override`.\n"]
    pub fn set_name_override(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_schema`.\n"]
    pub fn set_output_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_address`.\n"]
    pub fn set_server_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ServiceDirectoryConfig>>,
    ) -> Self {
        self.service_directory_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(mut self, v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_TlsConfig>>) -> Self {
        self.tls_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_McpTool {
    type O = terrars::BlockAssignable<CesTool_McpTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_McpTool {}

impl BuildCesTool_McpTool {
    pub fn build(self) -> CesTool_McpTool {
        CesTool_McpTool {
            api_authentication: core::default::Default::default(),
            custom_headers: core::default::Default::default(),
            description: core::default::Default::default(),
            input_schema: core::default::Default::default(),
            name: core::default::Default::default(),
            name_override: core::default::Default::default(),
            output_schema: core::default::Default::default(),
            server_address: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            state: core::default::Default::default(),
            tls_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_McpToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_McpToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_McpToolRef {
        CesTool_McpToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_McpToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> terrars::ListRef<CesTool_McpTool_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\n"]
    pub fn custom_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `name_override` after provisioning.\n"]
    pub fn name_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_override", self.base))
    }

    #[doc = "Get a reference to the value of field `output_schema` after provisioning.\n"]
    pub fn output_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `server_address` after provisioning.\n"]
    pub fn server_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_address", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(&self) -> terrars::ListRef<CesTool_McpTool_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<CesTool_McpTool_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_OpenApiTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std::option::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std::option::Option<terrars::ListField<CesTool_McpTool_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<terrars::ListField<CesTool_McpTool_TlsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_OpenApiTool {
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication>>,
    ) -> Self {
        self.api_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_unknown_fields`.\n"]
    pub fn set_ignore_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.open_api_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ServiceDirectoryConfig>>,
    ) -> Self {
        self.service_directory_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(mut self, v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_TlsConfig>>) -> Self {
        self.tls_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_OpenApiTool {
    type O = terrars::BlockAssignable<CesTool_OpenApiTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_OpenApiTool {}

impl BuildCesTool_OpenApiTool {
    pub fn build(self) -> CesTool_OpenApiTool {
        CesTool_OpenApiTool {
            api_authentication: core::default::Default::default(),
            description: core::default::Default::default(),
            ignore_unknown_fields: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_OpenApiToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_OpenApiToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_OpenApiToolRef {
        CesTool_OpenApiToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_OpenApiToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> terrars::ListRef<CesTool_McpTool_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\n"]
    pub fn ignore_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_api_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(&self) -> terrars::ListRef<CesTool_McpTool_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<CesTool_McpTool_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_RemoteAgentTool_AgentCard_Skills {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    examples: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CesTool_RemoteAgentTool_AgentCard_Skills {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `examples`.\n"]
    pub fn set_examples(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.examples = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_modes`.\n"]
    pub fn set_input_modes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_modes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_modes`.\n"]
    pub fn set_output_modes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_modes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_RemoteAgentTool_AgentCard_Skills {
    type O = terrars::BlockAssignable<CesTool_RemoteAgentTool_AgentCard_Skills>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_RemoteAgentTool_AgentCard_Skills {}

impl BuildCesTool_RemoteAgentTool_AgentCard_Skills {
    pub fn build(self) -> CesTool_RemoteAgentTool_AgentCard_Skills {
        CesTool_RemoteAgentTool_AgentCard_Skills {
            description: core::default::Default::default(),
            examples: core::default::Default::default(),
            id: core::default::Default::default(),
            input_modes: core::default::Default::default(),
            name: core::default::Default::default(),
            output_modes: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_RemoteAgentTool_AgentCard_SkillsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_RemoteAgentTool_AgentCard_SkillsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_RemoteAgentTool_AgentCard_SkillsRef {
        CesTool_RemoteAgentTool_AgentCard_SkillsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_RemoteAgentTool_AgentCard_SkillsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `examples` after provisioning.\n"]
    pub fn examples(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.examples", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `input_modes` after provisioning.\n"]
    pub fn input_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_modes", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_modes` after provisioning.\n"]
    pub fn output_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_modes", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_binding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces {
    #[doc = "Set the field `protocol_binding`.\n"]
    pub fn set_protocol_binding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol_binding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol_version`.\n"]
    pub fn set_protocol_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant`.\n"]
    pub fn set_tenant(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces {
    type O = terrars::BlockAssignable<CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_RemoteAgentTool_AgentCard_SupportedInterfaces {}

impl BuildCesTool_RemoteAgentTool_AgentCard_SupportedInterfaces {
    pub fn build(self) -> CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces {
        CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces {
            protocol_binding: core::default::Default::default(),
            protocol_version: core::default::Default::default(),
            tenant: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_RemoteAgentTool_AgentCard_SupportedInterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_RemoteAgentTool_AgentCard_SupportedInterfacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_RemoteAgentTool_AgentCard_SupportedInterfacesRef {
        CesTool_RemoteAgentTool_AgentCard_SupportedInterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_RemoteAgentTool_AgentCard_SupportedInterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `protocol_binding` after provisioning.\n"]
    pub fn protocol_binding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_binding", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\n"]
    pub fn protocol_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_version", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_RemoteAgentTool_AgentCard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skills: std::option::Option<terrars::ListField<CesTool_RemoteAgentTool_AgentCard_Skills>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_interfaces: std::option::Option<terrars::ListField<CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_RemoteAgentTool_AgentCard {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skills`.\n"]
    pub fn set_skills(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_RemoteAgentTool_AgentCard_Skills>>,
    ) -> Self {
        self.skills = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_interfaces`.\n"]
    pub fn set_supported_interfaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_RemoteAgentTool_AgentCard_SupportedInterfaces>>,
    ) -> Self {
        self.supported_interfaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_RemoteAgentTool_AgentCard {
    type O = terrars::BlockAssignable<CesTool_RemoteAgentTool_AgentCard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_RemoteAgentTool_AgentCard {}

impl BuildCesTool_RemoteAgentTool_AgentCard {
    pub fn build(self) -> CesTool_RemoteAgentTool_AgentCard {
        CesTool_RemoteAgentTool_AgentCard {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            skills: core::default::Default::default(),
            supported_interfaces: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_RemoteAgentTool_AgentCardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_RemoteAgentTool_AgentCardRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_RemoteAgentTool_AgentCardRef {
        CesTool_RemoteAgentTool_AgentCardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_RemoteAgentTool_AgentCardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `skills` after provisioning.\n"]
    pub fn skills(&self) -> terrars::ListRef<CesTool_RemoteAgentTool_AgentCard_SkillsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skills", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_interfaces` after provisioning.\n"]
    pub fn supported_interfaces(&self) -> terrars::ListRef<CesTool_RemoteAgentTool_AgentCard_SupportedInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_interfaces", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_RemoteAgentTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_card: std::option::Option<terrars::ListField<CesTool_RemoteAgentTool_AgentCard>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std::option::Option<terrars::ListField<CesTool_McpTool_ApiAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_agent: std::option::Option<terrars::PrimField<bool>>,
}

impl CesTool_RemoteAgentTool {
    #[doc = "Set the field `agent_card`.\n"]
    pub fn set_agent_card(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_RemoteAgentTool_AgentCard>>,
    ) -> Self {
        self.agent_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesTool_McpTool_ApiAuthentication>>,
    ) -> Self {
        self.api_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_variable_mapping`.\n"]
    pub fn set_input_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_variable_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_variable_mapping`.\n"]
    pub fn set_output_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_variable_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful_agent`.\n"]
    pub fn set_stateful_agent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.stateful_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_RemoteAgentTool {
    type O = terrars::BlockAssignable<CesTool_RemoteAgentTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_RemoteAgentTool {}

impl BuildCesTool_RemoteAgentTool {
    pub fn build(self) -> CesTool_RemoteAgentTool {
        CesTool_RemoteAgentTool {
            agent_card: core::default::Default::default(),
            api_authentication: core::default::Default::default(),
            description: core::default::Default::default(),
            input_variable_mapping: core::default::Default::default(),
            name: core::default::Default::default(),
            output_variable_mapping: core::default::Default::default(),
            stateful_agent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_RemoteAgentToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_RemoteAgentToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_RemoteAgentToolRef {
        CesTool_RemoteAgentToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_RemoteAgentToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_card` after provisioning.\n"]
    pub fn agent_card(&self) -> terrars::ListRef<CesTool_RemoteAgentTool_AgentCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_card", self.base))
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> terrars::ListRef<CesTool_McpTool_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `input_variable_mapping` after provisioning.\n"]
    pub fn input_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.input_variable_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_variable_mapping` after provisioning.\n"]
    pub fn output_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.output_variable_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_agent` after provisioning.\n"]
    pub fn stateful_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stateful_agent", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_SystemTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_SystemTool {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_SystemTool {
    type O = terrars::BlockAssignable<CesTool_SystemTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_SystemTool {}

impl BuildCesTool_SystemTool {
    pub fn build(self) -> CesTool_SystemTool {
        CesTool_SystemTool {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_SystemToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_SystemToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_SystemToolRef {
        CesTool_SystemToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_SystemToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_AgentTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl CesTool_AgentTool {
    #[doc =
        "Set the field `agent`.\nOptional. The resource name of the agent that is the entry point of the tool.\nFormat: projects/{project}/locations/{location}/agents/{agent}"]
    pub fn set_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. Description of the tool's purpose."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_AgentTool {
    type O = terrars::BlockAssignable<CesTool_AgentTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_AgentTool {
    #[doc = "Required. The name of the agent tool."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_AgentTool {
    pub fn build(self) -> CesTool_AgentTool {
        CesTool_AgentTool {
            agent: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_AgentToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_AgentToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_AgentToolRef {
        CesTool_AgentToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_AgentToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nOptional. The resource name of the agent that is the entry point of the tool.\nFormat: projects/{project}/locations/{location}/agents/{agent}"]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the tool's purpose."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The name of the agent tool."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_WidgetTool_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    defs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "enum", skip_serializing_if = "std::option::Option::is_none")]
    enum_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_items: std::option::Option<terrars::PrimField<bool>>,
}

impl CesTool_WidgetTool_Parameters {
    #[doc =
        "Set the field `additional_properties`.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn set_additional_properties(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `any_of`.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn set_any_of(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.any_of = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default`.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `defs`.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn set_defs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.defs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the data."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enum_`.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn set_enum(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enum_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\nSchema of the elements of Type.ARRAY."]
    pub fn set_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_max_items(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum`.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_items`.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_min_items(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum`.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nIndicates if the value may be null."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_items`.\nSchemas of initial elements of Type.ARRAY."]
    pub fn set_prefix_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\nProperties of Type.OBJECT."]
    pub fn set_properties(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ref_`.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\nRequired properties of Type.OBJECT."]
    pub fn set_required(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nThe title of the schema."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unique_items`.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn set_unique_items(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_WidgetTool_Parameters {
    type O = terrars::BlockAssignable<CesTool_WidgetTool_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_WidgetTool_Parameters {
    #[doc = "The type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_WidgetTool_Parameters {
    pub fn build(self) -> CesTool_WidgetTool_Parameters {
        CesTool_WidgetTool_Parameters {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            max_items: core::default::Default::default(),
            maximum: core::default::Default::default(),
            min_items: core::default::Default::default(),
            minimum: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: self.type_,
            unique_items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_WidgetTool_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_WidgetTool_ParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_WidgetTool_ParametersRef {
        CesTool_WidgetTool_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_WidgetTool_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_properties` after provisioning.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn additional_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `any_of` after provisioning.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn any_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `defs` after provisioning.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn defs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the data."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enum_` after provisioning.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn enum_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\nSchema of the elements of Type.ARRAY."]
    pub fn items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_items` after provisioning.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maximum` after provisioning.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_items` after provisioning.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn min_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum` after provisioning.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }

    #[doc = "Get a reference to the value of field `nullable` after provisioning.\nIndicates if the value may be null."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_items` after provisioning.\nSchemas of initial elements of Type.ARRAY."]
    pub fn prefix_items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\nProperties of Type.OBJECT."]
    pub fn properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\nRequired properties of Type.OBJECT."]
    pub fn required(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the schema."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unique_items` after provisioning.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn unique_items(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_ClientFunction_Response {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    defs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "enum", skip_serializing_if = "std::option::Option::is_none")]
    enum_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_items: std::option::Option<terrars::PrimField<bool>>,
}

impl CesTool_ClientFunction_Response {
    #[doc =
        "Set the field `additional_properties`.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn set_additional_properties(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `any_of`.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn set_any_of(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.any_of = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default`.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `defs`.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn set_defs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.defs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the data."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enum_`.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn set_enum(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enum_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\nSchema of the elements of Type.ARRAY."]
    pub fn set_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_max_items(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum`.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_items`.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_min_items(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum`.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nIndicates if the value may be null."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_items`.\nSchemas of initial elements of Type.ARRAY."]
    pub fn set_prefix_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\nProperties of Type.OBJECT."]
    pub fn set_properties(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ref_`.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\nRequired properties of Type.OBJECT."]
    pub fn set_required(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nThe title of the schema."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unique_items`.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn set_unique_items(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_ClientFunction_Response {
    type O = terrars::BlockAssignable<CesTool_ClientFunction_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_ClientFunction_Response {
    #[doc = "The type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_ClientFunction_Response {
    pub fn build(self) -> CesTool_ClientFunction_Response {
        CesTool_ClientFunction_Response {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            max_items: core::default::Default::default(),
            maximum: core::default::Default::default(),
            min_items: core::default::Default::default(),
            minimum: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: self.type_,
            unique_items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_ClientFunction_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_ClientFunction_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_ClientFunction_ResponseRef {
        CesTool_ClientFunction_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_ClientFunction_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_properties` after provisioning.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn additional_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `any_of` after provisioning.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn any_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `defs` after provisioning.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn defs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the data."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enum_` after provisioning.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn enum_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\nSchema of the elements of Type.ARRAY."]
    pub fn items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_items` after provisioning.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maximum` after provisioning.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_items` after provisioning.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn min_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_items", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum` after provisioning.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }

    #[doc = "Get a reference to the value of field `nullable` after provisioning.\nIndicates if the value may be null."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_items` after provisioning.\nSchemas of initial elements of Type.ARRAY."]
    pub fn prefix_items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\nProperties of Type.OBJECT."]
    pub fn properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\nRequired properties of Type.OBJECT."]
    pub fn required(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the schema."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unique_items` after provisioning.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn unique_items(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_ClientFunctionDynamic {
    parameters: std::option::Option<terrars::DynamicBlock<CesTool_WidgetTool_Parameters>>,
    response: std::option::Option<terrars::DynamicBlock<CesTool_ClientFunction_Response>>,
}

#[derive(Serialize)]
pub struct CesTool_ClientFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<CesTool_WidgetTool_Parameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<std::vec::Vec<CesTool_ClientFunction_Response>>,
    dynamic: CesTool_ClientFunctionDynamic,
}

impl CesTool_ClientFunction {
    #[doc = "Set the field `description`.\nThe function description."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_WidgetTool_Parameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_ClientFunction_Response>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_ClientFunction {
    type O = terrars::BlockAssignable<CesTool_ClientFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_ClientFunction {
    #[doc = "The function name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_ClientFunction {
    pub fn build(self) -> CesTool_ClientFunction {
        CesTool_ClientFunction {
            description: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_ClientFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_ClientFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_ClientFunctionRef {
        CesTool_ClientFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_ClientFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe function description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe function name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<CesTool_WidgetTool_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<CesTool_ClientFunction_ResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_amount: std::option::Option<terrars::PrimField<f64>>,
}

impl CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    #[doc =
        "Set the field `attribute_value`.\nCan be one of:\n1. The numerical field value.\n2. The duration spec for freshness:\nThe value must be formatted as an XSD 'dayTimeDuration' value (a\nrestricted subset of an ISO 8601 duration value). The pattern for\nthis is: 'nDnM]'."]
    pub fn set_attribute_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boost_amount`.\nThe value between -1 to 1 by which to boost the score if the\nattribute_value evaluates to the value specified above."]
    pub fn set_boost_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boost_amount = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    type O =
        terrars
        ::BlockAssignable<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {}

impl BuildCesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    pub fn build(self) -> CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
        CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
            attribute_value: core::default::Default::default(),
            boost_amount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
        CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nCan be one of:\n1. The numerical field value.\n2. The duration spec for freshness:\nThe value must be formatted as an XSD 'dayTimeDuration' value (a\nrestricted subset of an ISO 8601 duration value). The pattern for\nthis is: 'nDnM]'."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boost_amount` after provisioning.\nThe value between -1 to 1 by which to boost the score if the\nattribute_value evaluates to the value specified above."]
    pub fn boost_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boost_amount", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecDynamic {
    control_points: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints>,
    >,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpolation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_points: std
    ::option
    ::Option<std::vec::Vec<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints>>,
    dynamic: CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecDynamic,
}

impl CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    #[doc =
        "Set the field `attribute_type`.\nThe attribute type to be used to determine the boost amount. The\nattribute value can be derived from the field value of the specified\nfield_name. In the case of numerical it is straightforward i.e.\nattribute_value = numerical_field_value. In the case of freshness\nhowever, attribute_value = (time.now() - datetime_field_value).\nPossible values:\nNUMERICAL\nFRESHNESS"]
    pub fn set_attribute_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_name`.\nThe name of the field whose value will be used to determine the\nboost amount."]
    pub fn set_field_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interpolation_type`.\nThe interpolation type to be applied to connect the control points\nlisted below.\nPossible values:\nLINEAR"]
    pub fn set_interpolation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.interpolation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_points`.\n"]
    pub fn set_control_points(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.control_points = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.control_points = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {}

impl BuildCesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    pub fn build(self) -> CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
        CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
            attribute_type: core::default::Default::default(),
            field_name: core::default::Default::default(),
            interpolation_type: core::default::Default::default(),
            control_points: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
        CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_type` after provisioning.\nThe attribute type to be used to determine the boost amount. The\nattribute value can be derived from the field value of the specified\nfield_name. In the case of numerical it is straightforward i.e.\nattribute_value = numerical_field_value. In the case of freshness\nhowever, attribute_value = (time.now() - datetime_field_value).\nPossible values:\nNUMERICAL\nFRESHNESS"]
    pub fn attribute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_name` after provisioning.\nThe name of the field whose value will be used to determine the\nboost amount."]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpolation_type` after provisioning.\nThe interpolation type to be applied to connect the control points\nlisted below.\nPossible values:\nLINEAR"]
    pub fn interpolation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpolation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `control_points` after provisioning.\n"]
    pub fn control_points(
        &self,
    ) -> terrars
    ::ListRef<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_points", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsDynamic {
    boost_control_spec: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost: std::option::Option<terrars::PrimField<f64>>,
    condition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_control_spec: std
    ::option
    ::Option<std::vec::Vec<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec>>,
    dynamic: CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsDynamic,
}

impl CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    #[doc =
        "Set the field `boost`.\nStrength of the boost, which should be in [-1, 1]. Negative boost means\ndemotion. Default is 0.0.\nSetting to 1.0 gives the suggestions a big promotion. However, it does\nnot necessarily mean that the top result will be a boosted suggestion.\nSetting to -1.0 gives the suggestions a big demotion. However, other\nsuggestions that are relevant might still be shown.\nSetting to 0.0 means no boost applied. The boosting condition is\nignored."]
    pub fn set_boost(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boost_control_spec`.\n"]
    pub fn set_boost_control_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boost_control_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boost_control_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    #[doc =
        "An expression which specifies a boost condition. The syntax is the same\nas filter expression syntax. Currently, the only supported condition is\na list of BCP-47 lang codes.\nExample: To boost suggestions in languages en or fr:\n(lang_code: ANY(\"en\", \"fr\"))"]
    pub condition: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    pub fn build(self) -> CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
        CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
            boost: core::default::Default::default(),
            condition: self.condition,
            boost_control_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
        CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boost` after provisioning.\nStrength of the boost, which should be in [-1, 1]. Negative boost means\ndemotion. Default is 0.0.\nSetting to 1.0 gives the suggestions a big promotion. However, it does\nnot necessarily mean that the top result will be a boosted suggestion.\nSetting to -1.0 gives the suggestions a big demotion. However, other\nsuggestions that are relevant might still be shown.\nSetting to 0.0 means no boost applied. The boosting condition is\nignored."]
    pub fn boost(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boost", self.base))
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nAn expression which specifies a boost condition. The syntax is the same\nas filter expression syntax. Currently, the only supported condition is\na list of BCP-47 lang codes.\nExample: To boost suggestions in languages en or fr:\n(lang_code: ANY(\"en\", \"fr\"))"]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `boost_control_spec` after provisioning.\n"]
    pub fn boost_control_spec(
        &self,
    ) -> terrars::ListRef<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_control_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_BoostSpecs_SpecDynamic {
    condition_boost_specs: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_BoostSpecs_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_boost_specs: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs>>,
    dynamic: CesTool_DataStoreTool_BoostSpecs_SpecDynamic,
}

impl CesTool_DataStoreTool_BoostSpecs_Spec {
    #[doc = "Set the field `condition_boost_specs`.\n"]
    pub fn set_condition_boost_specs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_boost_specs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_boost_specs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_BoostSpecs_Spec {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_BoostSpecs_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_BoostSpecs_Spec {}

impl BuildCesTool_DataStoreTool_BoostSpecs_Spec {
    pub fn build(self) -> CesTool_DataStoreTool_BoostSpecs_Spec {
        CesTool_DataStoreTool_BoostSpecs_Spec {
            condition_boost_specs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_BoostSpecs_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_BoostSpecs_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_DataStoreTool_BoostSpecs_SpecRef {
        CesTool_DataStoreTool_BoostSpecs_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_BoostSpecs_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_boost_specs` after provisioning.\n"]
    pub fn condition_boost_specs(
        &self,
    ) -> terrars::ListRef<CesTool_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_boost_specs", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_BoostSpecsDynamic {
    spec: std::option::Option<terrars::DynamicBlock<CesTool_DataStoreTool_BoostSpecs_Spec>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_BoostSpecs {
    data_stores: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_BoostSpecs_Spec>>,
    dynamic: CesTool_DataStoreTool_BoostSpecsDynamic,
}

impl CesTool_DataStoreTool_BoostSpecs {
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_BoostSpecs_Spec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_BoostSpecs {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_BoostSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_BoostSpecs {
    #[doc =
        "The Data Store where the boosting configuration is applied. Full resource\nname of DataStore, such as\nprojects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}."]
    pub data_stores: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCesTool_DataStoreTool_BoostSpecs {
    pub fn build(self) -> CesTool_DataStoreTool_BoostSpecs {
        CesTool_DataStoreTool_BoostSpecs {
            data_stores: self.data_stores,
            spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_BoostSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_BoostSpecsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_DataStoreTool_BoostSpecsRef {
        CesTool_DataStoreTool_BoostSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_BoostSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_stores` after provisioning.\nThe Data Store where the boosting configuration is applied. Full resource\nname of DataStore, such as\nprojects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}."]
    pub fn data_stores(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_stores", self.base))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<CesTool_DataStoreTool_BoostSpecs_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig {
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collection_display_name`.\n"]
    pub fn set_collection_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.collection_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig {}

impl BuildCesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig {
    pub fn build(self) -> CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig {
        CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfig {
            collection: core::default::Default::default(),
            collection_display_name: core::default::Default::default(),
            data_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfigRef {
        CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `collection_display_name` after provisioning.\n"]
    pub fn collection_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_DataStoreSource_DataStore {
    name: terrars::PrimField<std::string::String>,
}

impl CesTool_DataStoreTool_DataStoreSource_DataStore { }

impl terrars::ToListMappable for CesTool_DataStoreTool_DataStoreSource_DataStore {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_DataStoreSource_DataStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_DataStoreSource_DataStore {
    #[doc =
        "Full resource name of the DataStore.\nFormat: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_DataStoreTool_DataStoreSource_DataStore {
    pub fn build(self) -> CesTool_DataStoreTool_DataStoreSource_DataStore {
        CesTool_DataStoreTool_DataStoreSource_DataStore { name: self.name }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_DataStoreSource_DataStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_DataStoreSource_DataStoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_DataStoreSource_DataStoreRef {
        CesTool_DataStoreTool_DataStoreSource_DataStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_DataStoreSource_DataStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connector_config` after provisioning.\nThe connector config for the data store connection."]
    pub fn connector_config(
        &self,
    ) -> terrars::ListRef<CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the data store was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the data store."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `document_processing_mode` after provisioning.\nThe document processing mode for the data store connection.\nOnly set for PUBLIC_WEB and UNSTRUCTURED data stores.\nPossible values:\nDOCUMENTS\nCHUNKS"]
    pub fn document_processing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_processing_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull resource name of the DataStore.\nFormat: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the data store. This field is readonly and populated by the\nserver.\nPossible values:\nPUBLIC_WEB\nUNSTRUCTURED\nFAQ\nCONNECTOR"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_DataStoreSourceDynamic {
    data_store: std::option::Option<terrars::DynamicBlock<CesTool_DataStoreTool_DataStoreSource_DataStore>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_DataStoreSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_DataStoreSource_DataStore>>,
    dynamic: CesTool_DataStoreTool_DataStoreSourceDynamic,
}

impl CesTool_DataStoreTool_DataStoreSource {
    #[doc =
        "Set the field `filter`.\nOptional. Filter specification for the DataStore.\nSee: https://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store`.\n"]
    pub fn set_data_store(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_DataStoreSource_DataStore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_store = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_DataStoreSource {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_DataStoreSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_DataStoreSource {}

impl BuildCesTool_DataStoreTool_DataStoreSource {
    pub fn build(self) -> CesTool_DataStoreTool_DataStoreSource {
        CesTool_DataStoreTool_DataStoreSource {
            filter: core::default::Default::default(),
            data_store: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_DataStoreSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_DataStoreSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_DataStoreTool_DataStoreSourceRef {
        CesTool_DataStoreTool_DataStoreSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_DataStoreSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nOptional. Filter specification for the DataStore.\nSee: https://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store` after provisioning.\n"]
    pub fn data_store(&self) -> terrars::ListRef<CesTool_DataStoreTool_DataStoreSource_DataStoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    name: terrars::PrimField<std::string::String>,
}

impl CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore { }

impl terrars::ToListMappable for CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    #[doc =
        "Full resource name of the DataStore.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    pub fn build(self) -> CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore {
        CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore { name: self.name }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
        CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connector_config` after provisioning.\nThe connector config for the data store connection."]
    pub fn connector_config(
        &self,
    ) -> terrars::ListRef<CesTool_DataStoreTool_DataStoreSource_DataStore_ConnectorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the data store was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the data store."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `document_processing_mode` after provisioning.\nThe document processing mode for the data store connection.\nOnly set for PUBLIC_WEB and UNSTRUCTURED data stores.\nPossible values:\nDOCUMENTS\nCHUNKS"]
    pub fn document_processing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_processing_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull resource name of the DataStore.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the data store. This field is readonly and populated by the\nserver.\nPossible values:\nPUBLIC_WEB\nUNSTRUCTURED\nFAQ\nCONNECTOR"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_EngineSource_DataStoreSourcesDynamic {
    data_store: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_EngineSource_DataStoreSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore>>,
    dynamic: CesTool_DataStoreTool_EngineSource_DataStoreSourcesDynamic,
}

impl CesTool_DataStoreTool_EngineSource_DataStoreSources {
    #[doc =
        "Set the field `filter`.\nFilter specification for the DataStore.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store`.\n"]
    pub fn set_data_store(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_store = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_EngineSource_DataStoreSources {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_EngineSource_DataStoreSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_EngineSource_DataStoreSources {}

impl BuildCesTool_DataStoreTool_EngineSource_DataStoreSources {
    pub fn build(self) -> CesTool_DataStoreTool_EngineSource_DataStoreSources {
        CesTool_DataStoreTool_EngineSource_DataStoreSources {
            filter: core::default::Default::default(),
            data_store: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_EngineSource_DataStoreSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_EngineSource_DataStoreSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_EngineSource_DataStoreSourcesRef {
        CesTool_DataStoreTool_EngineSource_DataStoreSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_EngineSource_DataStoreSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter specification for the DataStore.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store` after provisioning.\n"]
    pub fn data_store(&self) -> terrars::ListRef<CesTool_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_EngineSourceDynamic {
    data_store_sources: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_EngineSource_DataStoreSources>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_EngineSource {
    engine: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_sources: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_EngineSource_DataStoreSources>>,
    dynamic: CesTool_DataStoreTool_EngineSourceDynamic,
}

impl CesTool_DataStoreTool_EngineSource {
    #[doc =
        "Set the field `filter`.\nA filter applied to the search across the Engine. Not relevant and not\nused if 'data_store_sources' is provided.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store_sources`.\n"]
    pub fn set_data_store_sources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_EngineSource_DataStoreSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_store_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_EngineSource {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_EngineSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_EngineSource {
    #[doc =
        "Full resource name of the Engine.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}'"]
    pub engine: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_DataStoreTool_EngineSource {
    pub fn build(self) -> CesTool_DataStoreTool_EngineSource {
        CesTool_DataStoreTool_EngineSource {
            engine: self.engine,
            filter: core::default::Default::default(),
            data_store_sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_EngineSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_EngineSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_DataStoreTool_EngineSourceRef {
        CesTool_DataStoreTool_EngineSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_EngineSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `engine` after provisioning.\nFull resource name of the Engine.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}'"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA filter applied to the search across the Engine. Not relevant and not\nused if 'data_store_sources' is provided.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store_sources` after provisioning.\n"]
    pub fn data_store_sources(&self) -> terrars::ListRef<CesTool_DataStoreTool_EngineSource_DataStoreSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_sources", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_ModalityConfigs_GroundingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grounding_level: std::option::Option<terrars::PrimField<f64>>,
}

impl CesTool_DataStoreTool_ModalityConfigs_GroundingConfig {
    #[doc = "Set the field `disabled`.\nWhether grounding is disabled."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `grounding_level`.\nThe groundedness threshold of the answer based on the retrieved sources.\nThe value has a configurable range of [1, 5]. The level is used to\nthreshold the groundedness of the answer, meaning that all responses with\na groundedness score below the threshold will fall back to returning\nrelevant snippets only.\nFor example, a level of 3 means that the groundedness score must be\n3 or higher for the response to be returned."]
    pub fn set_grounding_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.grounding_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_ModalityConfigs_GroundingConfig {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_GroundingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_ModalityConfigs_GroundingConfig {}

impl BuildCesTool_DataStoreTool_ModalityConfigs_GroundingConfig {
    pub fn build(self) -> CesTool_DataStoreTool_ModalityConfigs_GroundingConfig {
        CesTool_DataStoreTool_ModalityConfigs_GroundingConfig {
            disabled: core::default::Default::default(),
            grounding_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_ModalityConfigs_GroundingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_ModalityConfigs_GroundingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_ModalityConfigs_GroundingConfigRef {
        CesTool_DataStoreTool_ModalityConfigs_GroundingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_ModalityConfigs_GroundingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether grounding is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `grounding_level` after provisioning.\nThe groundedness threshold of the answer based on the retrieved sources.\nThe value has a configurable range of [1, 5]. The level is used to\nthreshold the groundedness of the answer, meaning that all responses with\na groundedness score below the threshold will fall back to returning\nrelevant snippets only.\nFor example, a level of 3 means that the groundedness score must be\n3 or higher for the response to be returned."]
    pub fn grounding_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grounding_level", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thinking_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings {
    #[doc =
        "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `thinking_level`.\nThe thinking level of the model.\nPossible values:\nDEFAULT\nLOW\nMEDIUM\nHIGH Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn set_thinking_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thinking_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings {}

impl BuildCesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings {
    pub fn build(self) -> CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings {
        CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
            thinking_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettingsRef {
        CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thinking_level` after provisioning.\nThe thinking level of the model.\nPossible values:\nDEFAULT\nLOW\nMEDIUM\nHIGH Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn thinking_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thinking_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_ModalityConfigs_RewriterConfigDynamic {
    model_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_ModalityConfigs_RewriterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std
    ::option
    ::Option<std::vec::Vec<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings>>,
    dynamic: CesTool_DataStoreTool_ModalityConfigs_RewriterConfigDynamic,
}

impl CesTool_DataStoreTool_ModalityConfigs_RewriterConfig {
    #[doc = "Set the field `disabled`.\nWhether the rewriter is disabled."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt`.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_ModalityConfigs_RewriterConfig {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_ModalityConfigs_RewriterConfig {}

impl BuildCesTool_DataStoreTool_ModalityConfigs_RewriterConfig {
    pub fn build(self) -> CesTool_DataStoreTool_ModalityConfigs_RewriterConfig {
        CesTool_DataStoreTool_ModalityConfigs_RewriterConfig {
            disabled: core::default::Default::default(),
            prompt: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_ModalityConfigs_RewriterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_ModalityConfigs_RewriterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_ModalityConfigs_RewriterConfigRef {
        CesTool_DataStoreTool_ModalityConfigs_RewriterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_ModalityConfigs_RewriterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the rewriter is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prompt` after provisioning.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(
        &self,
    ) -> terrars::ListRef<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_snippets: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_snippets: std::option::Option<terrars::PrimField<f64>>,
}

impl CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig {
    #[doc = "Set the field `enable_snippets`.\nWhether snippets are enabled."]
    pub fn set_enable_snippets(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_snippets = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_snippets`.\nNumber of snippets to return per query.\nIf unset, returns all snippets from the service by default."]
    pub fn set_max_snippets(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_snippets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_ModalityConfigs_SnippetsConfig {}

impl BuildCesTool_DataStoreTool_ModalityConfigs_SnippetsConfig {
    pub fn build(self) -> CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig {
        CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig {
            enable_snippets: core::default::Default::default(),
            max_snippets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_ModalityConfigs_SnippetsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_ModalityConfigs_SnippetsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_ModalityConfigs_SnippetsConfigRef {
        CesTool_DataStoreTool_ModalityConfigs_SnippetsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_ModalityConfigs_SnippetsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_snippets` after provisioning.\nWhether snippets are enabled."]
    pub fn enable_snippets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_snippets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_snippets` after provisioning.\nNumber of snippets to return per query.\nIf unset, returns all snippets from the service by default."]
    pub fn max_snippets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_snippets", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigDynamic {
    model_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std
    ::option
    ::Option<std::vec::Vec<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings>>,
    dynamic: CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigDynamic,
}

impl CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig {
    #[doc = "Set the field `disabled`.\nWhether summarization is disabled."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt`.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_ModalityConfigs_SummarizationConfig {}

impl BuildCesTool_DataStoreTool_ModalityConfigs_SummarizationConfig {
    pub fn build(self) -> CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig {
        CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig {
            disabled: core::default::Default::default(),
            prompt: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
        CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether summarization is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prompt` after provisioning.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(
        &self,
    ) -> terrars::ListRef<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreTool_ModalityConfigsDynamic {
    grounding_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_ModalityConfigs_GroundingConfig>>,
    rewriter_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig>>,
    snippets_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig>>,
    summarization_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool_ModalityConfigs {
    modality_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grounding_config: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_ModalityConfigs_GroundingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewriter_config: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snippets_config: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_config: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig>>,
    dynamic: CesTool_DataStoreTool_ModalityConfigsDynamic,
}

impl CesTool_DataStoreTool_ModalityConfigs {
    #[doc = "Set the field `grounding_config`.\n"]
    pub fn set_grounding_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_GroundingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grounding_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grounding_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rewriter_config`.\n"]
    pub fn set_rewriter_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_RewriterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rewriter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rewriter_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snippets_config`.\n"]
    pub fn set_snippets_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_SnippetsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snippets_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snippets_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `summarization_config`.\n"]
    pub fn set_summarization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs_SummarizationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summarization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool_ModalityConfigs {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool_ModalityConfigs {
    #[doc = "The modality type.\nPossible values:\nTEXT\nAUDIO"]
    pub modality_type: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_DataStoreTool_ModalityConfigs {
    pub fn build(self) -> CesTool_DataStoreTool_ModalityConfigs {
        CesTool_DataStoreTool_ModalityConfigs {
            modality_type: self.modality_type,
            grounding_config: core::default::Default::default(),
            rewriter_config: core::default::Default::default(),
            snippets_config: core::default::Default::default(),
            summarization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreTool_ModalityConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreTool_ModalityConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_DataStoreTool_ModalityConfigsRef {
        CesTool_DataStoreTool_ModalityConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreTool_ModalityConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `modality_type` after provisioning.\nThe modality type.\nPossible values:\nTEXT\nAUDIO"]
    pub fn modality_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modality_type", self.base))
    }

    #[doc = "Get a reference to the value of field `grounding_config` after provisioning.\n"]
    pub fn grounding_config(&self) -> terrars::ListRef<CesTool_DataStoreTool_ModalityConfigs_GroundingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grounding_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rewriter_config` after provisioning.\n"]
    pub fn rewriter_config(&self) -> terrars::ListRef<CesTool_DataStoreTool_ModalityConfigs_RewriterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewriter_config", self.base))
    }

    #[doc = "Get a reference to the value of field `snippets_config` after provisioning.\n"]
    pub fn snippets_config(&self) -> terrars::ListRef<CesTool_DataStoreTool_ModalityConfigs_SnippetsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snippets_config", self.base))
    }

    #[doc = "Get a reference to the value of field `summarization_config` after provisioning.\n"]
    pub fn summarization_config(&self) -> terrars::ListRef<CesTool_DataStoreTool_ModalityConfigs_SummarizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_DataStoreToolDynamic {
    boost_specs: std::option::Option<terrars::DynamicBlock<CesTool_DataStoreTool_BoostSpecs>>,
    data_store_source: std::option::Option<terrars::DynamicBlock<CesTool_DataStoreTool_DataStoreSource>>,
    engine_source: std::option::Option<terrars::DynamicBlock<CesTool_DataStoreTool_EngineSource>>,
    modality_configs: std::option::Option<terrars::DynamicBlock<CesTool_DataStoreTool_ModalityConfigs>>,
}

#[derive(Serialize)]
pub struct CesTool_DataStoreTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_parameter_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_results: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_specs: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_BoostSpecs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_source: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_DataStoreSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_source: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_EngineSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modality_configs: std::option::Option<std::vec::Vec<CesTool_DataStoreTool_ModalityConfigs>>,
    dynamic: CesTool_DataStoreToolDynamic,
}

impl CesTool_DataStoreTool {
    #[doc = "Set the field `description`.\nThe tool description."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter_parameter_behavior`.\nOptional. The filter parameter behavior.\nPossible values:\nFILTER_PARAMETER_BEHAVIOR_UNSPECIFIED\nALWAYS_INCLUDE\nNEVER_INCLUDE Possible values: [\"FILTER_PARAMETER_BEHAVIOR_UNSPECIFIED\", \"ALWAYS_INCLUDE\", \"NEVER_INCLUDE\"]"]
    pub fn set_filter_parameter_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_parameter_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_results`.\nNumber of search results to return per query.\nThe default value is 10. The maximum allowed value is 10."]
    pub fn set_max_results(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boost_specs`.\n"]
    pub fn set_boost_specs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_BoostSpecs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boost_specs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boost_specs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_store_source`.\n"]
    pub fn set_data_store_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_DataStoreSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_store_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `engine_source`.\n"]
    pub fn set_engine_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_EngineSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.engine_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.engine_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `modality_configs`.\n"]
    pub fn set_modality_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_DataStoreTool_ModalityConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.modality_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.modality_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_DataStoreTool {
    type O = terrars::BlockAssignable<CesTool_DataStoreTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_DataStoreTool {
    #[doc = "The data store tool name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_DataStoreTool {
    pub fn build(self) -> CesTool_DataStoreTool {
        CesTool_DataStoreTool {
            description: core::default::Default::default(),
            filter_parameter_behavior: core::default::Default::default(),
            max_results: core::default::Default::default(),
            name: self.name,
            boost_specs: core::default::Default::default(),
            data_store_source: core::default::Default::default(),
            engine_source: core::default::Default::default(),
            modality_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_DataStoreToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_DataStoreToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_DataStoreToolRef {
        CesTool_DataStoreToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_DataStoreToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe tool description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_parameter_behavior` after provisioning.\nOptional. The filter parameter behavior.\nPossible values:\nFILTER_PARAMETER_BEHAVIOR_UNSPECIFIED\nALWAYS_INCLUDE\nNEVER_INCLUDE Possible values: [\"FILTER_PARAMETER_BEHAVIOR_UNSPECIFIED\", \"ALWAYS_INCLUDE\", \"NEVER_INCLUDE\"]"]
    pub fn filter_parameter_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_parameter_behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_results` after provisioning.\nNumber of search results to return per query.\nThe default value is 10. The maximum allowed value is 10."]
    pub fn max_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_results", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe data store tool name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `boost_specs` after provisioning.\n"]
    pub fn boost_specs(&self) -> terrars::ListRef<CesTool_DataStoreTool_BoostSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_specs", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store_source` after provisioning.\n"]
    pub fn data_store_source(&self) -> terrars::ListRef<CesTool_DataStoreTool_DataStoreSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_source", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_source` after provisioning.\n"]
    pub fn engine_source(&self) -> terrars::ListRef<CesTool_DataStoreTool_EngineSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engine_source", self.base))
    }

    #[doc = "Get a reference to the value of field `modality_configs` after provisioning.\n"]
    pub fn modality_configs(&self) -> terrars::ListRef<CesTool_DataStoreTool_ModalityConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.modality_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_FileSearchTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    corpus_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_corpus: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl CesTool_FileSearchTool {
    #[doc =
        "Set the field `corpus_type`.\nOptional. The type of the corpus. Default is FULLY_MANAGED.\nPossible values:\nCORPUS_TYPE_UNSPECIFIED\nUSER_OWNED\nFULLY_MANAGED Possible values: [\"CORPUS_TYPE_UNSPECIFIED\", \"USER_OWNED\", \"FULLY_MANAGED\"]"]
    pub fn set_corpus_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.corpus_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. The tool description."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_corpus`.\nOptional. The corpus where files are stored.\nFormat: projects/{project}/locations/{location}/ragCorpora/{rag_corpus}"]
    pub fn set_file_corpus(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_corpus = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_FileSearchTool {
    type O = terrars::BlockAssignable<CesTool_FileSearchTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_FileSearchTool {
    #[doc = "Required. The tool name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_FileSearchTool {
    pub fn build(self) -> CesTool_FileSearchTool {
        CesTool_FileSearchTool {
            corpus_type: core::default::Default::default(),
            description: core::default::Default::default(),
            file_corpus: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_FileSearchToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_FileSearchToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_FileSearchToolRef {
        CesTool_FileSearchToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_FileSearchToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `corpus_type` after provisioning.\nOptional. The type of the corpus. Default is FULLY_MANAGED.\nPossible values:\nCORPUS_TYPE_UNSPECIFIED\nUSER_OWNED\nFULLY_MANAGED Possible values: [\"CORPUS_TYPE_UNSPECIFIED\", \"USER_OWNED\", \"FULLY_MANAGED\"]"]
    pub fn corpus_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.corpus_type", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. The tool description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_corpus` after provisioning.\nOptional. The corpus where files are stored.\nFormat: projects/{project}/locations/{location}/ragCorpora/{rag_corpus}"]
    pub fn file_corpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_corpus", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The tool name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_GoogleSearchTool_PromptConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_GoogleSearchTool_PromptConfig {
    #[doc =
        "Set the field `text_prompt`.\nOptional. Defines the prompt used for the system instructions when interacting with the\nagent in chat conversations. If not set, default prompt will be used."]
    pub fn set_text_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `voice_prompt`.\nOptional. Defines the prompt used for the system instructions when interacting with the\nagent in voice conversations. If not set, default prompt will be used."]
    pub fn set_voice_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.voice_prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_GoogleSearchTool_PromptConfig {
    type O = terrars::BlockAssignable<CesTool_GoogleSearchTool_PromptConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_GoogleSearchTool_PromptConfig {}

impl BuildCesTool_GoogleSearchTool_PromptConfig {
    pub fn build(self) -> CesTool_GoogleSearchTool_PromptConfig {
        CesTool_GoogleSearchTool_PromptConfig {
            text_prompt: core::default::Default::default(),
            voice_prompt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_GoogleSearchTool_PromptConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_GoogleSearchTool_PromptConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_GoogleSearchTool_PromptConfigRef {
        CesTool_GoogleSearchTool_PromptConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_GoogleSearchTool_PromptConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `text_prompt` after provisioning.\nOptional. Defines the prompt used for the system instructions when interacting with the\nagent in chat conversations. If not set, default prompt will be used."]
    pub fn text_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_prompt", self.base))
    }

    #[doc =
        "Get a reference to the value of field `voice_prompt` after provisioning.\nOptional. Defines the prompt used for the system instructions when interacting with the\nagent in voice conversations. If not set, default prompt will be used."]
    pub fn voice_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice_prompt", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_GoogleSearchToolDynamic {
    prompt_config: std::option::Option<terrars::DynamicBlock<CesTool_GoogleSearchTool_PromptConfig>>,
}

#[derive(Serialize)]
pub struct CesTool_GoogleSearchTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_config: std::option::Option<std::vec::Vec<CesTool_GoogleSearchTool_PromptConfig>>,
    dynamic: CesTool_GoogleSearchToolDynamic,
}

impl CesTool_GoogleSearchTool {
    #[doc =
        "Set the field `context_urls`.\nContent will be fetched directly from these URLs for context and grounding.\nMore details: https://cloud.google.com/vertex-ai/generative-ai/docs/url-context.\nExample: \"https://example.com/path.html\". A maximum of 20 URLs are allowed."]
    pub fn set_context_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.context_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the tool's purpose."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude_domains`.\nList of domains to be excluded from the search results.\nExample: \"example.com\".\nA maximum of 2000 domains can be excluded."]
    pub fn set_exclude_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preferred_domains`.\nSpecifies domain names to guide the search.\nThe model will be instructed to prioritize these domains\nwhen formulating queries for google search.\nThis is a best-effort hint and these domains may or may\nnot be exclusively reflected in the final search results.\nExample: \"example.com\", \"another.site\".\nA maximum of 20 domains can be specified."]
    pub fn set_preferred_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.preferred_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_config`.\n"]
    pub fn set_prompt_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_GoogleSearchTool_PromptConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prompt_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_GoogleSearchTool {
    type O = terrars::BlockAssignable<CesTool_GoogleSearchTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_GoogleSearchTool {
    #[doc = "The name of the tool."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_GoogleSearchTool {
    pub fn build(self) -> CesTool_GoogleSearchTool {
        CesTool_GoogleSearchTool {
            context_urls: core::default::Default::default(),
            description: core::default::Default::default(),
            exclude_domains: core::default::Default::default(),
            name: self.name,
            preferred_domains: core::default::Default::default(),
            prompt_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_GoogleSearchToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_GoogleSearchToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_GoogleSearchToolRef {
        CesTool_GoogleSearchToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_GoogleSearchToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `context_urls` after provisioning.\nContent will be fetched directly from these URLs for context and grounding.\nMore details: https://cloud.google.com/vertex-ai/generative-ai/docs/url-context.\nExample: \"https://example.com/path.html\". A maximum of 20 URLs are allowed."]
    pub fn context_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.context_urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the tool's purpose."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_domains` after provisioning.\nList of domains to be excluded from the search results.\nExample: \"example.com\".\nA maximum of 2000 domains can be excluded."]
    pub fn exclude_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_domains", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the tool."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preferred_domains` after provisioning.\nSpecifies domain names to guide the search.\nThe model will be instructed to prioritize these domains\nwhen formulating queries for google search.\nThis is a best-effort hint and these domains may or may\nnot be exclusively reflected in the final search results.\nExample: \"example.com\", \"another.site\".\nA maximum of 20 domains can be specified."]
    pub fn preferred_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_domains", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_config` after provisioning.\n"]
    pub fn prompt_config(&self) -> terrars::ListRef<CesTool_GoogleSearchTool_PromptConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_PythonFunction_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl CesTool_PythonFunction_ServiceDirectoryConfig { }

impl terrars::ToListMappable for CesTool_PythonFunction_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<CesTool_PythonFunction_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_PythonFunction_ServiceDirectoryConfig {
    #[doc =
        "The name of Service Directory service.\nFormat: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}"]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_PythonFunction_ServiceDirectoryConfig {
    pub fn build(self) -> CesTool_PythonFunction_ServiceDirectoryConfig {
        CesTool_PythonFunction_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct CesTool_PythonFunction_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_PythonFunction_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_PythonFunction_ServiceDirectoryConfigRef {
        CesTool_PythonFunction_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_PythonFunction_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of Service Directory service.\nFormat: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_PythonFunctionDynamic {
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesTool_PythonFunction_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct CesTool_PythonFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std::option::Option<std::vec::Vec<CesTool_PythonFunction_ServiceDirectoryConfig>>,
    dynamic: CesTool_PythonFunctionDynamic,
}

impl CesTool_PythonFunction {
    #[doc =
        "Set the field `name`.\nThe name of the Python function to execute. Must match a Python function\nname defined in the python code. Case sensitive. If the name is not\nprovided, the first function defined in the python code will be used."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\nThe Python code to execute for the tool."]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_PythonFunction_ServiceDirectoryConfig>>,
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
}

impl terrars::ToListMappable for CesTool_PythonFunction {
    type O = terrars::BlockAssignable<CesTool_PythonFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_PythonFunction {}

impl BuildCesTool_PythonFunction {
    pub fn build(self) -> CesTool_PythonFunction {
        CesTool_PythonFunction {
            name: core::default::Default::default(),
            python_code: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_PythonFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_PythonFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_PythonFunctionRef {
        CesTool_PythonFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_PythonFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the Python function, parsed from the python code's\ndocstring."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Python function to execute. Must match a Python function\nname defined in the python code. Case sensitive. If the name is not\nprovided, the first function defined in the python code will be used."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe Python code to execute for the tool."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(&self) -> terrars::ListRef<CesTool_PythonFunction_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_Timeouts {
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

impl terrars::ToListMappable for CesTool_Timeouts {
    type O = terrars::BlockAssignable<CesTool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_Timeouts {}

impl BuildCesTool_Timeouts {
    pub fn build(self) -> CesTool_Timeouts {
        CesTool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_TimeoutsRef {
        CesTool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_TimeoutsRef {
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

#[derive(Serialize)]
pub struct CesTool_ToolFakeConfig_CodeBlock {
    python_code: terrars::PrimField<std::string::String>,
}

impl CesTool_ToolFakeConfig_CodeBlock { }

impl terrars::ToListMappable for CesTool_ToolFakeConfig_CodeBlock {
    type O = terrars::BlockAssignable<CesTool_ToolFakeConfig_CodeBlock>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_ToolFakeConfig_CodeBlock {
    #[doc = "Python code which will be invoked in tool fake mode."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_ToolFakeConfig_CodeBlock {
    pub fn build(self) -> CesTool_ToolFakeConfig_CodeBlock {
        CesTool_ToolFakeConfig_CodeBlock { python_code: self.python_code }
    }
}

#[allow(dead_code)]
pub struct CesTool_ToolFakeConfig_CodeBlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_ToolFakeConfig_CodeBlockRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_ToolFakeConfig_CodeBlockRef {
        CesTool_ToolFakeConfig_CodeBlockRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_ToolFakeConfig_CodeBlockRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nPython code which will be invoked in tool fake mode."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_ToolFakeConfigDynamic {
    code_block: std::option::Option<terrars::DynamicBlock<CesTool_ToolFakeConfig_CodeBlock>>,
}

#[derive(Serialize)]
pub struct CesTool_ToolFakeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_fake_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_block: std::option::Option<std::vec::Vec<CesTool_ToolFakeConfig_CodeBlock>>,
    dynamic: CesTool_ToolFakeConfigDynamic,
}

impl CesTool_ToolFakeConfig {
    #[doc = "Set the field `enable_fake_mode`.\nWhether the tool is using fake mode."]
    pub fn set_enable_fake_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_fake_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_block`.\n"]
    pub fn set_code_block(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_ToolFakeConfig_CodeBlock>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_block = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_block = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_ToolFakeConfig {
    type O = terrars::BlockAssignable<CesTool_ToolFakeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_ToolFakeConfig {}

impl BuildCesTool_ToolFakeConfig {
    pub fn build(self) -> CesTool_ToolFakeConfig {
        CesTool_ToolFakeConfig {
            enable_fake_mode: core::default::Default::default(),
            code_block: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_ToolFakeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_ToolFakeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_ToolFakeConfigRef {
        CesTool_ToolFakeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_ToolFakeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_fake_mode` after provisioning.\nWhether the tool is using fake mode."]
    pub fn enable_fake_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_fake_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `code_block` after provisioning.\n"]
    pub fn code_block(&self) -> terrars::ListRef<CesTool_ToolFakeConfig_CodeBlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_block", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_WidgetTool_DataMapping_PythonFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_WidgetTool_DataMapping_PythonFunction {
    #[doc =
        "Set the field `name`.\nOptional. The name of the Python function to execute. Must match a Python function\nname defined in the python code. Case sensitive. If the name is not\nprovided, the first function defined in the python code will be used."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\nOptional. The Python code to execute for the tool."]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_WidgetTool_DataMapping_PythonFunction {
    type O = terrars::BlockAssignable<CesTool_WidgetTool_DataMapping_PythonFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_WidgetTool_DataMapping_PythonFunction {}

impl BuildCesTool_WidgetTool_DataMapping_PythonFunction {
    pub fn build(self) -> CesTool_WidgetTool_DataMapping_PythonFunction {
        CesTool_WidgetTool_DataMapping_PythonFunction {
            name: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_WidgetTool_DataMapping_PythonFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_WidgetTool_DataMapping_PythonFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesTool_WidgetTool_DataMapping_PythonFunctionRef {
        CesTool_WidgetTool_DataMapping_PythonFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_WidgetTool_DataMapping_PythonFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the Python function, parsed from the python code's\ndocstring."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOptional. The name of the Python function to execute. Must match a Python function\nname defined in the python code. Case sensitive. If the name is not\nprovided, the first function defined in the python code will be used."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nOptional. The Python code to execute for the tool."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_WidgetTool_DataMappingDynamic {
    python_function: std::option::Option<terrars::DynamicBlock<CesTool_WidgetTool_DataMapping_PythonFunction>>,
}

#[derive(Serialize)]
pub struct CesTool_WidgetTool_DataMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mappings: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_tool_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_function: std::option::Option<std::vec::Vec<CesTool_WidgetTool_DataMapping_PythonFunction>>,
    dynamic: CesTool_WidgetTool_DataMappingDynamic,
}

impl CesTool_WidgetTool_DataMapping {
    #[doc =
        "Set the field `field_mappings`.\nOptional. A map of widget input parameter fields to the corresponding output fields of the source tool.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_field_mappings(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.field_mappings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nOptional. The mode of the data mapping.\nPossible values:\nMODE_UNSPECIFIED\nFIELD_MAPPING\nPYTHON_SCRIPT Possible values: [\"MODE_UNSPECIFIED\", \"FIELD_MAPPING\", \"PYTHON_SCRIPT\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_tool_name`.\nOptional. The resource name of the tool that provides the data for the widget (e.g., a search tool or a custom function).\nFormat: projects/{project}/locations/{location}/agents/{agent}/tools/{tool}"]
    pub fn set_source_tool_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_tool_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_function`.\n"]
    pub fn set_python_function(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_WidgetTool_DataMapping_PythonFunction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.python_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.python_function = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_WidgetTool_DataMapping {
    type O = terrars::BlockAssignable<CesTool_WidgetTool_DataMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_WidgetTool_DataMapping {}

impl BuildCesTool_WidgetTool_DataMapping {
    pub fn build(self) -> CesTool_WidgetTool_DataMapping {
        CesTool_WidgetTool_DataMapping {
            field_mappings: core::default::Default::default(),
            mode: core::default::Default::default(),
            source_tool_name: core::default::Default::default(),
            python_function: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_WidgetTool_DataMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_WidgetTool_DataMappingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_WidgetTool_DataMappingRef {
        CesTool_WidgetTool_DataMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_WidgetTool_DataMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field_mappings` after provisioning.\nOptional. A map of widget input parameter fields to the corresponding output fields of the source tool.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn field_mappings(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.field_mappings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nOptional. The mode of the data mapping.\nPossible values:\nMODE_UNSPECIFIED\nFIELD_MAPPING\nPYTHON_SCRIPT Possible values: [\"MODE_UNSPECIFIED\", \"FIELD_MAPPING\", \"PYTHON_SCRIPT\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_tool_name` after provisioning.\nOptional. The resource name of the tool that provides the data for the widget (e.g., a search tool or a custom function).\nFormat: projects/{project}/locations/{location}/agents/{agent}/tools/{tool}"]
    pub fn source_tool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_tool_name", self.base))
    }

    #[doc = "Get a reference to the value of field `python_function` after provisioning.\n"]
    pub fn python_function(&self) -> terrars::ListRef<CesTool_WidgetTool_DataMapping_PythonFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_function", self.base))
    }
}

#[derive(Serialize)]
pub struct CesTool_WidgetTool_TextResponseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_response_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesTool_WidgetTool_TextResponseConfig {
    #[doc = "Set the field `static_text`.\nOptional. The static text response to return when type is STATIC."]
    pub fn set_static_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.static_text = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `text_response_instruction`.\nOptional. Instruction for the LLM on how to generate the text response. Used as\nthe description for the text response parameter if type is LLM_GENERATED."]
    pub fn set_text_response_instruction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.text_response_instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. The strategy for providing the text response.\nPossible values:\nTYPE_UNSPECIFIED\nNONE\nLLM_GENERATED\nSTATIC Possible values: [\"TYPE_UNSPECIFIED\", \"NONE\", \"LLM_GENERATED\", \"STATIC\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesTool_WidgetTool_TextResponseConfig {
    type O = terrars::BlockAssignable<CesTool_WidgetTool_TextResponseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_WidgetTool_TextResponseConfig {}

impl BuildCesTool_WidgetTool_TextResponseConfig {
    pub fn build(self) -> CesTool_WidgetTool_TextResponseConfig {
        CesTool_WidgetTool_TextResponseConfig {
            static_text: core::default::Default::default(),
            text_response_instruction: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_WidgetTool_TextResponseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_WidgetTool_TextResponseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_WidgetTool_TextResponseConfigRef {
        CesTool_WidgetTool_TextResponseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_WidgetTool_TextResponseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `static_text` after provisioning.\nOptional. The static text response to return when type is STATIC."]
    pub fn static_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text_response_instruction` after provisioning.\nOptional. Instruction for the LLM on how to generate the text response. Used as\nthe description for the text response parameter if type is LLM_GENERATED."]
    pub fn text_response_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_response_instruction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. The strategy for providing the text response.\nPossible values:\nTYPE_UNSPECIFIED\nNONE\nLLM_GENERATED\nSTATIC Possible values: [\"TYPE_UNSPECIFIED\", \"NONE\", \"LLM_GENERATED\", \"STATIC\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesTool_WidgetToolDynamic {
    data_mapping: std::option::Option<terrars::DynamicBlock<CesTool_WidgetTool_DataMapping>>,
    parameters: std::option::Option<terrars::DynamicBlock<CesTool_WidgetTool_Parameters>>,
    text_response_config: std::option::Option<terrars::DynamicBlock<CesTool_WidgetTool_TextResponseConfig>>,
}

#[derive(Serialize)]
pub struct CesTool_WidgetTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    widget_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_mapping: std::option::Option<std::vec::Vec<CesTool_WidgetTool_DataMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<CesTool_WidgetTool_Parameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_response_config: std::option::Option<std::vec::Vec<CesTool_WidgetTool_TextResponseConfig>>,
    dynamic: CesTool_WidgetToolDynamic,
}

impl CesTool_WidgetTool {
    #[doc = "Set the field `description`.\nOptional. The description of the widget tool."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ui_config`.\nOptional. Configuration for rendering the widget. Represents a JSON object."]
    pub fn set_ui_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ui_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `widget_type`.\nOptional. The type of the widget tool. If not specified, the default type will be CUSTOMIZED.\nPossible values:\nWIDGET_TYPE_UNSPECIFIED\nCUSTOM\nPRODUCT_CAROUSEL\nPRODUCT_DETAILS\nQUICK_ACTIONS\nPRODUCT_COMPARISON\nADVANCED_PRODUCT_DETAILS\nSHORT_FORM\nOVERALL_SATISFACTION\nORDER_SUMMARY\nAPPOINTMENT_DETAILS\nAPPOINTMENT_SCHEDULER\nCONTACT_FORM Possible values: [\"WIDGET_TYPE_UNSPECIFIED\", \"CUSTOM\", \"PRODUCT_CAROUSEL\", \"PRODUCT_DETAILS\", \"QUICK_ACTIONS\", \"PRODUCT_COMPARISON\", \"ADVANCED_PRODUCT_DETAILS\", \"SHORT_FORM\", \"OVERALL_SATISFACTION\", \"ORDER_SUMMARY\", \"APPOINTMENT_DETAILS\", \"APPOINTMENT_SCHEDULER\", \"CONTACT_FORM\"]"]
    pub fn set_widget_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.widget_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_mapping`.\n"]
    pub fn set_data_mapping(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_WidgetTool_DataMapping>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_WidgetTool_Parameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_response_config`.\n"]
    pub fn set_text_response_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesTool_WidgetTool_TextResponseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_response_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_response_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesTool_WidgetTool {
    type O = terrars::BlockAssignable<CesTool_WidgetTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesTool_WidgetTool {
    #[doc = "Required. The display name of the widget tool."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesTool_WidgetTool {
    pub fn build(self) -> CesTool_WidgetTool {
        CesTool_WidgetTool {
            description: core::default::Default::default(),
            name: self.name,
            ui_config: core::default::Default::default(),
            widget_type: core::default::Default::default(),
            data_mapping: core::default::Default::default(),
            parameters: core::default::Default::default(),
            text_response_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesTool_WidgetToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesTool_WidgetToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesTool_WidgetToolRef {
        CesTool_WidgetToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesTool_WidgetToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. The description of the widget tool."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nRequired. The display name of the widget tool."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ui_config` after provisioning.\nOptional. Configuration for rendering the widget. Represents a JSON object."]
    pub fn ui_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ui_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `widget_type` after provisioning.\nOptional. The type of the widget tool. If not specified, the default type will be CUSTOMIZED.\nPossible values:\nWIDGET_TYPE_UNSPECIFIED\nCUSTOM\nPRODUCT_CAROUSEL\nPRODUCT_DETAILS\nQUICK_ACTIONS\nPRODUCT_COMPARISON\nADVANCED_PRODUCT_DETAILS\nSHORT_FORM\nOVERALL_SATISFACTION\nORDER_SUMMARY\nAPPOINTMENT_DETAILS\nAPPOINTMENT_SCHEDULER\nCONTACT_FORM Possible values: [\"WIDGET_TYPE_UNSPECIFIED\", \"CUSTOM\", \"PRODUCT_CAROUSEL\", \"PRODUCT_DETAILS\", \"QUICK_ACTIONS\", \"PRODUCT_COMPARISON\", \"ADVANCED_PRODUCT_DETAILS\", \"SHORT_FORM\", \"OVERALL_SATISFACTION\", \"ORDER_SUMMARY\", \"APPOINTMENT_DETAILS\", \"APPOINTMENT_SCHEDULER\", \"CONTACT_FORM\"]"]
    pub fn widget_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.widget_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_mapping` after provisioning.\n"]
    pub fn data_mapping(&self) -> terrars::ListRef<CesTool_WidgetTool_DataMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<CesTool_WidgetTool_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `text_response_config` after provisioning.\n"]
    pub fn text_response_config(&self) -> terrars::ListRef<CesTool_WidgetTool_TextResponseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_response_config", self.base))
    }
}
