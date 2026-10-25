#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBackupDrBackupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    backup_vault_id: terrars::PrimField<std::string::String>,
    data_source_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    project: terrars::PrimField<std::string::String>,
}

struct DataBackupDrBackup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupDrBackupData_>,
}

#[derive(Clone)]
pub struct DataBackupDrBackup(std::rc::Rc<DataBackupDrBackup_>);

impl DataBackupDrBackup {
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

    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backups` after provisioning.\nList of all backups under data source."]
    pub fn backups(&self) -> terrars::ListRef<DataBackupDrBackup_BackupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the backup was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupDrBackup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupDrBackup { }

impl terrars::ToListMappable for DataBackupDrBackup {
    type O = terrars::ListRef<DataBackupDrBackupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupDrBackup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_backup_dr_backup".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupDrBackup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub backup_vault_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub data_source_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupDrBackup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupDrBackup {
        let out = DataBackupDrBackup(std::rc::Rc::new(DataBackupDrBackup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupDrBackupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                backup_vault_id: self.backup_vault_id,
                data_source_id: self.data_source_id,
                id: core::default::Default::default(),
                location: self.location,
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupDrBackupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupDrBackupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backups` after provisioning.\nList of all backups under data source."]
    pub fn backups(&self) -> terrars::ListRef<DataBackupDrBackup_BackupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the backup was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrBackup_Backups_ComputeInstanceBackupProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_flush: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBackupDrBackup_Backups_ComputeInstanceBackupProperties {
    #[doc = "Set the field `guest_flush`.\n"]
    pub fn set_guest_flush(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.guest_flush = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrBackup_Backups_ComputeInstanceBackupProperties {
    type O = terrars::BlockAssignable<DataBackupDrBackup_Backups_ComputeInstanceBackupProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackup_Backups_ComputeInstanceBackupProperties {}

impl BuildDataBackupDrBackup_Backups_ComputeInstanceBackupProperties {
    pub fn build(self) -> DataBackupDrBackup_Backups_ComputeInstanceBackupProperties {
        DataBackupDrBackup_Backups_ComputeInstanceBackupProperties { guest_flush: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackup_Backups_ComputeInstanceBackupPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackup_Backups_ComputeInstanceBackupPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrBackup_Backups_ComputeInstanceBackupPropertiesRef {
        DataBackupDrBackup_Backups_ComputeInstanceBackupPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackup_Backups_ComputeInstanceBackupPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\n"]
    pub fn guest_flush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrBackup_Backups_DiskBackupProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_flush: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBackupDrBackup_Backups_DiskBackupProperties {
    #[doc = "Set the field `guest_flush`.\n"]
    pub fn set_guest_flush(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.guest_flush = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrBackup_Backups_DiskBackupProperties {
    type O = terrars::BlockAssignable<DataBackupDrBackup_Backups_DiskBackupProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackup_Backups_DiskBackupProperties {}

impl BuildDataBackupDrBackup_Backups_DiskBackupProperties {
    pub fn build(self) -> DataBackupDrBackup_Backups_DiskBackupProperties {
        DataBackupDrBackup_Backups_DiskBackupProperties { guest_flush: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackup_Backups_DiskBackupPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackup_Backups_DiskBackupPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrBackup_Backups_DiskBackupPropertiesRef {
        DataBackupDrBackup_Backups_DiskBackupPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackup_Backups_DiskBackupPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\n"]
    pub fn guest_flush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrBackup_Backups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_vault_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_instance_backup_properties: std
    ::option
    ::Option<terrars::ListField<DataBackupDrBackup_Backups_ComputeInstanceBackupProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_backup_properties: std::option::Option<terrars::ListField<DataBackupDrBackup_Backups_DiskBackupProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrBackup_Backups {
    #[doc = "Set the field `backup_id`.\n"]
    pub fn set_backup_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_vault_id`.\n"]
    pub fn set_backup_vault_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_vault_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_instance_backup_properties`.\n"]
    pub fn set_compute_instance_backup_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupDrBackup_Backups_ComputeInstanceBackupProperties>>,
    ) -> Self {
        self.compute_instance_backup_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_id`.\n"]
    pub fn set_data_source_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_backup_properties`.\n"]
    pub fn set_disk_backup_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupDrBackup_Backups_DiskBackupProperties>>,
    ) -> Self {
        self.disk_backup_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrBackup_Backups {
    type O = terrars::BlockAssignable<DataBackupDrBackup_Backups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackup_Backups {}

impl BuildDataBackupDrBackup_Backups {
    pub fn build(self) -> DataBackupDrBackup_Backups {
        DataBackupDrBackup_Backups {
            backup_id: core::default::Default::default(),
            backup_vault_id: core::default::Default::default(),
            compute_instance_backup_properties: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_source_id: core::default::Default::default(),
            disk_backup_properties: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackup_BackupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackup_BackupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupDrBackup_BackupsRef {
        DataBackupDrBackup_BackupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackup_BackupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_instance_backup_properties` after provisioning.\n"]
    pub fn compute_instance_backup_properties(
        &self,
    ) -> terrars::ListRef<DataBackupDrBackup_Backups_ComputeInstanceBackupPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_instance_backup_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_backup_properties` after provisioning.\n"]
    pub fn disk_backup_properties(&self) -> terrars::ListRef<DataBackupDrBackup_Backups_DiskBackupPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_backup_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
