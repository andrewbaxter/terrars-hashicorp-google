use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComposerEnvironmentData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_config: std::option::Option<std::vec::Vec<ComposerEnvironmentStorageConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComposerEnvironmentTimeoutsEl>,
    dynamic: ComposerEnvironmentDynamic,
}

struct ComposerEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComposerEnvironmentData>,
}

#[derive(Clone)]
pub struct ComposerEnvironment(std::rc::Rc<ComposerEnvironment_>);

impl ComposerEnvironment {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe location or Compute Engine region for the environment."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentStorageConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComposerEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe location or Compute Engine region for the environment."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<ComposerEnvironmentStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComposerEnvironmentTimeoutsElRef {
        ComposerEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComposerEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComposerEnvironment { }

impl terrars::ToListMappable for ComposerEnvironment {
    type O = terrars::ListRef<ComposerEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComposerEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_composer_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComposerEnvironment {
    pub tf_id: std::string::String,
    #[doc = "Name of the environment."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> ComposerEnvironment {
        let out = ComposerEnvironment(std::rc::Rc::new(ComposerEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComposerEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                config: core::default::Default::default(),
                storage_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComposerEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComposerEnvironmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe location or Compute Engine region for the environment."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<ComposerEnvironmentStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComposerEnvironmentTimeoutsElRef {
        ComposerEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElDatabaseConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironmentConfigElDatabaseConfigEl {
    #[doc =
        "Set the field `machine_type`.\nOptional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nOptional. Cloud SQL database preferred zone."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElDatabaseConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElDatabaseConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElDatabaseConfigEl {}

impl BuildComposerEnvironmentConfigElDatabaseConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElDatabaseConfigEl {
        ComposerEnvironmentConfigElDatabaseConfigEl {
            machine_type: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElDatabaseConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElDatabaseConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElDatabaseConfigElRef {
        ComposerEnvironmentConfigElDatabaseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElDatabaseConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nOptional. Cloud SQL database preferred zone."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElEncryptionConfigEl {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironmentConfigElEncryptionConfigEl { }

impl terrars::ToListMappable for ComposerEnvironmentConfigElEncryptionConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElEncryptionConfigEl {
    #[doc =
        "Optional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironmentConfigElEncryptionConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElEncryptionConfigEl {
        ComposerEnvironmentConfigElEncryptionConfigEl { kms_key_name: self.kms_key_name }
    }
}

pub struct ComposerEnvironmentConfigElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElEncryptionConfigElRef {
        ComposerEnvironmentConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElMaintenanceWindowEl {
    end_time: terrars::PrimField<std::string::String>,
    recurrence: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironmentConfigElMaintenanceWindowEl { }

impl terrars::ToListMappable for ComposerEnvironmentConfigElMaintenanceWindowEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElMaintenanceWindowEl {
    #[doc =
        "Maintenance window end time. It is used only to calculate the duration of the maintenance window. The value for end-time must be in the future, relative to 'start_time'."]
    pub end_time: terrars::PrimField<std::string::String>,
    #[doc =
        "Maintenance window recurrence. Format is a subset of RFC-5545 (https://tools.ietf.org/html/rfc5545) 'RRULE'. The only allowed values for 'FREQ' field are 'FREQ=DAILY' and 'FREQ=WEEKLY;BYDAY=...'. Example values: 'FREQ=WEEKLY;BYDAY=TU,WE', 'FREQ=DAILY'."]
    pub recurrence: terrars::PrimField<std::string::String>,
    #[doc = "Start time of the first recurrence of the maintenance window."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironmentConfigElMaintenanceWindowEl {
    pub fn build(self) -> ComposerEnvironmentConfigElMaintenanceWindowEl {
        ComposerEnvironmentConfigElMaintenanceWindowEl {
            end_time: self.end_time,
            recurrence: self.recurrence,
            start_time: self.start_time,
        }
    }
}

pub struct ComposerEnvironmentConfigElMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElMaintenanceWindowElRef {
        ComposerEnvironmentConfigElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElMaintenanceWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nMaintenance window end time. It is used only to calculate the duration of the maintenance window. The value for end-time must be in the future, relative to 'start_time'."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recurrence` after provisioning.\nMaintenance window recurrence. Format is a subset of RFC-5545 (https://tools.ietf.org/html/rfc5545) 'RRULE'. The only allowed values for 'FREQ' field are 'FREQ=DAILY' and 'FREQ=WEEKLY;BYDAY=...'. Example values: 'FREQ=WEEKLY;BYDAY=TU,WE', 'FREQ=DAILY'."]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nStart time of the first recurrence of the maintenance window."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    cidr_block: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "Set the field `display_name`.\ndisplay_name is a field for users to identify CIDR blocks."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "cidr_block must be specified in CIDR notation."]
    pub cidr_block: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    pub fn build(self) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
            cidr_block: self.cidr_block,
            display_name: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cidr_block` after provisioning.\ncidr_block must be specified in CIDR notation."]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\ndisplay_name is a field for users to identify CIDR blocks."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElDynamic {
    cidr_blocks: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    dynamic: ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElDynamic,
}

impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cidr_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cidr_blocks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    #[doc = "Whether or not master authorized networks is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
            enabled: self.enabled,
            cidr_blocks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not master authorized networks is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_ip_aliases: std::option::Option<terrars::PrimField<bool>>,
}

impl ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    #[doc = "Set the field `cluster_ipv4_cidr_block`.\n"]
    pub fn set_cluster_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_secondary_range_name`.\n"]
    pub fn set_cluster_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services_ipv4_cidr_block`.\n"]
    pub fn set_services_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services_secondary_range_name`.\n"]
    pub fn set_services_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_ip_aliases`.\n"]
    pub fn set_use_ip_aliases(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_ip_aliases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {}

impl BuildComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    pub fn build(self) -> ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
        ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            use_ip_aliases: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
        ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\n"]
    pub fn cluster_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\n"]
    pub fn cluster_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_secondary_range_name", self.base))
    }

    #[doc = "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\n"]
    pub fn services_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `services_secondary_range_name` after provisioning.\n"]
    pub fn services_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_secondary_range_name", self.base))
    }

    #[doc = "Get a reference to the value of field `use_ip_aliases` after provisioning.\n"]
    pub fn use_ip_aliases(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ip_aliases", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ip_masq_agent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_allocation_policy: std
    ::option
    ::Option<terrars::ListField<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironmentConfigElNodeConfigEl {
    #[doc =
        "Set the field `disk_size_gb`.\nThe disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_ip_masq_agent`.\nDeploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent"]
    pub fn set_enable_ip_masq_agent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ip_masq_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_allocation_policy`.\nConfiguration for controlling how IPs are allocated in the GKE cluster. Cannot be updated."]
    pub fn set_ip_allocation_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>>,
    ) -> Self {
        self.ip_allocation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. The network must belong to the environment's project. If unspecified, the \"default\" network ID in the environment's project is used. If a Custom Subnet Network is provided, subnetwork must also be provided."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `oauth_scopes`.\nThe set of Google API scopes to be made available on all node VMs. Cannot be updated. If empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_oauth_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated. If given, note that the service account must have roles/composer.worker for any GCP resources created under the Cloud Composer Environment."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe Compute Engine subnetwork to be used for machine communications, , specified as a self-link, relative resource name (e.g. \"projects/{project}/regions/{region}/subnetworks/{subnetwork}\"), or by name. If subnetwork is provided, network must also be provided and the subnetwork must belong to the enclosing environment's project and region."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nThe list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with RFC1035. Cannot be updated."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nThe Compute Engine zone in which to deploy the VMs running the Apache Airflow software, specified as the zone name or relative resource name (e.g. \"projects/{project}/zones/{zone}\"). Must belong to the enclosing environment's project and region. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElNodeConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElNodeConfigEl {}

impl BuildComposerEnvironmentConfigElNodeConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElNodeConfigEl {
        ComposerEnvironmentConfigElNodeConfigEl {
            disk_size_gb: core::default::Default::default(),
            enable_ip_masq_agent: core::default::Default::default(),
            ip_allocation_policy: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            network: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironmentConfigElNodeConfigElRef {
        ComposerEnvironmentConfigElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElNodeConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_ip_masq_agent` after provisioning.\nDeploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent"]
    pub fn enable_ip_masq_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ip_masq_agent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_allocation_policy` after provisioning.\nConfiguration for controlling how IPs are allocated in the GKE cluster. Cannot be updated."]
    pub fn ip_allocation_policy(
        &self,
    ) -> terrars::ListRef<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. The network must belong to the environment's project. If unspecified, the \"default\" network ID in the environment's project is used. If a Custom Subnet Network is provided, subnetwork must also be provided."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_scopes` after provisioning.\nThe set of Google API scopes to be made available on all node VMs. Cannot be updated. If empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn oauth_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated. If given, note that the service account must have roles/composer.worker for any GCP resources created under the Cloud Composer Environment."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe Compute Engine subnetwork to be used for machine communications, , specified as a self-link, relative resource name (e.g. \"projects/{project}/regions/{region}/subnetworks/{subnetwork}\"), or by name. If subnetwork is provided, network must also be provided and the subnetwork must belong to the enclosing environment's project and region."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with RFC1035. Cannot be updated."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe Compute Engine zone in which to deploy the VMs running the Apache Airflow software, specified as the zone name or relative resource name (e.g. \"projects/{project}/zones/{zone}\"). Must belong to the enclosing environment's project and region. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_composer_connection_subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_composer_network_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_privately_used_public_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    #[doc =
        "Set the field `cloud_composer_connection_subnetwork`.\nWhen specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_cloud_composer_connection_subnetwork(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_composer_connection_subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cloud_composer_network_ipv4_cidr_block`.\nThe CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_cloud_composer_network_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_composer_network_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cloud_sql_ipv4_cidr_block`.\nThe CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block."]
    pub fn set_cloud_sql_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_sql_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_type`.\nMode of internal communication within the Composer environment. Must be one of \"VPC_PEERING\" or \"PRIVATE_SERVICE_CONNECT\"."]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_private_endpoint`.\nIf true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_enable_private_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_privately_used_public_ips`.\nWhen enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block."]
    pub fn set_enable_privately_used_public_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_privately_used_public_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `master_ipv4_cidr_block`.\nThe IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used."]
    pub fn set_master_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `web_server_ipv4_cidr_block`.\nThe CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_web_server_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.web_server_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElPrivateEnvironmentConfigEl {}

impl BuildComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
        ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
            cloud_composer_connection_subnetwork: core::default::Default::default(),
            cloud_composer_network_ipv4_cidr_block: core::default::Default::default(),
            cloud_sql_ipv4_cidr_block: core::default::Default::default(),
            connection_type: core::default::Default::default(),
            enable_private_endpoint: core::default::Default::default(),
            enable_privately_used_public_ips: core::default::Default::default(),
            master_ipv4_cidr_block: core::default::Default::default(),
            web_server_ipv4_cidr_block: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
        ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_composer_connection_subnetwork` after provisioning.\nWhen specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn cloud_composer_connection_subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_composer_connection_subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloud_composer_network_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn cloud_composer_network_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_composer_network_ipv4_cidr_block", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `cloud_sql_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block."]
    pub fn cloud_sql_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_type` after provisioning.\nMode of internal communication within the Composer environment. Must be one of \"VPC_PEERING\" or \"PRIVATE_SERVICE_CONNECT\"."]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_endpoint` after provisioning.\nIf true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn enable_private_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_privately_used_public_ips` after provisioning.\nWhen enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block."]
    pub fn enable_privately_used_public_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_privately_used_public_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\nThe IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used."]
    pub fn master_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `web_server_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn web_server_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_server_ipv4_cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_creation_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    #[doc = "Set the field `snapshot_creation_schedule`.\nSnapshot schedule, in the unix-cron format."]
    pub fn set_snapshot_creation_schedule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.snapshot_creation_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_location`.\nthe URI of a bucket folder where to save the snapshot."]
    pub fn set_snapshot_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nA time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    #[doc = "When enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
        ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
            enabled: self.enabled,
            snapshot_creation_schedule: core::default::Default::default(),
            snapshot_location: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
        ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_creation_schedule` after provisioning.\nSnapshot schedule, in the unix-cron format."]
    pub fn snapshot_creation_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_creation_schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_location` after provisioning.\nthe URI of a bucket folder where to save the snapshot."]
    pub fn snapshot_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nA time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElRecoveryConfigElDynamic {
    scheduled_snapshots_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElRecoveryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_snapshots_config: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>>,
    dynamic: ComposerEnvironmentConfigElRecoveryConfigElDynamic,
}

impl ComposerEnvironmentConfigElRecoveryConfigEl {
    #[doc = "Set the field `scheduled_snapshots_config`.\n"]
    pub fn set_scheduled_snapshots_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduled_snapshots_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduled_snapshots_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElRecoveryConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElRecoveryConfigEl {}

impl BuildComposerEnvironmentConfigElRecoveryConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElRecoveryConfigEl {
        ComposerEnvironmentConfigElRecoveryConfigEl {
            scheduled_snapshots_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElRecoveryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElRecoveryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElRecoveryConfigElRef {
        ComposerEnvironmentConfigElRecoveryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElRecoveryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheduled_snapshots_config` after provisioning.\n"]
    pub fn scheduled_snapshots_config(
        &self,
    ) -> terrars::ListRef<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_snapshots_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElSoftwareConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airflow_config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pypi_packages: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ComposerEnvironmentConfigElSoftwareConfigEl {
    #[doc =
        "Set the field `airflow_config_overrides`.\nApache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and cannot contain \"=\" or \";\". Section and property names cannot contain characters: \".\" Apache Airflow configuration property names must be written in snake_case. Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are blacklisted, and cannot be overridden."]
    pub fn set_airflow_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.airflow_config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `env_variables`.\nAdditional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression [a-zA-Z_][a-zA-Z0-9_]*. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+), and they cannot match any of the following reserved names: AIRFLOW_HOME C_FORCE_ROOT CONTAINER_NAME DAGS_FOLDER GCP_PROJECT GCS_BUCKET GKE_CLUSTER_NAME SQL_DATABASE SQL_INSTANCE SQL_PASSWORD SQL_PROJECT SQL_REGION SQL_USER."]
    pub fn set_env_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_version`.\nThe version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression composer-([0-9]+(\\.[0-9]+\\.[0-9]+(-preview\\.[0-9]+)?)?|latest)-airflow-([0-9]+(\\.[0-9]+(\\.[0-9]+)?)?). The Cloud Composer portion of the image version is a full semantic version, or an alias in the form of major version number or 'latest'. The Apache Airflow portion of the image version is a full semantic version that points to one of the supported Apache Airflow versions, or an alias in the form of only major or major.minor versions specified. See documentation for more details and version list."]
    pub fn set_image_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pypi_packages`.\nCustom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name (e.g. \"numpy\"). Values are the lowercase extras and version specifier (e.g. \"==1.12.0\", \"[devel,gcp_api]\", \"[devel]>=1.8.2, <1.9.2\"). To specify a package without pinning it to a version specifier, use the empty string as the value."]
    pub fn set_pypi_packages(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pypi_packages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_version`.\nThe major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use Python major version 3."]
    pub fn set_python_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scheduler_count`.\nThe number of schedulers for Airflow. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-2.*.*."]
    pub fn set_scheduler_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scheduler_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElSoftwareConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElSoftwareConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElSoftwareConfigEl {}

impl BuildComposerEnvironmentConfigElSoftwareConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElSoftwareConfigEl {
        ComposerEnvironmentConfigElSoftwareConfigEl {
            airflow_config_overrides: core::default::Default::default(),
            env_variables: core::default::Default::default(),
            image_version: core::default::Default::default(),
            pypi_packages: core::default::Default::default(),
            python_version: core::default::Default::default(),
            scheduler_count: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElSoftwareConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElSoftwareConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElSoftwareConfigElRef {
        ComposerEnvironmentConfigElSoftwareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElSoftwareConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `airflow_config_overrides` after provisioning.\nApache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and cannot contain \"=\" or \";\". Section and property names cannot contain characters: \".\" Apache Airflow configuration property names must be written in snake_case. Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are blacklisted, and cannot be overridden."]
    pub fn airflow_config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.airflow_config_overrides", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env_variables` after provisioning.\nAdditional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression [a-zA-Z_][a-zA-Z0-9_]*. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+), and they cannot match any of the following reserved names: AIRFLOW_HOME C_FORCE_ROOT CONTAINER_NAME DAGS_FOLDER GCP_PROJECT GCS_BUCKET GKE_CLUSTER_NAME SQL_DATABASE SQL_INSTANCE SQL_PASSWORD SQL_PROJECT SQL_REGION SQL_USER."]
    pub fn env_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_variables", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_version` after provisioning.\nThe version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression composer-([0-9]+(\\.[0-9]+\\.[0-9]+(-preview\\.[0-9]+)?)?|latest)-airflow-([0-9]+(\\.[0-9]+(\\.[0-9]+)?)?). The Cloud Composer portion of the image version is a full semantic version, or an alias in the form of major version number or 'latest'. The Apache Airflow portion of the image version is a full semantic version that points to one of the supported Apache Airflow versions, or an alias in the form of only major or major.minor versions specified. See documentation for more details and version list."]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pypi_packages` after provisioning.\nCustom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name (e.g. \"numpy\"). Values are the lowercase extras and version specifier (e.g. \"==1.12.0\", \"[devel,gcp_api]\", \"[devel]>=1.8.2, <1.9.2\"). To specify a package without pinning it to a version specifier, use the empty string as the value."]
    pub fn pypi_packages(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pypi_packages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_version` after provisioning.\nThe major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use Python major version 3."]
    pub fn python_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheduler_count` after provisioning.\nThe number of schedulers for Airflow. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-2.*.*."]
    pub fn scheduler_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduler_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWebServerConfigEl {
    machine_type: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironmentConfigElWebServerConfigEl { }

impl terrars::ToListMappable for ComposerEnvironmentConfigElWebServerConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWebServerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWebServerConfigEl {
    #[doc =
        "Optional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
    pub machine_type: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironmentConfigElWebServerConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWebServerConfigEl {
        ComposerEnvironmentConfigElWebServerConfigEl { machine_type: self.machine_type }
    }
}

pub struct ComposerEnvironmentConfigElWebServerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWebServerConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWebServerConfigElRef {
        ComposerEnvironmentConfigElWebServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWebServerConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[doc = "Set the field `description`.\nA description of this ip range."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[doc =
        "IP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. IP range prefixes should be properly truncated. For example, 1.2.3.4/24 should be truncated to 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 should be truncated to 2001:db8::/32."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
            description: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this ip range."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nIP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. IP range prefixes should be properly truncated. For example, 1.2.3.4/24 should be truncated to 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 should be truncated to 2001:db8::/32."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElDynamic {
    allowed_ip_range: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ip_range: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>>,
    dynamic: ComposerEnvironmentConfigElWebServerNetworkAccessControlElDynamic,
}

impl ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    #[doc = "Set the field `allowed_ip_range`.\n"]
    pub fn set_allowed_ip_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_ip_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_ip_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWebServerNetworkAccessControlEl {}

impl BuildComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
            allowed_ip_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    #[doc = "Set the field `count`.\nThe number of schedulers."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\nCPU request and limit for a single Airflow scheduler replica"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for a single Airflow scheduler replica."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for a single Airflow scheduler replica."]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {}

impl BuildComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
        ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of schedulers."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow scheduler replica"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow scheduler replica."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for a single Airflow scheduler replica."]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    count: terrars::PrimField<f64>,
    cpu: terrars::PrimField<f64>,
    memory_gb: terrars::PrimField<f64>,
}

impl ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl { }

impl terrars::ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    #[doc = "The number of triggerers."]
    pub count: terrars::PrimField<f64>,
    #[doc = "CPU request and limit for a single Airflow triggerer replica."]
    pub cpu: terrars::PrimField<f64>,
    #[doc = "Memory (GB) request and limit for a single Airflow triggerer replica."]
    pub memory_gb: terrars::PrimField<f64>,
}

impl BuildComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
        ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
            count: self.count,
            cpu: self.cpu,
            memory_gb: self.memory_gb,
        }
    }
}

pub struct ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of triggerers."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow triggerer replica."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow triggerer replica."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    #[doc = "Set the field `cpu`.\nCPU request and limit for Airflow web server."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for Airflow web server."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for Airflow web server."]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {}

impl BuildComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
        ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for Airflow web server."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for Airflow web server."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for Airflow web server."]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    #[doc = "Set the field `cpu`.\nCPU request and limit for a single Airflow worker replica."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_count`.\nMaximum number of workers for autoscaling."]
    pub fn set_max_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for a single Airflow worker replica."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_count`.\nMinimum number of workers for autoscaling."]
    pub fn set_min_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for a single Airflow worker replica."]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {}

impl BuildComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
        ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
            cpu: core::default::Default::default(),
            max_count: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            min_count: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow worker replica."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_count` after provisioning.\nMaximum number of workers for autoscaling."]
    pub fn max_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow worker replica."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_count` after provisioning.\nMinimum number of workers for autoscaling."]
    pub fn min_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for a single Airflow worker replica."]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElWorkloadsConfigElDynamic {
    scheduler: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    triggerer: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    web_server: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    worker: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggerer: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
    dynamic: ComposerEnvironmentConfigElWorkloadsConfigElDynamic,
}

impl ComposerEnvironmentConfigElWorkloadsConfigEl {
    #[doc = "Set the field `scheduler`.\n"]
    pub fn set_scheduler(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduler = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduler = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `triggerer`.\n"]
    pub fn set_triggerer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.triggerer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.triggerer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_server`.\n"]
    pub fn set_web_server(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigElWorkloadsConfigEl {}

impl BuildComposerEnvironmentConfigElWorkloadsConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigEl {
        ComposerEnvironmentConfigElWorkloadsConfigEl {
            scheduler: core::default::Default::default(),
            triggerer: core::default::Default::default(),
            web_server: core::default::Default::default(),
            worker: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElWorkloadsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElWorkloadsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElWorkloadsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheduler` after provisioning.\n"]
    pub fn scheduler(&self) -> terrars::ListRef<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduler", self.base))
    }

