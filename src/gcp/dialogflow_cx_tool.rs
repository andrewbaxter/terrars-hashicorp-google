#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxToolData_ {
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
    description: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_spec: std::option::Option<std::vec::Vec<DialogflowCxTool_DataStoreSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_spec: std::option::Option<std::vec::Vec<DialogflowCxTool_FunctionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_spec: std::option::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxTool_Timeouts>,
    dynamic: DialogflowCxToolDynamic,
}

struct DialogflowCxTool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxToolData_>,
}

#[derive(Clone)]
pub struct DialogflowCxTool(std::rc::Rc<DialogflowCxTool_>);

impl DialogflowCxTool {
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

    #[doc =
        "Set the field `parent`.\nThe agent to create a Tool for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store_spec`.\n"]
    pub fn set_data_store_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_DataStoreSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_store_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_store_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function_spec`.\n"]
    pub fn set_function_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_FunctionSpec>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxTool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHigh level description of the Tool and its usage."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the tool, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Tool.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Tool for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool_type` after provisioning.\nThe tool type."]
    pub fn tool_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_store_spec` after provisioning.\n"]
    pub fn data_store_spec(&self) -> terrars::ListRef<DialogflowCxTool_DataStoreSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_spec` after provisioning.\n"]
    pub fn function_spec(&self) -> terrars::ListRef<DialogflowCxTool_FunctionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_api_spec` after provisioning.\n"]
    pub fn open_api_spec(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTool_TimeoutsRef {
        DialogflowCxTool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxTool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxTool { }

impl terrars::ToListMappable for DialogflowCxTool {
    type O = terrars::ListRef<DialogflowCxToolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxTool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_tool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxTool {
    pub tf_id: std::string::String,
    #[doc = "High level description of the Tool and its usage."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "The human-readable name of the tool, unique within the agent."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTool {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxTool {
        let out = DialogflowCxTool(std::rc::Rc::new(DialogflowCxTool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxToolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: self.description,
                display_name: self.display_name,
                id: core::default::Default::default(),
                parent: core::default::Default::default(),
                data_store_spec: core::default::Default::default(),
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

pub struct DialogflowCxToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxToolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHigh level description of the Tool and its usage."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the tool, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Tool.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Tool for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool_type` after provisioning.\nThe tool type."]
    pub fn tool_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_store_spec` after provisioning.\n"]
    pub fn data_store_spec(&self) -> terrars::ListRef<DialogflowCxTool_DataStoreSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_spec` after provisioning.\n"]
    pub fn function_spec(&self) -> terrars::ListRef<DialogflowCxTool_FunctionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_api_spec` after provisioning.\n"]
    pub fn open_api_spec(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTool_TimeoutsRef {
        DialogflowCxTool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxToolDynamic {
    data_store_spec: std::option::Option<terrars::DynamicBlock<DialogflowCxTool_DataStoreSpec>>,
    function_spec: std::option::Option<terrars::DynamicBlock<DialogflowCxTool_FunctionSpec>>,
    open_api_spec: std::option::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTool_DataStoreSpec_DataStoreConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_processing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTool_DataStoreSpec_DataStoreConnections {
    #[doc =
        "Set the field `data_store`.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn set_data_store(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_store = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_store_type`.\nThe type of the connected data store.\nSee [DataStoreType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#datastoretype) for valid values."]
    pub fn set_data_store_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_store_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `document_processing_mode`.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\nSee [DocumentProcessingMode](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#documentprocessingmode) for valid values."]
    pub fn set_document_processing_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.document_processing_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTool_DataStoreSpec_DataStoreConnections {
    type O = terrars::BlockAssignable<DialogflowCxTool_DataStoreSpec_DataStoreConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_DataStoreSpec_DataStoreConnections {}

impl BuildDialogflowCxTool_DataStoreSpec_DataStoreConnections {
    pub fn build(self) -> DialogflowCxTool_DataStoreSpec_DataStoreConnections {
        DialogflowCxTool_DataStoreSpec_DataStoreConnections {
            data_store: core::default::Default::default(),
            data_store_type: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_DataStoreSpec_DataStoreConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_DataStoreSpec_DataStoreConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_DataStoreSpec_DataStoreConnectionsRef {
        DialogflowCxTool_DataStoreSpec_DataStoreConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_DataStoreSpec_DataStoreConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_store` after provisioning.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn data_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_store_type` after provisioning.\nThe type of the connected data store.\nSee [DataStoreType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#datastoretype) for valid values."]
    pub fn data_store_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `document_processing_mode` after provisioning.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\nSee [DocumentProcessingMode](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#documentprocessingmode) for valid values."]
    pub fn document_processing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_processing_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_DataStoreSpec_FallbackPrompt {}

impl DialogflowCxTool_DataStoreSpec_FallbackPrompt { }

impl terrars::ToListMappable for DialogflowCxTool_DataStoreSpec_FallbackPrompt {
    type O = terrars::BlockAssignable<DialogflowCxTool_DataStoreSpec_FallbackPrompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_DataStoreSpec_FallbackPrompt {}

impl BuildDialogflowCxTool_DataStoreSpec_FallbackPrompt {
    pub fn build(self) -> DialogflowCxTool_DataStoreSpec_FallbackPrompt {
        DialogflowCxTool_DataStoreSpec_FallbackPrompt {}
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_DataStoreSpec_FallbackPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_DataStoreSpec_FallbackPromptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_DataStoreSpec_FallbackPromptRef {
        DialogflowCxTool_DataStoreSpec_FallbackPromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_DataStoreSpec_FallbackPromptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTool_DataStoreSpecDynamic {
    data_store_connections: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTool_DataStoreSpec_DataStoreConnections>>,
    fallback_prompt: std::option::Option<terrars::DynamicBlock<DialogflowCxTool_DataStoreSpec_FallbackPrompt>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTool_DataStoreSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_connections: std::option::Option<std::vec::Vec<DialogflowCxTool_DataStoreSpec_DataStoreConnections>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_prompt: std::option::Option<std::vec::Vec<DialogflowCxTool_DataStoreSpec_FallbackPrompt>>,
    dynamic: DialogflowCxTool_DataStoreSpecDynamic,
}

impl DialogflowCxTool_DataStoreSpec {
    #[doc = "Set the field `data_store_connections`.\n"]
    pub fn set_data_store_connections(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_DataStoreSpec_DataStoreConnections>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_store_connections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_connections = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fallback_prompt`.\n"]
    pub fn set_fallback_prompt(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_DataStoreSpec_FallbackPrompt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fallback_prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fallback_prompt = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTool_DataStoreSpec {
    type O = terrars::BlockAssignable<DialogflowCxTool_DataStoreSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_DataStoreSpec {}

impl BuildDialogflowCxTool_DataStoreSpec {
    pub fn build(self) -> DialogflowCxTool_DataStoreSpec {
        DialogflowCxTool_DataStoreSpec {
            data_store_connections: core::default::Default::default(),
            fallback_prompt: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_DataStoreSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_DataStoreSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTool_DataStoreSpecRef {
        DialogflowCxTool_DataStoreSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_DataStoreSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_store_connections` after provisioning.\n"]
    pub fn data_store_connections(&self) -> terrars::ListRef<DialogflowCxTool_DataStoreSpec_DataStoreConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `fallback_prompt` after provisioning.\n"]
    pub fn fallback_prompt(&self) -> terrars::ListRef<DialogflowCxTool_DataStoreSpec_FallbackPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_FunctionSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_schema: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTool_FunctionSpec {
    #[doc =
        "Set the field `input_schema`.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the input of the function.\nThis input is a JSON object that contains the function's parameters as properties of the object"]
    pub fn set_input_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_schema`.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the output of the function.\nThis output is a JSON object that contains the function's parameters as properties of the object"]
    pub fn set_output_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTool_FunctionSpec {
    type O = terrars::BlockAssignable<DialogflowCxTool_FunctionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_FunctionSpec {}

impl BuildDialogflowCxTool_FunctionSpec {
    pub fn build(self) -> DialogflowCxTool_FunctionSpec {
        DialogflowCxTool_FunctionSpec {
            input_schema: core::default::Default::default(),
            output_schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_FunctionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_FunctionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTool_FunctionSpecRef {
        DialogflowCxTool_FunctionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_FunctionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `input_schema` after provisioning.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the input of the function.\nThis input is a JSON object that contains the function's parameters as properties of the object"]
    pub fn input_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_schema` after provisioning.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the output of the function.\nThis output is a JSON object that contains the function's parameters as properties of the object"]
    pub fn output_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_schema", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    key_name: terrars::PrimField<std::string::String>,
    request_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_version_for_api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig {
    #[doc =
        "Set the field `api_key`.\nOptional. The API key. If the 'secretVersionForApiKey'' field is set, this field will be ignored."]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_version_for_api_key`.\nOptional. The name of the SecretManager secret version resource storing the API key.\nIf this field is set, the apiKey field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn set_secret_version_for_api_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig {
    #[doc =
        "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Key location in the request.\nSee [RequestLocation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#requestlocation) for valid values."]
    pub request_location: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig {
        DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig {
            api_key: core::default::Default::default(),
            key_name: self.key_name,
            request_location: self.request_location,
            secret_version_for_api_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
        DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key` after provisioning.\nOptional. The API key. If the 'secretVersionForApiKey'' field is set, this field will be ignored."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request.\nSee [RequestLocation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#requestlocation) for valid values."]
    pub fn request_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_api_key` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the API key.\nIf this field is set, the apiKey field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn secret_version_for_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version_for_api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_version_for_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig {
    #[doc =
        "Set the field `secret_version_for_token`.\nOptional. The name of the SecretManager secret version resource storing the Bearer token. If this field is set, the 'token' field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn set_secret_version_for_token(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token`.\nOptional. The text token appended to the text Bearer to the request Authorization header.\n[Session parameters reference](https://cloud.google.com/dialogflow/cx/docs/concept/parameter#session-ref) can be used to pass the token dynamically, e.g. '$session.params.parameter-id'."]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig {}

impl BuildDialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig {
        DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig {
            secret_version_for_token: core::default::Default::default(),
            token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
        DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_token` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the Bearer token. If this field is set, the 'token' field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn secret_version_for_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version_for_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token` after provisioning.\nOptional. The text token appended to the text Bearer to the request Authorization header.\n[Session parameters reference](https://cloud.google.com/dialogflow/cx/docs/concept/parameter#session-ref) can be used to pass the token dynamically, e.g. '$session.params.parameter-id'."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig {
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

impl DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig {
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
        "Set the field `secret_version_for_client_secret`.\nOptional. The name of the SecretManager secret version resource storing the client secret.\nIf this field is set, the clientSecret field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn set_secret_version_for_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_Authentication_OauthConfig {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc =
        "OAuth grant types.\nSee [OauthGrantType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#oauthgranttype) for valid values"]
    pub oauth_grant_type: terrars::PrimField<std::string::String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTool_OpenApiSpec_Authentication_OauthConfig {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig {
        DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig {
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
pub struct DialogflowCxTool_OpenApiSpec_Authentication_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_Authentication_OauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_OpenApiSpec_Authentication_OauthConfigRef {
        DialogflowCxTool_OpenApiSpec_Authentication_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_Authentication_OauthConfigRef {
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
        "Get a reference to the value of field `oauth_grant_type` after provisioning.\nOAuth grant types.\nSee [OauthGrantType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#oauthgranttype) for valid values"]
    pub fn oauth_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nOptional. The OAuth scopes to grant."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_client_secret` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the client secret.\nIf this field is set, the clientSecret field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
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
pub struct DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_auth: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    #[doc =
        "Set the field `service_agent_auth`.\nOptional. Indicate the auth token type generated from the Diglogflow service agent.\nThe generated token is sent in the Authorization header.\nSee [ServiceAgentAuth](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#serviceagentauth) for valid values."]
    pub fn set_service_agent_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_agent_auth = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {}

impl BuildDialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
        DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig {
            service_agent_auth: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
        DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_agent_auth` after provisioning.\nOptional. Indicate the auth token type generated from the Diglogflow service agent.\nThe generated token is sent in the Authorization header.\nSee [ServiceAgentAuth](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#serviceagentauth) for valid values."]
    pub fn service_agent_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_agent_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTool_OpenApiSpec_AuthenticationDynamic {
    api_key_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig>>,
    bearer_token_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig>>,
    oauth_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig>>,
    service_agent_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_auth_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>>,
    dynamic: DialogflowCxTool_OpenApiSpec_AuthenticationDynamic,
}

impl DialogflowCxTool_OpenApiSpec_Authentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfig>>,
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
                    ::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfig>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_OauthConfig>>,
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
                        terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfig>,
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

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_Authentication {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_Authentication {}

impl BuildDialogflowCxTool_OpenApiSpec_Authentication {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_Authentication {
        DialogflowCxTool_OpenApiSpec_Authentication {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_agent_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpec_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_OpenApiSpec_AuthenticationRef {
        DialogflowCxTool_OpenApiSpec_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_Authentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_Authentication_BearerTokenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_Authentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_auth_config` after provisioning.\n"]
    pub fn service_agent_auth_config(
        &self,
    ) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_Authentication_ServiceAgentAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_auth_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig { }

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig {
    #[doc =
        "The name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>. LocationID of the service directory must be the same as the location of the agent."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig {
        DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfigRef {
        DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>. LocationID of the service directory must be the same as the location of the agent."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts {
    cert: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
}

impl DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts { }

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts {
    #[doc =
        "The allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store.\nIf this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt name\".\nFor instance a certificate can be self-signed using the following command:\n'''\n  openssl x509 -req -days 200 -in example.com.csr \\\n    -signkey example.com.key \\\n    -out example.com.crt \\\n    -extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")\n'''\nA base64-encoded string."]
    pub cert: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the allowed custom CA certificates. This can be used to disambiguate the custom CA certificates."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts {
        DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts {
            cert: self.cert,
            display_name: self.display_name,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpec_TlsConfig_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_TlsConfig_CaCertsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTool_OpenApiSpec_TlsConfig_CaCertsRef {
        DialogflowCxTool_OpenApiSpec_TlsConfig_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_TlsConfig_CaCertsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cert` after provisioning.\nThe allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store.\nIf this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt name\".\nFor instance a certificate can be self-signed using the following command:\n'''\n  openssl x509 -req -days 200 -in example.com.csr \\\n    -signkey example.com.key \\\n    -out example.com.crt \\\n    -extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")\n'''\nA base64-encoded string."]
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
struct DialogflowCxTool_OpenApiSpec_TlsConfigDynamic {
    ca_certs: std::option::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec_TlsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts>>,
    dynamic: DialogflowCxTool_OpenApiSpec_TlsConfigDynamic,
}

impl DialogflowCxTool_OpenApiSpec_TlsConfig {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_TlsConfig_CaCerts>>,
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

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec_TlsConfig {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_TlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec_TlsConfig {}

impl BuildDialogflowCxTool_OpenApiSpec_TlsConfig {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec_TlsConfig {
        DialogflowCxTool_OpenApiSpec_TlsConfig {
            ca_certs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpec_TlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpec_TlsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTool_OpenApiSpec_TlsConfigRef {
        DialogflowCxTool_OpenApiSpec_TlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpec_TlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_TlsConfig_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTool_OpenApiSpecDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_Authentication>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig>>,
    tls_config: std::option::Option<terrars::DynamicBlock<DialogflowCxTool_OpenApiSpec_TlsConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTool_OpenApiSpec {
    text_schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_Authentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std::option::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<std::vec::Vec<DialogflowCxTool_OpenApiSpec_TlsConfig>>,
    dynamic: DialogflowCxTool_OpenApiSpecDynamic,
}

impl DialogflowCxTool_OpenApiSpec {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_Authentication>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec_TlsConfig>>,
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

impl terrars::ToListMappable for DialogflowCxTool_OpenApiSpec {
    type O = terrars::BlockAssignable<DialogflowCxTool_OpenApiSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_OpenApiSpec {
    #[doc =
        "The OpenAPI schema specified as a text.\nThis field is part of a union field 'schema': only one of 'textSchema' may be set."]
    pub text_schema: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTool_OpenApiSpec {
    pub fn build(self) -> DialogflowCxTool_OpenApiSpec {
        DialogflowCxTool_OpenApiSpec {
            text_schema: self.text_schema,
            authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_OpenApiSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_OpenApiSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTool_OpenApiSpecRef {
        DialogflowCxTool_OpenApiSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_OpenApiSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `text_schema` after provisioning.\nThe OpenAPI schema specified as a text.\nThis field is part of a union field 'schema': only one of 'textSchema' may be set."]
    pub fn text_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<DialogflowCxTool_OpenApiSpec_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTool_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxTool_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxTool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTool_Timeouts {}

impl BuildDialogflowCxTool_Timeouts {
    pub fn build(self) -> DialogflowCxTool_Timeouts {
        DialogflowCxTool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTool_TimeoutsRef {
        DialogflowCxTool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTool_TimeoutsRef {
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
