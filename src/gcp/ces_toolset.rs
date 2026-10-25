#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesToolsetData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    toolset_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_toolset: std::option::Option<std::vec::Vec<CesToolset_ConnectorToolset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_toolset: std::option::Option<std::vec::Vec<CesToolset_McpToolset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_toolset: std::option::Option<std::vec::Vec<CesToolset_OpenApiToolset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesToolset_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_fake_config: std::option::Option<std::vec::Vec<CesToolset_ToolFakeConfig>>,
    dynamic: CesToolsetDynamic,
}

struct CesToolset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesToolsetData_>,
}

#[derive(Clone)]
pub struct CesToolset(std::rc::Rc<CesToolset_>);

impl CesToolset {
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

    #[doc = "Set the field `description`.\nThe description of the toolset."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name of the toolset. Must be unique within the same app."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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
        "Set the field `timeout`.\nThe timeout for the toolset execution. If not set, the default timeout is\n30 seconds for 'SYNCHRONOUS' toolsets and 60 seconds for 'ASYNCHRONOUS'\ntoolsets."]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_toolset`.\n"]
    pub fn set_connector_toolset(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_ConnectorToolset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connector_toolset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connector_toolset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mcp_toolset`.\n"]
    pub fn set_mcp_toolset(self, v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mcp_toolset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mcp_toolset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `open_api_toolset`.\n"]
    pub fn set_open_api_toolset(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_OpenApiToolset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_api_toolset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_api_toolset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesToolset_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool_fake_config`.\n"]
    pub fn set_tool_fake_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_ToolFakeConfig>>,
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

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the toolset was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the toolset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the toolset. Must be unique within the same app."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nETag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_type` after provisioning.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn execution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_type", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout for the toolset execution. If not set, the default timeout is\n30 seconds for 'SYNCHRONOUS' toolsets and 60 seconds for 'ASYNCHRONOUS'\ntoolsets."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `toolset_id` after provisioning.\nThe ID to use for the toolset, which will become the final component of\nthe toolset's resource name. If not provided, a unique ID will be\nautomatically assigned for the toolset."]
    pub fn toolset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.toolset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the toolset was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_toolset` after provisioning.\n"]
    pub fn connector_toolset(&self) -> terrars::ListRef<CesToolset_ConnectorToolsetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_toolset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_toolset` after provisioning.\n"]
    pub fn mcp_toolset(&self) -> terrars::ListRef<CesToolset_McpToolsetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_toolset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_api_toolset` after provisioning.\n"]
    pub fn open_api_toolset(&self) -> terrars::ListRef<CesToolset_OpenApiToolsetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_toolset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesToolset_TimeoutsRef {
        CesToolset_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool_fake_config` after provisioning.\n"]
    pub fn tool_fake_config(&self) -> terrars::ListRef<CesToolset_ToolFakeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_fake_config", self.extract_ref()))
    }
}