    #[doc = "Get a reference to the value of field `triggerer` after provisioning.\n"]
    pub fn triggerer(&self) -> terrars::ListRef<ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggerer", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server` after provisioning.\n"]
    pub fn web_server(&self) -> terrars::ListRef<ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> terrars::ListRef<ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElDynamic {
    database_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElDatabaseConfigEl>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElEncryptionConfigEl>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElMaintenanceWindowEl>>,
    master_authorized_networks_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElNodeConfigEl>>,
    private_environment_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    recovery_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElRecoveryConfigEl>>,
    software_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElSoftwareConfigEl>>,
    web_server_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWebServerConfigEl>>,
    web_server_network_access_control: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    workloads_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigEl>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironmentConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resilience_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElDatabaseConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_authorized_networks_config: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_environment_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElRecoveryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElSoftwareConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElWebServerConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_network_access_control: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workloads_config: std::option::Option<std::vec::Vec<ComposerEnvironmentConfigElWorkloadsConfigEl>>,
    dynamic: ComposerEnvironmentConfigElDynamic,
}

impl ComposerEnvironmentConfigEl {
    #[doc =
        "Set the field `environment_size`.\nThe size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_environment_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_count`.\nThe number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resilience_mode`.\nWhether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer."]
    pub fn set_resilience_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resilience_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_config`.\n"]
    pub fn set_database_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElDatabaseConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_authorized_networks_config`.\n"]
    pub fn set_master_authorized_networks_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.master_authorized_networks_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.master_authorized_networks_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_environment_config`.\n"]
    pub fn set_private_environment_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_environment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_environment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recovery_config`.\n"]
    pub fn set_recovery_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recovery_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recovery_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElSoftwareConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.software_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.software_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_server_config`.\n"]
    pub fn set_web_server_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElWebServerConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_server_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_server_network_access_control`.\n"]
    pub fn set_web_server_network_access_control(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_server_network_access_control = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server_network_access_control = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workloads_config`.\n"]
    pub fn set_workloads_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workloads_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workloads_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironmentConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentConfigEl {}

impl BuildComposerEnvironmentConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigEl {
        ComposerEnvironmentConfigEl {
            environment_size: core::default::Default::default(),
            node_count: core::default::Default::default(),
            resilience_mode: core::default::Default::default(),
            database_config: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            master_authorized_networks_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            private_environment_config: core::default::Default::default(),
            recovery_config: core::default::Default::default(),
            software_config: core::default::Default::default(),
            web_server_config: core::default::Default::default(),
            web_server_network_access_control: core::default::Default::default(),
            workloads_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironmentConfigElRef {
        ComposerEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `airflow_uri` after provisioning.\nThe URI of the Apache Airflow Web UI hosted within this environment."]
    pub fn airflow_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.airflow_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dag_gcs_prefix` after provisioning.\nThe Cloud Storage prefix of the DAGs for this environment. Although Cloud Storage objects reside in a flat namespace, a hierarchical file tree can be simulated using '/'-delimited object name prefixes. DAG objects for this environment reside in a simulated directory with this prefix."]
    pub fn dag_gcs_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dag_gcs_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_size` after provisioning.\nThe size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn environment_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gke_cluster` after provisioning.\nThe Kubernetes Engine cluster used to run this environment."]
    pub fn gke_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resilience_mode` after provisioning.\nWhether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer."]
    pub fn resilience_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resilience_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `database_config` after provisioning.\n"]
    pub fn database_config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElDatabaseConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_config", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<ComposerEnvironmentConfigElMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_authorized_networks_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `private_environment_config` after provisioning.\n"]
    pub fn private_environment_config(
        &self,
    ) -> terrars::ListRef<ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_environment_config", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_config` after provisioning.\n"]
    pub fn recovery_config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElRecoveryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recovery_config", self.base))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElSoftwareConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_config` after provisioning.\n"]
    pub fn web_server_config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElWebServerConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server_config", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_network_access_control` after provisioning.\n"]
    pub fn web_server_network_access_control(
        &self,
    ) -> terrars::ListRef<ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server_network_access_control", self.base))
    }

