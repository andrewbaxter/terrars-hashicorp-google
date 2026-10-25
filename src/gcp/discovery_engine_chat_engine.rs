#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineChatEngineData_ {
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
    collection_id: terrars::PrimField<std::string::String>,
    data_store_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    engine_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    industry_vertical: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chat_engine_config: std::option::Option<std::vec::Vec<DiscoveryEngineChatEngine_ChatEngineConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_config: std::option::Option<std::vec::Vec<DiscoveryEngineChatEngine_CommonConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineChatEngine_Timeouts>,
    dynamic: DiscoveryEngineChatEngineDynamic,
}

struct DiscoveryEngineChatEngine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineChatEngineData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineChatEngine(std::rc::Rc<DiscoveryEngineChatEngine_>);

impl DiscoveryEngineChatEngine {
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
        "Set the field `industry_vertical`.\nThe industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\"]"]
    pub fn set_industry_vertical(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().industry_vertical = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chat_engine_config`.\n"]
    pub fn set_chat_engine_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineChatEngine_ChatEngineConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().chat_engine_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.chat_engine_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `common_config`.\n"]
    pub fn set_common_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineChatEngine_CommonConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().common_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.common_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineChatEngine_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `chat_engine_metadata` after provisioning.\nAdditional information of the Chat Engine."]
    pub fn chat_engine_metadata(&self) -> terrars::ListRef<DiscoveryEngineChatEngine_ChatEngineMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chat_engine_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be 'SOLUTION_TYPE_CHAT'."]
    pub fn data_store_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe ID to use for chat engine."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the chat engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chat_engine_config` after provisioning.\n"]
    pub fn chat_engine_config(&self) -> terrars::ListRef<DiscoveryEngineChatEngine_ChatEngineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chat_engine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> terrars::ListRef<DiscoveryEngineChatEngine_CommonConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineChatEngine_TimeoutsRef {
        DiscoveryEngineChatEngine_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DiscoveryEngineChatEngine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineChatEngine { }

impl terrars::ToListMappable for DiscoveryEngineChatEngine {
    type O = terrars::ListRef<DiscoveryEngineChatEngineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineChatEngine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_chat_engine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineChatEngine {
    pub tf_id: std::string::String,
    #[doc = "The collection ID."]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be 'SOLUTION_TYPE_CHAT'."]
    pub data_store_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The ID to use for chat engine."]
    pub engine_id: terrars::PrimField<std::string::String>,
    #[doc = "Location."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineChatEngine {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineChatEngine {
        let out = DiscoveryEngineChatEngine(std::rc::Rc::new(DiscoveryEngineChatEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineChatEngineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                collection_id: self.collection_id,
                data_store_ids: self.data_store_ids,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                industry_vertical: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                chat_engine_config: core::default::Default::default(),
                common_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineChatEngineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineChatEngineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineChatEngineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chat_engine_metadata` after provisioning.\nAdditional information of the Chat Engine."]
    pub fn chat_engine_metadata(&self) -> terrars::ListRef<DiscoveryEngineChatEngine_ChatEngineMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chat_engine_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be 'SOLUTION_TYPE_CHAT'."]
    pub fn data_store_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe ID to use for chat engine."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the chat engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chat_engine_config` after provisioning.\n"]
    pub fn chat_engine_config(&self) -> terrars::ListRef<DiscoveryEngineChatEngine_ChatEngineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chat_engine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> terrars::ListRef<DiscoveryEngineChatEngine_CommonConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineChatEngine_TimeoutsRef {
        DiscoveryEngineChatEngine_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineChatEngineDynamic {
    chat_engine_config: std::option::Option<terrars::DynamicBlock<DiscoveryEngineChatEngine_ChatEngineConfig>>,
    common_config: std::option::Option<terrars::DynamicBlock<DiscoveryEngineChatEngine_CommonConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineChatEngine_ChatEngineMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialogflow_agent: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineChatEngine_ChatEngineMetadata {
    #[doc = "Set the field `dialogflow_agent`.\n"]
    pub fn set_dialogflow_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dialogflow_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineChatEngine_ChatEngineMetadata {
    type O = terrars::BlockAssignable<DiscoveryEngineChatEngine_ChatEngineMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineChatEngine_ChatEngineMetadata {}

impl BuildDiscoveryEngineChatEngine_ChatEngineMetadata {
    pub fn build(self) -> DiscoveryEngineChatEngine_ChatEngineMetadata {
        DiscoveryEngineChatEngine_ChatEngineMetadata { dialogflow_agent: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineChatEngine_ChatEngineMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineChatEngine_ChatEngineMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineChatEngine_ChatEngineMetadataRef {
        DiscoveryEngineChatEngine_ChatEngineMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineChatEngine_ChatEngineMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dialogflow_agent` after provisioning.\n"]
    pub fn dialogflow_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dialogflow_agent", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business: std::option::Option<terrars::PrimField<std::string::String>>,
    default_language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    time_zone: terrars::PrimField<std::string::String>,
}

impl DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig {
    #[doc =
        "Set the field `business`.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn set_business(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.business = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nAgent location for Agent creation, currently supported values: global/us/eu, it needs to be the same region as the Chat Engine."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig {
    #[doc =
        "The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
    pub default_language_code: terrars::PrimField<std::string::String>,
    #[doc =
        "The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris."]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig {
    pub fn build(self) -> DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig {
        DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig {
            business: core::default::Default::default(),
            default_language_code: self.default_language_code,
            location: core::default::Default::default(),
            time_zone: self.time_zone,
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfigRef {
        DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `business` after provisioning.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn business(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
    pub fn default_language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nAgent location for Agent creation, currently supported values: global/us/eu, it needs to be the same region as the Chat Engine."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineChatEngine_ChatEngineConfigDynamic {
    agent_creation_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineChatEngine_ChatEngineConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_cross_region: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialogflow_agent_to_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_creation_config: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig>>,
    dynamic: DiscoveryEngineChatEngine_ChatEngineConfigDynamic,
}

impl DiscoveryEngineChatEngine_ChatEngineConfig {
    #[doc =
        "Set the field `allow_cross_region`.\nIf the flag set to true, we allow the agent and engine are in\ndifferent locations, otherwise the agent and engine are required to be\nin the same location. The flag is set to false by default.\nNote that the 'allow_cross_region' are one-time consumed by and passed\nto EngineService.CreateEngine. It means they cannot be retrieved using\nEngineService.GetEngine or EngineService.ListEngines API after engine\ncreation."]
    pub fn set_allow_cross_region(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_cross_region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dialogflow_agent_to_link`.\nThe resource name of an existing Dialogflow agent to link to this Chat Engine. Format: 'projects/<Project_ID>/locations/<Location_ID>/agents/<Agent_ID>'.\nExactly one of 'agent_creation_config' or 'dialogflow_agent_to_link' must be set."]
    pub fn set_dialogflow_agent_to_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dialogflow_agent_to_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_creation_config`.\n"]
    pub fn set_agent_creation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent_creation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_creation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineChatEngine_ChatEngineConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineChatEngine_ChatEngineConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineChatEngine_ChatEngineConfig {}

impl BuildDiscoveryEngineChatEngine_ChatEngineConfig {
    pub fn build(self) -> DiscoveryEngineChatEngine_ChatEngineConfig {
        DiscoveryEngineChatEngine_ChatEngineConfig {
            allow_cross_region: core::default::Default::default(),
            dialogflow_agent_to_link: core::default::Default::default(),
            agent_creation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineChatEngine_ChatEngineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineChatEngine_ChatEngineConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineChatEngine_ChatEngineConfigRef {
        DiscoveryEngineChatEngine_ChatEngineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineChatEngine_ChatEngineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_cross_region` after provisioning.\nIf the flag set to true, we allow the agent and engine are in\ndifferent locations, otherwise the agent and engine are required to be\nin the same location. The flag is set to false by default.\nNote that the 'allow_cross_region' are one-time consumed by and passed\nto EngineService.CreateEngine. It means they cannot be retrieved using\nEngineService.GetEngine or EngineService.ListEngines API after engine\ncreation."]
    pub fn allow_cross_region(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_cross_region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dialogflow_agent_to_link` after provisioning.\nThe resource name of an existing Dialogflow agent to link to this Chat Engine. Format: 'projects/<Project_ID>/locations/<Location_ID>/agents/<Agent_ID>'.\nExactly one of 'agent_creation_config' or 'dialogflow_agent_to_link' must be set."]
    pub fn dialogflow_agent_to_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dialogflow_agent_to_link", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_creation_config` after provisioning.\n"]
    pub fn agent_creation_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineChatEngine_ChatEngineConfig_AgentCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_creation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineChatEngine_CommonConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineChatEngine_CommonConfig {
    #[doc =
        "Set the field `company_name`.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features."]
    pub fn set_company_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.company_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineChatEngine_CommonConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineChatEngine_CommonConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineChatEngine_CommonConfig {}

impl BuildDiscoveryEngineChatEngine_CommonConfig {
    pub fn build(self) -> DiscoveryEngineChatEngine_CommonConfig {
        DiscoveryEngineChatEngine_CommonConfig { company_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineChatEngine_CommonConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineChatEngine_CommonConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineChatEngine_CommonConfigRef {
        DiscoveryEngineChatEngine_CommonConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineChatEngine_CommonConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `company_name` after provisioning.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features."]
    pub fn company_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineChatEngine_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineChatEngine_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineChatEngine_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineChatEngine_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineChatEngine_Timeouts {}

impl BuildDiscoveryEngineChatEngine_Timeouts {
    pub fn build(self) -> DiscoveryEngineChatEngine_Timeouts {
        DiscoveryEngineChatEngine_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineChatEngine_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineChatEngine_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineChatEngine_TimeoutsRef {
        DiscoveryEngineChatEngine_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineChatEngine_TimeoutsRef {
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
