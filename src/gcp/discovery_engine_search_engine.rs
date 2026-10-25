#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineSearchEngineData_ {
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
    app_type: std::option::Option<terrars::PrimField<std::string::String>>,
    collection_id: terrars::PrimField<std::string::String>,
    data_store_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_analytics: std::option::Option<terrars::PrimField<bool>>,
    display_name: terrars::PrimField<std::string::String>,
    engine_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    industry_vertical: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    procurement_contact_emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_config: std::option::Option<std::vec::Vec<DiscoveryEngineSearchEngine_CommonConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_graph_config: std::option::Option<std::vec::Vec<DiscoveryEngineSearchEngine_KnowledgeGraphConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_engine_config: std::option::Option<std::vec::Vec<DiscoveryEngineSearchEngine_SearchEngineConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineSearchEngine_Timeouts>,
    dynamic: DiscoveryEngineSearchEngineDynamic,
}

struct DiscoveryEngineSearchEngine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineSearchEngineData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineSearchEngine(std::rc::Rc<DiscoveryEngineSearchEngine_>);

impl DiscoveryEngineSearchEngine {
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
        "Set the field `app_type`.\nThis is the application type this engine resource represents.\nThe supported values: 'APP_TYPE_UNSPECIFIED', 'APP_TYPE_INTRANET'."]
    pub fn set_app_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().app_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_analytics`.\nWhether to disable analytics for searches performed on this engine."]
    pub fn set_disable_analytics(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_analytics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features`.\nA map of the feature config for the engine to opt in or opt out of features."]
    pub fn set_features(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().features = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `industry_vertical`.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn set_industry_vertical(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().industry_vertical = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nThe KMS key to be used to protect this Engine at creation time.\n\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\n\nIf this field is set and processed successfully, the Engine will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `procurement_contact_emails`.\nThe emails of the procurement contacts."]
    pub fn set_procurement_contact_emails(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().procurement_contact_emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_config`.\n"]
    pub fn set_common_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineSearchEngine_CommonConfig>>,
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

    #[doc = "Set the field `knowledge_graph_config`.\n"]
    pub fn set_knowledge_graph_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineSearchEngine_KnowledgeGraphConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().knowledge_graph_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.knowledge_graph_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `search_engine_config`.\n"]
    pub fn set_search_engine_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineSearchEngine_SearchEngineConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().search_engine_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.search_engine_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineSearchEngine_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `app_type` after provisioning.\nThis is the application type this engine resource represents.\nThe supported values: 'APP_TYPE_UNSPECIFIED', 'APP_TYPE_INTRANET'."]
    pub fn app_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_type", self.extract_ref()))
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
        "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_analytics` after provisioning.\nWhether to disable analytics for searches performed on this engine."]
    pub fn disable_analytics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_analytics", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Search Engine App."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `features` after provisioning.\nA map of the feature config for the engine to opt in or opt out of features."]
    pub fn features(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect this Engine at creation time.\n\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\n\nIf this field is set and processed successfully, the Engine will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the search engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `procurement_contact_emails` after provisioning.\nThe emails of the procurement contacts."]
    pub fn procurement_contact_emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.procurement_contact_emails", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> terrars::ListRef<DiscoveryEngineSearchEngine_CommonConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_graph_config` after provisioning.\n"]
    pub fn knowledge_graph_config(&self) -> terrars::ListRef<DiscoveryEngineSearchEngine_KnowledgeGraphConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_graph_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_engine_config` after provisioning.\n"]
    pub fn search_engine_config(&self) -> terrars::ListRef<DiscoveryEngineSearchEngine_SearchEngineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search_engine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineSearchEngine_TimeoutsRef {
        DiscoveryEngineSearchEngine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DiscoveryEngineSearchEngine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineSearchEngine { }

impl terrars::ToListMappable for DiscoveryEngineSearchEngine {
    type O = terrars::ListRef<DiscoveryEngineSearchEngineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineSearchEngine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_search_engine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineSearchEngine {
    pub tf_id: std::string::String,
    #[doc = "The collection ID."]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store."]
    pub data_store_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Unique ID to use for Search Engine App."]
    pub engine_id: terrars::PrimField<std::string::String>,
    #[doc = "Location."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineSearchEngine {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineSearchEngine {
        let out = DiscoveryEngineSearchEngine(std::rc::Rc::new(DiscoveryEngineSearchEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineSearchEngineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_type: core::default::Default::default(),
                collection_id: self.collection_id,
                data_store_ids: self.data_store_ids,
                deletion_policy: core::default::Default::default(),
                disable_analytics: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                features: core::default::Default::default(),
                id: core::default::Default::default(),
                industry_vertical: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                location: self.location,
                procurement_contact_emails: core::default::Default::default(),
                project: core::default::Default::default(),
                common_config: core::default::Default::default(),
                knowledge_graph_config: core::default::Default::default(),
                search_engine_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineSearchEngineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineSearchEngineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineSearchEngineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_type` after provisioning.\nThis is the application type this engine resource represents.\nThe supported values: 'APP_TYPE_UNSPECIFIED', 'APP_TYPE_INTRANET'."]
    pub fn app_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_type", self.extract_ref()))
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
        "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_analytics` after provisioning.\nWhether to disable analytics for searches performed on this engine."]
    pub fn disable_analytics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_analytics", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Search Engine App."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `features` after provisioning.\nA map of the feature config for the engine to opt in or opt out of features."]
    pub fn features(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect this Engine at creation time.\n\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\n\nIf this field is set and processed successfully, the Engine will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the search engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `procurement_contact_emails` after provisioning.\nThe emails of the procurement contacts."]
    pub fn procurement_contact_emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.procurement_contact_emails", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> terrars::ListRef<DiscoveryEngineSearchEngine_CommonConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_graph_config` after provisioning.\n"]
    pub fn knowledge_graph_config(&self) -> terrars::ListRef<DiscoveryEngineSearchEngine_KnowledgeGraphConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_graph_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_engine_config` after provisioning.\n"]
    pub fn search_engine_config(&self) -> terrars::ListRef<DiscoveryEngineSearchEngine_SearchEngineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search_engine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineSearchEngine_TimeoutsRef {
        DiscoveryEngineSearchEngine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineSearchEngineDynamic {
    common_config: std::option::Option<terrars::DynamicBlock<DiscoveryEngineSearchEngine_CommonConfig>>,
    knowledge_graph_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineSearchEngine_KnowledgeGraphConfig>>,
    search_engine_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineSearchEngine_SearchEngineConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngine_CommonConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineSearchEngine_CommonConfig {
    #[doc =
        "Set the field `company_name`.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd"]
    pub fn set_company_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.company_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineSearchEngine_CommonConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineSearchEngine_CommonConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineSearchEngine_CommonConfig {}

impl BuildDiscoveryEngineSearchEngine_CommonConfig {
    pub fn build(self) -> DiscoveryEngineSearchEngine_CommonConfig {
        DiscoveryEngineSearchEngine_CommonConfig { company_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineSearchEngine_CommonConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineSearchEngine_CommonConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineSearchEngine_CommonConfigRef {
        DiscoveryEngineSearchEngine_CommonConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineSearchEngine_CommonConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `company_name` after provisioning.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd"]
    pub fn company_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_private_kg_auto_complete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_private_kg_enrichment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_private_kg_query_ui_chips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_private_kg_query_understanding: std::option::Option<terrars::PrimField<bool>>,
}

impl DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig {
    #[doc =
        "Set the field `disable_private_kg_auto_complete`.\nWhether to disable the private KG auto complete for the engine."]
    pub fn set_disable_private_kg_auto_complete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_private_kg_auto_complete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_private_kg_enrichment`.\nWhether to disable the private KG enrichment for the engine."]
    pub fn set_disable_private_kg_enrichment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_private_kg_enrichment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_private_kg_query_ui_chips`.\nWhether to disable the private KG for query UI chips."]
    pub fn set_disable_private_kg_query_ui_chips(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_private_kg_query_ui_chips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_private_kg_query_understanding`.\nWhether to disable the private KG query understanding for the engine."]
    pub fn set_disable_private_kg_query_understanding(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_private_kg_query_understanding = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig {}

impl BuildDiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig {
    pub fn build(self) -> DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig {
        DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig {
            disable_private_kg_auto_complete: core::default::Default::default(),
            disable_private_kg_enrichment: core::default::Default::default(),
            disable_private_kg_query_ui_chips: core::default::Default::default(),
            disable_private_kg_query_understanding: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfigRef {
        DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_private_kg_auto_complete` after provisioning.\nWhether to disable the private KG auto complete for the engine."]
    pub fn disable_private_kg_auto_complete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_private_kg_auto_complete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_private_kg_enrichment` after provisioning.\nWhether to disable the private KG enrichment for the engine."]
    pub fn disable_private_kg_enrichment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_private_kg_enrichment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_private_kg_query_ui_chips` after provisioning.\nWhether to disable the private KG for query UI chips."]
    pub fn disable_private_kg_query_ui_chips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_private_kg_query_ui_chips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_private_kg_query_understanding` after provisioning.\nWhether to disable the private KG query understanding for the engine."]
    pub fn disable_private_kg_query_understanding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_private_kg_query_understanding", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineSearchEngine_KnowledgeGraphConfigDynamic {
    feature_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngine_KnowledgeGraphConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_knowledge_graph_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cloud_knowledge_graph: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_knowledge_graph: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_config: std::option::Option<std::vec::Vec<DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig>>,
    dynamic: DiscoveryEngineSearchEngine_KnowledgeGraphConfigDynamic,
}

impl DiscoveryEngineSearchEngine_KnowledgeGraphConfig {
    #[doc = "Set the field `cloud_knowledge_graph_types`.\nSpecify entity types to support."]
    pub fn set_cloud_knowledge_graph_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cloud_knowledge_graph_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_cloud_knowledge_graph`.\nWhether to enable the Cloud Knowledge Graph for the engine."]
    pub fn set_enable_cloud_knowledge_graph(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_cloud_knowledge_graph = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_private_knowledge_graph`.\nWhether to enable the Private Knowledge Graph for the engine."]
    pub fn set_enable_private_knowledge_graph(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_knowledge_graph = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_config`.\n"]
    pub fn set_feature_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.feature_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineSearchEngine_KnowledgeGraphConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineSearchEngine_KnowledgeGraphConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineSearchEngine_KnowledgeGraphConfig {}

impl BuildDiscoveryEngineSearchEngine_KnowledgeGraphConfig {
    pub fn build(self) -> DiscoveryEngineSearchEngine_KnowledgeGraphConfig {
        DiscoveryEngineSearchEngine_KnowledgeGraphConfig {
            cloud_knowledge_graph_types: core::default::Default::default(),
            enable_cloud_knowledge_graph: core::default::Default::default(),
            enable_private_knowledge_graph: core::default::Default::default(),
            feature_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineSearchEngine_KnowledgeGraphConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineSearchEngine_KnowledgeGraphConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineSearchEngine_KnowledgeGraphConfigRef {
        DiscoveryEngineSearchEngine_KnowledgeGraphConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineSearchEngine_KnowledgeGraphConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_knowledge_graph_types` after provisioning.\nSpecify entity types to support."]
    pub fn cloud_knowledge_graph_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_knowledge_graph_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_cloud_knowledge_graph` after provisioning.\nWhether to enable the Cloud Knowledge Graph for the engine."]
    pub fn enable_cloud_knowledge_graph(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cloud_knowledge_graph", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_knowledge_graph` after provisioning.\nWhether to enable the Private Knowledge Graph for the engine."]
    pub fn enable_private_knowledge_graph(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_knowledge_graph", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_config` after provisioning.\n"]
    pub fn feature_config(&self) -> terrars::ListRef<DiscoveryEngineSearchEngine_KnowledgeGraphConfig_FeatureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngine_SearchEngineConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_subscription_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_add_ons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineSearchEngine_SearchEngineConfig {
    #[doc =
        "Set the field `required_subscription_tier`.\nThe required subscription tier of this engine.\n\nIf the required subscription tier is search, user with higher license tier like assist can still access the standalone app associated with this engine. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_ENTERPRISE_EMERGING\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\", \"SUBSCRIPTION_TIER_EDU_EMERGING\", \"SUBSCRIPTION_TIER_EDU_PRO_EMERGING\", \"SUBSCRIPTION_TIER_FRONTLINE_STARTER\"]"]
    pub fn set_required_subscription_tier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.required_subscription_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search_add_ons`.\nThe add-on that this search engine enables. Possible values: [\"SEARCH_ADD_ON_LLM\"]"]
    pub fn set_search_add_ons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.search_add_ons = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search_tier`.\nThe search feature tier of this engine. Defaults to SearchTier.SEARCH_TIER_STANDARD if not specified. Default value: \"SEARCH_TIER_STANDARD\" Possible values: [\"SEARCH_TIER_STANDARD\", \"SEARCH_TIER_ENTERPRISE\"]"]
    pub fn set_search_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search_tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineSearchEngine_SearchEngineConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineSearchEngine_SearchEngineConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineSearchEngine_SearchEngineConfig {}

impl BuildDiscoveryEngineSearchEngine_SearchEngineConfig {
    pub fn build(self) -> DiscoveryEngineSearchEngine_SearchEngineConfig {
        DiscoveryEngineSearchEngine_SearchEngineConfig {
            required_subscription_tier: core::default::Default::default(),
            search_add_ons: core::default::Default::default(),
            search_tier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineSearchEngine_SearchEngineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineSearchEngine_SearchEngineConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineSearchEngine_SearchEngineConfigRef {
        DiscoveryEngineSearchEngine_SearchEngineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineSearchEngine_SearchEngineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `required_subscription_tier` after provisioning.\nThe required subscription tier of this engine.\n\nIf the required subscription tier is search, user with higher license tier like assist can still access the standalone app associated with this engine. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_ENTERPRISE_EMERGING\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\", \"SUBSCRIPTION_TIER_EDU_EMERGING\", \"SUBSCRIPTION_TIER_EDU_PRO_EMERGING\", \"SUBSCRIPTION_TIER_FRONTLINE_STARTER\"]"]
    pub fn required_subscription_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required_subscription_tier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search_add_ons` after provisioning.\nThe add-on that this search engine enables. Possible values: [\"SEARCH_ADD_ON_LLM\"]"]
    pub fn search_add_ons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search_add_ons", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search_tier` after provisioning.\nThe search feature tier of this engine. Defaults to SearchTier.SEARCH_TIER_STANDARD if not specified. Default value: \"SEARCH_TIER_STANDARD\" Possible values: [\"SEARCH_TIER_STANDARD\", \"SEARCH_TIER_ENTERPRISE\"]"]
    pub fn search_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngine_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineSearchEngine_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineSearchEngine_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineSearchEngine_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineSearchEngine_Timeouts {}

impl BuildDiscoveryEngineSearchEngine_Timeouts {
    pub fn build(self) -> DiscoveryEngineSearchEngine_Timeouts {
        DiscoveryEngineSearchEngine_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineSearchEngine_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineSearchEngine_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineSearchEngine_TimeoutsRef {
        DiscoveryEngineSearchEngine_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineSearchEngine_TimeoutsRef {
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
