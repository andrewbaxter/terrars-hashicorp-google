#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAgentRegistryMcpServerData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_server_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAgentRegistryMcpServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentRegistryMcpServerData_>,
}

#[derive(Clone)]
pub struct DataAgentRegistryMcpServer(std::rc::Rc<DataAgentRegistryMcpServer_>);

impl DataAgentRegistryMcpServer {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `filter`.\nA filter string that identifies a unique MCP server."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mcp_server_id`.\nThe unique identifier for the MCP server."]
    pub fn set_mcp_server_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mcp_server_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\nAttributes of the MCP Server."]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the MCP Server."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the MCP Server."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA filter string that identifies a unique MCP server."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interfaces` after provisioning.\nThe connection details for the MCP Server."]
    pub fn interfaces(&self) -> terrars::ListRef<DataAgentRegistryMcpServer_InterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mcp_server_id` after provisioning.\nThe unique identifier for the MCP server."]
    pub fn mcp_server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcp_server_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tools` after provisioning.\nA list of tools available with the MCP Server."]
    pub fn tools(&self) -> terrars::ListRef<DataAgentRegistryMcpServer_ToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\nThe URN of the MCP Server."]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentRegistryMcpServer {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentRegistryMcpServer { }

impl terrars::ToListMappable for DataAgentRegistryMcpServer {
    type O = terrars::ListRef<DataAgentRegistryMcpServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentRegistryMcpServer_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_agent_registry_mcp_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentRegistryMcpServer {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentRegistryMcpServer {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentRegistryMcpServer {
        let out = DataAgentRegistryMcpServer(std::rc::Rc::new(DataAgentRegistryMcpServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentRegistryMcpServerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                mcp_server_id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentRegistryMcpServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentRegistryMcpServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentRegistryMcpServerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\nAttributes of the MCP Server."]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the MCP Server."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the MCP Server."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA filter string that identifies a unique MCP server."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interfaces` after provisioning.\nThe connection details for the MCP Server."]
    pub fn interfaces(&self) -> terrars::ListRef<DataAgentRegistryMcpServer_InterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mcp_server_id` after provisioning.\nThe unique identifier for the MCP server."]
    pub fn mcp_server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcp_server_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tools` after provisioning.\nA list of tools available with the MCP Server."]
    pub fn tools(&self) -> terrars::ListRef<DataAgentRegistryMcpServer_ToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\nThe URN of the MCP Server."]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentRegistryMcpServer_Interfaces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_binding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentRegistryMcpServer_Interfaces {
    #[doc = "Set the field `protocol_binding`.\n"]
    pub fn set_protocol_binding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol_binding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentRegistryMcpServer_Interfaces {
    type O = terrars::BlockAssignable<DataAgentRegistryMcpServer_Interfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentRegistryMcpServer_Interfaces {}

impl BuildDataAgentRegistryMcpServer_Interfaces {
    pub fn build(self) -> DataAgentRegistryMcpServer_Interfaces {
        DataAgentRegistryMcpServer_Interfaces {
            protocol_binding: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentRegistryMcpServer_InterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentRegistryMcpServer_InterfacesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentRegistryMcpServer_InterfacesRef {
        DataAgentRegistryMcpServer_InterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentRegistryMcpServer_InterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `protocol_binding` after provisioning.\n"]
    pub fn protocol_binding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_binding", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentRegistryMcpServer_Tools_Annotations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destructive_hint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idempotent_hint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_world_hint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only_hint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentRegistryMcpServer_Tools_Annotations {
    #[doc = "Set the field `destructive_hint`.\n"]
    pub fn set_destructive_hint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.destructive_hint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idempotent_hint`.\n"]
    pub fn set_idempotent_hint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.idempotent_hint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_world_hint`.\n"]
    pub fn set_open_world_hint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.open_world_hint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only_hint`.\n"]
    pub fn set_read_only_hint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only_hint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentRegistryMcpServer_Tools_Annotations {
    type O = terrars::BlockAssignable<DataAgentRegistryMcpServer_Tools_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentRegistryMcpServer_Tools_Annotations {}

impl BuildDataAgentRegistryMcpServer_Tools_Annotations {
    pub fn build(self) -> DataAgentRegistryMcpServer_Tools_Annotations {
        DataAgentRegistryMcpServer_Tools_Annotations {
            destructive_hint: core::default::Default::default(),
            idempotent_hint: core::default::Default::default(),
            open_world_hint: core::default::Default::default(),
            read_only_hint: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentRegistryMcpServer_Tools_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentRegistryMcpServer_Tools_AnnotationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentRegistryMcpServer_Tools_AnnotationsRef {
        DataAgentRegistryMcpServer_Tools_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentRegistryMcpServer_Tools_AnnotationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destructive_hint` after provisioning.\n"]
    pub fn destructive_hint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destructive_hint", self.base))
    }

    #[doc = "Get a reference to the value of field `idempotent_hint` after provisioning.\n"]
    pub fn idempotent_hint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idempotent_hint", self.base))
    }

    #[doc = "Get a reference to the value of field `open_world_hint` after provisioning.\n"]
    pub fn open_world_hint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_world_hint", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only_hint` after provisioning.\n"]
    pub fn read_only_hint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only_hint", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentRegistryMcpServer_Tools {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::ListField<DataAgentRegistryMcpServer_Tools_Annotations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentRegistryMcpServer_Tools {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentRegistryMcpServer_Tools_Annotations>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataAgentRegistryMcpServer_Tools {
    type O = terrars::BlockAssignable<DataAgentRegistryMcpServer_Tools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentRegistryMcpServer_Tools {}

impl BuildDataAgentRegistryMcpServer_Tools {
    pub fn build(self) -> DataAgentRegistryMcpServer_Tools {
        DataAgentRegistryMcpServer_Tools {
            annotations: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentRegistryMcpServer_ToolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentRegistryMcpServer_ToolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentRegistryMcpServer_ToolsRef {
        DataAgentRegistryMcpServer_ToolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentRegistryMcpServer_ToolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> terrars::ListRef<DataAgentRegistryMcpServer_Tools_AnnotationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.annotations", self.base))
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
