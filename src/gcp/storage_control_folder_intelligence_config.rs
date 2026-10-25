#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageControlFolderIntelligenceConfigData_ {
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
    edition_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<StorageControlFolderIntelligenceConfig_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageControlFolderIntelligenceConfig_Timeouts>,
    dynamic: StorageControlFolderIntelligenceConfigDynamic,
}

struct StorageControlFolderIntelligenceConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageControlFolderIntelligenceConfigData_>,
}

#[derive(Clone)]
pub struct StorageControlFolderIntelligenceConfig(std::rc::Rc<StorageControlFolderIntelligenceConfig_>);

impl StorageControlFolderIntelligenceConfig {
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
        "Set the field `edition_config`.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, TRIAL, DISABLED and STANDARD."]
    pub fn set_edition_config(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().edition_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageControlFolderIntelligenceConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, TRIAL, DISABLED and STANDARD."]
    pub fn edition_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_intelligence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Folder. For GCP Folder, this field can be folder number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(&self) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_TrialConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the Storage Intelligence Config resource is last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlFolderIntelligenceConfig_TimeoutsRef {
        StorageControlFolderIntelligenceConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for StorageControlFolderIntelligenceConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageControlFolderIntelligenceConfig { }

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig {
    type O = terrars::ListRef<StorageControlFolderIntelligenceConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageControlFolderIntelligenceConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_control_folder_intelligence_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig {
    pub tf_id: std::string::String,
    #[doc = "Identifier of the GCP Folder. For GCP Folder, this field can be folder number."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildStorageControlFolderIntelligenceConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageControlFolderIntelligenceConfig {
        let out = StorageControlFolderIntelligenceConfig(std::rc::Rc::new(StorageControlFolderIntelligenceConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageControlFolderIntelligenceConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                edition_config: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageControlFolderIntelligenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageControlFolderIntelligenceConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, TRIAL, DISABLED and STANDARD."]
    pub fn edition_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_intelligence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Folder. For GCP Folder, this field can be folder number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(&self) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_TrialConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the Storage Intelligence Config resource is last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlFolderIntelligenceConfig_TimeoutsRef {
        StorageControlFolderIntelligenceConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct StorageControlFolderIntelligenceConfigDynamic {
    filter: std::option::Option<terrars::DynamicBlock<StorageControlFolderIntelligenceConfig_Filter>>,
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intelligence_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig {
    #[doc = "Set the field `effective_edition`.\n"]
    pub fn set_effective_edition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effective_edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `intelligence_config`.\n"]
    pub fn set_intelligence_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.intelligence_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig {}

impl BuildStorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig {
        StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfig {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfigRef {
        StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_EffectiveIntelligenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_edition` after provisioning.\n"]
    pub fn effective_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_edition", self.base))
    }

    #[doc = "Get a reference to the value of field `intelligence_config` after provisioning.\n"]
    pub fn intelligence_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intelligence_config", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_TrialConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageControlFolderIntelligenceConfig_TrialConfig {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_TrialConfig {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_TrialConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_TrialConfig {}

impl BuildStorageControlFolderIntelligenceConfig_TrialConfig {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_TrialConfig {
        StorageControlFolderIntelligenceConfig_TrialConfig { expire_time: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_TrialConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_TrialConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_TrialConfigRef {
        StorageControlFolderIntelligenceConfig_TrialConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_TrialConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    bucket_id_regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets { }

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
        StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
        StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_id_regexes` after provisioning.\nList of bucket id regexes to exclude in the storage intelligence plan."]
    pub fn bucket_id_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_id_regexes", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations { }

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    #[doc = "List of locations."]
    pub locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
        StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations { locations: self.locations }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
        StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    bucket_id_regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets { }

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
        StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
        StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_id_regexes` after provisioning.\nList of bucket id regexes to exclude in the storage intelligence plan."]
    pub fn bucket_id_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_id_regexes", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations { }

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    #[doc = "List of locations."]
    pub locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations {
        StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations { locations: self.locations }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
        StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageControlFolderIntelligenceConfig_FilterDynamic {
    excluded_cloud_storage_buckets: std
    ::option
    ::Option<terrars::DynamicBlock<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets>>,
    excluded_cloud_storage_locations: std
    ::option
    ::Option<terrars::DynamicBlock<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations>>,
    included_cloud_storage_buckets: std
    ::option
    ::Option<terrars::DynamicBlock<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets>>,
    included_cloud_storage_locations: std
    ::option
    ::Option<terrars::DynamicBlock<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations>>,
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_cloud_storage_buckets: std
    ::option
    ::Option<std::vec::Vec<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_cloud_storage_locations: std
    ::option
    ::Option<std::vec::Vec<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_cloud_storage_buckets: std
    ::option
    ::Option<std::vec::Vec<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_cloud_storage_locations: std
    ::option
    ::Option<std::vec::Vec<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations>>,
    dynamic: StorageControlFolderIntelligenceConfig_FilterDynamic,
}

impl StorageControlFolderIntelligenceConfig_Filter {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBuckets>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.excluded_cloud_storage_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_cloud_storage_buckets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `excluded_cloud_storage_locations`.\n"]
    pub fn set_excluded_cloud_storage_locations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocations>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.excluded_cloud_storage_locations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_cloud_storage_locations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `included_cloud_storage_buckets`.\n"]
    pub fn set_included_cloud_storage_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBuckets>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.included_cloud_storage_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.included_cloud_storage_buckets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `included_cloud_storage_locations`.\n"]
    pub fn set_included_cloud_storage_locations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocations>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.included_cloud_storage_locations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.included_cloud_storage_locations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_Filter {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_Filter {}

impl BuildStorageControlFolderIntelligenceConfig_Filter {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_Filter {
        StorageControlFolderIntelligenceConfig_Filter {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_FilterRef {
        StorageControlFolderIntelligenceConfig_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_cloud_storage_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_cloud_storage_locations` after provisioning.\n"]
    pub fn excluded_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_cloud_storage_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_cloud_storage_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `included_cloud_storage_locations` after provisioning.\n"]
    pub fn included_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<StorageControlFolderIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_cloud_storage_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageControlFolderIntelligenceConfig_Timeouts {
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

impl terrars::ToListMappable for StorageControlFolderIntelligenceConfig_Timeouts {
    type O = terrars::BlockAssignable<StorageControlFolderIntelligenceConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageControlFolderIntelligenceConfig_Timeouts {}

impl BuildStorageControlFolderIntelligenceConfig_Timeouts {
    pub fn build(self) -> StorageControlFolderIntelligenceConfig_Timeouts {
        StorageControlFolderIntelligenceConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageControlFolderIntelligenceConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageControlFolderIntelligenceConfig_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageControlFolderIntelligenceConfig_TimeoutsRef {
        StorageControlFolderIntelligenceConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageControlFolderIntelligenceConfig_TimeoutsRef {
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