    #[doc = "Get a reference to the value of field `workloads_config` after provisioning.\n"]
    pub fn workloads_config(&self) -> terrars::ListRef<ComposerEnvironmentConfigElWorkloadsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workloads_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentStorageConfigEl {
    bucket: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironmentStorageConfigEl { }

impl terrars::ToListMappable for ComposerEnvironmentStorageConfigEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentStorageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentStorageConfigEl {
    #[doc = "Optional. Name of an existing Cloud Storage bucket to be used by the environment."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironmentStorageConfigEl {
    pub fn build(self) -> ComposerEnvironmentStorageConfigEl {
        ComposerEnvironmentStorageConfigEl { bucket: self.bucket }
    }
}

pub struct ComposerEnvironmentStorageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentStorageConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironmentStorageConfigElRef {
        ComposerEnvironmentStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentStorageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nOptional. Name of an existing Cloud Storage bucket to be used by the environment."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironmentTimeoutsEl {
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

impl terrars::ToListMappable for ComposerEnvironmentTimeoutsEl {
    type O = terrars::BlockAssignable<ComposerEnvironmentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironmentTimeoutsEl {}

impl BuildComposerEnvironmentTimeoutsEl {
    pub fn build(self) -> ComposerEnvironmentTimeoutsEl {
        ComposerEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComposerEnvironmentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironmentTimeoutsElRef {
        ComposerEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironmentTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct ComposerEnvironmentDynamic {
    config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentConfigEl>>,
    storage_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironmentStorageConfigEl>>,
}
