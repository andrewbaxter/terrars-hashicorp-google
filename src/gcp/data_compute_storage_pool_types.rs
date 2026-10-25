#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeStoragePoolTypesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    storage_pool_type: terrars::PrimField<std::string::String>,
    zone: terrars::PrimField<std::string::String>,
}

struct DataComputeStoragePoolTypes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeStoragePoolTypesData_>,
}

#[derive(Clone)]
pub struct DataComputeStoragePoolTypes(std::rc::Rc<DataComputeStoragePoolTypes_>);

impl DataComputeStoragePoolTypes {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deprecated` after provisioning.\nThe deprecation status associated with this storage pool type."]
    pub fn deprecated(&self) -> terrars::ListRef<DataComputeStoragePoolTypes_DeprecatedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deprecated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#storagePoolType for storage pool types."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_pool_provisioned_capacity_gb` after provisioning.\nMaximum storage pool size in GB."]
    pub fn max_pool_provisioned_capacity_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_pool_provisioned_iops` after provisioning.\nMaximum provisioned IOPS."]
    pub fn max_pool_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pool_provisioned_iops", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_pool_provisioned_throughput` after provisioning.\nMaximum provisioned throughput."]
    pub fn max_pool_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_throughput", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `min_pool_provisioned_capacity_gb` after provisioning.\nMinimum storage pool size in GB."]
    pub fn min_pool_provisioned_capacity_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `min_pool_provisioned_iops` after provisioning.\nMinimum provisioned IOPS."]
    pub fn min_pool_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_pool_provisioned_iops", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_pool_provisioned_throughput` after provisioning.\nMinimum provisioned throughput."]
    pub fn min_pool_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool_type` after provisioning.\nName of the storage pool type."]
    pub fn storage_pool_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_disk_types` after provisioning.\nThe list of disk types supported in this storage pool type."]
    pub fn supported_disk_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_disk_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeStoragePoolTypes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeStoragePoolTypes { }

impl terrars::ToListMappable for DataComputeStoragePoolTypes {
    type O = terrars::ListRef<DataComputeStoragePoolTypesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeStoragePoolTypes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_storage_pool_types".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeStoragePoolTypes {
    pub tf_id: std::string::String,
    #[doc = "Name of the storage pool type."]
    pub storage_pool_type: terrars::PrimField<std::string::String>,
    #[doc = "The name of the zone."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeStoragePoolTypes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeStoragePoolTypes {
        let out = DataComputeStoragePoolTypes(std::rc::Rc::new(DataComputeStoragePoolTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeStoragePoolTypesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                project: core::default::Default::default(),
                storage_pool_type: self.storage_pool_type,
                zone: self.zone,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeStoragePoolTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeStoragePoolTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeStoragePoolTypesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deprecated` after provisioning.\nThe deprecation status associated with this storage pool type."]
    pub fn deprecated(&self) -> terrars::ListRef<DataComputeStoragePoolTypes_DeprecatedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deprecated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#storagePoolType for storage pool types."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_pool_provisioned_capacity_gb` after provisioning.\nMaximum storage pool size in GB."]
    pub fn max_pool_provisioned_capacity_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_pool_provisioned_iops` after provisioning.\nMaximum provisioned IOPS."]
    pub fn max_pool_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pool_provisioned_iops", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_pool_provisioned_throughput` after provisioning.\nMaximum provisioned throughput."]
    pub fn max_pool_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_throughput", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `min_pool_provisioned_capacity_gb` after provisioning.\nMinimum storage pool size in GB."]
    pub fn min_pool_provisioned_capacity_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `min_pool_provisioned_iops` after provisioning.\nMinimum provisioned IOPS."]
    pub fn min_pool_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_pool_provisioned_iops", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_pool_provisioned_throughput` after provisioning.\nMinimum provisioned throughput."]
    pub fn min_pool_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool_type` after provisioning.\nName of the storage pool type."]
    pub fn storage_pool_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_disk_types` after provisioning.\nThe list of disk types supported in this storage pool type."]
    pub fn supported_disk_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_disk_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeStoragePoolTypes_Deprecated {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecated: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    obsolete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeStoragePoolTypes_Deprecated {
    #[doc = "Set the field `deleted`.\n"]
    pub fn set_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deprecated`.\n"]
    pub fn set_deprecated(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deprecated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `obsolete`.\n"]
    pub fn set_obsolete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.obsolete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replacement`.\n"]
    pub fn set_replacement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.replacement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeStoragePoolTypes_Deprecated {
    type O = terrars::BlockAssignable<DataComputeStoragePoolTypes_Deprecated>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeStoragePoolTypes_Deprecated {}

impl BuildDataComputeStoragePoolTypes_Deprecated {
    pub fn build(self) -> DataComputeStoragePoolTypes_Deprecated {
        DataComputeStoragePoolTypes_Deprecated {
            deleted: core::default::Default::default(),
            deprecated: core::default::Default::default(),
            obsolete: core::default::Default::default(),
            replacement: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeStoragePoolTypes_DeprecatedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeStoragePoolTypes_DeprecatedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeStoragePoolTypes_DeprecatedRef {
        DataComputeStoragePoolTypes_DeprecatedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeStoragePoolTypes_DeprecatedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecated` after provisioning.\n"]
    pub fn deprecated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated", self.base))
    }

    #[doc = "Get a reference to the value of field `obsolete` after provisioning.\n"]
    pub fn obsolete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.obsolete", self.base))
    }

    #[doc = "Get a reference to the value of field `replacement` after provisioning.\n"]
    pub fn replacement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replacement", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
