#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BackupDrRestoreWorkloadData_ {
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
    backup_id: terrars::PrimField<std::string::String>,
    backup_vault_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clear_overrides_field_mask: std::option::Option<terrars::PrimField<std::string::String>>,
    data_source_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_restored_instance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_instance_restore_properties: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_instance_target_environment: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_restore_properties: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskRestoreProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_target_environment: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskTargetEnvironment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_disk_target_environment: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_RegionDiskTargetEnvironment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BackupDrRestoreWorkload_Timeouts>,
    dynamic: BackupDrRestoreWorkloadDynamic,
}

struct BackupDrRestoreWorkload_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BackupDrRestoreWorkloadData_>,
}

#[derive(Clone)]
pub struct BackupDrRestoreWorkload(std::rc::Rc<BackupDrRestoreWorkload_>);

impl BackupDrRestoreWorkload {
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
        "Set the field `clear_overrides_field_mask`.\nOptional. A field mask used to clear server-side default values during restore."]
    pub fn set_clear_overrides_field_mask(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().clear_overrides_field_mask = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_restored_instance`.\nOptional. If true (default), running terraform destroy will delete the live resource in GCP.\nIf false, only the restore record is removed from the state, leaving the resource active."]
    pub fn set_delete_restored_instance(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_restored_instance = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `name`.\nThe resource name of the backup instance."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_id`.\nOptional. An optional request ID to identify requests. Specify a unique request ID\nso that if you must retry your request, the server will know to ignore\nthe request if it has already been completed."]
    pub fn set_request_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().request_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_instance_restore_properties`.\n"]
    pub fn set_compute_instance_restore_properties(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_instance_restore_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_instance_restore_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compute_instance_target_environment`.\n"]
    pub fn set_compute_instance_target_environment(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_instance_target_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_instance_target_environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_restore_properties`.\n"]
    pub fn set_disk_restore_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_restore_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_restore_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_target_environment`.\n"]
    pub fn set_disk_target_environment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_DiskTargetEnvironment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_target_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_target_environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region_disk_target_environment`.\n"]
    pub fn set_region_disk_target_environment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_RegionDiskTargetEnvironment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().region_disk_target_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.region_disk_target_environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BackupDrRestoreWorkload_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `backup_id` after provisioning.\nRequired. The ID of the backup to restore from."]
    pub fn backup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. The ID of the backup vault."]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clear_overrides_field_mask` after provisioning.\nOptional. A field mask used to clear server-side default values during restore."]
    pub fn clear_overrides_field_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clear_overrides_field_mask", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_id` after provisioning.\nRequired. The ID of the data source."]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_restored_instance` after provisioning.\nOptional. If true (default), running terraform destroy will delete the live resource in GCP.\nIf false, only the restore record is removed from the state, leaving the resource active."]
    pub fn delete_restored_instance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_restored_instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nRequired. The location for the backup vault."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the backup instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_id` after provisioning.\nOptional. An optional request ID to identify requests. Specify a unique request ID\nso that if you must retry your request, the server will know to ignore\nthe request if it has already been completed."]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_resource` after provisioning.\nOutput only. Details of the target resource created/modified as part of restore."]
    pub fn target_resource(&self) -> terrars::ListRef<BackupDrRestoreWorkload_TargetResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_instance_restore_properties` after provisioning.\n"]
    pub fn compute_instance_restore_properties(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_restore_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_instance_target_environment` after provisioning.\n"]
    pub fn compute_instance_target_environment(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceTargetEnvironmentRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_target_environment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_restore_properties` after provisioning.\n"]
    pub fn disk_restore_properties(&self) -> terrars::ListRef<BackupDrRestoreWorkload_DiskRestorePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_restore_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_target_environment` after provisioning.\n"]
    pub fn disk_target_environment(&self) -> terrars::ListRef<BackupDrRestoreWorkload_DiskTargetEnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_target_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region_disk_target_environment` after provisioning.\n"]
    pub fn region_disk_target_environment(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_RegionDiskTargetEnvironmentRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.region_disk_target_environment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrRestoreWorkload_TimeoutsRef {
        BackupDrRestoreWorkload_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BackupDrRestoreWorkload {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BackupDrRestoreWorkload { }

impl terrars::ToListMappable for BackupDrRestoreWorkload {
    type O = terrars::ListRef<BackupDrRestoreWorkloadRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BackupDrRestoreWorkload_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_backup_dr_restore_workload".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupDrRestoreWorkload {
    pub tf_id: std::string::String,
    #[doc = "Required. The ID of the backup to restore from."]
    pub backup_id: terrars::PrimField<std::string::String>,
    #[doc = "Required. The ID of the backup vault."]
    pub backup_vault_id: terrars::PrimField<std::string::String>,
    #[doc = "Required. The ID of the data source."]
    pub data_source_id: terrars::PrimField<std::string::String>,
    #[doc = "Required. The location for the backup vault."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrRestoreWorkload {
    pub fn build(self, stack: &mut terrars::Stack) -> BackupDrRestoreWorkload {
        let out = BackupDrRestoreWorkload(std::rc::Rc::new(BackupDrRestoreWorkload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BackupDrRestoreWorkloadData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backup_id: self.backup_id,
                backup_vault_id: self.backup_vault_id,
                clear_overrides_field_mask: core::default::Default::default(),
                data_source_id: self.data_source_id,
                delete_restored_instance: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                request_id: core::default::Default::default(),
                compute_instance_restore_properties: core::default::Default::default(),
                compute_instance_target_environment: core::default::Default::default(),
                disk_restore_properties: core::default::Default::default(),
                disk_target_environment: core::default::Default::default(),
                region_disk_target_environment: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupDrRestoreWorkloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkloadRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BackupDrRestoreWorkloadRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_id` after provisioning.\nRequired. The ID of the backup to restore from."]
    pub fn backup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. The ID of the backup vault."]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clear_overrides_field_mask` after provisioning.\nOptional. A field mask used to clear server-side default values during restore."]
    pub fn clear_overrides_field_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clear_overrides_field_mask", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_id` after provisioning.\nRequired. The ID of the data source."]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_restored_instance` after provisioning.\nOptional. If true (default), running terraform destroy will delete the live resource in GCP.\nIf false, only the restore record is removed from the state, leaving the resource active."]
    pub fn delete_restored_instance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_restored_instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nRequired. The location for the backup vault."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the backup instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_id` after provisioning.\nOptional. An optional request ID to identify requests. Specify a unique request ID\nso that if you must retry your request, the server will know to ignore\nthe request if it has already been completed."]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_resource` after provisioning.\nOutput only. Details of the target resource created/modified as part of restore."]
    pub fn target_resource(&self) -> terrars::ListRef<BackupDrRestoreWorkload_TargetResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_instance_restore_properties` after provisioning.\n"]
    pub fn compute_instance_restore_properties(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_restore_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_instance_target_environment` after provisioning.\n"]
    pub fn compute_instance_target_environment(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceTargetEnvironmentRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_target_environment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_restore_properties` after provisioning.\n"]
    pub fn disk_restore_properties(&self) -> terrars::ListRef<BackupDrRestoreWorkload_DiskRestorePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_restore_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_target_environment` after provisioning.\n"]
    pub fn disk_target_environment(&self) -> terrars::ListRef<BackupDrRestoreWorkload_DiskTargetEnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_target_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region_disk_target_environment` after provisioning.\n"]
    pub fn region_disk_target_environment(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_RegionDiskTargetEnvironmentRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.region_disk_target_environment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrRestoreWorkload_TimeoutsRef {
        BackupDrRestoreWorkload_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadDynamic {
    compute_instance_restore_properties: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties>>,
    compute_instance_target_environment: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment>>,
    disk_restore_properties: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskRestoreProperties>>,
    disk_target_environment: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskTargetEnvironment>>,
    region_disk_target_environment: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_RegionDiskTargetEnvironment>>,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_TargetResource_GcpResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_resourcename: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_TargetResource_GcpResource {
    #[doc = "Set the field `gcp_resourcename`.\n"]
    pub fn set_gcp_resourcename(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcp_resourcename = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_TargetResource_GcpResource {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_TargetResource_GcpResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_TargetResource_GcpResource {}

impl BuildBackupDrRestoreWorkload_TargetResource_GcpResource {
    pub fn build(self) -> BackupDrRestoreWorkload_TargetResource_GcpResource {
        BackupDrRestoreWorkload_TargetResource_GcpResource {
            gcp_resourcename: core::default::Default::default(),
            location: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_TargetResource_GcpResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_TargetResource_GcpResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_TargetResource_GcpResourceRef {
        BackupDrRestoreWorkload_TargetResource_GcpResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_TargetResource_GcpResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_resourcename` after provisioning.\n"]
    pub fn gcp_resourcename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_resourcename", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_TargetResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_resource: std::option::Option<terrars::ListField<BackupDrRestoreWorkload_TargetResource_GcpResource>>,
}

