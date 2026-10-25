#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageControlOrganizationIntelligenceConfigData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

struct DataStorageControlOrganizationIntelligenceConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStorageControlOrganizationIntelligenceConfigData_>,
}

#[derive(Clone)]
pub struct DataStorageControlOrganizationIntelligenceConfig(
    std::rc::Rc<DataStorageControlOrganizationIntelligenceConfig_>,
);

impl DataStorageControlOrganizationIntelligenceConfig {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, DISABLED, TRIAL and STANDARD."]
    pub fn edition_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_intelligence_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Organization. For GCP org, this field should be organization number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(&self) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_TrialConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the Storage Intelligence Config resource is last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataStorageControlOrganizationIntelligenceConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStorageControlOrganizationIntelligenceConfig { }

impl terrars::ToListMappable for DataStorageControlOrganizationIntelligenceConfig {
    type O = terrars::ListRef<DataStorageControlOrganizationIntelligenceConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStorageControlOrganizationIntelligenceConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_storage_control_organization_intelligence_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig {
    pub tf_id: std::string::String,
    #[doc = "Identifier of the GCP Organization. For GCP org, this field should be organization number."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataStorageControlOrganizationIntelligenceConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStorageControlOrganizationIntelligenceConfig {
        let out =
            DataStorageControlOrganizationIntelligenceConfig(
                std::rc::Rc::new(DataStorageControlOrganizationIntelligenceConfig_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataStorageControlOrganizationIntelligenceConfigData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        id: core::default::Default::default(),
                        name: self.name,
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStorageControlOrganizationIntelligenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, DISABLED, TRIAL and STANDARD."]
    pub fn edition_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_intelligence_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Organization. For GCP org, this field should be organization number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(&self) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_TrialConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the Storage Intelligence Config resource is last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intelligence_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig {
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

impl terrars::ToListMappable for DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig {
    type O = terrars::BlockAssignable<DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig {}

impl BuildDataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig {
        DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfig {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfigRef {
        DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfig_EffectiveIntelligenceConfigRef {
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
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_id_regexes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bucket_id_regexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {}

impl BuildDataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
        DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
        DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_id_regexes` after provisioning.\n"]
    pub fn bucket_id_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_id_regexes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations {}

impl BuildDataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
        DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations {
            locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
        DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_id_regexes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bucket_id_regexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets {}

impl BuildDataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
        DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
        DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_id_regexes` after provisioning.\n"]
    pub fn bucket_id_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_id_regexes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations {}

impl BuildDataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations {
        DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations {
            locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
        DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfig_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_cloud_storage_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_cloud_storage_locations: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_cloud_storage_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_cloud_storage_locations: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations>,
    >,
}

impl DataStorageControlOrganizationIntelligenceConfig_Filter {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBuckets,
                        >,
                    >,
    ) -> Self {
        self.excluded_cloud_storage_buckets = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocations,
                        >,
                    >,
    ) -> Self {
        self.excluded_cloud_storage_locations = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBuckets,
                        >,
                    >,
    ) -> Self {
        self.included_cloud_storage_buckets = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocations,
                        >,
                    >,
    ) -> Self {
        self.included_cloud_storage_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlOrganizationIntelligenceConfig_Filter {
    type O = terrars::BlockAssignable<DataStorageControlOrganizationIntelligenceConfig_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig_Filter {}

impl BuildDataStorageControlOrganizationIntelligenceConfig_Filter {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfig_Filter {
        DataStorageControlOrganizationIntelligenceConfig_Filter {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlOrganizationIntelligenceConfig_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfig_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlOrganizationIntelligenceConfig_FilterRef {
        DataStorageControlOrganizationIntelligenceConfig_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfig_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_cloud_storage_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_cloud_storage_locations` after provisioning.\n"]
    pub fn excluded_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_Filter_ExcludedCloudStorageLocationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_cloud_storage_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_cloud_storage_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `included_cloud_storage_locations` after provisioning.\n"]
    pub fn included_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<DataStorageControlOrganizationIntelligenceConfig_Filter_IncludedCloudStorageLocationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_cloud_storage_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfig_TrialConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlOrganizationIntelligenceConfig_TrialConfig {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlOrganizationIntelligenceConfig_TrialConfig {
    type O = terrars::BlockAssignable<DataStorageControlOrganizationIntelligenceConfig_TrialConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlOrganizationIntelligenceConfig_TrialConfig {}

impl BuildDataStorageControlOrganizationIntelligenceConfig_TrialConfig {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfig_TrialConfig {
        DataStorageControlOrganizationIntelligenceConfig_TrialConfig {
            expire_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlOrganizationIntelligenceConfig_TrialConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlOrganizationIntelligenceConfig_TrialConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlOrganizationIntelligenceConfig_TrialConfigRef {
        DataStorageControlOrganizationIntelligenceConfig_TrialConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlOrganizationIntelligenceConfig_TrialConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
