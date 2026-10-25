#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeStoragePoolData_ {
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
    capacity_provisioning_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_provisioning_type: std::option::Option<terrars::PrimField<std::string::String>>,
    pool_provisioned_capacity_gb: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_provisioned_iops: std::option::Option<terrars::PrimField<std::string::String>>,
    pool_provisioned_throughput: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    storage_pool_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeStoragePool_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeStoragePool_Timeouts>,
    dynamic: ComputeStoragePoolDynamic,
}

struct ComputeStoragePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeStoragePoolData_>,
}

#[derive(Clone)]
pub struct ComputeStoragePool(std::rc::Rc<ComputeStoragePool_>);

impl ComputeStoragePool {
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
        "Set the field `capacity_provisioning_type`.\nProvisioning type of the byte capacity of the pool. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn set_capacity_provisioning_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().capacity_provisioning_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the StoragePool.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the StoragePool will fail.\nWhen the field is set to false, deleting the StoragePool is allowed."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nA description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels to apply to this storage pool. These can be later modified by the setLabels method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `performance_provisioning_type`.\nProvisioning type of the performance-related parameters of the pool, such as throughput and IOPS. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn set_performance_provisioning_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().performance_provisioning_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pool_provisioned_iops`.\nProvisioned IOPS of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced'."]
    pub fn set_pool_provisioned_iops(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pool_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nA reference to the zone where the storage pool resides."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeStoragePool_Params>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeStoragePool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `capacity_provisioning_type` after provisioning.\nProvisioning type of the byte capacity of the pool. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn capacity_provisioning_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provisioning_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the StoragePool.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the StoragePool will fail.\nWhen the field is set to false, deleting the StoragePool is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.\nUsed internally during updates."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this storage pool. These can be later modified by the setLabels method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter,\nand all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `performance_provisioning_type` after provisioning.\nProvisioning type of the performance-related parameters of the pool, such as throughput and IOPS. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn performance_provisioning_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_provisioning_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pool_provisioned_capacity_gb` after provisioning.\nSize, in GiB, of the storage pool. For more information about the size limits,\nsee https://cloud.google.com/compute/docs/disks/storage-pools."]
    pub fn pool_provisioned_capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_provisioned_capacity_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool_provisioned_iops` after provisioning.\nProvisioned IOPS of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced'."]
    pub fn pool_provisioned_iops(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_provisioned_iops", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool_provisioned_throughput` after provisioning.\nProvisioned throughput, in MB/s, of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced' or 'hyperdisk-throughput'."]
    pub fn pool_provisioned_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_provisioned_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn resource_status(&self) -> terrars::ListRef<ComputeStoragePool_ResourceStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn status(&self) -> terrars::ListRef<ComputeStoragePool_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool_type` after provisioning.\nType of the storage pool. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/{project_id}/zones/{zone}/storagePoolTypes/hyperdisk-balanced'\n* 'hyperdisk-throughput'"]
    pub fn storage_pool_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the storage pool resides."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeStoragePool_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeStoragePool_TimeoutsRef {
        ComputeStoragePool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeStoragePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeStoragePool { }

impl terrars::ToListMappable for ComputeStoragePool {
    type O = terrars::ListRef<ComputeStoragePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeStoragePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_storage_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeStoragePool {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter,\nand all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Size, in GiB, of the storage pool. For more information about the size limits,\nsee https://cloud.google.com/compute/docs/disks/storage-pools."]
    pub pool_provisioned_capacity_gb: terrars::PrimField<std::string::String>,
    #[doc =
        "Provisioned throughput, in MB/s, of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced' or 'hyperdisk-throughput'."]
    pub pool_provisioned_throughput: terrars::PrimField<std::string::String>,
    #[doc =
        "Type of the storage pool. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/{project_id}/zones/{zone}/storagePoolTypes/hyperdisk-balanced'\n* 'hyperdisk-throughput'"]
    pub storage_pool_type: terrars::PrimField<std::string::String>,
}

impl BuildComputeStoragePool {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeStoragePool {
        let out = ComputeStoragePool(std::rc::Rc::new(ComputeStoragePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeStoragePoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_provisioning_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                performance_provisioning_type: core::default::Default::default(),
                pool_provisioned_capacity_gb: self.pool_provisioned_capacity_gb,
                pool_provisioned_iops: core::default::Default::default(),
                pool_provisioned_throughput: self.pool_provisioned_throughput,
                project: core::default::Default::default(),
                storage_pool_type: self.storage_pool_type,
                zone: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeStoragePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeStoragePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeStoragePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `capacity_provisioning_type` after provisioning.\nProvisioning type of the byte capacity of the pool. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn capacity_provisioning_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provisioning_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the StoragePool.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the StoragePool will fail.\nWhen the field is set to false, deleting the StoragePool is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.\nUsed internally during updates."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this storage pool. These can be later modified by the setLabels method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter,\nand all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `performance_provisioning_type` after provisioning.\nProvisioning type of the performance-related parameters of the pool, such as throughput and IOPS. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn performance_provisioning_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_provisioning_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pool_provisioned_capacity_gb` after provisioning.\nSize, in GiB, of the storage pool. For more information about the size limits,\nsee https://cloud.google.com/compute/docs/disks/storage-pools."]
    pub fn pool_provisioned_capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_provisioned_capacity_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool_provisioned_iops` after provisioning.\nProvisioned IOPS of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced'."]
    pub fn pool_provisioned_iops(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_provisioned_iops", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool_provisioned_throughput` after provisioning.\nProvisioned throughput, in MB/s, of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced' or 'hyperdisk-throughput'."]
    pub fn pool_provisioned_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_provisioned_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn resource_status(&self) -> terrars::ListRef<ComputeStoragePool_ResourceStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn status(&self) -> terrars::ListRef<ComputeStoragePool_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool_type` after provisioning.\nType of the storage pool. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/{project_id}/zones/{zone}/storagePoolTypes/hyperdisk-balanced'\n* 'hyperdisk-throughput'"]
    pub fn storage_pool_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the storage pool resides."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeStoragePool_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeStoragePool_TimeoutsRef {
        ComputeStoragePool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeStoragePoolDynamic {
    params: std::option::Option<terrars::DynamicBlock<ComputeStoragePool_Params>>,
}

#[derive(Serialize)]
pub struct ComputeStoragePool_ResourceStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_resize_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_total_provisioned_disk_capacity_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_used_capacity_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_used_iops: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_used_throughput: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_user_written_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_provisioned_disk_capacity_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_provisioned_disk_iops: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_provisioned_disk_throughput: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeStoragePool_ResourceStatus {
    #[doc = "Set the field `disk_count`.\n"]
    pub fn set_disk_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_resize_timestamp`.\n"]
    pub fn set_last_resize_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_resize_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_max_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_total_provisioned_disk_capacity_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_used_capacity_bytes`.\n"]
    pub fn set_pool_used_capacity_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pool_used_capacity_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_used_iops`.\n"]
    pub fn set_pool_used_iops(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pool_used_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_used_throughput`.\n"]
    pub fn set_pool_used_throughput(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pool_used_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_user_written_bytes`.\n"]
    pub fn set_pool_user_written_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pool_user_written_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_provisioned_disk_capacity_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_provisioned_disk_iops`.\n"]
    pub fn set_total_provisioned_disk_iops(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_provisioned_disk_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_provisioned_disk_throughput`.\n"]
    pub fn set_total_provisioned_disk_throughput(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_provisioned_disk_throughput = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeStoragePool_ResourceStatus {
    type O = terrars::BlockAssignable<ComputeStoragePool_ResourceStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeStoragePool_ResourceStatus {}

impl BuildComputeStoragePool_ResourceStatus {
    pub fn build(self) -> ComputeStoragePool_ResourceStatus {
        ComputeStoragePool_ResourceStatus {
            disk_count: core::default::Default::default(),
            last_resize_timestamp: core::default::Default::default(),
            max_total_provisioned_disk_capacity_gb: core::default::Default::default(),
            pool_used_capacity_bytes: core::default::Default::default(),
            pool_used_iops: core::default::Default::default(),
            pool_used_throughput: core::default::Default::default(),
            pool_user_written_bytes: core::default::Default::default(),
            total_provisioned_disk_capacity_gb: core::default::Default::default(),
            total_provisioned_disk_iops: core::default::Default::default(),
            total_provisioned_disk_throughput: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeStoragePool_ResourceStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeStoragePool_ResourceStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeStoragePool_ResourceStatusRef {
        ComputeStoragePool_ResourceStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeStoragePool_ResourceStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\n"]
    pub fn disk_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }

    #[doc = "Get a reference to the value of field `last_resize_timestamp` after provisioning.\n"]
    pub fn last_resize_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_resize_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `max_total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn max_total_provisioned_disk_capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_total_provisioned_disk_capacity_gb", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pool_used_capacity_bytes` after provisioning.\n"]
    pub fn pool_used_capacity_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_used_capacity_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_used_iops` after provisioning.\n"]
    pub fn pool_used_iops(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_used_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_used_throughput` after provisioning.\n"]
    pub fn pool_used_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_used_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_user_written_bytes` after provisioning.\n"]
    pub fn pool_user_written_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_user_written_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn total_provisioned_disk_capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_provisioned_disk_capacity_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `total_provisioned_disk_iops` after provisioning.\n"]
    pub fn total_provisioned_disk_iops(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_provisioned_disk_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `total_provisioned_disk_throughput` after provisioning.\n"]
    pub fn total_provisioned_disk_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_provisioned_disk_throughput", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeStoragePool_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_resize_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_total_provisioned_disk_capacity_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_used_capacity_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_used_iops: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_used_throughput: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_user_written_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_provisioned_disk_capacity_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_provisioned_disk_iops: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_provisioned_disk_throughput: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeStoragePool_Status {
    #[doc = "Set the field `disk_count`.\n"]
    pub fn set_disk_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_resize_timestamp`.\n"]
    pub fn set_last_resize_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_resize_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_max_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_total_provisioned_disk_capacity_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_used_capacity_bytes`.\n"]
    pub fn set_pool_used_capacity_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pool_used_capacity_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_used_iops`.\n"]
    pub fn set_pool_used_iops(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pool_used_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_used_throughput`.\n"]
    pub fn set_pool_used_throughput(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pool_used_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_user_written_bytes`.\n"]
    pub fn set_pool_user_written_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pool_user_written_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_provisioned_disk_capacity_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_provisioned_disk_iops`.\n"]
    pub fn set_total_provisioned_disk_iops(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_provisioned_disk_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_provisioned_disk_throughput`.\n"]
    pub fn set_total_provisioned_disk_throughput(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_provisioned_disk_throughput = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeStoragePool_Status {
    type O = terrars::BlockAssignable<ComputeStoragePool_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeStoragePool_Status {}

impl BuildComputeStoragePool_Status {
    pub fn build(self) -> ComputeStoragePool_Status {
        ComputeStoragePool_Status {
            disk_count: core::default::Default::default(),
            last_resize_timestamp: core::default::Default::default(),
            max_total_provisioned_disk_capacity_gb: core::default::Default::default(),
            pool_used_capacity_bytes: core::default::Default::default(),
            pool_used_iops: core::default::Default::default(),
            pool_used_throughput: core::default::Default::default(),
            pool_user_written_bytes: core::default::Default::default(),
            total_provisioned_disk_capacity_gb: core::default::Default::default(),
            total_provisioned_disk_iops: core::default::Default::default(),
            total_provisioned_disk_throughput: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeStoragePool_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeStoragePool_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeStoragePool_StatusRef {
        ComputeStoragePool_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeStoragePool_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\n"]
    pub fn disk_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }

    #[doc = "Get a reference to the value of field `last_resize_timestamp` after provisioning.\n"]
    pub fn last_resize_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_resize_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `max_total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn max_total_provisioned_disk_capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_total_provisioned_disk_capacity_gb", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pool_used_capacity_bytes` after provisioning.\n"]
    pub fn pool_used_capacity_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_used_capacity_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_used_iops` after provisioning.\n"]
    pub fn pool_used_iops(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_used_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_used_throughput` after provisioning.\n"]
    pub fn pool_used_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_used_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_user_written_bytes` after provisioning.\n"]
    pub fn pool_user_written_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_user_written_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn total_provisioned_disk_capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_provisioned_disk_capacity_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `total_provisioned_disk_iops` after provisioning.\n"]
    pub fn total_provisioned_disk_iops(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_provisioned_disk_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `total_provisioned_disk_throughput` after provisioning.\n"]
    pub fn total_provisioned_disk_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_provisioned_disk_throughput", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeStoragePool_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeStoragePool_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the storage pool. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeStoragePool_Params {
    type O = terrars::BlockAssignable<ComputeStoragePool_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeStoragePool_Params {}

impl BuildComputeStoragePool_Params {
    pub fn build(self) -> ComputeStoragePool_Params {
        ComputeStoragePool_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeStoragePool_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeStoragePool_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeStoragePool_ParamsRef {
        ComputeStoragePool_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeStoragePool_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the storage pool. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeStoragePool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeStoragePool_Timeouts {
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

impl terrars::ToListMappable for ComputeStoragePool_Timeouts {
    type O = terrars::BlockAssignable<ComputeStoragePool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeStoragePool_Timeouts {}

impl BuildComputeStoragePool_Timeouts {
    pub fn build(self) -> ComputeStoragePool_Timeouts {
        ComputeStoragePool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeStoragePool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeStoragePool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeStoragePool_TimeoutsRef {
        ComputeStoragePool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeStoragePool_TimeoutsRef {
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
