#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineAssistantData_ {
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
    assistant_id: terrars::PrimField<std::string::String>,
    collection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    engine_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_grounding_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_policy: std::option::Option<std::vec::Vec<DiscoveryEngineAssistant_CustomerPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_config: std::option::Option<std::vec::Vec<DiscoveryEngineAssistant_GenerationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineAssistant_Timeouts>,
    dynamic: DiscoveryEngineAssistantDynamic,
}

struct DiscoveryEngineAssistant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineAssistantData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineAssistant(std::rc::Rc<DiscoveryEngineAssistant_>);

impl DiscoveryEngineAssistant {
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
        "Set the field `description`.\nDescription for additional information. Expected to be shown on the\nconfiguration UI, not to the users of the assistant."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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
        "Set the field `web_grounding_type`.\nThe type of web grounding to use.\nThe supported values: 'WEB_GROUNDING_TYPE_DISABLED', 'WEB_GROUNDING_TYPE_GOOGLE_SEARCH', 'WEB_GROUNDING_TYPE_ENTERPRISE_WEB_SEARCH'."]
    pub fn set_web_grounding_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().web_grounding_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_policy`.\n"]
    pub fn set_customer_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineAssistant_CustomerPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generation_config`.\n"]
    pub fn set_generation_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineAssistant_GenerationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineAssistant_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `assistant_id` after provisioning.\nThe unique id of the assistant."]
    pub fn assistant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assistant_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe unique id of the collection."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription for additional information. Expected to be shown on the\nconfiguration UI, not to the users of the assistant."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe assistant display name.\n\nIt must be a UTF-8 encoded string with a length limit of 128 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe unique id of the engine."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name of the assistant.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}'\n\nIt must be a UTF-8 encoded string with a length limit of 1024 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_grounding_type` after provisioning.\nThe type of web grounding to use.\nThe supported values: 'WEB_GROUNDING_TYPE_DISABLED', 'WEB_GROUNDING_TYPE_GOOGLE_SEARCH', 'WEB_GROUNDING_TYPE_ENTERPRISE_WEB_SEARCH'."]
    pub fn web_grounding_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_grounding_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_policy` after provisioning.\n"]
    pub fn customer_policy(&self) -> terrars::ListRef<DiscoveryEngineAssistant_CustomerPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generation_config` after provisioning.\n"]
    pub fn generation_config(&self) -> terrars::ListRef<DiscoveryEngineAssistant_GenerationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineAssistant_TimeoutsRef {
        DiscoveryEngineAssistant_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DiscoveryEngineAssistant {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineAssistant { }

impl terrars::ToListMappable for DiscoveryEngineAssistant {
    type O = terrars::ListRef<DiscoveryEngineAssistantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineAssistant_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_assistant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineAssistant {
    pub tf_id: std::string::String,
    #[doc = "The unique id of the assistant."]
    pub assistant_id: terrars::PrimField<std::string::String>,
    #[doc = "The unique id of the collection."]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc = "The assistant display name.\n\nIt must be a UTF-8 encoded string with a length limit of 128 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The unique id of the engine."]
    pub engine_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineAssistant {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineAssistant {
        let out = DiscoveryEngineAssistant(std::rc::Rc::new(DiscoveryEngineAssistant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineAssistantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                assistant_id: self.assistant_id,
                collection_id: self.collection_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                web_grounding_type: core::default::Default::default(),
                customer_policy: core::default::Default::default(),
                generation_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineAssistantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineAssistantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineAssistantRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assistant_id` after provisioning.\nThe unique id of the assistant."]
    pub fn assistant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assistant_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe unique id of the collection."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription for additional information. Expected to be shown on the\nconfiguration UI, not to the users of the assistant."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe assistant display name.\n\nIt must be a UTF-8 encoded string with a length limit of 128 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe unique id of the engine."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name of the assistant.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}'\n\nIt must be a UTF-8 encoded string with a length limit of 1024 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_grounding_type` after provisioning.\nThe type of web grounding to use.\nThe supported values: 'WEB_GROUNDING_TYPE_DISABLED', 'WEB_GROUNDING_TYPE_GOOGLE_SEARCH', 'WEB_GROUNDING_TYPE_ENTERPRISE_WEB_SEARCH'."]
    pub fn web_grounding_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_grounding_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_policy` after provisioning.\n"]
    pub fn customer_policy(&self) -> terrars::ListRef<DiscoveryEngineAssistant_CustomerPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generation_config` after provisioning.\n"]
    pub fn generation_config(&self) -> terrars::ListRef<DiscoveryEngineAssistant_GenerationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineAssistant_TimeoutsRef {
        DiscoveryEngineAssistant_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineAssistantDynamic {
    customer_policy: std::option::Option<terrars::DynamicBlock<DiscoveryEngineAssistant_CustomerPolicy>>,
    generation_config: std::option::Option<terrars::DynamicBlock<DiscoveryEngineAssistant_GenerationConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_diacritics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_type: std::option::Option<terrars::PrimField<std::string::String>>,
    phrase: terrars::PrimField<std::string::String>,
}

impl DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases {
    #[doc =
        "Set the field `ignore_diacritics`.\nIf true, diacritical marks (e.g., accents, umlauts) are ignored when\nmatching banned phrases. For example, \"cafe\" would match \"café\"."]
    pub fn set_ignore_diacritics(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_diacritics = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `match_type`.\nMatch type for the banned phrase.\nThe supported values: 'SIMPLE_STRING_MATCH', 'WORD_BOUNDARY_STRING_MATCH'."]
    pub fn set_match_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases {
    type O = terrars::BlockAssignable<DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineAssistant_CustomerPolicy_BannedPhrases {
    #[doc = "The raw string content to be banned."]
    pub phrase: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineAssistant_CustomerPolicy_BannedPhrases {
    pub fn build(self) -> DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases {
        DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases {
            ignore_diacritics: core::default::Default::default(),
            match_type: core::default::Default::default(),
            phrase: self.phrase,
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineAssistant_CustomerPolicy_BannedPhrasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineAssistant_CustomerPolicy_BannedPhrasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineAssistant_CustomerPolicy_BannedPhrasesRef {
        DiscoveryEngineAssistant_CustomerPolicy_BannedPhrasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineAssistant_CustomerPolicy_BannedPhrasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ignore_diacritics` after provisioning.\nIf true, diacritical marks (e.g., accents, umlauts) are ignored when\nmatching banned phrases. For example, \"cafe\" would match \"café\"."]
    pub fn ignore_diacritics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_diacritics", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_type` after provisioning.\nMatch type for the banned phrase.\nThe supported values: 'SIMPLE_STRING_MATCH', 'WORD_BOUNDARY_STRING_MATCH'."]
    pub fn match_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }

    #[doc = "Get a reference to the value of field `phrase` after provisioning.\nThe raw string content to be banned."]
    pub fn phrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phrase", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    response_template: terrars::PrimField<std::string::String>,
    user_prompt_template: terrars::PrimField<std::string::String>,
}

impl DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig {
    #[doc =
        "Set the field `failure_mode`.\nDefines the failure mode for Model Armor sanitization.\nThe supported values: 'FAIL_OPEN', 'FAIL_CLOSED'."]
    pub fn set_failure_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failure_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig {
    #[doc =
        "The resource name of the Model Armor template for sanitizing assistant\nresponses. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the assistant\nresponse."]
    pub response_template: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name of the Model Armor template for sanitizing user\nprompts. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the user prompt."]
    pub user_prompt_template: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig {
    pub fn build(self) -> DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig {
        DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig {
            failure_mode: core::default::Default::default(),
            response_template: self.response_template,
            user_prompt_template: self.user_prompt_template,
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfigRef {
        DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_mode` after provisioning.\nDefines the failure mode for Model Armor sanitization.\nThe supported values: 'FAIL_OPEN', 'FAIL_CLOSED'."]
    pub fn failure_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_template` after provisioning.\nThe resource name of the Model Armor template for sanitizing assistant\nresponses. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the assistant\nresponse."]
    pub fn response_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_prompt_template` after provisioning.\nThe resource name of the Model Armor template for sanitizing user\nprompts. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the user prompt."]
    pub fn user_prompt_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_prompt_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineAssistant_CustomerPolicyDynamic {
    banned_phrases: std::option::Option<terrars::DynamicBlock<DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases>>,
    model_armor_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineAssistant_CustomerPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_phrases: std::option::Option<std::vec::Vec<DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_armor_config: std::option::Option<std::vec::Vec<DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig>>,
    dynamic: DiscoveryEngineAssistant_CustomerPolicyDynamic,
}

impl DiscoveryEngineAssistant_CustomerPolicy {
    #[doc = "Set the field `banned_phrases`.\n"]
    pub fn set_banned_phrases(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineAssistant_CustomerPolicy_BannedPhrases>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.banned_phrases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.banned_phrases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_armor_config`.\n"]
    pub fn set_model_armor_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_armor_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_armor_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineAssistant_CustomerPolicy {
    type O = terrars::BlockAssignable<DiscoveryEngineAssistant_CustomerPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineAssistant_CustomerPolicy {}

impl BuildDiscoveryEngineAssistant_CustomerPolicy {
    pub fn build(self) -> DiscoveryEngineAssistant_CustomerPolicy {
        DiscoveryEngineAssistant_CustomerPolicy {
            banned_phrases: core::default::Default::default(),
            model_armor_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineAssistant_CustomerPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineAssistant_CustomerPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineAssistant_CustomerPolicyRef {
        DiscoveryEngineAssistant_CustomerPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineAssistant_CustomerPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `banned_phrases` after provisioning.\n"]
    pub fn banned_phrases(&self) -> terrars::ListRef<DiscoveryEngineAssistant_CustomerPolicy_BannedPhrasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_phrases", self.base))
    }

    #[doc = "Get a reference to the value of field `model_armor_config` after provisioning.\n"]
    pub fn model_armor_config(&self) -> terrars::ListRef<DiscoveryEngineAssistant_CustomerPolicy_ModelArmorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_armor_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineAssistant_GenerationConfig_SystemInstruction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_system_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineAssistant_GenerationConfig_SystemInstruction {
    #[doc =
        "Set the field `additional_system_instruction`.\nAdditional system instruction that will be added to the default system instruction."]
    pub fn set_additional_system_instruction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_system_instruction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineAssistant_GenerationConfig_SystemInstruction {
    type O = terrars::BlockAssignable<DiscoveryEngineAssistant_GenerationConfig_SystemInstruction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineAssistant_GenerationConfig_SystemInstruction {}

impl BuildDiscoveryEngineAssistant_GenerationConfig_SystemInstruction {
    pub fn build(self) -> DiscoveryEngineAssistant_GenerationConfig_SystemInstruction {
        DiscoveryEngineAssistant_GenerationConfig_SystemInstruction {
            additional_system_instruction: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineAssistant_GenerationConfig_SystemInstructionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineAssistant_GenerationConfig_SystemInstructionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineAssistant_GenerationConfig_SystemInstructionRef {
        DiscoveryEngineAssistant_GenerationConfig_SystemInstructionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineAssistant_GenerationConfig_SystemInstructionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_system_instruction` after provisioning.\nAdditional system instruction that will be added to the default system instruction."]
    pub fn additional_system_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_system_instruction", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineAssistant_GenerationConfigDynamic {
    system_instruction: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineAssistant_GenerationConfig_SystemInstruction>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineAssistant_GenerationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_instruction: std::option::Option<std::vec::Vec<DiscoveryEngineAssistant_GenerationConfig_SystemInstruction>>,
    dynamic: DiscoveryEngineAssistant_GenerationConfigDynamic,
}

impl DiscoveryEngineAssistant_GenerationConfig {
    #[doc =
        "Set the field `default_language`.\nThe default language to use for the generation of the assistant response.\nUse an ISO 639-1 language code such as 'en'.\nIf not specified, the language will be automatically detected."]
    pub fn set_default_language(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_instruction`.\n"]
    pub fn set_system_instruction(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineAssistant_GenerationConfig_SystemInstruction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.system_instruction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.system_instruction = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineAssistant_GenerationConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineAssistant_GenerationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineAssistant_GenerationConfig {}

impl BuildDiscoveryEngineAssistant_GenerationConfig {
    pub fn build(self) -> DiscoveryEngineAssistant_GenerationConfig {
        DiscoveryEngineAssistant_GenerationConfig {
            default_language: core::default::Default::default(),
            system_instruction: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineAssistant_GenerationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineAssistant_GenerationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineAssistant_GenerationConfigRef {
        DiscoveryEngineAssistant_GenerationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineAssistant_GenerationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_language` after provisioning.\nThe default language to use for the generation of the assistant response.\nUse an ISO 639-1 language code such as 'en'.\nIf not specified, the language will be automatically detected."]
    pub fn default_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_language", self.base))
    }

    #[doc = "Get a reference to the value of field `system_instruction` after provisioning.\n"]
    pub fn system_instruction(&self) -> terrars::ListRef<DiscoveryEngineAssistant_GenerationConfig_SystemInstructionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_instruction", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineAssistant_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineAssistant_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineAssistant_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineAssistant_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineAssistant_Timeouts {}

impl BuildDiscoveryEngineAssistant_Timeouts {
    pub fn build(self) -> DiscoveryEngineAssistant_Timeouts {
        DiscoveryEngineAssistant_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineAssistant_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineAssistant_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineAssistant_TimeoutsRef {
        DiscoveryEngineAssistant_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineAssistant_TimeoutsRef {
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
