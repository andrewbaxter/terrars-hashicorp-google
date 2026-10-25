#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ModelArmorFloorsettingData_ {
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
    enable_floor_setting_enforcement: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integrated_services: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_platform_floor_setting: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_AiPlatformFloorSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_config: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    floor_setting_metadata: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_FloorSettingMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_mcp_server_floor_setting: std
    ::option
    ::Option<std::vec::Vec<ModelArmorFloorsetting_GoogleMcpServerFloorSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ModelArmorFloorsetting_Timeouts>,
    dynamic: ModelArmorFloorsettingDynamic,
}

struct ModelArmorFloorsetting_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ModelArmorFloorsettingData_>,
}

#[derive(Clone)]
pub struct ModelArmorFloorsetting(std::rc::Rc<ModelArmorFloorsetting_>);

impl ModelArmorFloorsetting {
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

    #[doc = "Set the field `enable_floor_setting_enforcement`.\nFloor Settings enforcement status."]
    pub fn set_enable_floor_setting_enforcement(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_floor_setting_enforcement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `integrated_services`.\nList of integrated services for which the floor setting is applicable."]
    pub fn set_integrated_services(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().integrated_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ai_platform_floor_setting`.\n"]
    pub fn set_ai_platform_floor_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorFloorsetting_AiPlatformFloorSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ai_platform_floor_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ai_platform_floor_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_config`.\n"]
    pub fn set_filter_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `floor_setting_metadata`.\n"]
    pub fn set_floor_setting_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FloorSettingMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().floor_setting_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.floor_setting_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_mcp_server_floor_setting`.\n"]
    pub fn set_google_mcp_server_floor_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorFloorsetting_GoogleMcpServerFloorSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().google_mcp_server_floor_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.google_mcp_server_floor_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ModelArmorFloorsetting_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_floor_setting_enforcement` after provisioning.\nFloor Settings enforcement status."]
    pub fn enable_floor_setting_enforcement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_floor_setting_enforcement", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integrated_services` after provisioning.\nList of integrated services for which the floor setting is applicable."]
    pub fn integrated_services(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integrated_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nWill be any one of these:\n\n* 'projects/{project}'\n* 'folders/{folder}'\n* 'organizations/{organizationId}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ai_platform_floor_setting` after provisioning.\n"]
    pub fn ai_platform_floor_setting(&self) -> terrars::ListRef<ModelArmorFloorsetting_AiPlatformFloorSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ai_platform_floor_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `floor_setting_metadata` after provisioning.\n"]
    pub fn floor_setting_metadata(&self) -> terrars::ListRef<ModelArmorFloorsetting_FloorSettingMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.floor_setting_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `google_mcp_server_floor_setting` after provisioning.\n"]
    pub fn google_mcp_server_floor_setting(
        &self,
    ) -> terrars::ListRef<ModelArmorFloorsetting_GoogleMcpServerFloorSettingRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.google_mcp_server_floor_setting", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorFloorsetting_TimeoutsRef {
        ModelArmorFloorsetting_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ModelArmorFloorsetting {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ModelArmorFloorsetting { }

impl terrars::ToListMappable for ModelArmorFloorsetting {
    type O = terrars::ListRef<ModelArmorFloorsettingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ModelArmorFloorsetting_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_model_armor_floorsetting".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildModelArmorFloorsetting {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Will be any one of these:\n\n* 'projects/{project}'\n* 'folders/{folder}'\n* 'organizations/{organizationId}'"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildModelArmorFloorsetting {
    pub fn build(self, stack: &mut terrars::Stack) -> ModelArmorFloorsetting {
        let out = ModelArmorFloorsetting(std::rc::Rc::new(ModelArmorFloorsetting_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ModelArmorFloorsettingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enable_floor_setting_enforcement: core::default::Default::default(),
                id: core::default::Default::default(),
                integrated_services: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                ai_platform_floor_setting: core::default::Default::default(),
                filter_config: core::default::Default::default(),
                floor_setting_metadata: core::default::Default::default(),
                google_mcp_server_floor_setting: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ModelArmorFloorsettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ModelArmorFloorsettingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_floor_setting_enforcement` after provisioning.\nFloor Settings enforcement status."]
    pub fn enable_floor_setting_enforcement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_floor_setting_enforcement", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integrated_services` after provisioning.\nList of integrated services for which the floor setting is applicable."]
    pub fn integrated_services(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integrated_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nWill be any one of these:\n\n* 'projects/{project}'\n* 'folders/{folder}'\n* 'organizations/{organizationId}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ai_platform_floor_setting` after provisioning.\n"]
    pub fn ai_platform_floor_setting(&self) -> terrars::ListRef<ModelArmorFloorsetting_AiPlatformFloorSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ai_platform_floor_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `floor_setting_metadata` after provisioning.\n"]
    pub fn floor_setting_metadata(&self) -> terrars::ListRef<ModelArmorFloorsetting_FloorSettingMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.floor_setting_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `google_mcp_server_floor_setting` after provisioning.\n"]
    pub fn google_mcp_server_floor_setting(
        &self,
    ) -> terrars::ListRef<ModelArmorFloorsetting_GoogleMcpServerFloorSettingRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.google_mcp_server_floor_setting", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorFloorsetting_TimeoutsRef {
        ModelArmorFloorsetting_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorFloorsettingDynamic {
    ai_platform_floor_setting: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_AiPlatformFloorSetting>>,
    filter_config: std::option::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig>>,
    floor_setting_metadata: std::option::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FloorSettingMetadata>>,
    google_mcp_server_floor_setting: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_GoogleMcpServerFloorSetting>>,
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_AiPlatformFloorSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cloud_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_and_block: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_only: std::option::Option<terrars::PrimField<bool>>,
}

impl ModelArmorFloorsetting_AiPlatformFloorSetting {
    #[doc = "Set the field `enable_cloud_logging`.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn set_enable_cloud_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_cloud_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_and_block`.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn set_inspect_and_block(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inspect_and_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_only`.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn set_inspect_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inspect_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_AiPlatformFloorSetting {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_AiPlatformFloorSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_AiPlatformFloorSetting {}

impl BuildModelArmorFloorsetting_AiPlatformFloorSetting {
    pub fn build(self) -> ModelArmorFloorsetting_AiPlatformFloorSetting {
        ModelArmorFloorsetting_AiPlatformFloorSetting {
            enable_cloud_logging: core::default::Default::default(),
            inspect_and_block: core::default::Default::default(),
            inspect_only: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_AiPlatformFloorSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_AiPlatformFloorSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_AiPlatformFloorSettingRef {
        ModelArmorFloorsetting_AiPlatformFloorSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_AiPlatformFloorSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_cloud_logging` after provisioning.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn enable_cloud_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cloud_logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_and_block` after provisioning.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn inspect_and_block(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_and_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_only` after provisioning.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn inspect_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_only", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings {
    #[doc =
        "Set the field `filter_enforcement`.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_enforcement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings {}

impl BuildModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings {
        ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings {
            filter_enforcement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettingsRef {
        ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_enforcement", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.confidence_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter_enforcement`.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_enforcement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings {}

impl BuildModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings {
        ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings {
            confidence_level: core::default::Default::default(),
            filter_enforcement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettingsRef {
        ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_enforcement", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_level: std::option::Option<terrars::PrimField<std::string::String>>,
    filter_type: terrars::PrimField<std::string::String>,
}

impl ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.confidence_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters {
    #[doc = "Possible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub filter_type: terrars::PrimField<std::string::String>,
}

impl BuildModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters {
        ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters {
            confidence_level: core::default::Default::default(),
            filter_type: self.filter_type,
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFiltersRef {
        ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_type` after provisioning.\nPossible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub fn filter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorFloorsetting_FilterConfig_RaiSettingsDynamic {
    rai_filters: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters>>,
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig_RaiSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rai_filters: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters>>,
    dynamic: ModelArmorFloorsetting_FilterConfig_RaiSettingsDynamic,
}

impl ModelArmorFloorsetting_FilterConfig_RaiSettings {
    #[doc = "Set the field `rai_filters`.\n"]
    pub fn set_rai_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFilters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rai_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig_RaiSettings {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_RaiSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig_RaiSettings {}

impl BuildModelArmorFloorsetting_FilterConfig_RaiSettings {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig_RaiSettings {
        ModelArmorFloorsetting_FilterConfig_RaiSettings {
            rai_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfig_RaiSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfig_RaiSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FilterConfig_RaiSettingsRef {
        ModelArmorFloorsetting_FilterConfig_RaiSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfig_RaiSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rai_filters` after provisioning.\n"]
    pub fn rai_filters(&self) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfig_RaiSettings_RaiFiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rai_filters", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig {
    #[doc =
        "Set the field `deidentify_template`.\nOptional Sensitive Data Protection Deidentify template resource name.\n\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\n\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn set_deidentify_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deidentify_template = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_template`.\nSensitive Data Protection inspect template resource name\n\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\n\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn set_inspect_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inspect_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig {}

impl BuildModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig {
        ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig {
            deidentify_template: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfigRef {
        ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deidentify_template` after provisioning.\nOptional Sensitive Data Protection Deidentify template resource name.\n\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\n\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deidentify_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_template` after provisioning.\nSensitive Data Protection inspect template resource name\n\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\n\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn inspect_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig {
    #[doc =
        "Set the field `filter_enforcement`.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_enforcement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig {}

impl BuildModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig {
        ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig {
            filter_enforcement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfigRef {
        ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_enforcement", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorFloorsetting_FilterConfig_SdpSettingsDynamic {
    advanced_config: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig>>,
    basic_config: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig>>,
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig_SdpSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_config: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_config: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig>>,
    dynamic: ModelArmorFloorsetting_FilterConfig_SdpSettingsDynamic,
}

impl ModelArmorFloorsetting_FilterConfig_SdpSettings {
    #[doc = "Set the field `advanced_config`.\n"]
    pub fn set_advanced_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `basic_config`.\n"]
    pub fn set_basic_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig_SdpSettings {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_SdpSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig_SdpSettings {}

impl BuildModelArmorFloorsetting_FilterConfig_SdpSettings {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig_SdpSettings {
        ModelArmorFloorsetting_FilterConfig_SdpSettings {
            advanced_config: core::default::Default::default(),
            basic_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfig_SdpSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfig_SdpSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FilterConfig_SdpSettingsRef {
        ModelArmorFloorsetting_FilterConfig_SdpSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfig_SdpSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_config` after provisioning.\n"]
    pub fn advanced_config(&self) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfig_SdpSettings_AdvancedConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_config", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_config` after provisioning.\n"]
    pub fn basic_config(&self) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfig_SdpSettings_BasicConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorFloorsetting_FilterConfigDynamic {
    malicious_uri_filter_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings>>,
    pi_and_jailbreak_filter_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings>>,
    rai_settings: std::option::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig_RaiSettings>>,
    sdp_settings: std::option::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FilterConfig_SdpSettings>>,
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FilterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malicious_uri_filter_settings: std
    ::option
    ::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pi_and_jailbreak_filter_settings: std
    ::option
    ::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rai_settings: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig_RaiSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sdp_settings: std::option::Option<std::vec::Vec<ModelArmorFloorsetting_FilterConfig_SdpSettings>>,
    dynamic: ModelArmorFloorsetting_FilterConfigDynamic,
}

impl ModelArmorFloorsetting_FilterConfig {
    #[doc = "Set the field `malicious_uri_filter_settings`.\n"]
    pub fn set_malicious_uri_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malicious_uri_filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malicious_uri_filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pi_and_jailbreak_filter_settings`.\n"]
    pub fn set_pi_and_jailbreak_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pi_and_jailbreak_filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pi_and_jailbreak_filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rai_settings`.\n"]
    pub fn set_rai_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_RaiSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rai_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sdp_settings`.\n"]
    pub fn set_sdp_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig_SdpSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sdp_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sdp_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FilterConfig {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FilterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FilterConfig {}

impl BuildModelArmorFloorsetting_FilterConfig {
    pub fn build(self) -> ModelArmorFloorsetting_FilterConfig {
        ModelArmorFloorsetting_FilterConfig {
            malicious_uri_filter_settings: core::default::Default::default(),
            pi_and_jailbreak_filter_settings: core::default::Default::default(),
            rai_settings: core::default::Default::default(),
            sdp_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FilterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FilterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ModelArmorFloorsetting_FilterConfigRef {
        ModelArmorFloorsetting_FilterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FilterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `malicious_uri_filter_settings` after provisioning.\n"]
    pub fn malicious_uri_filter_settings(
        &self,
    ) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfig_MaliciousUriFilterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_uri_filter_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `pi_and_jailbreak_filter_settings` after provisioning.\n"]
    pub fn pi_and_jailbreak_filter_settings(
        &self,
    ) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfig_PiAndJailbreakFilterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pi_and_jailbreak_filter_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rai_settings` after provisioning.\n"]
    pub fn rai_settings(&self) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfig_RaiSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rai_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sdp_settings` after provisioning.\n"]
    pub fn sdp_settings(&self) -> terrars::ListRef<ModelArmorFloorsetting_FilterConfig_SdpSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sdp_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection {
    enable_multi_language_detection: terrars::PrimField<bool>,
}

impl ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection { }

impl terrars::ToListMappable for ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection {
    #[doc = "If true, multi language detection will be enabled."]
    pub enable_multi_language_detection: terrars::PrimField<bool>,
}

impl BuildModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection {
    pub fn build(self) -> ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection {
        ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection {
            enable_multi_language_detection: self.enable_multi_language_detection,
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetectionRef {
        ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_multi_language_detection` after provisioning.\nIf true, multi language detection will be enabled."]
    pub fn enable_multi_language_detection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multi_language_detection", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorFloorsetting_FloorSettingMetadataDynamic {
    multi_language_detection: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection>>,
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_FloorSettingMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_language_detection: std
    ::option
    ::Option<std::vec::Vec<ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection>>,
    dynamic: ModelArmorFloorsetting_FloorSettingMetadataDynamic,
}

impl ModelArmorFloorsetting_FloorSettingMetadata {
    #[doc = "Set the field `multi_language_detection`.\n"]
    pub fn set_multi_language_detection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetection>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_language_detection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_language_detection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_FloorSettingMetadata {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_FloorSettingMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_FloorSettingMetadata {}

impl BuildModelArmorFloorsetting_FloorSettingMetadata {
    pub fn build(self) -> ModelArmorFloorsetting_FloorSettingMetadata {
        ModelArmorFloorsetting_FloorSettingMetadata {
            multi_language_detection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_FloorSettingMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_FloorSettingMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_FloorSettingMetadataRef {
        ModelArmorFloorsetting_FloorSettingMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_FloorSettingMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `multi_language_detection` after provisioning.\n"]
    pub fn multi_language_detection(
        &self,
    ) -> terrars::ListRef<ModelArmorFloorsetting_FloorSettingMetadata_MultiLanguageDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_language_detection", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_GoogleMcpServerFloorSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cloud_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_and_block: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_only: std::option::Option<terrars::PrimField<bool>>,
}

impl ModelArmorFloorsetting_GoogleMcpServerFloorSetting {
    #[doc = "Set the field `enable_cloud_logging`.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn set_enable_cloud_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_cloud_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_and_block`.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn set_inspect_and_block(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inspect_and_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_only`.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn set_inspect_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inspect_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorFloorsetting_GoogleMcpServerFloorSetting {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_GoogleMcpServerFloorSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_GoogleMcpServerFloorSetting {}

impl BuildModelArmorFloorsetting_GoogleMcpServerFloorSetting {
    pub fn build(self) -> ModelArmorFloorsetting_GoogleMcpServerFloorSetting {
        ModelArmorFloorsetting_GoogleMcpServerFloorSetting {
            enable_cloud_logging: core::default::Default::default(),
            inspect_and_block: core::default::Default::default(),
            inspect_only: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_GoogleMcpServerFloorSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_GoogleMcpServerFloorSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorFloorsetting_GoogleMcpServerFloorSettingRef {
        ModelArmorFloorsetting_GoogleMcpServerFloorSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_GoogleMcpServerFloorSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_cloud_logging` after provisioning.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn enable_cloud_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cloud_logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_and_block` after provisioning.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn inspect_and_block(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_and_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_only` after provisioning.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn inspect_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_only", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorFloorsetting_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorFloorsetting_Timeouts {
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

impl terrars::ToListMappable for ModelArmorFloorsetting_Timeouts {
    type O = terrars::BlockAssignable<ModelArmorFloorsetting_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorFloorsetting_Timeouts {}

impl BuildModelArmorFloorsetting_Timeouts {
    pub fn build(self) -> ModelArmorFloorsetting_Timeouts {
        ModelArmorFloorsetting_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorFloorsetting_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorFloorsetting_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ModelArmorFloorsetting_TimeoutsRef {
        ModelArmorFloorsetting_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorFloorsetting_TimeoutsRef {
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
