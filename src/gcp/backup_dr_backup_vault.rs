#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BackupDrBackupVaultData_ {
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
    access_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_missing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    backup_minimum_enforced_retention_duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_inheritance: std::option::Option<terrars::PrimField<std::string::String>>,
    backup_vault_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_access_restriction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_backup_plan_references: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_inactive_datasources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<BackupDrBackupVault_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BackupDrBackupVault_Timeouts>,
    dynamic: BackupDrBackupVaultDynamic,
}

struct BackupDrBackupVault_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BackupDrBackupVaultData_>,
}

#[derive(Clone)]
pub struct BackupDrBackupVault(std::rc::Rc<BackupDrBackupVault_>);

impl BackupDrBackupVault {
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
        "Set the field `access_restriction`.\nAccess restriction for the backup vault. Default value is 'WITHIN_ORGANIZATION' if not provided during creation. Default value: \"WITHIN_ORGANIZATION\" Possible values: [\"ACCESS_RESTRICTION_UNSPECIFIED\", \"WITHIN_PROJECT\", \"WITHIN_ORGANIZATION\", \"UNRESTRICTED\", \"WITHIN_ORG_BUT_UNRESTRICTED_FOR_BA\"]"]
    pub fn set_access_restriction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_missing`.\nAllow idempotent deletion of backup vault. The request will still succeed in case the backup vault does not exist."]
    pub fn set_allow_missing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_missing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `annotations`.\nOptional. User annotations. See https://google.aip.dev/128#annotations\nStores small amounts of arbitrary data. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `backup_retention_inheritance`.\nHow a backup's enforced retention end time is inherited. Default value is 'INHERIT_VAULT_RETENTION' if not provided during creation. Possible values: [\"BACKUP_RETENTION_INHERITANCE_UNSPECIFIED\", \"INHERIT_VAULT_RETENTION\", \"MATCH_BACKUP_EXPIRE_TIME\"]"]
    pub fn set_backup_retention_inheritance(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().backup_retention_inheritance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nOptional. The description of the BackupVault instance (2048 characters or less)."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective_time`.\nOptional. Time after which the BackupVault resource is locked."]
    pub fn set_effective_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().effective_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_delete`.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources.\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn set_force_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_update`.\nIf set, allow update to extend the minimum enforced retention for backup vault. This overrides\n the restriction against conflicting retention periods. This conflict may occur when the\n expiration schedule defined by the associated backup plan is shorter than the minimum\n retention set by the backup vault."]
    pub fn set_force_update(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_update = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_update_access_restriction`.\nIf set to true, we will force update access restriction even if some non compliant data sources are present."]
    pub fn set_force_update_access_restriction(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_update_access_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_backup_plan_references`.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn set_ignore_backup_plan_references(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_backup_plan_references = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_inactive_datasources`.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources."]
    pub fn set_ignore_inactive_datasources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_inactive_datasources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Resource labels to represent user provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrBackupVault_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BackupDrBackupVault_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_restriction` after provisioning.\nAccess restriction for the backup vault. Default value is 'WITHIN_ORGANIZATION' if not provided during creation. Default value: \"WITHIN_ORGANIZATION\" Possible values: [\"ACCESS_RESTRICTION_UNSPECIFIED\", \"WITHIN_PROJECT\", \"WITHIN_ORGANIZATION\", \"UNRESTRICTED\", \"WITHIN_ORG_BUT_UNRESTRICTED_FOR_BA\"]"]
    pub fn access_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_restriction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_missing` after provisioning.\nAllow idempotent deletion of backup vault. The request will still succeed in case the backup vault does not exist."]
    pub fn allow_missing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_missing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. See https://google.aip.dev/128#annotations\nStores small amounts of arbitrary data. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_count` after provisioning.\nOutput only. The number of backups in this backup vault."]
    pub fn backup_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_minimum_enforced_retention_duration` after provisioning.\nRequired. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended."]
    pub fn backup_minimum_enforced_retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_minimum_enforced_retention_duration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_inheritance` after provisioning.\nHow a backup's enforced retention end time is inherited. Default value is 'INHERIT_VAULT_RETENTION' if not provided during creation. Possible values: [\"BACKUP_RETENTION_INHERITANCE_UNSPECIFIED\", \"INHERIT_VAULT_RETENTION\", \"MATCH_BACKUP_EXPIRE_TIME\"]"]
    pub fn backup_retention_inheritance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_inheritance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. ID of the requesting object."]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletable` after provisioning.\nOutput only. Set to true when there are no backups nested under this resource."]
    pub fn deletable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. The description of the BackupVault instance (2048 characters or less)."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_time` after provisioning.\nOptional. Time after which the BackupVault resource is locked."]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOptional. Server specified ETag for the backup vault resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_delete` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources.\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_update` after provisioning.\nIf set, allow update to extend the minimum enforced retention for backup vault. This overrides\n the restriction against conflicting retention periods. This conflict may occur when the\n expiration schedule defined by the associated backup plan is shorter than the minimum\n retention set by the backup vault."]
    pub fn force_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_update_access_restriction` after provisioning.\nIf set to true, we will force update access restriction even if some non compliant data sources are present."]
    pub fn force_update_access_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update_access_restriction", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_backup_plan_references` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn ignore_backup_plan_references(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_backup_plan_references", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ignore_inactive_datasources` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources."]
    pub fn ignore_inactive_datasources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_inactive_datasources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Resource labels to represent user provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe GCP location for the backup vault."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier. The resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOutput only. Service account used by the BackupVault Service for this BackupVault.  The user should grant this account permissions in their workload project to enable the service to run backups and restores there."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The BackupVault resource instance state. \n Possible values:\n STATE_UNSPECIFIED\n CREATING\n ACTIVE\n DELETING\n ERROR\n UPDATING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_stored_bytes` after provisioning.\nOutput only. Total size of the storage used by all backup resources."]
    pub fn total_stored_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_stored_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Output only Immutable after resource creation until resource deletion."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<BackupDrBackupVault_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupVault_TimeoutsRef {
        BackupDrBackupVault_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BackupDrBackupVault {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BackupDrBackupVault { }

impl terrars::ToListMappable for BackupDrBackupVault {
    type O = terrars::ListRef<BackupDrBackupVaultRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BackupDrBackupVault_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_backup_dr_backup_vault".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupDrBackupVault {
    pub tf_id: std::string::String,
    #[doc =
        "Required. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended."]
    pub backup_minimum_enforced_retention_duration: terrars::PrimField<std::string::String>,
    #[doc = "Required. ID of the requesting object."]
    pub backup_vault_id: terrars::PrimField<std::string::String>,
    #[doc = "The GCP location for the backup vault."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrBackupVault {
    pub fn build(self, stack: &mut terrars::Stack) -> BackupDrBackupVault {
        let out = BackupDrBackupVault(std::rc::Rc::new(BackupDrBackupVault_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BackupDrBackupVaultData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_restriction: core::default::Default::default(),
                allow_missing: core::default::Default::default(),
                annotations: core::default::Default::default(),
                backup_minimum_enforced_retention_duration: self.backup_minimum_enforced_retention_duration,
                backup_retention_inheritance: core::default::Default::default(),
                backup_vault_id: self.backup_vault_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                effective_time: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                force_update: core::default::Default::default(),
                force_update_access_restriction: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_backup_plan_references: core::default::Default::default(),
                ignore_inactive_datasources: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupDrBackupVaultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupVaultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BackupDrBackupVaultRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_restriction` after provisioning.\nAccess restriction for the backup vault. Default value is 'WITHIN_ORGANIZATION' if not provided during creation. Default value: \"WITHIN_ORGANIZATION\" Possible values: [\"ACCESS_RESTRICTION_UNSPECIFIED\", \"WITHIN_PROJECT\", \"WITHIN_ORGANIZATION\", \"UNRESTRICTED\", \"WITHIN_ORG_BUT_UNRESTRICTED_FOR_BA\"]"]
    pub fn access_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_restriction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_missing` after provisioning.\nAllow idempotent deletion of backup vault. The request will still succeed in case the backup vault does not exist."]
    pub fn allow_missing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_missing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. See https://google.aip.dev/128#annotations\nStores small amounts of arbitrary data. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_count` after provisioning.\nOutput only. The number of backups in this backup vault."]
    pub fn backup_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_minimum_enforced_retention_duration` after provisioning.\nRequired. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended."]
    pub fn backup_minimum_enforced_retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_minimum_enforced_retention_duration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_inheritance` after provisioning.\nHow a backup's enforced retention end time is inherited. Default value is 'INHERIT_VAULT_RETENTION' if not provided during creation. Possible values: [\"BACKUP_RETENTION_INHERITANCE_UNSPECIFIED\", \"INHERIT_VAULT_RETENTION\", \"MATCH_BACKUP_EXPIRE_TIME\"]"]
    pub fn backup_retention_inheritance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_inheritance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. ID of the requesting object."]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletable` after provisioning.\nOutput only. Set to true when there are no backups nested under this resource."]
    pub fn deletable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. The description of the BackupVault instance (2048 characters or less)."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_time` after provisioning.\nOptional. Time after which the BackupVault resource is locked."]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOptional. Server specified ETag for the backup vault resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_delete` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources.\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_update` after provisioning.\nIf set, allow update to extend the minimum enforced retention for backup vault. This overrides\n the restriction against conflicting retention periods. This conflict may occur when the\n expiration schedule defined by the associated backup plan is shorter than the minimum\n retention set by the backup vault."]
    pub fn force_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_update_access_restriction` after provisioning.\nIf set to true, we will force update access restriction even if some non compliant data sources are present."]
    pub fn force_update_access_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update_access_restriction", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_backup_plan_references` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn ignore_backup_plan_references(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_backup_plan_references", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ignore_inactive_datasources` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources."]
    pub fn ignore_inactive_datasources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_inactive_datasources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Resource labels to represent user provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe GCP location for the backup vault."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier. The resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOutput only. Service account used by the BackupVault Service for this BackupVault.  The user should grant this account permissions in their workload project to enable the service to run backups and restores there."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The BackupVault resource instance state. \n Possible values:\n STATE_UNSPECIFIED\n CREATING\n ACTIVE\n DELETING\n ERROR\n UPDATING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_stored_bytes` after provisioning.\nOutput only. Total size of the storage used by all backup resources."]
    pub fn total_stored_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_stored_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Output only Immutable after resource creation until resource deletion."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<BackupDrBackupVault_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupVault_TimeoutsRef {
        BackupDrBackupVault_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BackupDrBackupVaultDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<BackupDrBackupVault_EncryptionConfig>>,
}

#[derive(Serialize)]
pub struct BackupDrBackupVault_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrBackupVault_EncryptionConfig {
    #[doc =
        "Set the field `kms_key_name`.\nThe Resource name of the Cloud KMS key to be used to encrypt new backups. The key must be in the same location as the backup vault. The key must be a Cloud KMS CryptoKey."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrBackupVault_EncryptionConfig {
    type O = terrars::BlockAssignable<BackupDrBackupVault_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupVault_EncryptionConfig {}

impl BuildBackupDrBackupVault_EncryptionConfig {
    pub fn build(self) -> BackupDrBackupVault_EncryptionConfig {
        BackupDrBackupVault_EncryptionConfig { kms_key_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupVault_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupVault_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupDrBackupVault_EncryptionConfigRef {
        BackupDrBackupVault_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupVault_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Resource name of the Cloud KMS key to be used to encrypt new backups. The key must be in the same location as the backup vault. The key must be a Cloud KMS CryptoKey."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrBackupVault_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrBackupVault_Timeouts {
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

impl terrars::ToListMappable for BackupDrBackupVault_Timeouts {
    type O = terrars::BlockAssignable<BackupDrBackupVault_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupVault_Timeouts {}

impl BuildBackupDrBackupVault_Timeouts {
    pub fn build(self) -> BackupDrBackupVault_Timeouts {
        BackupDrBackupVault_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupVault_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupVault_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupDrBackupVault_TimeoutsRef {
        BackupDrBackupVault_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupVault_TimeoutsRef {
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