impl BackupDrRestoreWorkload_TargetResource {
    #[doc = "Set the field `gcp_resource`.\n"]
    pub fn set_gcp_resource(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BackupDrRestoreWorkload_TargetResource_GcpResource>>,
    ) -> Self {
        self.gcp_resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_TargetResource {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_TargetResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_TargetResource {}

impl BuildBackupDrRestoreWorkload_TargetResource {
    pub fn build(self) -> BackupDrRestoreWorkload_TargetResource {
        BackupDrRestoreWorkload_TargetResource { gcp_resource: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_TargetResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_TargetResourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupDrRestoreWorkload_TargetResourceRef {
        BackupDrRestoreWorkload_TargetResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_TargetResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_resource` after provisioning.\n"]
    pub fn gcp_resource(&self) -> terrars::ListRef<BackupDrRestoreWorkload_TargetResource_GcpResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_uefi_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visible_core_count: std::option::Option<terrars::PrimField<f64>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures {
    #[doc =
        "Set the field `enable_nested_virtualization`.\nOptional. Whether to enable nested virtualization or not (default is false)."]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_uefi_networking`.\nOptional. Whether to enable UEFI networking for instance creation."]
    pub fn set_enable_uefi_networking(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_uefi_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threads_per_core`.\nOptional. The number of threads per physical core."]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visible_core_count`.\nOptional. The number of physical cores to expose to an instance."]
    pub fn set_visible_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.visible_core_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures {
            enable_nested_virtualization: core::default::Default::default(),
            enable_uefi_networking: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeaturesRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nOptional. Whether to enable nested virtualization or not (default is false)."]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_uefi_networking` after provisioning.\nOptional. Whether to enable UEFI networking for instance creation."]
    pub fn enable_uefi_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_uefi_networking", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threads_per_core` after provisioning.\nOptional. The number of threads per physical core."]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }

    #[doc =
        "Get a reference to the value of field `visible_core_count` after provisioning.\nOptional. The number of physical cores to expose to an instance."]
    pub fn visible_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visible_core_count", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consume_allocation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity {
    #[doc =
        "Set the field `consume_allocation_type`.\n Possible values: [\"TYPE_UNSPECIFIED\", \"NO_RESERVATION\", \"ANY_RESERVATION\", \"SPECIFIC_RESERVATION\"]"]
    pub fn set_consume_allocation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consume_allocation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity {
            consume_allocation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinityRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consume_allocation_type` after provisioning.\n Possible values: [\"TYPE_UNSPECIFIED\", \"NO_RESERVATION\", \"ANY_RESERVATION\", \"SPECIFIC_RESERVATION\"]"]
    pub fn consume_allocation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consume_allocation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_compute: std::option::Option<terrars::PrimField<bool>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig {
    #[doc =
        "Set the field `enable_confidential_compute`.\nOptional. Defines whether the instance should have confidential compute enabled."]
    pub fn set_enable_confidential_compute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_compute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig {
    type O =
        terrars
        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig {
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfigRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nOptional. Defines whether the instance should have confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey {
    #[doc =
        "Set the field `kms_key_name`.\nOptional. The name of the encryption key that is stored in Google Cloud KMS."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nOptional. The service account being used for the encryption request."]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `raw_key`.\nOptional. Specifies a 256-bit customer-supplied encryption key."]
    pub fn set_raw_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rsa_encrypted_key`.\nOptional. RSA-wrapped 2048-bit customer-supplied encryption key."]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey {
            kms_key_name: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKeyRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. The name of the encryption key that is stored in Google Cloud KMS."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nOptional. The service account being used for the encryption request."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nOptional. Specifies a 256-bit customer-supplied encryption key."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nOptional. RSA-wrapped 2048-bit customer-supplied encryption key."]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature {
    #[doc =
        "Set the field `type_`.\nOptional. The ID of a supported feature. Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature {
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeatureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeatureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeatureRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeatureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeatureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. The ID of a supported feature. Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams {
    #[doc = "Set the field `disk_name`.\nOptional. Specifies the disk name."]
    pub fn set_disk_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_zones`.\nOptional. URL of the zone where the disk should be created."]
    pub fn set_replica_zones(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.replica_zones = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams {
            disk_name: core::default::Default::default(),
            replica_zones: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParamsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_name` after provisioning.\nOptional. Specifies the disk name."]
    pub fn disk_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replica_zones` after provisioning.\nOptional. URL of the zone where the disk should be created."]
    pub fn replica_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_zones", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksDynamic {
    disk_encryption_key: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey>>,
    guest_os_feature: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature>>,
    initialize_params: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams>>,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saved_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_os_feature: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initialize_params: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams>>,
    dynamic: BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksDynamic,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks {
    #[doc =
        "Set the field `auto_delete`.\nOptional. Specifies whether the disk will be auto-deleted when the instance is deleted."]
    pub fn set_auto_delete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boot`.\nOptional. Indicates that this is a boot disk."]
    pub fn set_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_name`.\nOptional. This is used as an identifier for the disks."]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_interface`.\nOptional. Specifies the disk interface to use for attaching this disk. Possible values: [\"DISK_INTERFACE_UNSPECIFIED\", \"SCSI\", \"NVME\", \"NVDIMM\", \"ISCSI\"]"]
    pub fn set_disk_interface(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size_gb`.\nOptional. The size of the disk in GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\nOutput only. The URI of the disk type resource."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index`.\nOptional. A zero-based index to this disk, where 0 is reserved for the boot disk."]
    pub fn set_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.index = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\nOptional. Type of the resource."]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `license`.\nOptional. Any valid publicly visible licenses."]
    pub fn set_license(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.license = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nOptional. The mode in which to attach this disk. Possible values: [\"DISK_MODE_UNSPECIFIED\", \"READ_WRITE\", \"READ_ONLY\", \"LOCKED\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `saved_state`.\nOptional. Specifies the saved state of the disk. Possible values: [\"DISK_SAVED_STATE_UNSPECIFIED\", \"PRESERVED\"]"]
    pub fn set_saved_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.saved_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source`.\nOptional. Specifies a valid partial or full URL to an existing Persistent Disk resource."]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. Specifies the type of the disk. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"SCRATCH\", \"PERSISTENT\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKey,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `guest_os_feature`.\n"]
    pub fn set_guest_os_feature(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeature,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guest_os_feature = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_os_feature = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initialize_params`.\n"]
    pub fn set_initialize_params(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParams,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initialize_params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initialize_params = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks {
            auto_delete: core::default::Default::default(),
            boot: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_interface: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            index: core::default::Default::default(),
            kind: core::default::Default::default(),
            license: core::default::Default::default(),
            mode: core::default::Default::default(),
            saved_state: core::default::Default::default(),
            source: core::default::Default::default(),
            type_: core::default::Default::default(),
            disk_encryption_key: core::default::Default::default(),
            guest_os_feature: core::default::Default::default(),
            initialize_params: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_delete` after provisioning.\nOptional. Specifies whether the disk will be auto-deleted when the instance is deleted."]
    pub fn auto_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot` after provisioning.\nOptional. Indicates that this is a boot disk."]
    pub fn boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `device_name` after provisioning.\nOptional. This is used as an identifier for the disks."]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_interface` after provisioning.\nOptional. Specifies the disk interface to use for attaching this disk. Possible values: [\"DISK_INTERFACE_UNSPECIFIED\", \"SCSI\", \"NVME\", \"NVDIMM\", \"ISCSI\"]"]
    pub fn disk_interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nOptional. The size of the disk in GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nOutput only. The URI of the disk type resource."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `index` after provisioning.\nOptional. A zero-based index to this disk, where 0 is reserved for the boot disk."]
    pub fn index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\nOptional. Type of the resource."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc =
        "Get a reference to the value of field `license` after provisioning.\nOptional. Any valid publicly visible licenses."]
    pub fn license(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.license", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nOptional. The mode in which to attach this disk. Possible values: [\"DISK_MODE_UNSPECIFIED\", \"READ_WRITE\", \"READ_ONLY\", \"LOCKED\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `saved_state` after provisioning.\nOptional. Specifies the saved state of the disk. Possible values: [\"DISK_SAVED_STATE_UNSPECIFIED\", \"PRESERVED\"]"]
    pub fn saved_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.saved_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nOptional. Specifies a valid partial or full URL to an existing Persistent Disk resource."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. Specifies the type of the disk. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"SCRATCH\", \"PERSISTENT\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_DiskEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_os_feature` after provisioning.\n"]
    pub fn guest_os_feature(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_GuestOsFeatureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_os_feature", self.base))
    }

    #[doc = "Get a reference to the value of field `initialize_params` after provisioning.\n"]
    pub fn initialize_params(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks_InitializeParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initialize_params", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_display: std::option::Option<terrars::PrimField<bool>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice {
    #[doc = "Set the field `enable_display`.\nOptional. Enables display for the Compute Engine VM."]
    pub fn set_enable_display(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_display = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice {
            enable_display: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDeviceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDeviceRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDeviceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDeviceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_display` after provisioning.\nOptional. Enables display for the Compute Engine VM."]
    pub fn enable_display(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_display", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators {
    #[doc =
        "Set the field `accelerator_count`.\nOptional. The number of the guest accelerator cards exposed to this instance."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_type`.\nOptional. Full or partial URL of the accelerator type resource."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAcceleratorsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nOptional. The number of the guest accelerator cards exposed to this instance."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nOptional. Full or partial URL of the accelerator type resource."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `raw_key`.\n"]
    pub fn set_raw_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rsa_encrypted_key`.\n"]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey {
            kms_key_name: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKeyRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\n"]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\n"]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_Labels {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_Labels {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_DiskRestoreProperties_Labels {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_Labels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_DiskRestoreProperties_Labels {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrRestoreWorkload_DiskRestoreProperties_Labels {
    pub fn build(self) -> BackupDrRestoreWorkload_DiskRestoreProperties_Labels {
        BackupDrRestoreWorkload_DiskRestoreProperties_Labels {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_LabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_DiskRestoreProperties_LabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_DiskRestoreProperties_LabelsRef {
        BackupDrRestoreWorkload_DiskRestoreProperties_LabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_LabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_ItemsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataDynamic {
    items: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items>>,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items>>,
    dynamic: BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataDynamic,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_Items>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ptr_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_public_ptr: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs {
    #[doc = "Set the field `external_ip`.\n"]
    pub fn set_external_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ipv6`.\n"]
    pub fn set_external_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ipv6_prefix_length`.\n"]
    pub fn set_external_ipv6_prefix_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.external_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nOptional. The name of this access configuration."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_tier`.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"]
    pub fn set_network_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_ptr_domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `set_public_ptr`.\n"]
    pub fn set_set_public_ptr(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.set_public_ptr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs {
    type O =
        terrars
        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs {
            external_ip: core::default::Default::default(),
            external_ipv6: core::default::Default::default(),
            external_ipv6_prefix_length: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
            set_public_ptr: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_ip` after provisioning.\n"]
    pub fn external_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ipv6` after provisioning.\n"]
    pub fn external_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\n"]
    pub fn external_ipv6_prefix_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOptional. The name of this access configuration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tier` after provisioning.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"]
    pub fn network_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `set_public_ptr` after provisioning.\n"]
    pub fn set_public_ptr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.set_public_ptr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_cidr_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges {
    #[doc = "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_cidr_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork_range_name`.\n"]
    pub fn set_subnetwork_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.subnetwork_range_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges {
    type O =
        terrars
        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges {
            ip_cidr_range: core::default::Default::default(),
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRangesRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork_range_name` after provisioning.\n"]
    pub fn subnetwork_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_range_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ptr_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_public_ptr: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs {
    #[doc = "Set the field `external_ip`.\n"]
    pub fn set_external_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ipv6`.\n"]
    pub fn set_external_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ipv6_prefix_length`.\n"]
    pub fn set_external_ipv6_prefix_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.external_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nOptional. The name of this access configuration."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_tier`.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"]
    pub fn set_network_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_ptr_domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `set_public_ptr`.\n"]
    pub fn set_set_public_ptr(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.set_public_ptr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs {
    type O =
        terrars
        ::BlockAssignable<
            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs {
            external_ip: core::default::Default::default(),
            external_ipv6: core::default::Default::default(),
            external_ipv6_prefix_length: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
            set_public_ptr: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_ip` after provisioning.\n"]
    pub fn external_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ipv6` after provisioning.\n"]
    pub fn external_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\n"]
    pub fn external_ipv6_prefix_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOptional. The name of this access configuration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tier` after provisioning.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"]
    pub fn network_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `set_public_ptr` after provisioning.\n"]
    pub fn set_public_ptr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.set_public_ptr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesDynamic {
    access_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs>,
    >,
    alias_ip_ranges: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges>,
    >,
    ipv6_access_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs>,
    >,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ipv6_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_configs: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias_ip_ranges: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_access_configs: std
    ::option
    ::Option<
        std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs>,
    >,
    dynamic: BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesDynamic,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces {
    #[doc =
        "Set the field `internal_ipv6_prefix_length`.\nOptional. The prefix length of the primary internal IPv6 range."]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\nOptional. An IPv4 internal IP address to assign to the instance."]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_access_type`.\n Possible values: [\"UNSPECIFIED_IPV6_ACCESS_TYPE\", \"INTERNAL\", \"EXTERNAL\"]"]
    pub fn set_ipv6_access_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address`.\nOptional. An IPv6 internal network address for this network interface."]
    pub fn set_ipv6_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nOptional. URL of the VPC network resource for this instance."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_attachment`.\n"]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nic_type`.\n Possible values: [\"NIC_TYPE_UNSPECIFIED\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn set_nic_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nic_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_count`.\n"]
    pub fn set_queue_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.queue_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stack_type`.\n Possible values: [\"STACK_TYPE_UNSPECIFIED\", \"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn set_stack_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stack_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\nOptional. The URL of the Subnetwork resource for this instance."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_configs`.\n"]
    pub fn set_access_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `alias_ip_ranges`.\n"]
    pub fn set_alias_ip_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRanges,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alias_ip_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alias_ip_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ipv6_access_configs`.\n"]
    pub fn set_ipv6_access_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ipv6_access_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ipv6_access_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces {
            internal_ipv6_prefix_length: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            ipv6_access_type: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_attachment: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            access_configs: core::default::Default::default(),
            alias_ip_ranges: core::default::Default::default(),
            ipv6_access_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\nOptional. The prefix length of the primary internal IPv6 range."]
    pub fn internal_ipv6_prefix_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nOptional. An IPv4 internal IP address to assign to the instance."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_access_type` after provisioning.\n Possible values: [\"UNSPECIFIED_IPV6_ACCESS_TYPE\", \"INTERNAL\", \"EXTERNAL\"]"]
    pub fn ipv6_access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address` after provisioning.\nOptional. An IPv6 internal network address for this network interface."]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nOptional. URL of the VPC network resource for this instance."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\n"]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nic_type` after provisioning.\n Possible values: [\"NIC_TYPE_UNSPECIFIED\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn nic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_count` after provisioning.\n"]
    pub fn queue_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stack_type` after provisioning.\n Possible values: [\"STACK_TYPE_UNSPECIFIED\", \"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nOptional. The URL of the Subnetwork resource for this instance."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `access_configs` after provisioning.\n"]
    pub fn access_configs(
        &self,
    ) -> terrars
    ::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AccessConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `alias_ip_ranges` after provisioning.\n"]
    pub fn alias_ip_ranges(
        &self,
    ) -> terrars
    ::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_AliasIpRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_access_configs` after provisioning.\n"]
    pub fn ipv6_access_configs(
        &self,
    ) -> terrars
    ::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces_Ipv6AccessConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_access_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bandwidth_tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig {
    #[doc =
        "Set the field `total_egress_bandwidth_tier`.\n Possible values: [\"TIER_UNSPECIFIED\", \"DEFAULT\", \"TIER_1\"]"]
    pub fn set_total_egress_bandwidth_tier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_egress_bandwidth_tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfigRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n Possible values: [\"TIER_UNSPECIFIED\", \"DEFAULT\", \"TIER_1\"]"]
    pub fn total_egress_bandwidth_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags {
    pub fn build(self) -> BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags {
        BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTagsRef {
        BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsDynamic {
    resource_manager_tags: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags>>,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags>>,
    dynamic: BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsDynamic,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_manager_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_manager_tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params {
            resource_manager_tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout {
    type O =
        terrars
        ::BlockAssignable<
            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeoutRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDurationRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n Possible values: [\"OPERATOR_UNSPECIFIED\", \"IN\", \"NOT_IN\"]"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinitiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinitiesRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\n Possible values: [\"OPERATOR_UNSPECIFIED\", \"IN\", \"NOT_IN\"]"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingDynamic {
    local_ssd_recovery_timeout: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout>,
    >,
    max_run_duration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration>,
    >,
    node_affinities: std
    ::option
    ::Option<
        terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities>,
    >,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_restart: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_termination_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_host_maintenance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_recovery_timeout: std
    ::option
    ::Option<
        std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_run_duration: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinities: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities>>,
    dynamic: BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingDynamic,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling {
    #[doc = "Set the field `automatic_restart`.\n"]
    pub fn set_automatic_restart(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_restart = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_termination_action`.\n Possible values: [\"INSTANCE_TERMINATION_ACTION_UNSPECIFIED\", \"DELETE\", \"STOP\"]"]
    pub fn set_instance_termination_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_termination_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `on_host_maintenance`.\n Possible values: [\"ON_HOST_MAINTENANCE_UNSPECIFIED\", \"TERMINATE\", \"MIGRATE\"]"]
    pub fn set_on_host_maintenance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_host_maintenance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preemptible = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioning_model`.\n Possible values: [\"PROVISIONING_MODEL_UNSPECIFIED\", \"STANDARD\", \"SPOT\"]"]
    pub fn set_provisioning_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.provisioning_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `termination_time`.\n"]
    pub fn set_termination_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.termination_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeout,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_ssd_recovery_timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssd_recovery_timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDuration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_run_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_run_duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinities,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_affinities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinities = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling {
            automatic_restart: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
            termination_time: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatic_restart` after provisioning.\n"]
    pub fn automatic_restart(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_restart", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_termination_action` after provisioning.\n Possible values: [\"INSTANCE_TERMINATION_ACTION_UNSPECIFIED\", \"DELETE\", \"STOP\"]"]
    pub fn instance_termination_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_termination_action", self.base))
    }

    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_cpus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `on_host_maintenance` after provisioning.\n Possible values: [\"ON_HOST_MAINTENANCE_UNSPECIFIED\", \"TERMINATE\", \"MIGRATE\"]"]
    pub fn on_host_maintenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_host_maintenance", self.base))
    }

    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioning_model` after provisioning.\n Possible values: [\"PROVISIONING_MODEL_UNSPECIFIED\", \"STANDARD\", \"SPOT\"]"]
    pub fn provisioning_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_model", self.base))
    }

    #[doc = "Get a reference to the value of field `termination_time` after provisioning.\n"]
    pub fn termination_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_time", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(
        &self,
    ) -> terrars
    ::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_LocalSsdRecoveryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_ssd_recovery_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_MaxRunDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_run_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `node_affinities` after provisioning.\n"]
    pub fn node_affinities(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling_NodeAffinitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_affinities", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts {
            email: core::default::Default::default(),
            scopes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccountsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccountsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_vtpm`.\n"]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig {
    type O =
        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfigRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\n"]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags {}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_TagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_TagsRef {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesDynamic {
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures>>,
    allocation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity>>,
    confidential_instance_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig>,
    >,
    disks: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks>>,
    display_device: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice>>,
    guest_accelerators: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators>>,
    instance_encryption_key: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey>>,
    labels: std::option::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskRestoreProperties_Labels>>,
    metadata: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata>>,
    network_interfaces: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces>>,
    network_performance_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig>,
    >,
    params: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params>>,
    scheduling: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling>>,
    service_accounts: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig>>,
    tags: std::option::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags>>,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestoreProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    can_ip_forward: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_revocation_action_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ipv6_google_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_affinity: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_config: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disks: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_device: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerators: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_encryption_key: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskRestoreProperties_Labels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_accounts: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags>>,
    dynamic: BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesDynamic,
}

impl BackupDrRestoreWorkload_ComputeInstanceRestoreProperties {
    #[doc =
        "Set the field `can_ip_forward`.\nOptional. Allows this instance to send and receive packets with non-matching destination or source IPs."]
    pub fn set_can_ip_forward(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.can_ip_forward = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nOptional. Whether the resource should be protected against deletion."]
    pub fn set_deletion_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. An optional description of this resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nOptional. Specifies the hostname of the instance."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_revocation_action_type`.\nOptional. KeyRevocationActionType of the instance. Possible values: [\"KEY_REVOCATION_ACTION_TYPE_UNSPECIFIED\", \"NONE\", \"STOP\"]"]
    pub fn set_key_revocation_action_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_revocation_action_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nOptional. Full or partial URL of the machine type resource to use for this instance."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cpu_platform`.\nOptional. Minimum CPU platform to use for this instance."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_ipv6_google_access`.\nOptional. The private IPv6 google access type for the VM. Possible values: [\"INSTANCE_PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED\", \"INHERIT_FROM_SUBNETWORK\", \"ENABLE_OUTBOUND_VM_ACCESS_TO_GOOGLE\", \"ENABLE_BIDIRECTIONAL_ACCESS_TO_GOOGLE\"]"]
    pub fn set_private_ipv6_google_access(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ipv6_google_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_policies`.\nOptional. Resource policies applied to this instance."]
    pub fn set_resource_policies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeatures,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_machine_features = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_machine_features = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `allocation_affinity`.\n"]
    pub fn set_allocation_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinity>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allocation_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allocation_affinity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidential_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disks`.\n"]
    pub fn set_disks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Disks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `display_device`.\n"]
    pub fn set_display_device(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDevice>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.display_device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.display_device = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `guest_accelerators`.\n"]
    pub fn set_guest_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAccelerators>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guest_accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_encryption_key`.\n"]
    pub fn set_instance_encryption_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKey,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_Labels>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Metadata>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfaces>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_interfaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interfaces = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_performance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_performance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Params>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduling`.\n"]
    pub fn set_scheduling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Scheduling>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_accounts`.\n"]
    pub fn set_service_accounts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccounts>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_accounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_accounts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shielded_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceRestoreProperties {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties {
    #[doc = "Required. Name of the compute instance."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrRestoreWorkload_ComputeInstanceRestoreProperties {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceRestoreProperties {
        BackupDrRestoreWorkload_ComputeInstanceRestoreProperties {
            can_ip_forward: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            description: core::default::Default::default(),
            hostname: core::default::Default::default(),
            key_revocation_action_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            name: self.name,
            private_ipv6_google_access: core::default::Default::default(),
            resource_policies: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            allocation_affinity: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            disks: core::default::Default::default(),
            display_device: core::default::Default::default(),
            guest_accelerators: core::default::Default::default(),
            instance_encryption_key: core::default::Default::default(),
            labels: core::default::Default::default(),
            metadata: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            params: core::default::Default::default(),
            scheduling: core::default::Default::default(),
            service_accounts: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesRef {
        BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceRestorePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `can_ip_forward` after provisioning.\nOptional. Allows this instance to send and receive packets with non-matching destination or source IPs."]
    pub fn can_ip_forward(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nOptional. Whether the resource should be protected against deletion."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. An optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nOptional. Specifies the hostname of the instance."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_revocation_action_type` after provisioning.\nOptional. KeyRevocationActionType of the instance. Possible values: [\"KEY_REVOCATION_ACTION_TYPE_UNSPECIFIED\", \"NONE\", \"STOP\"]"]
    pub fn key_revocation_action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_revocation_action_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Full or partial URL of the machine type resource to use for this instance."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nOptional. Minimum CPU platform to use for this instance."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Name of the compute instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nOptional. The private IPv6 google access type for the VM. Possible values: [\"INSTANCE_PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED\", \"INHERIT_FROM_SUBNETWORK\", \"ENABLE_OUTBOUND_VM_ACCESS_TO_GOOGLE\", \"ENABLE_BIDIRECTIONAL_ACCESS_TO_GOOGLE\"]"]
    pub fn private_ipv6_google_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nOptional. Resource policies applied to this instance."]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_affinity` after provisioning.\n"]
    pub fn allocation_affinity(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_AllocationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allocation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ConfidentialInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `disks` after provisioning.\n"]
    pub fn disks(&self) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disks", self.base))
    }

    #[doc = "Get a reference to the value of field `display_device` after provisioning.\n"]
    pub fn display_device(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_DisplayDeviceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.display_device", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_accelerators` after provisioning.\n"]
    pub fn guest_accelerators(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_GuestAcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_encryption_key` after provisioning.\n"]
    pub fn instance_encryption_key(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_InstanceEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.base))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_SchedulingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.base))
    }

    #[doc = "Get a reference to the value of field `service_accounts` after provisioning.\n"]
    pub fn service_accounts(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ServiceAccountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BackupDrRestoreWorkload_ComputeInstanceRestoreProperties_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment {
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_project_service_account: std::option::Option<terrars::PrimField<bool>>,
    zone: terrars::PrimField<std::string::String>,
}

impl BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment {
    #[doc =
        "Set the field `use_project_service_account`.\nIf true, use the BackupDR P4SA credentials for same-project restores. Default is false."]
    pub fn set_use_project_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_project_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_ComputeInstanceTargetEnvironment {
    #[doc = "Required. Target project for the Compute Engine instance."]
    pub project: terrars::PrimField<std::string::String>,
    #[doc = "Required. The zone of the Compute Engine instance."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrRestoreWorkload_ComputeInstanceTargetEnvironment {
    pub fn build(self) -> BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment {
        BackupDrRestoreWorkload_ComputeInstanceTargetEnvironment {
            project: self.project,
            use_project_service_account: core::default::Default::default(),
            zone: self.zone,
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_ComputeInstanceTargetEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_ComputeInstanceTargetEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_ComputeInstanceTargetEnvironmentRef {
        BackupDrRestoreWorkload_ComputeInstanceTargetEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_ComputeInstanceTargetEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nRequired. Target project for the Compute Engine instance."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_project_service_account` after provisioning.\nIf true, use the BackupDR P4SA credentials for same-project restores. Default is false."]
    pub fn use_project_service_account(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_project_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nRequired. The zone of the Compute Engine instance."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `raw_key`.\n"]
    pub fn set_raw_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rsa_encrypted_key`.\n"]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey {}

impl BuildBackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey {
    pub fn build(self) -> BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey {
        BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey {
            kms_key_name: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKeyRef {
        BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\n"]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\n"]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature {
    #[doc =
        "Set the field `type_`.\n Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature {}

impl BuildBackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature {
    pub fn build(self) -> BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature {
        BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeatureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeatureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeatureRef {
        BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeatureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeatureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\n Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrRestoreWorkload_DiskRestorePropertiesDynamic {
    disk_encryption_key: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey>>,
    guest_os_feature: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature>>,
    labels: std::option::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskRestoreProperties_Labels>>,
    resource_manager_tags: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags>>,
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_DiskRestoreProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_compute: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    licenses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    physical_block_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policy: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    size_gb: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_os_feature: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskRestoreProperties_Labels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std
    ::option
    ::Option<std::vec::Vec<BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags>>,
    dynamic: BackupDrRestoreWorkload_DiskRestorePropertiesDynamic,
}

impl BackupDrRestoreWorkload_DiskRestoreProperties {
    #[doc =
        "Set the field `access_mode`.\nOptional. The access mode of the disk. Possible values: [\"READ_WRITE_SINGLE\", \"READ_WRITE_MANY\", \"READ_ONLY_MANY\"]"]
    pub fn set_access_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `architecture`.\nOptional. The architecture of the source disk. Possible values: [\"ARCHITECTURE_UNSPECIFIED\", \"X86_64\", \"ARM64\"]"]
    pub fn set_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. An optional description of this resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_confidential_compute`.\nOptional. Indicates whether this disk is using confidential compute mode."]
    pub fn set_enable_confidential_compute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_compute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `licenses`.\nOptional. A list of publicly available licenses that are applicable to this backup."]
    pub fn set_licenses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.licenses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `physical_block_size_bytes`.\nOptional. Physical block size of the persistent disk, in bytes."]
    pub fn set_physical_block_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.physical_block_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_iops`.\nOptional. Indicates how many IOPS to provision for the disk."]
    pub fn set_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_throughput`.\nOptional. Indicates how much throughput to provision for the disk."]
    pub fn set_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_policy`.\nOptional. Resource policies applied to this disk."]
    pub fn set_resource_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_pool`.\nOptional. The storage pool in which the new disk is created."]
    pub fn set_storage_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `guest_os_feature`.\n"]
    pub fn set_guest_os_feature(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeature>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guest_os_feature = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_os_feature = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_Labels>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties_ResourceManagerTags>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_manager_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_manager_tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_DiskRestoreProperties {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_DiskRestoreProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_DiskRestoreProperties {
    #[doc = "Required. Name of the disk."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Required. The size of the disk in GB."]
    pub size_gb: terrars::PrimField<f64>,
    #[doc = "Required. URL of the disk type resource describing which disk type to use."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrRestoreWorkload_DiskRestoreProperties {
    pub fn build(self) -> BackupDrRestoreWorkload_DiskRestoreProperties {
        BackupDrRestoreWorkload_DiskRestoreProperties {
            access_mode: core::default::Default::default(),
            architecture: core::default::Default::default(),
            description: core::default::Default::default(),
            enable_confidential_compute: core::default::Default::default(),
            licenses: core::default::Default::default(),
            name: self.name,
            physical_block_size_bytes: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            resource_policy: core::default::Default::default(),
            size_gb: self.size_gb,
            storage_pool: core::default::Default::default(),
            type_: self.type_,
            disk_encryption_key: core::default::Default::default(),
            guest_os_feature: core::default::Default::default(),
            labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_DiskRestorePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_DiskRestorePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_DiskRestorePropertiesRef {
        BackupDrRestoreWorkload_DiskRestorePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_DiskRestorePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_mode` after provisioning.\nOptional. The access mode of the disk. Possible values: [\"READ_WRITE_SINGLE\", \"READ_WRITE_MANY\", \"READ_ONLY_MANY\"]"]
    pub fn access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `architecture` after provisioning.\nOptional. The architecture of the source disk. Possible values: [\"ARCHITECTURE_UNSPECIFIED\", \"X86_64\", \"ARM64\"]"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. An optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nOptional. Indicates whether this disk is using confidential compute mode."]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `licenses` after provisioning.\nOptional. A list of publicly available licenses that are applicable to this backup."]
    pub fn licenses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.licenses", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Name of the disk."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `physical_block_size_bytes` after provisioning.\nOptional. Physical block size of the persistent disk, in bytes."]
    pub fn physical_block_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physical_block_size_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_iops` after provisioning.\nOptional. Indicates how many IOPS to provision for the disk."]
    pub fn provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_throughput` after provisioning.\nOptional. Indicates how much throughput to provision for the disk."]
    pub fn provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_policy` after provisioning.\nOptional. Resource policies applied to this disk."]
    pub fn resource_policy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_gb` after provisioning.\nRequired. The size of the disk in GB."]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool` after provisioning.\nOptional. The storage pool in which the new disk is created."]
    pub fn storage_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRequired. URL of the disk type resource describing which disk type to use."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(
        &self,
    ) -> terrars::ListRef<BackupDrRestoreWorkload_DiskRestoreProperties_DiskEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_os_feature` after provisioning.\n"]
    pub fn guest_os_feature(&self) -> terrars::ListRef<BackupDrRestoreWorkload_DiskRestoreProperties_GuestOsFeatureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_os_feature", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_DiskTargetEnvironment {
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_project_service_account: std::option::Option<terrars::PrimField<bool>>,
    zone: terrars::PrimField<std::string::String>,
}

impl BackupDrRestoreWorkload_DiskTargetEnvironment {
    #[doc =
        "Set the field `use_project_service_account`.\nIf true, use the BackupDR P4SA credentials for same-project restores. Default is false."]
    pub fn set_use_project_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_project_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_DiskTargetEnvironment {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_DiskTargetEnvironment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_DiskTargetEnvironment {
    #[doc = "Required. Target project for the disk."]
    pub project: terrars::PrimField<std::string::String>,
    #[doc = "Required. Target zone for the disk."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrRestoreWorkload_DiskTargetEnvironment {
    pub fn build(self) -> BackupDrRestoreWorkload_DiskTargetEnvironment {
        BackupDrRestoreWorkload_DiskTargetEnvironment {
            project: self.project,
            use_project_service_account: core::default::Default::default(),
            zone: self.zone,
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_DiskTargetEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_DiskTargetEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_DiskTargetEnvironmentRef {
        BackupDrRestoreWorkload_DiskTargetEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_DiskTargetEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nRequired. Target project for the disk."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_project_service_account` after provisioning.\nIf true, use the BackupDR P4SA credentials for same-project restores. Default is false."]
    pub fn use_project_service_account(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_project_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nRequired. Target zone for the disk."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_RegionDiskTargetEnvironment {
    project: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
    replica_zones: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_project_service_account: std::option::Option<terrars::PrimField<bool>>,
}

impl BackupDrRestoreWorkload_RegionDiskTargetEnvironment {
    #[doc =
        "Set the field `use_project_service_account`.\nIf true, use the BackupDR P4SA credentials for same-project restores. Default is false."]
    pub fn set_use_project_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_project_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_RegionDiskTargetEnvironment {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_RegionDiskTargetEnvironment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_RegionDiskTargetEnvironment {
    #[doc = "Required. Target project for the disk."]
    pub project: terrars::PrimField<std::string::String>,
    #[doc = "Required. Target region for the disk."]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = "Required. Target URLs of the replica zones for the disk."]
    pub replica_zones: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBackupDrRestoreWorkload_RegionDiskTargetEnvironment {
    pub fn build(self) -> BackupDrRestoreWorkload_RegionDiskTargetEnvironment {
        BackupDrRestoreWorkload_RegionDiskTargetEnvironment {
            project: self.project,
            region: self.region,
            replica_zones: self.replica_zones,
            use_project_service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_RegionDiskTargetEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_RegionDiskTargetEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrRestoreWorkload_RegionDiskTargetEnvironmentRef {
        BackupDrRestoreWorkload_RegionDiskTargetEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_RegionDiskTargetEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nRequired. Target project for the disk."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRequired. Target region for the disk."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replica_zones` after provisioning.\nRequired. Target URLs of the replica zones for the disk."]
    pub fn replica_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_zones", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_project_service_account` after provisioning.\nIf true, use the BackupDR P4SA credentials for same-project restores. Default is false."]
    pub fn use_project_service_account(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_project_service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrRestoreWorkload_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrRestoreWorkload_Timeouts {
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
}

impl terrars::ToListMappable for BackupDrRestoreWorkload_Timeouts {
    type O = terrars::BlockAssignable<BackupDrRestoreWorkload_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrRestoreWorkload_Timeouts {}

impl BuildBackupDrRestoreWorkload_Timeouts {
    pub fn build(self) -> BackupDrRestoreWorkload_Timeouts {
        BackupDrRestoreWorkload_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrRestoreWorkload_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrRestoreWorkload_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupDrRestoreWorkload_TimeoutsRef {
        BackupDrRestoreWorkload_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrRestoreWorkload_TimeoutsRef {
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
}
