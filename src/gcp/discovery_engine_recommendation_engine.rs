#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineRecommendationEngineData_ {
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
    common_config: std::option::Option<std::vec::Vec<DiscoveryEngineRecommendationEngine_CommonConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_recommendation_engine_config: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineRecommendationEngine_Timeouts>,
    dynamic: DiscoveryEngineRecommendationEngineDynamic,
}

struct DiscoveryEngineRecommendationEngine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineRecommendationEngineData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineRecommendationEngine(std::rc::Rc<DiscoveryEngineRecommendationEngine_>);

impl DiscoveryEngineRecommendationEngine {
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
        "Set the field `industry_vertical`.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\"]"]
    pub fn set_industry_vertical(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().industry_vertical = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineRecommendationEngine_CommonConfig>>,
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

    #[doc = "Set the field `media_recommendation_engine_config`.\n"]
    pub fn set_media_recommendation_engine_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_recommendation_engine_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.media_recommendation_engine_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineRecommendationEngine_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Recommendation Engine."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the recommendation engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024 characters."]
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

    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> terrars::ListRef<DiscoveryEngineRecommendationEngine_CommonConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_recommendation_engine_config` after provisioning.\n"]
    pub fn media_recommendation_engine_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.media_recommendation_engine_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineRecommendationEngine_TimeoutsRef {
        DiscoveryEngineRecommendationEngine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DiscoveryEngineRecommendationEngine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineRecommendationEngine { }

impl terrars::ToListMappable for DiscoveryEngineRecommendationEngine {
    type O = terrars::ListRef<DiscoveryEngineRecommendationEngineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineRecommendationEngine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_recommendation_engine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine {
    pub tf_id: std::string::String,
    #[doc =
        "The data stores associated with this engine. For SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store."]
    pub data_store_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Unique ID to use for Recommendation Engine."]
    pub engine_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineRecommendationEngine {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineRecommendationEngine {
        let out = DiscoveryEngineRecommendationEngine(std::rc::Rc::new(DiscoveryEngineRecommendationEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineRecommendationEngineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_store_ids: self.data_store_ids,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                industry_vertical: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                common_config: core::default::Default::default(),
                media_recommendation_engine_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineRecommendationEngineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineRecommendationEngineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineRecommendationEngineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Recommendation Engine."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the recommendation engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024 characters."]
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

    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> terrars::ListRef<DiscoveryEngineRecommendationEngine_CommonConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_recommendation_engine_config` after provisioning.\n"]
    pub fn media_recommendation_engine_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.media_recommendation_engine_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineRecommendationEngine_TimeoutsRef {
        DiscoveryEngineRecommendationEngine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineRecommendationEngineDynamic {
    common_config: std::option::Option<terrars::DynamicBlock<DiscoveryEngineRecommendationEngine_CommonConfig>>,
    media_recommendation_engine_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngine_CommonConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineRecommendationEngine_CommonConfig {
    #[doc =
        "Set the field `company_name`.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd"]
    pub fn set_company_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.company_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineRecommendationEngine_CommonConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineRecommendationEngine_CommonConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine_CommonConfig {}

impl BuildDiscoveryEngineRecommendationEngine_CommonConfig {
    pub fn build(self) -> DiscoveryEngineRecommendationEngine_CommonConfig {
        DiscoveryEngineRecommendationEngine_CommonConfig { company_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineRecommendationEngine_CommonConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineRecommendationEngine_CommonConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineRecommendationEngine_CommonConfigRef {
        DiscoveryEngineRecommendationEngine_CommonConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineRecommendationEngine_CommonConfigRef {
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
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_window_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig {
    #[doc =
        "Set the field `time_window_days`.\nThe time window of which the engine is queried at training and\nprediction time. Positive integers only. The value translates to the\nlast X days of events. Currently required for the 'most-popular-items'\nengine."]
    pub fn set_time_window_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.time_window_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig {
    type O =
        terrars
        ::BlockAssignable<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig {}

impl BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig {
    pub fn build(
        self,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig {
            time_window_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfigRef {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `time_window_days` after provisioning.\nThe time window of which the engine is queried at training and\nprediction time. Positive integers only. The value translates to the\nlast X days of events. Currently required for the 'most-popular-items'\nengine."]
    pub fn time_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_window_days", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context_event_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig {
    #[doc =
        "Set the field `context_event_type`.\nThe type of event with which the engine is queried at prediction time.\nIf set to 'generic', only 'view-item', 'media-play',and\n'media-complete' will be used as 'context-event' in engine training. If\nset to 'view-home-page', 'view-home-page' will also be used as\n'context-events' in addition to 'view-item', 'media-play', and\n'media-complete'. Currently supported for the 'recommended-for-you'\nengine. Currently supported values: 'view-home-page', 'generic'."]
    pub fn set_context_event_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.context_event_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig {
    type O =
        terrars
        ::BlockAssignable<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig {}

impl BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig {
    pub fn build(
        self,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig {
            context_event_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfigRef {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `context_event_type` after provisioning.\nThe type of event with which the engine is queried at prediction time.\nIf set to 'generic', only 'view-item', 'media-play',and\n'media-complete' will be used as 'context-event' in engine training. If\nset to 'view-home-page', 'view-home-page' will also be used as\n'context-events' in addition to 'view-item', 'media-play', and\n'media-complete'. Currently supported for the 'recommended-for-you'\nengine. Currently supported values: 'view-home-page', 'generic'."]
    pub fn context_event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context_event_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigDynamic {
    most_popular_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig,
        >,
    >,
    recommended_for_you_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_popular_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recommended_for_you_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig,
        >,
    >,
    dynamic: DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigDynamic,
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig {
    #[doc = "Set the field `most_popular_config`.\n"]
    pub fn set_most_popular_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.most_popular_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.most_popular_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recommended_for_you_config`.\n"]
    pub fn set_recommended_for_you_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recommended_for_you_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recommended_for_you_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig {
    type O =
        terrars
        ::BlockAssignable<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig {}

impl BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig {
    pub fn build(self) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig {
            most_popular_config: core::default::Default::default(),
            recommended_for_you_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigRef {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `most_popular_config` after provisioning.\n"]
    pub fn most_popular_config(
        &self,
    ) -> terrars
    ::ListRef<
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_MostPopularConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.most_popular_config", self.base))
    }

    #[doc = "Get a reference to the value of field `recommended_for_you_config` after provisioning.\n"]
    pub fn recommended_for_you_config(
        &self,
    ) -> terrars
    ::ListRef<
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig_RecommendedForYouConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recommended_for_you_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_field_value_float: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig {
    #[doc =
        "Set the field `target_field`.\nThe name of the field to target. Currently supported values: 'watch-percentage', 'watch-time'."]
    pub fn set_target_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_field_value_float`.\nThe threshold to be applied to the target (e.g., 0.5)."]
    pub fn set_target_field_value_float(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_field_value_float = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig {
    type O =
        terrars
        ::BlockAssignable<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig {}

impl BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig {
    pub fn build(
        self,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig {
            target_field: core::default::Default::default(),
            target_field_value_float: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfigRef {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_field` after provisioning.\nThe name of the field to target. Currently supported values: 'watch-percentage', 'watch-time'."]
    pub fn target_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_field", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_field_value_float` after provisioning.\nThe threshold to be applied to the target (e.g., 0.5)."]
    pub fn target_field_value_float(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_field_value_float", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigDynamic {
    engine_features_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig>,
    >,
    optimization_objective_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optimization_objective: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_features_config: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optimization_objective_config: std
    ::option
    ::Option<
        std::vec::Vec<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig>,
    >,
    dynamic: DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigDynamic,
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig {
    #[doc =
        "Set the field `optimization_objective`.\nThe optimization objective. e.g., 'cvr'.\nThis field together with MediaRecommendationEngineConfig.type describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'ctr', 'cvr'.\nIf not specified, we choose default based on engine type. Default depends on type of recommendation:\n'recommended-for-you' => 'ctr'\n'others-you-may-like' => 'ctr'"]
    pub fn set_optimization_objective(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.optimization_objective = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `training_state`.\nThe training state that the engine is in (e.g. 'TRAINING' or 'PAUSED').\nSince part of the cost of running the service\nis frequency of training - this can be used to determine when to train\nengine in order to control cost. If not specified: the default value for\n'CreateEngine' method is 'TRAINING'. The default value for\n'UpdateEngine' method is to keep the state the same as before. Possible values: [\"PAUSED\", \"TRAINING\"]"]
    pub fn set_training_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.training_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of engine. e.g., 'recommended-for-you'.\nThis field together with MediaRecommendationEngineConfig.optimizationObjective describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'recommended-for-you', 'others-you-may-like',\n'more-like-this', 'most-popular-items'."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_features_config`.\n"]
    pub fn set_engine_features_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.engine_features_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.engine_features_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `optimization_objective_config`.\n"]
    pub fn set_optimization_objective_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.optimization_objective_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.optimization_objective_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig {}

impl BuildDiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig {
    pub fn build(self) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig {
            optimization_objective: core::default::Default::default(),
            training_state: core::default::Default::default(),
            type_: core::default::Default::default(),
            engine_features_config: core::default::Default::default(),
            optimization_objective_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigRef {
        DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `optimization_objective` after provisioning.\nThe optimization objective. e.g., 'cvr'.\nThis field together with MediaRecommendationEngineConfig.type describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'ctr', 'cvr'.\nIf not specified, we choose default based on engine type. Default depends on type of recommendation:\n'recommended-for-you' => 'ctr'\n'others-you-may-like' => 'ctr'"]
    pub fn optimization_objective(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optimization_objective", self.base))
    }

    #[doc =
        "Get a reference to the value of field `training_state` after provisioning.\nThe training state that the engine is in (e.g. 'TRAINING' or 'PAUSED').\nSince part of the cost of running the service\nis frequency of training - this can be used to determine when to train\nengine in order to control cost. If not specified: the default value for\n'CreateEngine' method is 'TRAINING'. The default value for\n'UpdateEngine' method is to keep the state the same as before. Possible values: [\"PAUSED\", \"TRAINING\"]"]
    pub fn training_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of engine. e.g., 'recommended-for-you'.\nThis field together with MediaRecommendationEngineConfig.optimizationObjective describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'recommended-for-you', 'others-you-may-like',\n'more-like-this', 'most-popular-items'."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_features_config` after provisioning.\n"]
    pub fn engine_features_config(
        &self,
    ) -> terrars
    ::ListRef<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_EngineFeaturesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engine_features_config", self.base))
    }

    #[doc = "Get a reference to the value of field `optimization_objective_config` after provisioning.\n"]
    pub fn optimization_objective_config(
        &self,
    ) -> terrars
    ::ListRef<DiscoveryEngineRecommendationEngine_MediaRecommendationEngineConfig_OptimizationObjectiveConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optimization_objective_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngine_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineRecommendationEngine_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineRecommendationEngine_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineRecommendationEngine_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineRecommendationEngine_Timeouts {}

impl BuildDiscoveryEngineRecommendationEngine_Timeouts {
    pub fn build(self) -> DiscoveryEngineRecommendationEngine_Timeouts {
        DiscoveryEngineRecommendationEngine_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineRecommendationEngine_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineRecommendationEngine_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineRecommendationEngine_TimeoutsRef {
        DiscoveryEngineRecommendationEngine_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineRecommendationEngine_TimeoutsRef {
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
