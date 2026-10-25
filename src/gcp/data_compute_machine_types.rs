#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeMachineTypesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeMachineTypes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeMachineTypesData_>,
}

#[derive(Clone)]
pub struct DataComputeMachineTypes(std::rc::Rc<DataComputeMachineTypes_>);

impl DataComputeMachineTypes {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nProject ID for this request."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe name of the zone for this request."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_types` after provisioning.\nThe list of machine types"]
    pub fn machine_types(&self) -> terrars::ListRef<DataComputeMachineTypes_MachineTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID for this request."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone for this request."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeMachineTypes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeMachineTypes { }

impl terrars::ToListMappable for DataComputeMachineTypes {
    type O = terrars::ListRef<DataComputeMachineTypesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeMachineTypes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_machine_types".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeMachineTypes {
    pub tf_id: std::string::String,
}

impl BuildDataComputeMachineTypes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeMachineTypes {
        let out = DataComputeMachineTypes(std::rc::Rc::new(DataComputeMachineTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeMachineTypesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeMachineTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeMachineTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeMachineTypesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_types` after provisioning.\nThe list of machine types"]
    pub fn machine_types(&self) -> terrars::ListRef<DataComputeMachineTypes_MachineTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID for this request."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone for this request."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeMachineTypes_MachineTypes_Accelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeMachineTypes_MachineTypes_Accelerators {
    #[doc = "Set the field `guest_accelerator_count`.\n"]
    pub fn set_guest_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.guest_accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_accelerator_type`.\n"]
    pub fn set_guest_accelerator_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.guest_accelerator_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeMachineTypes_MachineTypes_Accelerators {
    type O = terrars::BlockAssignable<DataComputeMachineTypes_MachineTypes_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeMachineTypes_MachineTypes_Accelerators {}

impl BuildDataComputeMachineTypes_MachineTypes_Accelerators {
    pub fn build(self) -> DataComputeMachineTypes_MachineTypes_Accelerators {
        DataComputeMachineTypes_MachineTypes_Accelerators {
            guest_accelerator_count: core::default::Default::default(),
            guest_accelerator_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeMachineTypes_MachineTypes_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeMachineTypes_MachineTypes_AcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeMachineTypes_MachineTypes_AcceleratorsRef {
        DataComputeMachineTypes_MachineTypes_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeMachineTypes_MachineTypes_AcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guest_accelerator_count` after provisioning.\n"]
    pub fn guest_accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_accelerator_type` after provisioning.\n"]
    pub fn guest_accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeMachineTypes_MachineTypes_Deprecated {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeMachineTypes_MachineTypes_Deprecated {
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

impl terrars::ToListMappable for DataComputeMachineTypes_MachineTypes_Deprecated {
    type O = terrars::BlockAssignable<DataComputeMachineTypes_MachineTypes_Deprecated>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeMachineTypes_MachineTypes_Deprecated {}

impl BuildDataComputeMachineTypes_MachineTypes_Deprecated {
    pub fn build(self) -> DataComputeMachineTypes_MachineTypes_Deprecated {
        DataComputeMachineTypes_MachineTypes_Deprecated {
            replacement: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeMachineTypes_MachineTypes_DeprecatedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeMachineTypes_MachineTypes_DeprecatedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeMachineTypes_MachineTypes_DeprecatedRef {
        DataComputeMachineTypes_MachineTypes_DeprecatedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeMachineTypes_MachineTypes_DeprecatedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

#[derive(Serialize)]
pub struct DataComputeMachineTypes_MachineTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<terrars::ListField<DataComputeMachineTypes_MachineTypes_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecated: std::option::Option<terrars::SetField<DataComputeMachineTypes_MachineTypes_Deprecated>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_shared_cpus: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_persistent_disks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_persistent_disks_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeMachineTypes_MachineTypes {
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeMachineTypes_MachineTypes_Accelerators>>,
    ) -> Self {
        self.accelerators = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deprecated`.\n"]
    pub fn set_deprecated(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataComputeMachineTypes_MachineTypes_Deprecated>>,
    ) -> Self {
        self.deprecated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_cpus`.\n"]
    pub fn set_guest_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.guest_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_shared_cpus`.\n"]
    pub fn set_is_shared_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_shared_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_persistent_disks`.\n"]
    pub fn set_maximum_persistent_disks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_persistent_disks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_persistent_disks_size_gb`.\n"]
    pub fn set_maximum_persistent_disks_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_persistent_disks_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_mb`.\n"]
    pub fn set_memory_mb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.self_link = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeMachineTypes_MachineTypes {
    type O = terrars::BlockAssignable<DataComputeMachineTypes_MachineTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeMachineTypes_MachineTypes {}

impl BuildDataComputeMachineTypes_MachineTypes {
    pub fn build(self) -> DataComputeMachineTypes_MachineTypes {
        DataComputeMachineTypes_MachineTypes {
            accelerators: core::default::Default::default(),
            architecture: core::default::Default::default(),
            deprecated: core::default::Default::default(),
            description: core::default::Default::default(),
            guest_cpus: core::default::Default::default(),
            is_shared_cpus: core::default::Default::default(),
            maximum_persistent_disks: core::default::Default::default(),
            maximum_persistent_disks_size_gb: core::default::Default::default(),
            memory_mb: core::default::Default::default(),
            name: core::default::Default::default(),
            self_link: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeMachineTypes_MachineTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeMachineTypes_MachineTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeMachineTypes_MachineTypesRef {
        DataComputeMachineTypes_MachineTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeMachineTypes_MachineTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<DataComputeMachineTypes_MachineTypes_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecated` after provisioning.\n"]
    pub fn deprecated(&self) -> terrars::SetRef<DataComputeMachineTypes_MachineTypes_DeprecatedRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.deprecated", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_cpus` after provisioning.\n"]
    pub fn guest_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_cpus", self.base))
    }

    #[doc = "Get a reference to the value of field `is_shared_cpus` after provisioning.\n"]
    pub fn is_shared_cpus(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_shared_cpus", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_persistent_disks` after provisioning.\n"]
    pub fn maximum_persistent_disks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_persistent_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_persistent_disks_size_gb` after provisioning.\n"]
    pub fn maximum_persistent_disks_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_persistent_disks_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mb` after provisioning.\n"]
    pub fn memory_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
}
