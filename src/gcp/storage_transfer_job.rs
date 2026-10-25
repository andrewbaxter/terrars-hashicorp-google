#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageTransferJobData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_stream: std::option::Option<std::vec::Vec<StorageTransferJob_EventStream>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<StorageTransferJob_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std::option::Option<std::vec::Vec<StorageTransferJob_NotificationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_spec: std::option::Option<std::vec::Vec<StorageTransferJob_ReplicationSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<StorageTransferJob_Schedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_spec: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec>>,
    dynamic: StorageTransferJobDynamic,
}

struct StorageTransferJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageTransferJobData_>,
}

#[derive(Clone)]
pub struct StorageTransferJob(std::rc::Rc<StorageTransferJob_>);

impl StorageTransferJob {
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

    #[doc = "Set the field `name`.\nThe name of the Transfer Job."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe user-managed service account to run the job. If this field is specified, the given service account is granted the necessary permissions to all applicable resources (e.g. GCS buckets) required by the job."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_stream`.\n"]
    pub fn set_event_stream(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_EventStream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_stream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_NotificationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_spec`.\n"]
    pub fn set_replication_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_ReplicationSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_Schedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transfer_spec`.\n"]
    pub fn set_transfer_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transfer_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transfer_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nWhen the Transfer Job was created."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_time` after provisioning.\nWhen the Transfer Job was deleted."]
    pub fn deletion_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUnique description to identify the Transfer Job."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modification_time` after provisioning.\nWhen the Transfer Job was last modified."]
    pub fn last_modification_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modification_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Transfer Job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe user-managed service account to run the job. If this field is specified, the given service account is granted the necessary permissions to all applicable resources (e.g. GCS buckets) required by the job."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_stream` after provisioning.\n"]
    pub fn event_stream(&self) -> terrars::ListRef<StorageTransferJob_EventStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_stream", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<StorageTransferJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<StorageTransferJob_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_spec` after provisioning.\n"]
    pub fn replication_spec(&self) -> terrars::ListRef<StorageTransferJob_ReplicationSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<StorageTransferJob_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transfer_spec` after provisioning.\n"]
    pub fn transfer_spec(&self) -> terrars::ListRef<StorageTransferJob_TransferSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_spec", self.extract_ref()))
    }
}

impl terrars::Referable for StorageTransferJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageTransferJob { }

impl terrars::ToListMappable for StorageTransferJob {
    type O = terrars::ListRef<StorageTransferJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageTransferJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_transfer_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageTransferJob {
    pub tf_id: std::string::String,
    #[doc = "Unique description to identify the Transfer Job."]
    pub description: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageTransferJob {
        let out = StorageTransferJob(std::rc::Rc::new(StorageTransferJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageTransferJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                status: core::default::Default::default(),
                event_stream: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                replication_spec: core::default::Default::default(),
                schedule: core::default::Default::default(),
                transfer_spec: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageTransferJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageTransferJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nWhen the Transfer Job was created."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_time` after provisioning.\nWhen the Transfer Job was deleted."]
    pub fn deletion_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUnique description to identify the Transfer Job."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modification_time` after provisioning.\nWhen the Transfer Job was last modified."]
    pub fn last_modification_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modification_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Transfer Job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe user-managed service account to run the job. If this field is specified, the given service account is granted the necessary permissions to all applicable resources (e.g. GCS buckets) required by the job."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_stream` after provisioning.\n"]
    pub fn event_stream(&self) -> terrars::ListRef<StorageTransferJob_EventStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_stream", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<StorageTransferJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<StorageTransferJob_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_spec` after provisioning.\n"]
    pub fn replication_spec(&self) -> terrars::ListRef<StorageTransferJob_ReplicationSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<StorageTransferJob_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transfer_spec` after provisioning.\n"]
    pub fn transfer_spec(&self) -> terrars::ListRef<StorageTransferJob_TransferSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_spec", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJobDynamic {
    event_stream: std::option::Option<terrars::DynamicBlock<StorageTransferJob_EventStream>>,
    logging_config: std::option::Option<terrars::DynamicBlock<StorageTransferJob_LoggingConfig>>,
    notification_config: std::option::Option<terrars::DynamicBlock<StorageTransferJob_NotificationConfig>>,
    replication_spec: std::option::Option<terrars::DynamicBlock<StorageTransferJob_ReplicationSpec>>,
    schedule: std::option::Option<terrars::DynamicBlock<StorageTransferJob_Schedule>>,
    transfer_spec: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec>>,
}

#[derive(Serialize)]
pub struct StorageTransferJob_EventStream {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_stream_expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_stream_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_EventStream {
    #[doc =
        "Set the field `event_stream_expiration_time`.\nSpecifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated"]
    pub fn set_event_stream_expiration_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.event_stream_expiration_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `event_stream_start_time`.\nSpecifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately"]
    pub fn set_event_stream_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.event_stream_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_EventStream {
    type O = terrars::BlockAssignable<StorageTransferJob_EventStream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_EventStream {
    #[doc =
        "Specifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_EventStream {
    pub fn build(self) -> StorageTransferJob_EventStream {
        StorageTransferJob_EventStream {
            event_stream_expiration_time: core::default::Default::default(),
            event_stream_start_time: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_EventStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_EventStreamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageTransferJob_EventStreamRef {
        StorageTransferJob_EventStreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_EventStreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_stream_expiration_time` after provisioning.\nSpecifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated"]
    pub fn event_stream_expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_stream_expiration_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `event_stream_start_time` after provisioning.\nSpecifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately"]
    pub fn event_stream_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_stream_start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_on_prem_gcs_transfer_logs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_action_states: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl StorageTransferJob_LoggingConfig {
    #[doc =
        "Set the field `enable_on_prem_gcs_transfer_logs`.\nFor transfers with a PosixFilesystem source, this option enables the Cloud Storage transfer logs for this transfer."]
    pub fn set_enable_on_prem_gcs_transfer_logs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_on_prem_gcs_transfer_logs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `log_action_states`.\nStates in which logActions are logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn set_log_action_states(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.log_action_states = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `log_actions`.\nSpecifies the actions to be logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn set_log_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.log_actions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_LoggingConfig {
    type O = terrars::BlockAssignable<StorageTransferJob_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_LoggingConfig {}

impl BuildStorageTransferJob_LoggingConfig {
    pub fn build(self) -> StorageTransferJob_LoggingConfig {
        StorageTransferJob_LoggingConfig {
            enable_on_prem_gcs_transfer_logs: core::default::Default::default(),
            log_action_states: core::default::Default::default(),
            log_actions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageTransferJob_LoggingConfigRef {
        StorageTransferJob_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_on_prem_gcs_transfer_logs` after provisioning.\nFor transfers with a PosixFilesystem source, this option enables the Cloud Storage transfer logs for this transfer."]
    pub fn enable_on_prem_gcs_transfer_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_on_prem_gcs_transfer_logs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_action_states` after provisioning.\nStates in which logActions are logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn log_action_states(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_action_states", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_actions` after provisioning.\nSpecifies the actions to be logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn log_actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_actions", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_NotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    payload_format: terrars::PrimField<std::string::String>,
    pubsub_topic: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_NotificationConfig {
    #[doc =
        "Set the field `event_types`.\nEvent types for which a notification is desired. If empty, send notifications for all event types. The valid types are \"TRANSFER_OPERATION_SUCCESS\", \"TRANSFER_OPERATION_FAILED\", \"TRANSFER_OPERATION_ABORTED\"."]
    pub fn set_event_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_NotificationConfig {
    type O = terrars::BlockAssignable<StorageTransferJob_NotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_NotificationConfig {
    #[doc = "The desired format of the notification message payloads. One of \"NONE\" or \"JSON\"."]
    pub payload_format: terrars::PrimField<std::string::String>,
    #[doc = "The Topic.name of the Pub/Sub topic to which to publish notifications."]
    pub pubsub_topic: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_NotificationConfig {
    pub fn build(self) -> StorageTransferJob_NotificationConfig {
        StorageTransferJob_NotificationConfig {
            event_types: core::default::Default::default(),
            payload_format: self.payload_format,
            pubsub_topic: self.pubsub_topic,
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_NotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_NotificationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageTransferJob_NotificationConfigRef {
        StorageTransferJob_NotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_NotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_types` after provisioning.\nEvent types for which a notification is desired. If empty, send notifications for all event types. The valid types are \"TRANSFER_OPERATION_SUCCESS\", \"TRANSFER_OPERATION_FAILED\", \"TRANSFER_OPERATION_ABORTED\"."]
    pub fn event_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.event_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `payload_format` after provisioning.\nThe desired format of the notification message payloads. One of \"NONE\" or \"JSON\"."]
    pub fn payload_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Topic.name of the Pub/Sub topic to which to publish notifications."]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_GcsDataSink {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageTransferJob_TransferSpec_GcsDataSink {
    #[doc =
        "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_GcsDataSink {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_GcsDataSink>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_GcsDataSink {
    #[doc = "Google Cloud Storage bucket name."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_GcsDataSink {
    pub fn build(self) -> StorageTransferJob_TransferSpec_GcsDataSink {
        StorageTransferJob_TransferSpec_GcsDataSink {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_GcsDataSinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_GcsDataSinkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_GcsDataSinkRef {
        StorageTransferJob_TransferSpec_GcsDataSinkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_GcsDataSinkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_GcsDataSource {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageTransferJob_TransferSpec_GcsDataSource {
    #[doc =
        "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_GcsDataSource {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_GcsDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_GcsDataSource {
    #[doc = "Google Cloud Storage bucket name."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_GcsDataSource {
    pub fn build(self) -> StorageTransferJob_TransferSpec_GcsDataSource {
        StorageTransferJob_TransferSpec_GcsDataSource {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_GcsDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_GcsDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_GcsDataSourceRef {
        StorageTransferJob_TransferSpec_GcsDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_GcsDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_ObjectConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_modified_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_modified_since: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_time_elapsed_since_last_modification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_time_elapsed_since_last_modification: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageTransferJob_TransferSpec_ObjectConditions {
    #[doc =
        "Set the field `exclude_prefixes`.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn set_exclude_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_prefixes`.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn set_include_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `last_modified_before`.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_before(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_modified_before = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `last_modified_since`.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_since(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_modified_since = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_max_time_elapsed_since_last_modification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_time_elapsed_since_last_modification = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_min_time_elapsed_since_last_modification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.min_time_elapsed_since_last_modification = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_ObjectConditions {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_ObjectConditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_ObjectConditions {}

impl BuildStorageTransferJob_TransferSpec_ObjectConditions {
    pub fn build(self) -> StorageTransferJob_TransferSpec_ObjectConditions {
        StorageTransferJob_TransferSpec_ObjectConditions {
            exclude_prefixes: core::default::Default::default(),
            include_prefixes: core::default::Default::default(),
            last_modified_before: core::default::Default::default(),
            last_modified_since: core::default::Default::default(),
            max_time_elapsed_since_last_modification: core::default::Default::default(),
            min_time_elapsed_since_last_modification: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_ObjectConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_ObjectConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_ObjectConditionsRef {
        StorageTransferJob_TransferSpec_ObjectConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_ObjectConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_prefixes` after provisioning.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn exclude_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_prefixes` after provisioning.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn include_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_before` after provisioning.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_before", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_since` after provisioning.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_since(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_since", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_time_elapsed_since_last_modification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_elapsed_since_last_modification", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `min_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn min_time_elapsed_since_last_modification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_time_elapsed_since_last_modification", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    symlink: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporary_hold: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_created: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions {
    #[doc =
        "Set the field `acl`.\nSpecifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.acl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gid`.\nSpecifies how each file's POSIX group ID (GID) attribute should be handled by the transfer."]
    pub fn set_gid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gid = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\nSpecifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\nSpecifies how each file's mode attribute should be handled by the transfer."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_class`.\nSpecifies the storage class to set on objects being transferred to Google Cloud Storage buckets"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `symlink`.\nSpecifies how symlinks should be handled by the transfer."]
    pub fn set_symlink(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.symlink = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temporary_hold`.\nSSpecifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_temporary_hold(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.temporary_hold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_created`.\nSpecifies how each object's timeCreated metadata is preserved for transfers."]
    pub fn set_time_created(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_created = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `uid`.\nSpecifies how each file's POSIX user ID (UID) attribute should be handled by the transfer."]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_TransferOptions_MetadataOptions {}

impl BuildStorageTransferJob_TransferSpec_TransferOptions_MetadataOptions {
    pub fn build(self) -> StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions {
        StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions {
            acl: core::default::Default::default(),
            gid: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            mode: core::default::Default::default(),
            storage_class: core::default::Default::default(),
            symlink: core::default::Default::default(),
            temporary_hold: core::default::Default::default(),
            time_created: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_TransferOptions_MetadataOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_TransferOptions_MetadataOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_TransferOptions_MetadataOptionsRef {
        StorageTransferJob_TransferSpec_TransferOptions_MetadataOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_TransferOptions_MetadataOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `acl` after provisioning.\nSpecifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gid` after provisioning.\nSpecifies how each file's POSIX group ID (GID) attribute should be handled by the transfer."]
    pub fn gid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nSpecifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSpecifies how each file's mode attribute should be handled by the transfer."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nSpecifies the storage class to set on objects being transferred to Google Cloud Storage buckets"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `symlink` after provisioning.\nSpecifies how symlinks should be handled by the transfer."]
    pub fn symlink(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.symlink", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temporary_hold` after provisioning.\nSSpecifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn temporary_hold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporary_hold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_created` after provisioning.\nSpecifies how each object's timeCreated metadata is preserved for transfers."]
    pub fn time_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSpecifies how each file's POSIX user ID (UID) attribute should be handled by the transfer."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJob_TransferSpec_TransferOptionsDynamic {
    metadata_options: std
    ::option
    ::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions>>,
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_TransferOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_objects_from_source_after_transfer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_objects_unique_in_sink: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_objects_already_existing_in_sink: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_when: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_options: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions>>,
    dynamic: StorageTransferJob_TransferSpec_TransferOptionsDynamic,
}

impl StorageTransferJob_TransferSpec_TransferOptions {
    #[doc =
        "Set the field `delete_objects_from_source_after_transfer`.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn set_delete_objects_from_source_after_transfer(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.delete_objects_from_source_after_transfer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_objects_unique_in_sink`.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn set_delete_objects_unique_in_sink(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_objects_unique_in_sink = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `overwrite_objects_already_existing_in_sink`.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn set_overwrite_objects_already_existing_in_sink(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.overwrite_objects_already_existing_in_sink = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `overwrite_when`.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn set_overwrite_when(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.overwrite_when = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_options`.\n"]
    pub fn set_metadata_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_TransferOptions_MetadataOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_TransferOptions {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_TransferOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_TransferOptions {}

impl BuildStorageTransferJob_TransferSpec_TransferOptions {
    pub fn build(self) -> StorageTransferJob_TransferSpec_TransferOptions {
        StorageTransferJob_TransferSpec_TransferOptions {
            delete_objects_from_source_after_transfer: core::default::Default::default(),
            delete_objects_unique_in_sink: core::default::Default::default(),
            overwrite_objects_already_existing_in_sink: core::default::Default::default(),
            overwrite_when: core::default::Default::default(),
            metadata_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_TransferOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_TransferOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_TransferOptionsRef {
        StorageTransferJob_TransferSpec_TransferOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_TransferOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delete_objects_from_source_after_transfer` after provisioning.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn delete_objects_from_source_after_transfer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_objects_from_source_after_transfer", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `delete_objects_unique_in_sink` after provisioning.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn delete_objects_unique_in_sink(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_objects_unique_in_sink", self.base))
    }

    #[doc =
        "Get a reference to the value of field `overwrite_objects_already_existing_in_sink` after provisioning.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn overwrite_objects_already_existing_in_sink(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_objects_already_existing_in_sink", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `overwrite_when` after provisioning.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn overwrite_when(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_when", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(
        &self,
    ) -> terrars::ListRef<StorageTransferJob_TransferSpec_TransferOptions_MetadataOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJob_ReplicationSpecDynamic {
    gcs_data_sink: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_GcsDataSink>>,
    gcs_data_source: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_GcsDataSource>>,
    object_conditions: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_ObjectConditions>>,
    transfer_options: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_TransferOptions>>,
}

#[derive(Serialize)]
pub struct StorageTransferJob_ReplicationSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_data_sink: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_GcsDataSink>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_data_source: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_GcsDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_conditions: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_ObjectConditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_options: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_TransferOptions>>,
    dynamic: StorageTransferJob_ReplicationSpecDynamic,
}

impl StorageTransferJob_ReplicationSpec {
    #[doc = "Set the field `gcs_data_sink`.\n"]
    pub fn set_gcs_data_sink(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_GcsDataSink>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_data_sink = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_sink = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_data_source`.\n"]
    pub fn set_gcs_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_GcsDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `object_conditions`.\n"]
    pub fn set_object_conditions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_ObjectConditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.object_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.object_conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transfer_options`.\n"]
    pub fn set_transfer_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_TransferOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transfer_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_ReplicationSpec {
    type O = terrars::BlockAssignable<StorageTransferJob_ReplicationSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_ReplicationSpec {}

impl BuildStorageTransferJob_ReplicationSpec {
    pub fn build(self) -> StorageTransferJob_ReplicationSpec {
        StorageTransferJob_ReplicationSpec {
            gcs_data_sink: core::default::Default::default(),
            gcs_data_source: core::default::Default::default(),
            object_conditions: core::default::Default::default(),
            transfer_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_ReplicationSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_ReplicationSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageTransferJob_ReplicationSpecRef {
        StorageTransferJob_ReplicationSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_ReplicationSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcs_data_sink` after provisioning.\n"]
    pub fn gcs_data_sink(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_GcsDataSinkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_data_sink", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_data_source` after provisioning.\n"]
    pub fn gcs_data_source(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_GcsDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `object_conditions` after provisioning.\n"]
    pub fn object_conditions(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_ObjectConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_options` after provisioning.\n"]
    pub fn transfer_options(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_TransferOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_options", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_Schedule_ScheduleEndDate {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl StorageTransferJob_Schedule_ScheduleEndDate { }

impl terrars::ToListMappable for StorageTransferJob_Schedule_ScheduleEndDate {
    type O = terrars::BlockAssignable<StorageTransferJob_Schedule_ScheduleEndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_Schedule_ScheduleEndDate {
    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: terrars::PrimField<f64>,
    #[doc = "Month of year. Must be from 1 to 12."]
    pub month: terrars::PrimField<f64>,
    #[doc = "Year of date. Must be from 1 to 9999."]
    pub year: terrars::PrimField<f64>,
}

impl BuildStorageTransferJob_Schedule_ScheduleEndDate {
    pub fn build(self) -> StorageTransferJob_Schedule_ScheduleEndDate {
        StorageTransferJob_Schedule_ScheduleEndDate {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_Schedule_ScheduleEndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_Schedule_ScheduleEndDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_Schedule_ScheduleEndDateRef {
        StorageTransferJob_Schedule_ScheduleEndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_Schedule_ScheduleEndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of year. Must be from 1 to 12."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of date. Must be from 1 to 9999."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_Schedule_ScheduleStartDate {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl StorageTransferJob_Schedule_ScheduleStartDate { }

impl terrars::ToListMappable for StorageTransferJob_Schedule_ScheduleStartDate {
    type O = terrars::BlockAssignable<StorageTransferJob_Schedule_ScheduleStartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_Schedule_ScheduleStartDate {
    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: terrars::PrimField<f64>,
    #[doc = "Month of year. Must be from 1 to 12."]
    pub month: terrars::PrimField<f64>,
    #[doc = "Year of date. Must be from 1 to 9999."]
    pub year: terrars::PrimField<f64>,
}

impl BuildStorageTransferJob_Schedule_ScheduleStartDate {
    pub fn build(self) -> StorageTransferJob_Schedule_ScheduleStartDate {
        StorageTransferJob_Schedule_ScheduleStartDate {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_Schedule_ScheduleStartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_Schedule_ScheduleStartDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_Schedule_ScheduleStartDateRef {
        StorageTransferJob_Schedule_ScheduleStartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_Schedule_ScheduleStartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of year. Must be from 1 to 12."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of date. Must be from 1 to 9999."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_Schedule_StartTimeOfDay {
    hours: terrars::PrimField<f64>,
    minutes: terrars::PrimField<f64>,
    nanos: terrars::PrimField<f64>,
    seconds: terrars::PrimField<f64>,
}

impl StorageTransferJob_Schedule_StartTimeOfDay { }

impl terrars::ToListMappable for StorageTransferJob_Schedule_StartTimeOfDay {
    type O = terrars::BlockAssignable<StorageTransferJob_Schedule_StartTimeOfDay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_Schedule_StartTimeOfDay {
    #[doc = "Hours of day in 24 hour format. Should be from 0 to 23."]
    pub hours: terrars::PrimField<f64>,
    #[doc = "Minutes of hour of day. Must be from 0 to 59."]
    pub minutes: terrars::PrimField<f64>,
    #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub nanos: terrars::PrimField<f64>,
    #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildStorageTransferJob_Schedule_StartTimeOfDay {
    pub fn build(self) -> StorageTransferJob_Schedule_StartTimeOfDay {
        StorageTransferJob_Schedule_StartTimeOfDay {
            hours: self.hours,
            minutes: self.minutes,
            nanos: self.nanos,
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_Schedule_StartTimeOfDayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_Schedule_StartTimeOfDayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageTransferJob_Schedule_StartTimeOfDayRef {
        StorageTransferJob_Schedule_StartTimeOfDayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_Schedule_StartTimeOfDayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJob_ScheduleDynamic {
    schedule_end_date: std::option::Option<terrars::DynamicBlock<StorageTransferJob_Schedule_ScheduleEndDate>>,
    schedule_start_date: std::option::Option<terrars::DynamicBlock<StorageTransferJob_Schedule_ScheduleStartDate>>,
    start_time_of_day: std::option::Option<terrars::DynamicBlock<StorageTransferJob_Schedule_StartTimeOfDay>>,
}

#[derive(Serialize)]
pub struct StorageTransferJob_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repeat_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_end_date: std::option::Option<std::vec::Vec<StorageTransferJob_Schedule_ScheduleEndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_start_date: std::option::Option<std::vec::Vec<StorageTransferJob_Schedule_ScheduleStartDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time_of_day: std::option::Option<std::vec::Vec<StorageTransferJob_Schedule_StartTimeOfDay>>,
    dynamic: StorageTransferJob_ScheduleDynamic,
}

impl StorageTransferJob_Schedule {
    #[doc =
        "Set the field `repeat_interval`.\nInterval between the start of each scheduled transfer. If unspecified, the default value is 24 hours. This value may not be less than 1 hour. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_repeat_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repeat_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_end_date`.\n"]
    pub fn set_schedule_end_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_Schedule_ScheduleEndDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule_end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule_start_date`.\n"]
    pub fn set_schedule_start_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_Schedule_ScheduleStartDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule_start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_start_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_time_of_day`.\n"]
    pub fn set_start_time_of_day(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_Schedule_StartTimeOfDay>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time_of_day = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time_of_day = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_Schedule {
    type O = terrars::BlockAssignable<StorageTransferJob_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_Schedule {}

impl BuildStorageTransferJob_Schedule {
    pub fn build(self) -> StorageTransferJob_Schedule {
        StorageTransferJob_Schedule {
            repeat_interval: core::default::Default::default(),
            schedule_end_date: core::default::Default::default(),
            schedule_start_date: core::default::Default::default(),
            start_time_of_day: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageTransferJob_ScheduleRef {
        StorageTransferJob_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `repeat_interval` after provisioning.\nInterval between the start of each scheduled transfer. If unspecified, the default value is 24 hours. This value may not be less than 1 hour. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn repeat_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repeat_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_end_date` after provisioning.\n"]
    pub fn schedule_end_date(&self) -> terrars::ListRef<StorageTransferJob_Schedule_ScheduleEndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_start_date` after provisioning.\n"]
    pub fn schedule_start_date(&self) -> terrars::ListRef<StorageTransferJob_Schedule_ScheduleStartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time_of_day` after provisioning.\n"]
    pub fn start_time_of_day(&self) -> terrars::ListRef<StorageTransferJob_Schedule_StartTimeOfDayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time_of_day", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list_api: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_model: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata {
    #[doc =
        "Set the field `auth_method`.\nAuthentication and authorization method used by the storage service. When not specified, Transfer Service will attempt to determine right auth method to use."]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `list_api`.\nThe Listing API to use for discovering objects. When not specified, Transfer Service will attempt to determine the right API to use."]
    pub fn set_list_api(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.list_api = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protocol`.\nThe network protocol of the agent. When not specified, the default value of NetworkProtocol NETWORK_PROTOCOL_HTTPS is used."]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_model`.\nAPI request model used to call the storage service. When not specified, the default value of RequestModel REQUEST_MODEL_VIRTUAL_HOSTED_STYLE is used."]
    pub fn set_request_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_model = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata {}

impl BuildStorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata {
    pub fn build(self) -> StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata {
        StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata {
            auth_method: core::default::Default::default(),
            list_api: core::default::Default::default(),
            protocol: core::default::Default::default(),
            request_model: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3MetadataRef {
        StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nAuthentication and authorization method used by the storage service. When not specified, Transfer Service will attempt to determine right auth method to use."]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `list_api` after provisioning.\nThe Listing API to use for discovering objects. When not specified, Transfer Service will attempt to determine the right API to use."]
    pub fn list_api(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_api", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe network protocol of the agent. When not specified, the default value of NetworkProtocol NETWORK_PROTOCOL_HTTPS is used."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_model` after provisioning.\nAPI request model used to call the storage service. When not specified, the default value of RequestModel REQUEST_MODEL_VIRTUAL_HOSTED_STYLE is used."]
    pub fn request_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceDynamic {
    s3_metadata: std
    ::option
    ::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata>>,
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource {
    bucket_name: terrars::PrimField<std::string::String>,
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_metadata: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata>>,
    dynamic: StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceDynamic,
}

impl StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource {
    #[doc = "Set the field `path`.\nSpecifies the path to transfer objects."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nSpecifies the region to sign requests with. This can be left blank if requests should be signed with an empty region."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_metadata`.\n"]
    pub fn set_s3_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3Metadata>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_metadata = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_AwsS3CompatibleDataSource {
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = "Endpoint of the storage service."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_AwsS3CompatibleDataSource {
    pub fn build(self) -> StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource {
        StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource {
            bucket_name: self.bucket_name,
            endpoint: self.endpoint,
            path: core::default::Default::default(),
            region: core::default::Default::default(),
            s3_metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceRef {
        StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nName of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nEndpoint of the storage service."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nSpecifies the path to transfer objects."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nSpecifies the region to sign requests with. This can be left blank if requests should be signed with an empty region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_metadata` after provisioning.\n"]
    pub fn s3_metadata(
        &self,
    ) -> terrars::ListRef<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource_S3MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey {
    access_key_id: terrars::PrimField<std::string::String>,
    secret_access_key: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey {
    #[doc = "AWS Key ID."]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = "AWS Secret Access Key."]
    pub secret_access_key: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey {
    pub fn build(self) -> StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey {
        StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKeyRef {
        StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\nAWS Key ID."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_access_key` after provisioning.\nAWS Secret Access Key."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJob_TransferSpec_AwsS3DataSourceDynamic {
    aws_access_key: std
    ::option
    ::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey>>,
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_AwsS3DataSource {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudfront_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_private_network: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_access_key: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey>>,
    dynamic: StorageTransferJob_TransferSpec_AwsS3DataSourceDynamic,
}

impl StorageTransferJob_TransferSpec_AwsS3DataSource {
    #[doc =
        "Set the field `cloudfront_domain`.\nThe CloudFront distribution domain name pointing to this bucket, to use when fetching. See [Transfer from S3 via CloudFront](https://cloud.google.com/storage-transfer/docs/s3-cloudfront) for more information. Format: https://{id}.cloudfront.net or any valid custom domain. Must begin with https://."]
    pub fn set_cloudfront_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloudfront_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `credentials_secret`.\nThe Resource name of a secret in Secret Manager. AWS credentials must be stored in Secret Manager in JSON format. If credentials_secret is specified, do not specify role_arn or aws_access_key. Format: projects/{projectNumber}/secrets/{secret_name}."]
    pub fn set_credentials_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.credentials_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `managed_private_network`.\nEgress bytes over a Google-managed private network. This network is shared between other users of Storage Transfer Service."]
    pub fn set_managed_private_network(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.managed_private_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nS3 Bucket path in bucket to transfer."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `role_arn`.\nThe Amazon Resource Name (ARN) of the role to support temporary credentials via 'AssumeRoleWithWebIdentity'. For more information about ARNs, see [IAM ARNs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns). When a role ARN is provided, Transfer Service fetches temporary credentials for the session using a 'AssumeRoleWithWebIdentity' call for the provided role using the [GoogleServiceAccount][] for this project."]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_access_key`.\n"]
    pub fn set_aws_access_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_access_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_access_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_AwsS3DataSource {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_AwsS3DataSource {
    #[doc = "S3 Bucket name."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_AwsS3DataSource {
    pub fn build(self) -> StorageTransferJob_TransferSpec_AwsS3DataSource {
        StorageTransferJob_TransferSpec_AwsS3DataSource {
            bucket_name: self.bucket_name,
            cloudfront_domain: core::default::Default::default(),
            credentials_secret: core::default::Default::default(),
            managed_private_network: core::default::Default::default(),
            path: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            aws_access_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_AwsS3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_AwsS3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_AwsS3DataSourceRef {
        StorageTransferJob_TransferSpec_AwsS3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_AwsS3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nS3 Bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloudfront_domain` after provisioning.\nThe CloudFront distribution domain name pointing to this bucket, to use when fetching. See [Transfer from S3 via CloudFront](https://cloud.google.com/storage-transfer/docs/s3-cloudfront) for more information. Format: https://{id}.cloudfront.net or any valid custom domain. Must begin with https://."]
    pub fn cloudfront_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `credentials_secret` after provisioning.\nThe Resource name of a secret in Secret Manager. AWS credentials must be stored in Secret Manager in JSON format. If credentials_secret is specified, do not specify role_arn or aws_access_key. Format: projects/{projectNumber}/secrets/{secret_name}."]
    pub fn credentials_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_private_network` after provisioning.\nEgress bytes over a Google-managed private network. This network is shared between other users of Storage Transfer Service."]
    pub fn managed_private_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_private_network", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nS3 Bucket path in bucket to transfer."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the role to support temporary credentials via 'AssumeRoleWithWebIdentity'. For more information about ARNs, see [IAM ARNs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns). When a role ARN is provided, Transfer Service fetches temporary credentials for the session using a 'AssumeRoleWithWebIdentity' call for the provided role using the [GoogleServiceAccount][] for this project."]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_access_key` after provisioning.\n"]
    pub fn aws_access_key(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_AwsS3DataSource_AwsAccessKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_access_key", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials {
    sas_token: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials {
    #[doc = "Azure shared access signature."]
    pub sas_token: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials {
    pub fn build(self) -> StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials {
        StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials { sas_token: self.sas_token }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentialsRef {
        StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sas_token` after provisioning.\nAzure shared access signature."]
    pub fn sas_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sas_token", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig {
    client_id: terrars::PrimField<std::string::String>,
    tenant_id: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig {
    type O =
        terrars::BlockAssignable<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig {
    #[doc = "The client (application) ID of the application with federated credentials."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "The tenant (directory) ID of the application with federated credentials."]
    pub tenant_id: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig {
    pub fn build(self) -> StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig {
        StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig {
            client_id: self.client_id,
            tenant_id: self.tenant_id,
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfigRef {
        StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client (application) ID of the application with federated credentials."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nThe tenant (directory) ID of the application with federated credentials."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceDynamic {
    azure_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials>>,
    federated_identity_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig>,
    >,
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_AzureBlobStorageDataSource {
    container: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_network_service: std::option::Option<terrars::PrimField<std::string::String>>,
    storage_account: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_credentials: std
    ::option
    ::Option<std::vec::Vec<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    federated_identity_config: std
    ::option
    ::Option<std::vec::Vec<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig>>,
    dynamic: StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceDynamic,
}

impl StorageTransferJob_TransferSpec_AzureBlobStorageDataSource {
    #[doc =
        "Set the field `credentials_secret`.\nThe Resource name of a secret in Secret Manager containing SAS Credentials in JSON form. Service Agent must have permissions to access secret. If credentials_secret is specified, do not specify azure_credentials."]
    pub fn set_credentials_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.credentials_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nRoot path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_network_service`.\nService Directory Service used as the endpoint for transfers from a customer-managed VPC. Format: projects/{projectId}/locations/{location}/namespaces/{namespace}/services/{service}"]
    pub fn set_private_network_service(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_network_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_credentials`.\n"]
    pub fn set_azure_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `federated_identity_config`.\n"]
    pub fn set_federated_identity_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.federated_identity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.federated_identity_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_AzureBlobStorageDataSource {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_AzureBlobStorageDataSource {
    #[doc = "The container to transfer from the Azure Storage account."]
    pub container: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Azure Storage account."]
    pub storage_account: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_AzureBlobStorageDataSource {
    pub fn build(self) -> StorageTransferJob_TransferSpec_AzureBlobStorageDataSource {
        StorageTransferJob_TransferSpec_AzureBlobStorageDataSource {
            container: self.container,
            credentials_secret: core::default::Default::default(),
            path: core::default::Default::default(),
            private_network_service: core::default::Default::default(),
            storage_account: self.storage_account,
            azure_credentials: core::default::Default::default(),
            federated_identity_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceRef {
        StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container` after provisioning.\nThe container to transfer from the Azure Storage account."]
    pub fn container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc =
        "Get a reference to the value of field `credentials_secret` after provisioning.\nThe Resource name of a secret in Secret Manager containing SAS Credentials in JSON form. Service Agent must have permissions to access secret. If credentials_secret is specified, do not specify azure_credentials."]
    pub fn credentials_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRoot path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_network_service` after provisioning.\nService Directory Service used as the endpoint for transfers from a customer-managed VPC. Format: projects/{projectId}/locations/{location}/namespaces/{namespace}/services/{service}"]
    pub fn private_network_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_network_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_account` after provisioning.\nThe name of the Azure Storage account."]
    pub fn storage_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_account", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_credentials` after provisioning.\n"]
    pub fn azure_credentials(
        &self,
    ) -> terrars::ListRef<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_AzureCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `federated_identity_config` after provisioning.\n"]
    pub fn federated_identity_config(
        &self,
    ) -> terrars::ListRef<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource_FederatedIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.federated_identity_config", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_HdfsDataSource {
    path: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_HdfsDataSource { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_HdfsDataSource {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_HdfsDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_HdfsDataSource {
    #[doc = "Directory path to the filesystem."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_HdfsDataSource {
    pub fn build(self) -> StorageTransferJob_TransferSpec_HdfsDataSource {
        StorageTransferJob_TransferSpec_HdfsDataSource { path: self.path }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_HdfsDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_HdfsDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_HdfsDataSourceRef {
        StorageTransferJob_TransferSpec_HdfsDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_HdfsDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nDirectory path to the filesystem."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_HttpDataSource {
    list_url: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_HttpDataSource { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_HttpDataSource {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_HttpDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_HttpDataSource {
    #[doc =
        "The URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported."]
    pub list_url: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_HttpDataSource {
    pub fn build(self) -> StorageTransferJob_TransferSpec_HttpDataSource {
        StorageTransferJob_TransferSpec_HttpDataSource { list_url: self.list_url }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_HttpDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_HttpDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_HttpDataSourceRef {
        StorageTransferJob_TransferSpec_HttpDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_HttpDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `list_url` after provisioning.\nThe URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported."]
    pub fn list_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_url", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_PosixDataSink {
    root_directory: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_PosixDataSink { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_PosixDataSink {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_PosixDataSink>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_PosixDataSink {
    #[doc = "Root directory path to the filesystem."]
    pub root_directory: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_PosixDataSink {
    pub fn build(self) -> StorageTransferJob_TransferSpec_PosixDataSink {
        StorageTransferJob_TransferSpec_PosixDataSink { root_directory: self.root_directory }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_PosixDataSinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_PosixDataSinkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_PosixDataSinkRef {
        StorageTransferJob_TransferSpec_PosixDataSinkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_PosixDataSinkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `root_directory` after provisioning.\nRoot directory path to the filesystem."]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_PosixDataSource {
    root_directory: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_PosixDataSource { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_PosixDataSource {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_PosixDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_PosixDataSource {
    #[doc = "Root directory path to the filesystem."]
    pub root_directory: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_PosixDataSource {
    pub fn build(self) -> StorageTransferJob_TransferSpec_PosixDataSource {
        StorageTransferJob_TransferSpec_PosixDataSource { root_directory: self.root_directory }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_PosixDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_PosixDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_PosixDataSourceRef {
        StorageTransferJob_TransferSpec_PosixDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_PosixDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `root_directory` after provisioning.\nRoot directory path to the filesystem."]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec_TransferManifest {
    location: terrars::PrimField<std::string::String>,
}

impl StorageTransferJob_TransferSpec_TransferManifest { }

impl terrars::ToListMappable for StorageTransferJob_TransferSpec_TransferManifest {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec_TransferManifest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec_TransferManifest {
    #[doc = "Cloud Storage path to the manifest CSV."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildStorageTransferJob_TransferSpec_TransferManifest {
    pub fn build(self) -> StorageTransferJob_TransferSpec_TransferManifest {
        StorageTransferJob_TransferSpec_TransferManifest { location: self.location }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpec_TransferManifestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpec_TransferManifestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageTransferJob_TransferSpec_TransferManifestRef {
        StorageTransferJob_TransferSpec_TransferManifestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpec_TransferManifestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nCloud Storage path to the manifest CSV."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJob_TransferSpecDynamic {
    aws_s3_compatible_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource>>,
    aws_s3_data_source: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_AwsS3DataSource>>,
    azure_blob_storage_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource>>,
    gcs_data_sink: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_GcsDataSink>>,
    gcs_data_source: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_GcsDataSource>>,
    hdfs_data_source: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_HdfsDataSource>>,
    http_data_source: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_HttpDataSource>>,
    object_conditions: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_ObjectConditions>>,
    posix_data_sink: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_PosixDataSink>>,
    posix_data_source: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_PosixDataSource>>,
    transfer_manifest: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_TransferManifest>>,
    transfer_options: std::option::Option<terrars::DynamicBlock<StorageTransferJob_TransferSpec_TransferOptions>>,
}

#[derive(Serialize)]
pub struct StorageTransferJob_TransferSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sink_agent_pool_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_agent_pool_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_s3_compatible_data_source: std
    ::option
    ::Option<std::vec::Vec<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_s3_data_source: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_AwsS3DataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_blob_storage_data_source: std
    ::option
    ::Option<std::vec::Vec<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_data_sink: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_GcsDataSink>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_data_source: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_GcsDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hdfs_data_source: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_HdfsDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_data_source: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_HttpDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_conditions: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_ObjectConditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_data_sink: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_PosixDataSink>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_data_source: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_PosixDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_manifest: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_TransferManifest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_options: std::option::Option<std::vec::Vec<StorageTransferJob_TransferSpec_TransferOptions>>,
    dynamic: StorageTransferJob_TransferSpecDynamic,
}

impl StorageTransferJob_TransferSpec {
    #[doc =
        "Set the field `sink_agent_pool_name`.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn set_sink_agent_pool_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sink_agent_pool_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_agent_pool_name`.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn set_source_agent_pool_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_agent_pool_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_s3_compatible_data_source`.\n"]
    pub fn set_aws_s3_compatible_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_s3_compatible_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_s3_compatible_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_s3_data_source`.\n"]
    pub fn set_aws_s3_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_AwsS3DataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_blob_storage_data_source`.\n"]
    pub fn set_azure_blob_storage_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_AzureBlobStorageDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_blob_storage_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_blob_storage_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_data_sink`.\n"]
    pub fn set_gcs_data_sink(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_GcsDataSink>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_data_sink = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_sink = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_data_source`.\n"]
    pub fn set_gcs_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_GcsDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hdfs_data_source`.\n"]
    pub fn set_hdfs_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_HdfsDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hdfs_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hdfs_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_data_source`.\n"]
    pub fn set_http_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_HttpDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `object_conditions`.\n"]
    pub fn set_object_conditions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_ObjectConditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.object_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.object_conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `posix_data_sink`.\n"]
    pub fn set_posix_data_sink(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_PosixDataSink>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.posix_data_sink = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_data_sink = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `posix_data_source`.\n"]
    pub fn set_posix_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_PosixDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.posix_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transfer_manifest`.\n"]
    pub fn set_transfer_manifest(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_TransferManifest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transfer_manifest = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_manifest = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transfer_options`.\n"]
    pub fn set_transfer_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageTransferJob_TransferSpec_TransferOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transfer_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageTransferJob_TransferSpec {
    type O = terrars::BlockAssignable<StorageTransferJob_TransferSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJob_TransferSpec {}

impl BuildStorageTransferJob_TransferSpec {
    pub fn build(self) -> StorageTransferJob_TransferSpec {
        StorageTransferJob_TransferSpec {
            sink_agent_pool_name: core::default::Default::default(),
            source_agent_pool_name: core::default::Default::default(),
            aws_s3_compatible_data_source: core::default::Default::default(),
            aws_s3_data_source: core::default::Default::default(),
            azure_blob_storage_data_source: core::default::Default::default(),
            gcs_data_sink: core::default::Default::default(),
            gcs_data_source: core::default::Default::default(),
            hdfs_data_source: core::default::Default::default(),
            http_data_source: core::default::Default::default(),
            object_conditions: core::default::Default::default(),
            posix_data_sink: core::default::Default::default(),
            posix_data_source: core::default::Default::default(),
            transfer_manifest: core::default::Default::default(),
            transfer_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageTransferJob_TransferSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageTransferJob_TransferSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageTransferJob_TransferSpecRef {
        StorageTransferJob_TransferSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJob_TransferSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sink_agent_pool_name` after provisioning.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn sink_agent_pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sink_agent_pool_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_agent_pool_name` after provisioning.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn source_agent_pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_agent_pool_name", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_s3_compatible_data_source` after provisioning.\n"]
    pub fn aws_s3_compatible_data_source(
        &self,
    ) -> terrars::ListRef<StorageTransferJob_TransferSpec_AwsS3CompatibleDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_s3_compatible_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_s3_data_source` after provisioning.\n"]
    pub fn aws_s3_data_source(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_AwsS3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_s3_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_blob_storage_data_source` after provisioning.\n"]
    pub fn azure_blob_storage_data_source(
        &self,
    ) -> terrars::ListRef<StorageTransferJob_TransferSpec_AzureBlobStorageDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_blob_storage_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_data_sink` after provisioning.\n"]
    pub fn gcs_data_sink(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_GcsDataSinkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_data_sink", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_data_source` after provisioning.\n"]
    pub fn gcs_data_source(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_GcsDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `hdfs_data_source` after provisioning.\n"]
    pub fn hdfs_data_source(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_HdfsDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hdfs_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `http_data_source` after provisioning.\n"]
    pub fn http_data_source(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_HttpDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `object_conditions` after provisioning.\n"]
    pub fn object_conditions(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_ObjectConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `posix_data_sink` after provisioning.\n"]
    pub fn posix_data_sink(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_PosixDataSinkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_data_sink", self.base))
    }

    #[doc = "Get a reference to the value of field `posix_data_source` after provisioning.\n"]
    pub fn posix_data_source(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_PosixDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_manifest` after provisioning.\n"]
    pub fn transfer_manifest(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_TransferManifestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_manifest", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_options` after provisioning.\n"]
    pub fn transfer_options(&self) -> terrars::ListRef<StorageTransferJob_TransferSpec_TransferOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_options", self.base))
    }
}
