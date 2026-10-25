#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageBatchOperationsJobData_ {
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
    delete_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_list: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_BucketList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_object: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_DeleteObject>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    put_metadata: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_PutMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    put_object_hold: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_PutObjectHold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite_object: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_RewriteObject>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageBatchOperationsJob_Timeouts>,
    dynamic: StorageBatchOperationsJobDynamic,
}

struct StorageBatchOperationsJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageBatchOperationsJobData_>,
}

#[derive(Clone)]
pub struct StorageBatchOperationsJob(std::rc::Rc<StorageBatchOperationsJob_>);

impl StorageBatchOperationsJob {
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
        "Set the field `delete_protection`.\nIf set to 'true', the storage batch operation job will not be deleted and new job will be created."]
    pub fn set_delete_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nA description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_id`.\nThe ID of the job."]
    pub fn set_job_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().job_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_list`.\n"]
    pub fn set_bucket_list(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBatchOperationsJob_BucketList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bucket_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bucket_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delete_object`.\n"]
    pub fn set_delete_object(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBatchOperationsJob_DeleteObject>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delete_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delete_object = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `put_metadata`.\n"]
    pub fn set_put_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBatchOperationsJob_PutMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().put_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.put_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `put_object_hold`.\n"]
    pub fn set_put_object_hold(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBatchOperationsJob_PutObjectHold>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().put_object_hold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.put_object_hold = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rewrite_object`.\n"]
    pub fn set_rewrite_object(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBatchOperationsJob_RewriteObject>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rewrite_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rewrite_object = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageBatchOperationsJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `complete_time` after provisioning.\nThe time that the job was completed."]
    pub fn complete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which this storage batch operation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_protection` after provisioning.\nIf set to 'true', the storage batch operation job will not be deleted and new job will be created."]
    pub fn delete_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job."]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule_time` after provisioning.\nThe time that the job was scheduled."]
    pub fn schedule_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the job."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which this storage batch operation was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_list` after provisioning.\n"]
    pub fn bucket_list(&self) -> terrars::ListRef<StorageBatchOperationsJob_BucketListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_object` after provisioning.\n"]
    pub fn delete_object(&self) -> terrars::ListRef<StorageBatchOperationsJob_DeleteObjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `put_metadata` after provisioning.\n"]
    pub fn put_metadata(&self) -> terrars::ListRef<StorageBatchOperationsJob_PutMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.put_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `put_object_hold` after provisioning.\n"]
    pub fn put_object_hold(&self) -> terrars::ListRef<StorageBatchOperationsJob_PutObjectHoldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.put_object_hold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rewrite_object` after provisioning.\n"]
    pub fn rewrite_object(&self) -> terrars::ListRef<StorageBatchOperationsJob_RewriteObjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBatchOperationsJob_TimeoutsRef {
        StorageBatchOperationsJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for StorageBatchOperationsJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageBatchOperationsJob { }

impl terrars::ToListMappable for StorageBatchOperationsJob {
    type O = terrars::ListRef<StorageBatchOperationsJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageBatchOperationsJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_batch_operations_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageBatchOperationsJob {
    pub tf_id: std::string::String,
}

impl BuildStorageBatchOperationsJob {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageBatchOperationsJob {
        let out = StorageBatchOperationsJob(std::rc::Rc::new(StorageBatchOperationsJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageBatchOperationsJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delete_protection: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                job_id: core::default::Default::default(),
                project: core::default::Default::default(),
                bucket_list: core::default::Default::default(),
                delete_object: core::default::Default::default(),
                put_metadata: core::default::Default::default(),
                put_object_hold: core::default::Default::default(),
                rewrite_object: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageBatchOperationsJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageBatchOperationsJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `complete_time` after provisioning.\nThe time that the job was completed."]
    pub fn complete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which this storage batch operation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_protection` after provisioning.\nIf set to 'true', the storage batch operation job will not be deleted and new job will be created."]
    pub fn delete_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job."]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule_time` after provisioning.\nThe time that the job was scheduled."]
    pub fn schedule_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the job."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which this storage batch operation was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_list` after provisioning.\n"]
    pub fn bucket_list(&self) -> terrars::ListRef<StorageBatchOperationsJob_BucketListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_object` after provisioning.\n"]
    pub fn delete_object(&self) -> terrars::ListRef<StorageBatchOperationsJob_DeleteObjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `put_metadata` after provisioning.\n"]
    pub fn put_metadata(&self) -> terrars::ListRef<StorageBatchOperationsJob_PutMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.put_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `put_object_hold` after provisioning.\n"]
    pub fn put_object_hold(&self) -> terrars::ListRef<StorageBatchOperationsJob_PutObjectHoldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.put_object_hold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rewrite_object` after provisioning.\n"]
    pub fn rewrite_object(&self) -> terrars::ListRef<StorageBatchOperationsJob_RewriteObjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBatchOperationsJob_TimeoutsRef {
        StorageBatchOperationsJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct StorageBatchOperationsJobDynamic {
    bucket_list: std::option::Option<terrars::DynamicBlock<StorageBatchOperationsJob_BucketList>>,
    delete_object: std::option::Option<terrars::DynamicBlock<StorageBatchOperationsJob_DeleteObject>>,
    put_metadata: std::option::Option<terrars::DynamicBlock<StorageBatchOperationsJob_PutMetadata>>,
    put_object_hold: std::option::Option<terrars::DynamicBlock<StorageBatchOperationsJob_PutObjectHold>>,
    rewrite_object: std::option::Option<terrars::DynamicBlock<StorageBatchOperationsJob_RewriteObject>>,
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_BucketList_Buckets_Manifest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBatchOperationsJob_BucketList_Buckets_Manifest {
    #[doc = "Set the field `manifest_location`.\nSpecifies objects in a manifest file."]
    pub fn set_manifest_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manifest_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBatchOperationsJob_BucketList_Buckets_Manifest {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_BucketList_Buckets_Manifest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_BucketList_Buckets_Manifest {}

impl BuildStorageBatchOperationsJob_BucketList_Buckets_Manifest {
    pub fn build(self) -> StorageBatchOperationsJob_BucketList_Buckets_Manifest {
        StorageBatchOperationsJob_BucketList_Buckets_Manifest {
            manifest_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_BucketList_Buckets_ManifestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_BucketList_Buckets_ManifestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageBatchOperationsJob_BucketList_Buckets_ManifestRef {
        StorageBatchOperationsJob_BucketList_Buckets_ManifestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_BucketList_Buckets_ManifestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `manifest_location` after provisioning.\nSpecifies objects in a manifest file."]
    pub fn manifest_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest_location", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_BucketList_Buckets_PrefixList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_object_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl StorageBatchOperationsJob_BucketList_Buckets_PrefixList {
    #[doc = "Set the field `included_object_prefixes`.\n"]
    pub fn set_included_object_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_object_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBatchOperationsJob_BucketList_Buckets_PrefixList {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_BucketList_Buckets_PrefixList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_BucketList_Buckets_PrefixList {}

impl BuildStorageBatchOperationsJob_BucketList_Buckets_PrefixList {
    pub fn build(self) -> StorageBatchOperationsJob_BucketList_Buckets_PrefixList {
        StorageBatchOperationsJob_BucketList_Buckets_PrefixList {
            included_object_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_BucketList_Buckets_PrefixListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_BucketList_Buckets_PrefixListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageBatchOperationsJob_BucketList_Buckets_PrefixListRef {
        StorageBatchOperationsJob_BucketList_Buckets_PrefixListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_BucketList_Buckets_PrefixListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `included_object_prefixes` after provisioning.\n"]
    pub fn included_object_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_object_prefixes", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageBatchOperationsJob_BucketList_BucketsDynamic {
    manifest: std::option::Option<terrars::DynamicBlock<StorageBatchOperationsJob_BucketList_Buckets_Manifest>>,
    prefix_list: std
    ::option
    ::Option<terrars::DynamicBlock<StorageBatchOperationsJob_BucketList_Buckets_PrefixList>>,
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_BucketList_Buckets {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_BucketList_Buckets_Manifest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_BucketList_Buckets_PrefixList>>,
    dynamic: StorageBatchOperationsJob_BucketList_BucketsDynamic,
}

impl StorageBatchOperationsJob_BucketList_Buckets {
    #[doc = "Set the field `manifest`.\n"]
    pub fn set_manifest(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBatchOperationsJob_BucketList_Buckets_Manifest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.manifest = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.manifest = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_list`.\n"]
    pub fn set_prefix_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageBatchOperationsJob_BucketList_Buckets_PrefixList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageBatchOperationsJob_BucketList_Buckets {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_BucketList_Buckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_BucketList_Buckets {
    #[doc = "Bucket name for the objects to be transformed."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildStorageBatchOperationsJob_BucketList_Buckets {
    pub fn build(self) -> StorageBatchOperationsJob_BucketList_Buckets {
        StorageBatchOperationsJob_BucketList_Buckets {
            bucket: self.bucket,
            manifest: core::default::Default::default(),
            prefix_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_BucketList_BucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_BucketList_BucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageBatchOperationsJob_BucketList_BucketsRef {
        StorageBatchOperationsJob_BucketList_BucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_BucketList_BucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nBucket name for the objects to be transformed."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `manifest` after provisioning.\n"]
    pub fn manifest(&self) -> terrars::ListRef<StorageBatchOperationsJob_BucketList_Buckets_ManifestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manifest", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list` after provisioning.\n"]
    pub fn prefix_list(&self) -> terrars::ListRef<StorageBatchOperationsJob_BucketList_Buckets_PrefixListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageBatchOperationsJob_BucketListDynamic {
    buckets: std::option::Option<terrars::DynamicBlock<StorageBatchOperationsJob_BucketList_Buckets>>,
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_BucketList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buckets: std::option::Option<std::vec::Vec<StorageBatchOperationsJob_BucketList_Buckets>>,
    dynamic: StorageBatchOperationsJob_BucketListDynamic,
}

impl StorageBatchOperationsJob_BucketList {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBatchOperationsJob_BucketList_Buckets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.buckets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageBatchOperationsJob_BucketList {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_BucketList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_BucketList {}

impl BuildStorageBatchOperationsJob_BucketList {
    pub fn build(self) -> StorageBatchOperationsJob_BucketList {
        StorageBatchOperationsJob_BucketList {
            buckets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_BucketListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_BucketListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBatchOperationsJob_BucketListRef {
        StorageBatchOperationsJob_BucketListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_BucketListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> terrars::ListRef<StorageBatchOperationsJob_BucketList_BucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_DeleteObject {
    permanent_object_deletion_enabled: terrars::PrimField<bool>,
}

impl StorageBatchOperationsJob_DeleteObject { }

impl terrars::ToListMappable for StorageBatchOperationsJob_DeleteObject {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_DeleteObject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_DeleteObject {
    #[doc = "enable flag to permanently delete object and all object versions if versioning is enabled on bucket."]
    pub permanent_object_deletion_enabled: terrars::PrimField<bool>,
}

impl BuildStorageBatchOperationsJob_DeleteObject {
    pub fn build(self) -> StorageBatchOperationsJob_DeleteObject {
        StorageBatchOperationsJob_DeleteObject {
            permanent_object_deletion_enabled: self.permanent_object_deletion_enabled,
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_DeleteObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_DeleteObjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBatchOperationsJob_DeleteObjectRef {
        StorageBatchOperationsJob_DeleteObjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_DeleteObjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `permanent_object_deletion_enabled` after provisioning.\nenable flag to permanently delete object and all object versions if versioning is enabled on bucket."]
    pub fn permanent_object_deletion_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permanent_object_deletion_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_PutMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBatchOperationsJob_PutMetadata {
    #[doc =
        "Set the field `cache_control`.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn set_cache_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_disposition`.\nContent-Disposition of the object data."]
    pub fn set_content_disposition(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_encoding`.\nContent Encoding of the object data."]
    pub fn set_content_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_language`.\nContent-Language of the object data."]
    pub fn set_content_language(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\nContent-Type of the object data."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_metadata`.\nUser-provided metadata, in key/value pairs."]
    pub fn set_custom_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_time`.\nUpdates the objects fixed custom time metadata."]
    pub fn set_custom_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBatchOperationsJob_PutMetadata {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_PutMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_PutMetadata {}

impl BuildStorageBatchOperationsJob_PutMetadata {
    pub fn build(self) -> StorageBatchOperationsJob_PutMetadata {
        StorageBatchOperationsJob_PutMetadata {
            cache_control: core::default::Default::default(),
            content_disposition: core::default::Default::default(),
            content_encoding: core::default::Default::default(),
            content_language: core::default::Default::default(),
            content_type: core::default::Default::default(),
            custom_metadata: core::default::Default::default(),
            custom_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_PutMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_PutMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBatchOperationsJob_PutMetadataRef {
        StorageBatchOperationsJob_PutMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_PutMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_disposition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_encoding` after provisioning.\nContent Encoding of the object data."]
    pub fn content_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_language", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn custom_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_time` after provisioning.\nUpdates the objects fixed custom time metadata."]
    pub fn custom_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_time", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_PutObjectHold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_based_hold: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporary_hold: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBatchOperationsJob_PutObjectHold {
    #[doc = "Set the field `event_based_hold`.\nset/unset to update event based hold for objects."]
    pub fn set_event_based_hold(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_based_hold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temporary_hold`.\nset/unset to update temporary based hold for objects."]
    pub fn set_temporary_hold(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.temporary_hold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBatchOperationsJob_PutObjectHold {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_PutObjectHold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_PutObjectHold {}

impl BuildStorageBatchOperationsJob_PutObjectHold {
    pub fn build(self) -> StorageBatchOperationsJob_PutObjectHold {
        StorageBatchOperationsJob_PutObjectHold {
            event_based_hold: core::default::Default::default(),
            temporary_hold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_PutObjectHoldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_PutObjectHoldRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBatchOperationsJob_PutObjectHoldRef {
        StorageBatchOperationsJob_PutObjectHoldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_PutObjectHoldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_based_hold` after provisioning.\nset/unset to update event based hold for objects."]
    pub fn event_based_hold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_based_hold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temporary_hold` after provisioning.\nset/unset to update temporary based hold for objects."]
    pub fn temporary_hold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporary_hold", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_RewriteObject {
    kms_key: terrars::PrimField<std::string::String>,
}

impl StorageBatchOperationsJob_RewriteObject { }

impl terrars::ToListMappable for StorageBatchOperationsJob_RewriteObject {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_RewriteObject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_RewriteObject {
    #[doc = "valid kms key"]
    pub kms_key: terrars::PrimField<std::string::String>,
}

impl BuildStorageBatchOperationsJob_RewriteObject {
    pub fn build(self) -> StorageBatchOperationsJob_RewriteObject {
        StorageBatchOperationsJob_RewriteObject { kms_key: self.kms_key }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_RewriteObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_RewriteObjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBatchOperationsJob_RewriteObjectRef {
        StorageBatchOperationsJob_RewriteObjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_RewriteObjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nvalid kms key"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBatchOperationsJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBatchOperationsJob_Timeouts {
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

impl terrars::ToListMappable for StorageBatchOperationsJob_Timeouts {
    type O = terrars::BlockAssignable<StorageBatchOperationsJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBatchOperationsJob_Timeouts {}

impl BuildStorageBatchOperationsJob_Timeouts {
    pub fn build(self) -> StorageBatchOperationsJob_Timeouts {
        StorageBatchOperationsJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBatchOperationsJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBatchOperationsJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBatchOperationsJob_TimeoutsRef {
        StorageBatchOperationsJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBatchOperationsJob_TimeoutsRef {
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
