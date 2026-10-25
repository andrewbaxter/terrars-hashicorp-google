#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SpannerBackupScheduleData_ {
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
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    retention_duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<SpannerBackupSchedule_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_backup_spec: std::option::Option<std::vec::Vec<SpannerBackupSchedule_FullBackupSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incremental_backup_spec: std::option::Option<std::vec::Vec<SpannerBackupSchedule_IncrementalBackupSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<SpannerBackupSchedule_Spec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SpannerBackupSchedule_Timeouts>,
    dynamic: SpannerBackupScheduleDynamic,
}

struct SpannerBackupSchedule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpannerBackupScheduleData_>,
}

#[derive(Clone)]
pub struct SpannerBackupSchedule(std::rc::Rc<SpannerBackupSchedule_>);

impl SpannerBackupSchedule {
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
        "Set the field `name`.\nA unique identifier for the backup schedule, which cannot be changed after\nthe backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9]."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerBackupSchedule_EncryptionConfig>>,
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

    #[doc = "Set the field `full_backup_spec`.\n"]
    pub fn set_full_backup_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerBackupSchedule_FullBackupSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().full_backup_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.full_backup_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `incremental_backup_spec`.\n"]
    pub fn set_incremental_backup_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerBackupSchedule_IncrementalBackupSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().incremental_backup_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.incremental_backup_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<SpannerBackupSchedule_Spec>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SpannerBackupSchedule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe database to create the backup schedule on."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
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
        "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the backup schedule, which cannot be changed after\nthe backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9]."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_duration` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.\nYou can set this to a value up to 366 days."]
    pub fn retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<SpannerBackupSchedule_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_backup_spec` after provisioning.\n"]
    pub fn full_backup_spec(&self) -> terrars::ListRef<SpannerBackupSchedule_FullBackupSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.full_backup_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incremental_backup_spec` after provisioning.\n"]
    pub fn incremental_backup_spec(&self) -> terrars::ListRef<SpannerBackupSchedule_IncrementalBackupSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incremental_backup_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<SpannerBackupSchedule_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerBackupSchedule_TimeoutsRef {
        SpannerBackupSchedule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SpannerBackupSchedule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SpannerBackupSchedule { }

impl terrars::ToListMappable for SpannerBackupSchedule {
    type O = terrars::ListRef<SpannerBackupScheduleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SpannerBackupSchedule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_spanner_backup_schedule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpannerBackupSchedule {
    pub tf_id: std::string::String,
    #[doc = "The database to create the backup schedule on."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "The instance to create the database on."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.\nYou can set this to a value up to 366 days."]
    pub retention_duration: terrars::PrimField<std::string::String>,
}

impl BuildSpannerBackupSchedule {
    pub fn build(self, stack: &mut terrars::Stack) -> SpannerBackupSchedule {
        let out = SpannerBackupSchedule(std::rc::Rc::new(SpannerBackupSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpannerBackupScheduleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database: self.database,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                retention_duration: self.retention_duration,
                encryption_config: core::default::Default::default(),
                full_backup_spec: core::default::Default::default(),
                incremental_backup_spec: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpannerBackupScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerBackupScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpannerBackupScheduleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe database to create the backup schedule on."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
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
        "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the backup schedule, which cannot be changed after\nthe backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9]."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_duration` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.\nYou can set this to a value up to 366 days."]
    pub fn retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<SpannerBackupSchedule_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_backup_spec` after provisioning.\n"]
    pub fn full_backup_spec(&self) -> terrars::ListRef<SpannerBackupSchedule_FullBackupSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.full_backup_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incremental_backup_spec` after provisioning.\n"]
    pub fn incremental_backup_spec(&self) -> terrars::ListRef<SpannerBackupSchedule_IncrementalBackupSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incremental_backup_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<SpannerBackupSchedule_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerBackupSchedule_TimeoutsRef {
        SpannerBackupSchedule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SpannerBackupScheduleDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<SpannerBackupSchedule_EncryptionConfig>>,
    full_backup_spec: std::option::Option<terrars::DynamicBlock<SpannerBackupSchedule_FullBackupSpec>>,
    incremental_backup_spec: std::option::Option<terrars::DynamicBlock<SpannerBackupSchedule_IncrementalBackupSpec>>,
    spec: std::option::Option<terrars::DynamicBlock<SpannerBackupSchedule_Spec>>,
}

#[derive(Serialize)]
pub struct SpannerBackupSchedule_EncryptionConfig {
    encryption_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl SpannerBackupSchedule_EncryptionConfig {
    #[doc =
        "Set the field `kms_key_name`.\nThe resource name of the Cloud KMS key to use for encryption.\nFormat: 'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}'"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_names`.\nFully qualified name of the KMS keys to use to encrypt this database. The keys must exist\nin the same locations as the Spanner Database."]
    pub fn set_kms_key_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.kms_key_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpannerBackupSchedule_EncryptionConfig {
    type O = terrars::BlockAssignable<SpannerBackupSchedule_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerBackupSchedule_EncryptionConfig {
    #[doc =
        "The encryption type of backups created by the backup schedule.\nPossible values are USE_DATABASE_ENCRYPTION, GOOGLE_DEFAULT_ENCRYPTION, or CUSTOMER_MANAGED_ENCRYPTION.\nIf you use CUSTOMER_MANAGED_ENCRYPTION, you must specify a kmsKeyName or kmsKeyNames. Possible values: [\"USE_DATABASE_ENCRYPTION\", \"GOOGLE_DEFAULT_ENCRYPTION\", \"CUSTOMER_MANAGED_ENCRYPTION\"]"]
    pub encryption_type: terrars::PrimField<std::string::String>,
}

impl BuildSpannerBackupSchedule_EncryptionConfig {
    pub fn build(self) -> SpannerBackupSchedule_EncryptionConfig {
        SpannerBackupSchedule_EncryptionConfig {
            encryption_type: self.encryption_type,
            kms_key_name: core::default::Default::default(),
            kms_key_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpannerBackupSchedule_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerBackupSchedule_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerBackupSchedule_EncryptionConfigRef {
        SpannerBackupSchedule_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerBackupSchedule_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encryption_type` after provisioning.\nThe encryption type of backups created by the backup schedule.\nPossible values are USE_DATABASE_ENCRYPTION, GOOGLE_DEFAULT_ENCRYPTION, or CUSTOMER_MANAGED_ENCRYPTION.\nIf you use CUSTOMER_MANAGED_ENCRYPTION, you must specify a kmsKeyName or kmsKeyNames. Possible values: [\"USE_DATABASE_ENCRYPTION\", \"GOOGLE_DEFAULT_ENCRYPTION\", \"CUSTOMER_MANAGED_ENCRYPTION\"]"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS key to use for encryption.\nFormat: 'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}'"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_names` after provisioning.\nFully qualified name of the KMS keys to use to encrypt this database. The keys must exist\nin the same locations as the Spanner Database."]
    pub fn kms_key_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_key_names", self.base))
    }
}

#[derive(Serialize)]
pub struct SpannerBackupSchedule_FullBackupSpec {}

impl SpannerBackupSchedule_FullBackupSpec { }

impl terrars::ToListMappable for SpannerBackupSchedule_FullBackupSpec {
    type O = terrars::BlockAssignable<SpannerBackupSchedule_FullBackupSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerBackupSchedule_FullBackupSpec {}

impl BuildSpannerBackupSchedule_FullBackupSpec {
    pub fn build(self) -> SpannerBackupSchedule_FullBackupSpec {
        SpannerBackupSchedule_FullBackupSpec {}
    }
}

#[allow(dead_code)]
pub struct SpannerBackupSchedule_FullBackupSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerBackupSchedule_FullBackupSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerBackupSchedule_FullBackupSpecRef {
        SpannerBackupSchedule_FullBackupSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerBackupSchedule_FullBackupSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct SpannerBackupSchedule_IncrementalBackupSpec {}

impl SpannerBackupSchedule_IncrementalBackupSpec { }

impl terrars::ToListMappable for SpannerBackupSchedule_IncrementalBackupSpec {
    type O = terrars::BlockAssignable<SpannerBackupSchedule_IncrementalBackupSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerBackupSchedule_IncrementalBackupSpec {}

impl BuildSpannerBackupSchedule_IncrementalBackupSpec {
    pub fn build(self) -> SpannerBackupSchedule_IncrementalBackupSpec {
        SpannerBackupSchedule_IncrementalBackupSpec {}
    }
}

#[allow(dead_code)]
pub struct SpannerBackupSchedule_IncrementalBackupSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerBackupSchedule_IncrementalBackupSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpannerBackupSchedule_IncrementalBackupSpecRef {
        SpannerBackupSchedule_IncrementalBackupSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerBackupSchedule_IncrementalBackupSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct SpannerBackupSchedule_Spec_CronSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpannerBackupSchedule_Spec_CronSpec {
    #[doc =
        "Set the field `text`.\nTextual representation of the crontab. User can customize the\nbackup frequency and the backup version time using the cron\nexpression. The version time must be in UTC timzeone.\nThe backup will contain an externally consistent copy of the\ndatabase at the version time. Allowed frequencies are 12 hour, 1 day,\n1 week and 1 month. Examples of valid cron specifications:\n  0 2/12 * * * : every 12 hours at (2, 14) hours past midnight in UTC.\n  0 2,14 * * * : every 12 hours at (2,14) hours past midnight in UTC.\n  0 2 * * *    : once a day at 2 past midnight in UTC.\n  0 2 * * 0    : once a week every Sunday at 2 past midnight in UTC.\n  0 2 8 * *    : once a month on 8th day at 2 past midnight in UTC."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpannerBackupSchedule_Spec_CronSpec {
    type O = terrars::BlockAssignable<SpannerBackupSchedule_Spec_CronSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerBackupSchedule_Spec_CronSpec {}

impl BuildSpannerBackupSchedule_Spec_CronSpec {
    pub fn build(self) -> SpannerBackupSchedule_Spec_CronSpec {
        SpannerBackupSchedule_Spec_CronSpec { text: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SpannerBackupSchedule_Spec_CronSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerBackupSchedule_Spec_CronSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerBackupSchedule_Spec_CronSpecRef {
        SpannerBackupSchedule_Spec_CronSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerBackupSchedule_Spec_CronSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nTextual representation of the crontab. User can customize the\nbackup frequency and the backup version time using the cron\nexpression. The version time must be in UTC timzeone.\nThe backup will contain an externally consistent copy of the\ndatabase at the version time. Allowed frequencies are 12 hour, 1 day,\n1 week and 1 month. Examples of valid cron specifications:\n  0 2/12 * * * : every 12 hours at (2, 14) hours past midnight in UTC.\n  0 2,14 * * * : every 12 hours at (2,14) hours past midnight in UTC.\n  0 2 * * *    : once a day at 2 past midnight in UTC.\n  0 2 * * 0    : once a week every Sunday at 2 past midnight in UTC.\n  0 2 8 * *    : once a month on 8th day at 2 past midnight in UTC."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpannerBackupSchedule_SpecDynamic {
    cron_spec: std::option::Option<terrars::DynamicBlock<SpannerBackupSchedule_Spec_CronSpec>>,
}

#[derive(Serialize)]
pub struct SpannerBackupSchedule_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron_spec: std::option::Option<std::vec::Vec<SpannerBackupSchedule_Spec_CronSpec>>,
    dynamic: SpannerBackupSchedule_SpecDynamic,
}

impl SpannerBackupSchedule_Spec {
    #[doc = "Set the field `cron_spec`.\n"]
    pub fn set_cron_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerBackupSchedule_Spec_CronSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cron_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cron_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpannerBackupSchedule_Spec {
    type O = terrars::BlockAssignable<SpannerBackupSchedule_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerBackupSchedule_Spec {}

impl BuildSpannerBackupSchedule_Spec {
    pub fn build(self) -> SpannerBackupSchedule_Spec {
        SpannerBackupSchedule_Spec {
            cron_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpannerBackupSchedule_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerBackupSchedule_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerBackupSchedule_SpecRef {
        SpannerBackupSchedule_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerBackupSchedule_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron_spec` after provisioning.\n"]
    pub fn cron_spec(&self) -> terrars::ListRef<SpannerBackupSchedule_Spec_CronSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cron_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SpannerBackupSchedule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpannerBackupSchedule_Timeouts {
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

impl terrars::ToListMappable for SpannerBackupSchedule_Timeouts {
    type O = terrars::BlockAssignable<SpannerBackupSchedule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerBackupSchedule_Timeouts {}

impl BuildSpannerBackupSchedule_Timeouts {
    pub fn build(self) -> SpannerBackupSchedule_Timeouts {
        SpannerBackupSchedule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpannerBackupSchedule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerBackupSchedule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerBackupSchedule_TimeoutsRef {
        SpannerBackupSchedule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerBackupSchedule_TimeoutsRef {
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