impl terrars::Referable for CesToolset {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesToolset { }

impl terrars::ToListMappable for CesToolset {
    type O = terrars::ListRef<CesToolsetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesToolset_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_toolset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesToolset {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the toolset, which will become the final component of\nthe toolset's resource name. If not provided, a unique ID will be\nautomatically assigned for the toolset."]
    pub toolset_id: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset {
    pub fn build(self, stack: &mut terrars::Stack) -> CesToolset {
        let out = CesToolset(std::rc::Rc::new(CesToolset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesToolsetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                execution_type: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                timeout: core::default::Default::default(),
                toolset_id: self.toolset_id,
                connector_toolset: core::default::Default::default(),
                mcp_toolset: core::default::Default::default(),
                open_api_toolset: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tool_fake_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesToolsetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolsetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesToolsetRef {
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
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the toolset was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the toolset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the toolset. Must be unique within the same app."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nETag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_type` after provisioning.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn execution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_type", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout for the toolset execution. If not set, the default timeout is\n30 seconds for 'SYNCHRONOUS' toolsets and 60 seconds for 'ASYNCHRONOUS'\ntoolsets."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `toolset_id` after provisioning.\nThe ID to use for the toolset, which will become the final component of\nthe toolset's resource name. If not provided, a unique ID will be\nautomatically assigned for the toolset."]
    pub fn toolset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.toolset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the toolset was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_toolset` after provisioning.\n"]
    pub fn connector_toolset(&self) -> terrars::ListRef<CesToolset_ConnectorToolsetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_toolset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_toolset` after provisioning.\n"]
    pub fn mcp_toolset(&self) -> terrars::ListRef<CesToolset_McpToolsetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_toolset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_api_toolset` after provisioning.\n"]
    pub fn open_api_toolset(&self) -> terrars::ListRef<CesToolset_OpenApiToolsetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_toolset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesToolset_TimeoutsRef {
        CesToolset_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool_fake_config` after provisioning.\n"]
    pub fn tool_fake_config(&self) -> terrars::ListRef<CesToolset_ToolFakeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_fake_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CesToolsetDynamic {
    connector_toolset: std::option::Option<terrars::DynamicBlock<CesToolset_ConnectorToolset>>,
    mcp_toolset: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset>>,
    open_api_toolset: std::option::Option<terrars::DynamicBlock<CesToolset_OpenApiToolset>>,
    tool_fake_config: std::option::Option<terrars::DynamicBlock<CesToolset_ToolFakeConfig>>,
}

#[derive(Serialize)]
pub struct CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig {
    oauth_token: terrars::PrimField<std::string::String>,
}

impl CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig { }

impl terrars::ToListMappable for CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig {
    type O = terrars::BlockAssignable<CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig {
    #[doc =
        "Oauth token parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub oauth_token: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig {
    pub fn build(self) -> CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig {
        CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig { oauth_token: self.oauth_token }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfigRef {
        CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `oauth_token` after provisioning.\nOauth token parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub fn oauth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_token", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig {
    client_key: terrars::PrimField<std::string::String>,
    issuer: terrars::PrimField<std::string::String>,
    subject: terrars::PrimField<std::string::String>,
}

impl CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig { }

impl terrars::ToListMappable for CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig {
    type O = terrars::BlockAssignable<CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig {
    #[doc = "Client parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub client_key: terrars::PrimField<std::string::String>,
    #[doc = "Issuer parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub issuer: terrars::PrimField<std::string::String>,
    #[doc = "Subject parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub subject: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig {
    pub fn build(self) -> CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig {
        CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig {
            client_key: self.client_key,
            issuer: self.issuer,
            subject: self.subject,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfigRef {
        CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nClient parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nIssuer parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject` after provisioning.\nSubject parameter name to pass through.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesToolset_ConnectorToolset_AuthConfigDynamic {
    oauth2_auth_code_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig>>,
    oauth2_jwt_bearer_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig>>,
}

#[derive(Serialize)]
pub struct CesToolset_ConnectorToolset_AuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_auth_code_config: std
    ::option
    ::Option<std::vec::Vec<CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_jwt_bearer_config: std
    ::option
    ::Option<std::vec::Vec<CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig>>,
    dynamic: CesToolset_ConnectorToolset_AuthConfigDynamic,
}

impl CesToolset_ConnectorToolset_AuthConfig {
    #[doc = "Set the field `oauth2_auth_code_config`.\n"]
    pub fn set_oauth2_auth_code_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_auth_code_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_auth_code_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_jwt_bearer_config`.\n"]
    pub fn set_oauth2_jwt_bearer_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_jwt_bearer_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_jwt_bearer_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesToolset_ConnectorToolset_AuthConfig {
    type O = terrars::BlockAssignable<CesToolset_ConnectorToolset_AuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ConnectorToolset_AuthConfig {}

impl BuildCesToolset_ConnectorToolset_AuthConfig {
    pub fn build(self) -> CesToolset_ConnectorToolset_AuthConfig {
        CesToolset_ConnectorToolset_AuthConfig {
            oauth2_auth_code_config: core::default::Default::default(),
            oauth2_jwt_bearer_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ConnectorToolset_AuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ConnectorToolset_AuthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_ConnectorToolset_AuthConfigRef {
        CesToolset_ConnectorToolset_AuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ConnectorToolset_AuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oauth2_auth_code_config` after provisioning.\n"]
    pub fn oauth2_auth_code_config(
        &self,
    ) -> terrars::ListRef<CesToolset_ConnectorToolset_AuthConfig_Oauth2AuthCodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_auth_code_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_jwt_bearer_config` after provisioning.\n"]
    pub fn oauth2_jwt_bearer_config(
        &self,
    ) -> terrars::ListRef<CesToolset_ConnectorToolset_AuthConfig_Oauth2JwtBearerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_jwt_bearer_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_ConnectorToolset_ConnectorActions_EntityOperation {
    entity_id: terrars::PrimField<std::string::String>,
    operation: terrars::PrimField<std::string::String>,
}

impl CesToolset_ConnectorToolset_ConnectorActions_EntityOperation { }

impl terrars::ToListMappable for CesToolset_ConnectorToolset_ConnectorActions_EntityOperation {
    type O = terrars::BlockAssignable<CesToolset_ConnectorToolset_ConnectorActions_EntityOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ConnectorToolset_ConnectorActions_EntityOperation {
    #[doc = "ID of the entity."]
    pub entity_id: terrars::PrimField<std::string::String>,
    #[doc = "Operation to perform on the entity.\nPossible values:\nLIST\nGET\nCREATE\nUPDATE\nDELETE"]
    pub operation: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_ConnectorToolset_ConnectorActions_EntityOperation {
    pub fn build(self) -> CesToolset_ConnectorToolset_ConnectorActions_EntityOperation {
        CesToolset_ConnectorToolset_ConnectorActions_EntityOperation {
            entity_id: self.entity_id,
            operation: self.operation,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ConnectorToolset_ConnectorActions_EntityOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ConnectorToolset_ConnectorActions_EntityOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_ConnectorToolset_ConnectorActions_EntityOperationRef {
        CesToolset_ConnectorToolset_ConnectorActions_EntityOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ConnectorToolset_ConnectorActions_EntityOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\nID of the entity."]
    pub fn entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nOperation to perform on the entity.\nPossible values:\nLIST\nGET\nCREATE\nUPDATE\nDELETE"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesToolset_ConnectorToolset_ConnectorActionsDynamic {
    entity_operation: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_ConnectorToolset_ConnectorActions_EntityOperation>>,
}

#[derive(Serialize)]
pub struct CesToolset_ConnectorToolset_ConnectorActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_action_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_operation: std::option::Option<std::vec::Vec<CesToolset_ConnectorToolset_ConnectorActions_EntityOperation>>,
    dynamic: CesToolset_ConnectorToolset_ConnectorActionsDynamic,
}

impl CesToolset_ConnectorToolset_ConnectorActions {
    #[doc = "Set the field `connection_action_id`.\nID of a Connection action for the tool to use."]
    pub fn set_connection_action_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connection_action_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_fields`.\nEntity fields to use as inputs for the operation."]
    pub fn set_input_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_fields`.\nEntity fields to return from the operation."]
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesToolset_ConnectorToolset_ConnectorActions_EntityOperation>>,
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

impl terrars::ToListMappable for CesToolset_ConnectorToolset_ConnectorActions {
    type O = terrars::BlockAssignable<CesToolset_ConnectorToolset_ConnectorActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ConnectorToolset_ConnectorActions {}

impl BuildCesToolset_ConnectorToolset_ConnectorActions {
    pub fn build(self) -> CesToolset_ConnectorToolset_ConnectorActions {
        CesToolset_ConnectorToolset_ConnectorActions {
            connection_action_id: core::default::Default::default(),
            input_fields: core::default::Default::default(),
            output_fields: core::default::Default::default(),
            entity_operation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ConnectorToolset_ConnectorActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ConnectorToolset_ConnectorActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_ConnectorToolset_ConnectorActionsRef {
        CesToolset_ConnectorToolset_ConnectorActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ConnectorToolset_ConnectorActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_action_id` after provisioning.\nID of a Connection action for the tool to use."]
    pub fn connection_action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_action_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `input_fields` after provisioning.\nEntity fields to use as inputs for the operation."]
    pub fn input_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_fields` after provisioning.\nEntity fields to return from the operation."]
    pub fn output_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_operation` after provisioning.\n"]
    pub fn entity_operation(&self) -> terrars::ListRef<CesToolset_ConnectorToolset_ConnectorActions_EntityOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_operation", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesToolset_ConnectorToolsetDynamic {
    auth_config: std::option::Option<terrars::DynamicBlock<CesToolset_ConnectorToolset_AuthConfig>>,
    connector_actions: std::option::Option<terrars::DynamicBlock<CesToolset_ConnectorToolset_ConnectorActions>>,
}

#[derive(Serialize)]
pub struct CesToolset_ConnectorToolset {
    connection: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<CesToolset_ConnectorToolset_AuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_actions: std::option::Option<std::vec::Vec<CesToolset_ConnectorToolset_ConnectorActions>>,
    dynamic: CesToolset_ConnectorToolsetDynamic,
}

impl CesToolset_ConnectorToolset {
    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_ConnectorToolset_AuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connector_actions`.\n"]
    pub fn set_connector_actions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_ConnectorToolset_ConnectorActions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connector_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesToolset_ConnectorToolset {
    type O = terrars::BlockAssignable<CesToolset_ConnectorToolset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ConnectorToolset {
    #[doc =
        "The full resource name of the referenced Integration Connectors\nConnection.\nFormat:\n'projects/{project}/locations/{location}/connections/{connection}'"]
    pub connection: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_ConnectorToolset {
    pub fn build(self) -> CesToolset_ConnectorToolset {
        CesToolset_ConnectorToolset {
            connection: self.connection,
            auth_config: core::default::Default::default(),
            connector_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ConnectorToolsetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ConnectorToolsetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_ConnectorToolsetRef {
        CesToolset_ConnectorToolsetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ConnectorToolsetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection` after provisioning.\nThe full resource name of the referenced Integration Connectors\nConnection.\nFormat:\n'projects/{project}/locations/{location}/connections/{connection}'"]
    pub fn connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<CesToolset_ConnectorToolset_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_actions` after provisioning.\n"]
    pub fn connector_actions(&self) -> terrars::ListRef<CesToolset_ConnectorToolset_ConnectorActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_actions", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig {
    api_key_secret_version: terrars::PrimField<std::string::String>,
    key_name: terrars::PrimField<std::string::String>,
    request_location: terrars::PrimField<std::string::String>,
}

impl CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig { }

impl terrars::ToListMappable for CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ApiAuthentication_ApiKeyConfig {
    #[doc =
        "The name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub api_key_secret_version: terrars::PrimField<std::string::String>,
    #[doc =
        "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Key location in the request. For API key auth on MCP toolsets,\nthe API key can only be sent in the request header.\nPossible values:\nHEADER"]
    pub request_location: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_McpToolset_ApiAuthentication_ApiKeyConfig {
    pub fn build(self) -> CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig {
        CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig {
            api_key_secret_version: self.api_key_secret_version,
            key_name: self.key_name,
            request_location: self.request_location,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ApiAuthentication_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ApiAuthentication_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_McpToolset_ApiAuthentication_ApiKeyConfigRef {
        CesToolset_McpToolset_ApiAuthentication_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ApiAuthentication_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn api_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request. For API key auth on MCP toolsets,\nthe API key can only be sent in the request header.\nPossible values:\nHEADER"]
    pub fn request_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_location", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ApiAuthentication_BearerTokenConfig {}

impl BuildCesToolset_McpToolset_ApiAuthentication_BearerTokenConfig {
    pub fn build(self) -> CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig {
        CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig { token: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ApiAuthentication_BearerTokenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ApiAuthentication_BearerTokenConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_McpToolset_ApiAuthentication_BearerTokenConfigRef {
        CesToolset_McpToolset_ApiAuthentication_BearerTokenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ApiAuthentication_BearerTokenConfigRef {
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
pub struct CesToolset_McpToolset_ApiAuthentication_OauthConfig {
    client_id: terrars::PrimField<std::string::String>,
    client_secret_version: terrars::PrimField<std::string::String>,
    oauth_grant_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    token_endpoint: terrars::PrimField<std::string::String>,
}

impl CesToolset_McpToolset_ApiAuthentication_OauthConfig {
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant."]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesToolset_McpToolset_ApiAuthentication_OauthConfig {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ApiAuthentication_OauthConfig {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub client_secret_version: terrars::PrimField<std::string::String>,
    #[doc = "OAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub oauth_grant_type: terrars::PrimField<std::string::String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_McpToolset_ApiAuthentication_OauthConfig {
    pub fn build(self) -> CesToolset_McpToolset_ApiAuthentication_OauthConfig {
        CesToolset_McpToolset_ApiAuthentication_OauthConfig {
            client_id: self.client_id,
            client_secret_version: self.client_secret_version,
            oauth_grant_type: self.oauth_grant_type,
            scopes: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ApiAuthentication_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ApiAuthentication_OauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_McpToolset_ApiAuthentication_OauthConfigRef {
        CesToolset_McpToolset_ApiAuthentication_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ApiAuthentication_OauthConfigRef {
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
        "Get a reference to the value of field `client_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn client_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_grant_type` after provisioning.\nOAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub fn oauth_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint in the OAuth provider to exchange for an access token."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    service_account: terrars::PrimField<std::string::String>,
}

impl CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig {
    #[doc =
        "Set the field `scopes`.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig {
    #[doc =
        "The email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig {
    pub fn build(self) -> CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig {
        CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig {
            scopes: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfigRef {
        CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}

impl CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig { }

impl terrars::ToListMappable for CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}

impl BuildCesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
    pub fn build(self) -> CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
        CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
        CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct CesToolset_McpToolset_ApiAuthenticationDynamic {
    api_key_config: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig>>,
    bearer_token_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig>>,
    oauth_config: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_OauthConfig>>,
    service_account_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig>>,
    service_agent_id_token_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig>>,
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_ApiAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_auth_config: std
    ::option
    ::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_id_token_auth_config: std
    ::option
    ::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig>>,
    dynamic: CesToolset_McpToolset_ApiAuthenticationDynamic,
}

impl CesToolset_McpToolset_ApiAuthentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ApiKeyConfig>>,
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
                    ::Into<terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_OauthConfig>>,
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

    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_agent_id_token_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_id_token_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesToolset_McpToolset_ApiAuthentication {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ApiAuthentication {}

impl BuildCesToolset_McpToolset_ApiAuthentication {
    pub fn build(self) -> CesToolset_McpToolset_ApiAuthentication {
        CesToolset_McpToolset_ApiAuthentication {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ApiAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ApiAuthenticationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_McpToolset_ApiAuthenticationRef {
        CesToolset_McpToolset_ApiAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ApiAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(&self) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_id_token_auth_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl CesToolset_McpToolset_ServiceDirectoryConfig { }

impl terrars::ToListMappable for CesToolset_McpToolset_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ServiceDirectoryConfig {
    #[doc =
        "The name of [Service\nDirectory](https://cloud.google.com/service-directory) service.\nFormat:\n'projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}'.\nLocation of the service directory must be the same as the location of the\napp."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_McpToolset_ServiceDirectoryConfig {
    pub fn build(self) -> CesToolset_McpToolset_ServiceDirectoryConfig {
        CesToolset_McpToolset_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_McpToolset_ServiceDirectoryConfigRef {
        CesToolset_McpToolset_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of [Service\nDirectory](https://cloud.google.com/service-directory) service.\nFormat:\n'projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}'.\nLocation of the service directory must be the same as the location of the\napp."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_TlsConfig_CaCerts {
    cert: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
}

impl CesToolset_McpToolset_TlsConfig_CaCerts { }

impl terrars::ToListMappable for CesToolset_McpToolset_TlsConfig_CaCerts {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_TlsConfig_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_TlsConfig_CaCerts {
    #[doc =
        "The allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, CES will use Google's default trust\nstore to verify certificates. N.B. Make sure the HTTPS server\ncertificates are signed with \"subject alt name\". For instance a\ncertificate can be self-signed using the following command,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub cert: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the allowed custom CA certificates. This\ncan be used to disambiguate the custom CA certificates."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_McpToolset_TlsConfig_CaCerts {
    pub fn build(self) -> CesToolset_McpToolset_TlsConfig_CaCerts {
        CesToolset_McpToolset_TlsConfig_CaCerts {
            cert: self.cert,
            display_name: self.display_name,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_TlsConfig_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_TlsConfig_CaCertsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_McpToolset_TlsConfig_CaCertsRef {
        CesToolset_McpToolset_TlsConfig_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_TlsConfig_CaCertsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cert` after provisioning.\nThe allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, CES will use Google's default trust\nstore to verify certificates. N.B. Make sure the HTTPS server\ncertificates are signed with \"subject alt name\". For instance a\ncertificate can be self-signed using the following command,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe name of the allowed custom CA certificates. This\ncan be used to disambiguate the custom CA certificates."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesToolset_McpToolset_TlsConfigDynamic {
    ca_certs: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_TlsConfig_CaCerts>>,
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_TlsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<std::vec::Vec<CesToolset_McpToolset_TlsConfig_CaCerts>>,
    dynamic: CesToolset_McpToolset_TlsConfigDynamic,
}

impl CesToolset_McpToolset_TlsConfig {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_TlsConfig_CaCerts>>,
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

impl terrars::ToListMappable for CesToolset_McpToolset_TlsConfig {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_TlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_TlsConfig {}

impl BuildCesToolset_McpToolset_TlsConfig {
    pub fn build(self) -> CesToolset_McpToolset_TlsConfig {
        CesToolset_McpToolset_TlsConfig {
            ca_certs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_TlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_TlsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_McpToolset_TlsConfigRef {
        CesToolset_McpToolset_TlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_TlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<CesToolset_McpToolset_TlsConfig_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset_ToolOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description_override: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_override: std::option::Option<terrars::PrimField<std::string::String>>,
    tool: terrars::PrimField<std::string::String>,
}

impl CesToolset_McpToolset_ToolOverrides {
    #[doc = "Set the field `description_override`.\nThe description override for the tool."]
    pub fn set_description_override(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.description_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_override`.\nThe name override for the tool."]
    pub fn set_name_override(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_override = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesToolset_McpToolset_ToolOverrides {
    type O = terrars::BlockAssignable<CesToolset_McpToolset_ToolOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset_ToolOverrides {
    #[doc = "The name of the tool to be overridden."]
    pub tool: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_McpToolset_ToolOverrides {
    pub fn build(self) -> CesToolset_McpToolset_ToolOverrides {
        CesToolset_McpToolset_ToolOverrides {
            description_override: core::default::Default::default(),
            name_override: core::default::Default::default(),
            tool: self.tool,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolset_ToolOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolset_ToolOverridesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_McpToolset_ToolOverridesRef {
        CesToolset_McpToolset_ToolOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolset_ToolOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description_override` after provisioning.\nThe description override for the tool."]
    pub fn description_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description_override", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name_override` after provisioning.\nThe name override for the tool."]
    pub fn name_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_override", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\nThe name of the tool to be overridden."]
    pub fn tool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesToolset_McpToolsetDynamic {
    api_authentication: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ServiceDirectoryConfig>>,
    tls_config: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_TlsConfig>>,
    tool_overrides: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_ToolOverrides>>,
}

#[derive(Serialize)]
pub struct CesToolset_McpToolset {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    server_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_TlsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_overrides: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ToolOverrides>>,
    dynamic: CesToolset_McpToolsetDynamic,
}

impl CesToolset_McpToolset {
    #[doc =
        "Set the field `custom_headers`.\nThe custom headers to send in the request to the MCP server. The values\nmust be in the format '$context.variables.<name_of_variable>' and can be\nset in the session variables. See\nhttps://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/tool/open-api#openapi-injection\nfor more details."]
    pub fn set_custom_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_ServiceDirectoryConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_TlsConfig>>,
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

    #[doc = "Set the field `tool_overrides`.\n"]
    pub fn set_tool_overrides(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_ToolOverrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesToolset_McpToolset {
    type O = terrars::BlockAssignable<CesToolset_McpToolset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_McpToolset {
    #[doc =
        "The address of the MCP server, for example, \"https://example.com/mcp/\". If\nthe server is built with the MCP SDK, the url should be suffixed with\n\"/mcp/\". Only Streamable HTTP transport based servers are supported. See\nhttps://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http\nfor more details."]
    pub server_address: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_McpToolset {
    pub fn build(self) -> CesToolset_McpToolset {
        CesToolset_McpToolset {
            custom_headers: core::default::Default::default(),
            server_address: self.server_address,
            api_authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            tool_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_McpToolsetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_McpToolsetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_McpToolsetRef {
        CesToolset_McpToolsetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_McpToolsetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_headers` after provisioning.\nThe custom headers to send in the request to the MCP server. The values\nmust be in the format '$context.variables.<name_of_variable>' and can be\nset in the session variables. See\nhttps://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/tool/open-api#openapi-injection\nfor more details."]
    pub fn custom_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_address` after provisioning.\nThe address of the MCP server, for example, \"https://example.com/mcp/\". If\nthe server is built with the MCP SDK, the url should be suffixed with\n\"/mcp/\". Only Streamable HTTP transport based servers are supported. See\nhttps://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http\nfor more details."]
    pub fn server_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_address", self.base))
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(&self) -> terrars::ListRef<CesToolset_McpToolset_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<CesToolset_McpToolset_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_overrides` after provisioning.\n"]
    pub fn tool_overrides(&self) -> terrars::ListRef<CesToolset_McpToolset_ToolOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_overrides", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig {
    api_key_secret_version: terrars::PrimField<std::string::String>,
    key_name: terrars::PrimField<std::string::String>,
    request_location: terrars::PrimField<std::string::String>,
}

impl CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig { }

impl terrars::ToListMappable for CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig {
    type O = terrars::BlockAssignable<CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig {
    #[doc =
        "The name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub api_key_secret_version: terrars::PrimField<std::string::String>,
    #[doc =
        "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: terrars::PrimField<std::string::String>,
    #[doc = "Key location in the request.\nPossible values:\nHEADER\nQUERY_STRING"]
    pub request_location: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig {
    pub fn build(self) -> CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig {
        CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig {
            api_key_secret_version: self.api_key_secret_version,
            key_name: self.key_name,
            request_location: self.request_location,
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfigRef {
        CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn api_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request.\nPossible values:\nHEADER\nQUERY_STRING"]
    pub fn request_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesToolset_OpenApiToolset_ApiAuthenticationDynamic {
    api_key_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig>>,
    bearer_token_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig>>,
    oauth_config: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_OauthConfig>>,
    service_account_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig>>,
    service_agent_id_token_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig>>,
}

#[derive(Serialize)]
pub struct CesToolset_OpenApiToolset_ApiAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<std::vec::Vec<CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_auth_config: std
    ::option
    ::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_id_token_auth_config: std
    ::option
    ::Option<std::vec::Vec<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig>>,
    dynamic: CesToolset_OpenApiToolset_ApiAuthenticationDynamic,
}

impl CesToolset_OpenApiToolset_ApiAuthentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfig>>,
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
                    ::Into<terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_OauthConfig>>,
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

    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_agent_id_token_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_id_token_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesToolset_OpenApiToolset_ApiAuthentication {
    type O = terrars::BlockAssignable<CesToolset_OpenApiToolset_ApiAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_OpenApiToolset_ApiAuthentication {}

impl BuildCesToolset_OpenApiToolset_ApiAuthentication {
    pub fn build(self) -> CesToolset_OpenApiToolset_ApiAuthentication {
        CesToolset_OpenApiToolset_ApiAuthentication {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_OpenApiToolset_ApiAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_OpenApiToolset_ApiAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesToolset_OpenApiToolset_ApiAuthenticationRef {
        CesToolset_OpenApiToolset_ApiAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_OpenApiToolset_ApiAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> terrars::ListRef<CesToolset_OpenApiToolset_ApiAuthentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(&self) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_BearerTokenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_ServiceAccountAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> terrars::ListRef<CesToolset_McpToolset_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_id_token_auth_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesToolset_OpenApiToolsetDynamic {
    api_authentication: std::option::Option<terrars::DynamicBlock<CesToolset_OpenApiToolset_ApiAuthentication>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesToolset_McpToolset_ServiceDirectoryConfig>>,
    tls_config: std::option::Option<terrars::DynamicBlock<CesToolset_McpToolset_TlsConfig>>,
}

#[derive(Serialize)]
pub struct CesToolset_OpenApiToolset {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    open_api_schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std::option::Option<std::vec::Vec<CesToolset_OpenApiToolset_ApiAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<std::vec::Vec<CesToolset_McpToolset_TlsConfig>>,
    dynamic: CesToolset_OpenApiToolsetDynamic,
}

impl CesToolset_OpenApiToolset {
    #[doc =
        "Set the field `ignore_unknown_fields`.\nIf true, the agent will ignore unknown fields in the API response for all\noperations defined in the OpenAPI schema."]
    pub fn set_ignore_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_OpenApiToolset_ApiAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_ServiceDirectoryConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_McpToolset_TlsConfig>>,
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

impl terrars::ToListMappable for CesToolset_OpenApiToolset {
    type O = terrars::BlockAssignable<CesToolset_OpenApiToolset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_OpenApiToolset {
    #[doc = "The OpenAPI schema of the toolset."]
    pub open_api_schema: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_OpenApiToolset {
    pub fn build(self) -> CesToolset_OpenApiToolset {
        CesToolset_OpenApiToolset {
            ignore_unknown_fields: core::default::Default::default(),
            open_api_schema: self.open_api_schema,
            api_authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_OpenApiToolsetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_OpenApiToolsetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_OpenApiToolsetRef {
        CesToolset_OpenApiToolsetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_OpenApiToolsetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\nIf true, the agent will ignore unknown fields in the API response for all\noperations defined in the OpenAPI schema."]
    pub fn ignore_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `open_api_schema` after provisioning.\nThe OpenAPI schema of the toolset."]
    pub fn open_api_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_api_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe server URL of the Open API schema.\nThis field is only set in toolsets in the environment dependencies\nduring the export process if the schema contains a server url.\nDuring the import process, if this url is present in the environment dependencies\nand the schema has the $env_var placeholder,\nit will replace the placeholder in the schema."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> terrars::ListRef<CesToolset_OpenApiToolset_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(&self) -> terrars::ListRef<CesToolset_McpToolset_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<CesToolset_McpToolset_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesToolset_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesToolset_Timeouts {
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

impl terrars::ToListMappable for CesToolset_Timeouts {
    type O = terrars::BlockAssignable<CesToolset_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_Timeouts {}

impl BuildCesToolset_Timeouts {
    pub fn build(self) -> CesToolset_Timeouts {
        CesToolset_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_TimeoutsRef {
        CesToolset_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_TimeoutsRef {
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
pub struct CesToolset_ToolFakeConfig_CodeBlock {
    python_code: terrars::PrimField<std::string::String>,
}

impl CesToolset_ToolFakeConfig_CodeBlock { }

impl terrars::ToListMappable for CesToolset_ToolFakeConfig_CodeBlock {
    type O = terrars::BlockAssignable<CesToolset_ToolFakeConfig_CodeBlock>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ToolFakeConfig_CodeBlock {
    #[doc = "Python code which will be invoked in tool fake mode."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesToolset_ToolFakeConfig_CodeBlock {
    pub fn build(self) -> CesToolset_ToolFakeConfig_CodeBlock {
        CesToolset_ToolFakeConfig_CodeBlock { python_code: self.python_code }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ToolFakeConfig_CodeBlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ToolFakeConfig_CodeBlockRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_ToolFakeConfig_CodeBlockRef {
        CesToolset_ToolFakeConfig_CodeBlockRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ToolFakeConfig_CodeBlockRef {
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
struct CesToolset_ToolFakeConfigDynamic {
    code_block: std::option::Option<terrars::DynamicBlock<CesToolset_ToolFakeConfig_CodeBlock>>,
}

#[derive(Serialize)]
pub struct CesToolset_ToolFakeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_fake_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_block: std::option::Option<std::vec::Vec<CesToolset_ToolFakeConfig_CodeBlock>>,
    dynamic: CesToolset_ToolFakeConfigDynamic,
}

impl CesToolset_ToolFakeConfig {
    #[doc = "Set the field `enable_fake_mode`.\nWhether the tool is using fake mode."]
    pub fn set_enable_fake_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_fake_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_block`.\n"]
    pub fn set_code_block(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesToolset_ToolFakeConfig_CodeBlock>>,
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

impl terrars::ToListMappable for CesToolset_ToolFakeConfig {
    type O = terrars::BlockAssignable<CesToolset_ToolFakeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesToolset_ToolFakeConfig {}

impl BuildCesToolset_ToolFakeConfig {
    pub fn build(self) -> CesToolset_ToolFakeConfig {
        CesToolset_ToolFakeConfig {
            enable_fake_mode: core::default::Default::default(),
            code_block: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesToolset_ToolFakeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesToolset_ToolFakeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesToolset_ToolFakeConfigRef {
        CesToolset_ToolFakeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesToolset_ToolFakeConfigRef {
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
    pub fn code_block(&self) -> terrars::ListRef<CesToolset_ToolFakeConfig_CodeBlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_block", self.base))
    }
}
