#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FilestoreInstanceData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_replica_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_services: std::option::Option<std::vec::Vec<FilestoreInstance_DirectoryServices>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_shares: std::option::Option<std::vec::Vec<FilestoreInstance_FileShares>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_replication: std::option::Option<std::vec::Vec<FilestoreInstance_InitialReplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<FilestoreInstance_Networks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_config: std::option::Option<std::vec::Vec<FilestoreInstance_PerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FilestoreInstance_Timeouts>,
    dynamic: FilestoreInstanceDynamic,
}

struct FilestoreInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FilestoreInstanceData_>,
}

#[derive(Clone)]
pub struct FilestoreInstance(std::rc::Rc<FilestoreInstance_>);

impl FilestoreInstance {
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

    #[doc =
        "Set the field `deletion_protection_enabled`.\nIndicates whether the instance is protected against deletion."]
    pub fn set_deletion_protection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection_reason`.\nThe reason for enabling deletion protection."]
    pub fn set_deletion_protection_reason(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().deletion_protection_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the instance."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `desired_replica_state`.\nThe desired_replica_state field controls the state of a replica. Terraform will attempt to make the actual state of the replica match the desired state. Default value: \"READY\" Possible values: [\"PAUSED\", \"READY\"]"]
    pub fn set_desired_replica_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_replica_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_name`.\nKMS key name used for data encryption."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protocol`.\nEither NFSv3, for using NFS version 3 as file sharing protocol,\nor NFSv4.1, for using NFS version 4.1 as file sharing protocol.\nNFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE.\nThe default is NFSv3. Default value: \"NFS_V3\" Possible values: [\"NFS_V3\", \"NFS_V4_1\"]"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nA map of resource manager tags. Resource manager tag keys\nand values have the same definition as resource manager\ntags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is\nignored when empty. The field is immutable and causes\nresource replacement when mutated. This field is only set\nat create time and modifying this field after creation\nwill trigger recreation. To apply tags to an existing\nresource, see the 'google_tags_tag_value' resource."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe name of the Filestore zone of the instance."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `directory_services`.\n"]
    pub fn set_directory_services(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_DirectoryServices>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().directory_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.directory_services = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_shares`.\n"]
    pub fn set_file_shares(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_FileShares>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_shares = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_shares = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_replication`.\n"]
    pub fn set_initial_replication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_InitialReplication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_replication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_Networks>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `performance_config`.\n"]
    pub fn set_performance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_PerformanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().performance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.performance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FilestoreInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nIndicates whether the instance is protected against deletion."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_reason` after provisioning.\nThe reason for enabling deletion protection."]
    pub fn deletion_protection_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_replica_state` after provisioning.\nThe desired_replica_state field controls the state of a replica. Terraform will attempt to make the actual state of the replica match the desired state. Default value: \"READY\" Possible values: [\"PAUSED\", \"READY\"]"]
    pub fn desired_replica_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_replica_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_replication` after provisioning.\nOutput only fields for replication configuration."]
    pub fn effective_replication(&self) -> terrars::ListRef<FilestoreInstance_EffectiveReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key name used for data encryption."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nEither NFSv3, for using NFS version 3 as file sharing protocol,\nor NFSv4.1, for using NFS version 4.1 as file sharing protocol.\nNFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE.\nThe default is NFSv3. Default value: \"NFS_V3\" Possible values: [\"NFS_V3\", \"NFS_V4_1\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys\nand values have the same definition as resource manager\ntags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is\nignored when empty. The field is immutable and causes\nresource replacement when mutated. This field is only set\nat create time and modifying this field after creation\nwill trigger recreation. To apply tags to an existing\nresource, see the 'google_tags_tag_value' resource."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe name of the Filestore zone of the instance."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_services` after provisioning.\n"]
    pub fn directory_services(&self) -> terrars::ListRef<FilestoreInstance_DirectoryServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.directory_services", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(&self) -> terrars::ListRef<FilestoreInstance_FileSharesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_shares", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_replication` after provisioning.\n"]
    pub fn initial_replication(&self) -> terrars::ListRef<FilestoreInstance_InitialReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_replication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> terrars::ListRef<FilestoreInstance_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(&self) -> terrars::ListRef<FilestoreInstance_PerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FilestoreInstance_TimeoutsRef {
        FilestoreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FilestoreInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FilestoreInstance { }

impl terrars::ToListMappable for FilestoreInstance {
    type O = terrars::ListRef<FilestoreInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FilestoreInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_filestore_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFilestoreInstance {
    pub tf_id: std::string::String,
    #[doc = "The resource name of the instance."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE"]
    pub tier: terrars::PrimField<std::string::String>,
}

impl BuildFilestoreInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> FilestoreInstance {
        let out = FilestoreInstance(std::rc::Rc::new(FilestoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FilestoreInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                deletion_protection_reason: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_replica_state: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                tags: core::default::Default::default(),
                tier: self.tier,
                zone: core::default::Default::default(),
                directory_services: core::default::Default::default(),
                file_shares: core::default::Default::default(),
                initial_replication: core::default::Default::default(),
                networks: core::default::Default::default(),
                performance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FilestoreInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FilestoreInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nIndicates whether the instance is protected against deletion."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_reason` after provisioning.\nThe reason for enabling deletion protection."]
    pub fn deletion_protection_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_replica_state` after provisioning.\nThe desired_replica_state field controls the state of a replica. Terraform will attempt to make the actual state of the replica match the desired state. Default value: \"READY\" Possible values: [\"PAUSED\", \"READY\"]"]
    pub fn desired_replica_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_replica_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_replication` after provisioning.\nOutput only fields for replication configuration."]
    pub fn effective_replication(&self) -> terrars::ListRef<FilestoreInstance_EffectiveReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key name used for data encryption."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nEither NFSv3, for using NFS version 3 as file sharing protocol,\nor NFSv4.1, for using NFS version 4.1 as file sharing protocol.\nNFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE.\nThe default is NFSv3. Default value: \"NFS_V3\" Possible values: [\"NFS_V3\", \"NFS_V4_1\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys\nand values have the same definition as resource manager\ntags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is\nignored when empty. The field is immutable and causes\nresource replacement when mutated. This field is only set\nat create time and modifying this field after creation\nwill trigger recreation. To apply tags to an existing\nresource, see the 'google_tags_tag_value' resource."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe name of the Filestore zone of the instance."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_services` after provisioning.\n"]
    pub fn directory_services(&self) -> terrars::ListRef<FilestoreInstance_DirectoryServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.directory_services", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(&self) -> terrars::ListRef<FilestoreInstance_FileSharesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_shares", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_replication` after provisioning.\n"]
    pub fn initial_replication(&self) -> terrars::ListRef<FilestoreInstance_InitialReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_replication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> terrars::ListRef<FilestoreInstance_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(&self) -> terrars::ListRef<FilestoreInstance_PerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FilestoreInstance_TimeoutsRef {
        FilestoreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FilestoreInstanceDynamic {
    directory_services: std::option::Option<terrars::DynamicBlock<FilestoreInstance_DirectoryServices>>,
    file_shares: std::option::Option<terrars::DynamicBlock<FilestoreInstance_FileShares>>,
    initial_replication: std::option::Option<terrars::DynamicBlock<FilestoreInstance_InitialReplication>>,
    networks: std::option::Option<terrars::DynamicBlock<FilestoreInstance_Networks>>,
    performance_config: std::option::Option<terrars::DynamicBlock<FilestoreInstance_PerformanceConfig>>,
}

#[derive(Serialize)]
pub struct FilestoreInstance_EffectiveReplication_Replicas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_active_sync_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl FilestoreInstance_EffectiveReplication_Replicas {
    #[doc = "Set the field `last_active_sync_time`.\n"]
    pub fn set_last_active_sync_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_active_sync_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_instance`.\n"]
    pub fn set_peer_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_reasons`.\n"]
    pub fn set_state_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.state_reasons = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_EffectiveReplication_Replicas {
    type O = terrars::BlockAssignable<FilestoreInstance_EffectiveReplication_Replicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_EffectiveReplication_Replicas {}

impl BuildFilestoreInstance_EffectiveReplication_Replicas {
    pub fn build(self) -> FilestoreInstance_EffectiveReplication_Replicas {
        FilestoreInstance_EffectiveReplication_Replicas {
            last_active_sync_time: core::default::Default::default(),
            peer_instance: core::default::Default::default(),
            state: core::default::Default::default(),
            state_reasons: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_EffectiveReplication_ReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_EffectiveReplication_ReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FilestoreInstance_EffectiveReplication_ReplicasRef {
        FilestoreInstance_EffectiveReplication_ReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_EffectiveReplication_ReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_active_sync_time` after provisioning.\n"]
    pub fn last_active_sync_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_active_sync_time", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_instance` after provisioning.\n"]
    pub fn peer_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_reasons` after provisioning.\n"]
    pub fn state_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_reasons", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_EffectiveReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::ListField<FilestoreInstance_EffectiveReplication_Replicas>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FilestoreInstance_EffectiveReplication {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FilestoreInstance_EffectiveReplication_Replicas>>,
    ) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_EffectiveReplication {
    type O = terrars::BlockAssignable<FilestoreInstance_EffectiveReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_EffectiveReplication {}

impl BuildFilestoreInstance_EffectiveReplication {
    pub fn build(self) -> FilestoreInstance_EffectiveReplication {
        FilestoreInstance_EffectiveReplication {
            replicas: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_EffectiveReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_EffectiveReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_EffectiveReplicationRef {
        FilestoreInstance_EffectiveReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_EffectiveReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<FilestoreInstance_EffectiveReplication_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_DirectoryServices_Ldap {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups_ou: std::option::Option<terrars::PrimField<std::string::String>>,
    servers: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    users_ou: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FilestoreInstance_DirectoryServices_Ldap {
    #[doc =
        "Set the field `groups_ou`.\nThe groups Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn set_groups_ou(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.groups_ou = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `users_ou`.\nThe users Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn set_users_ou(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.users_ou = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_DirectoryServices_Ldap {
    type O = terrars::BlockAssignable<FilestoreInstance_DirectoryServices_Ldap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_DirectoryServices_Ldap {
    #[doc = "The LDAP domain name in the format of 'my-domain.com'."]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc =
        "The servers names are used for specifying the LDAP servers names.\nThe LDAP servers names can come with two formats:\n1. DNS name, for example: 'ldap.example1.com', 'ldap.example2.com'.\n2. IP address, for example: '10.0.0.1', '10.0.0.2', '10.0.0.3'.\nAll servers names must be in the same format: either all DNS names or all\nIP addresses."]
    pub servers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildFilestoreInstance_DirectoryServices_Ldap {
    pub fn build(self) -> FilestoreInstance_DirectoryServices_Ldap {
        FilestoreInstance_DirectoryServices_Ldap {
            domain: self.domain,
            groups_ou: core::default::Default::default(),
            servers: self.servers,
            users_ou: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_DirectoryServices_LdapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_DirectoryServices_LdapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_DirectoryServices_LdapRef {
        FilestoreInstance_DirectoryServices_LdapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_DirectoryServices_LdapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe LDAP domain name in the format of 'my-domain.com'."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `groups_ou` after provisioning.\nThe groups Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn groups_ou(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.groups_ou", self.base))
    }

    #[doc =
        "Get a reference to the value of field `servers` after provisioning.\nThe servers names are used for specifying the LDAP servers names.\nThe LDAP servers names can come with two formats:\n1. DNS name, for example: 'ldap.example1.com', 'ldap.example2.com'.\n2. IP address, for example: '10.0.0.1', '10.0.0.2', '10.0.0.3'.\nAll servers names must be in the same format: either all DNS names or all\nIP addresses."]
    pub fn servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `users_ou` after provisioning.\nThe users Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn users_ou(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.users_ou", self.base))
    }
}

#[derive(Serialize, Default)]
struct FilestoreInstance_DirectoryServicesDynamic {
    ldap: std::option::Option<terrars::DynamicBlock<FilestoreInstance_DirectoryServices_Ldap>>,
}

#[derive(Serialize)]
pub struct FilestoreInstance_DirectoryServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ldap: std::option::Option<std::vec::Vec<FilestoreInstance_DirectoryServices_Ldap>>,
    dynamic: FilestoreInstance_DirectoryServicesDynamic,
}

impl FilestoreInstance_DirectoryServices {
    #[doc = "Set the field `ldap`.\n"]
    pub fn set_ldap(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_DirectoryServices_Ldap>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ldap = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ldap = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_DirectoryServices {
    type O = terrars::BlockAssignable<FilestoreInstance_DirectoryServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_DirectoryServices {}

impl BuildFilestoreInstance_DirectoryServices {
    pub fn build(self) -> FilestoreInstance_DirectoryServices {
        FilestoreInstance_DirectoryServices {
            ldap: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_DirectoryServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_DirectoryServicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_DirectoryServicesRef {
        FilestoreInstance_DirectoryServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_DirectoryServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ldap` after provisioning.\n"]
    pub fn ldap(&self) -> terrars::ListRef<FilestoreInstance_DirectoryServices_LdapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_FileShares_NfsExportOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anon_gid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anon_uid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    squash_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FilestoreInstance_FileShares_NfsExportOptions {
    #[doc =
        "Set the field `access_mode`.\nEither READ_ONLY, for allowing only read requests on the exported directory,\nor READ_WRITE, for allowing both read and write requests. The default is READ_WRITE. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn set_access_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `anon_gid`.\nAn integer representing the anonymous group id with a default value of 65534.\nAnon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn set_anon_gid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.anon_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `anon_uid`.\nAn integer representing the anonymous user id with a default value of 65534.\nAnon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn set_anon_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.anon_uid = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_ranges`.\nList of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.\nOverlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.\nThe limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions."]
    pub fn set_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe source VPC network for 'ip_ranges'.\nRequired for instances using Private Service Connect, optional otherwise."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `squash_mode`.\nEither NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,\nfor not allowing root access. The default is NO_ROOT_SQUASH. Default value: \"NO_ROOT_SQUASH\" Possible values: [\"NO_ROOT_SQUASH\", \"ROOT_SQUASH\"]"]
    pub fn set_squash_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.squash_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_FileShares_NfsExportOptions {
    type O = terrars::BlockAssignable<FilestoreInstance_FileShares_NfsExportOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_FileShares_NfsExportOptions {}

impl BuildFilestoreInstance_FileShares_NfsExportOptions {
    pub fn build(self) -> FilestoreInstance_FileShares_NfsExportOptions {
        FilestoreInstance_FileShares_NfsExportOptions {
            access_mode: core::default::Default::default(),
            anon_gid: core::default::Default::default(),
            anon_uid: core::default::Default::default(),
            ip_ranges: core::default::Default::default(),
            network: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_FileShares_NfsExportOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_FileShares_NfsExportOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FilestoreInstance_FileShares_NfsExportOptionsRef {
        FilestoreInstance_FileShares_NfsExportOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_FileShares_NfsExportOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_mode` after provisioning.\nEither READ_ONLY, for allowing only read requests on the exported directory,\nor READ_WRITE, for allowing both read and write requests. The default is READ_WRITE. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `anon_gid` after provisioning.\nAn integer representing the anonymous group id with a default value of 65534.\nAnon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn anon_gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anon_gid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `anon_uid` after provisioning.\nAn integer representing the anonymous user id with a default value of 65534.\nAnon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn anon_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anon_uid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_ranges` after provisioning.\nList of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.\nOverlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.\nThe limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions."]
    pub fn ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe source VPC network for 'ip_ranges'.\nRequired for instances using Private Service Connect, optional otherwise."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `squash_mode` after provisioning.\nEither NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,\nfor not allowing root access. The default is NO_ROOT_SQUASH. Default value: \"NO_ROOT_SQUASH\" Possible values: [\"NO_ROOT_SQUASH\", \"ROOT_SQUASH\"]"]
    pub fn squash_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct FilestoreInstance_FileSharesDynamic {
    nfs_export_options: std::option::Option<terrars::DynamicBlock<FilestoreInstance_FileShares_NfsExportOptions>>,
}

#[derive(Serialize)]
pub struct FilestoreInstance_FileShares {
    capacity_gb: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_backup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_backupdr_backup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_export_options: std::option::Option<std::vec::Vec<FilestoreInstance_FileShares_NfsExportOptions>>,
    dynamic: FilestoreInstance_FileSharesDynamic,
}

impl FilestoreInstance_FileShares {
    #[doc =
        "Set the field `source_backup`.\nThe resource name of the backup, in the format\nprojects/{projectId}/locations/{locationId}/backups/{backupId},\nthat this file share has been restored from."]
    pub fn set_source_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_backupdr_backup`.\nThe resource name of the BackupDR backup, in the format\n'projects/{project_id}/locations/{location_id}/backupVaults/{backupvault_id}/dataSources/{datasource_id}/backups/{backup_id}',\nthat this file share has been restored from."]
    pub fn set_source_backupdr_backup(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_backupdr_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs_export_options`.\n"]
    pub fn set_nfs_export_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_FileShares_NfsExportOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nfs_export_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs_export_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_FileShares {
    type O = terrars::BlockAssignable<FilestoreInstance_FileShares>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_FileShares {
    #[doc =
        "File share capacity in GiB. Acceptable instance capacities for each tier are as follows:\n* BASIC_HDD: 1024-65433 GiB in 1 GiB increments or its multiples.\n* BASIC_SSD: 2560-65433 GiB in 1 GiB increments or its multiples.\n* HIGH_SCALE_SSD: 10240-102400 GiB in 2560 GiB increments or its multiples.\n* ZONAL: 100-102400 GiB (100-10239 GiB in 1 GiB increments or its multiples; 10240-102400 GiB in 2560 GiB increments or its multiples).\n* ENTERPRISE: 1024-10240 GiB in 256 GiB increments or its multiples.\n* REGIONAL: 1024-102400 GiB (100-102400 GiB in supported regions): ((100 or 1024)-10239 GiB in 1 GiB increments or its multiples; 10240-102400 GiB in 2560 GiB increments or its multiples)."]
    pub capacity_gb: terrars::PrimField<f64>,
    #[doc = "The name of the fileshare (16 characters or less)"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFilestoreInstance_FileShares {
    pub fn build(self) -> FilestoreInstance_FileShares {
        FilestoreInstance_FileShares {
            capacity_gb: self.capacity_gb,
            name: self.name,
            source_backup: core::default::Default::default(),
            source_backupdr_backup: core::default::Default::default(),
            nfs_export_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_FileSharesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_FileSharesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_FileSharesRef {
        FilestoreInstance_FileSharesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_FileSharesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `capacity_gb` after provisioning.\nFile share capacity in GiB. Acceptable instance capacities for each tier are as follows:\n* BASIC_HDD: 1024-65433 GiB in 1 GiB increments or its multiples.\n* BASIC_SSD: 2560-65433 GiB in 1 GiB increments or its multiples.\n* HIGH_SCALE_SSD: 10240-102400 GiB in 2560 GiB increments or its multiples.\n* ZONAL: 100-102400 GiB (100-10239 GiB in 1 GiB increments or its multiples; 10240-102400 GiB in 2560 GiB increments or its multiples).\n* ENTERPRISE: 1024-10240 GiB in 256 GiB increments or its multiples.\n* REGIONAL: 1024-102400 GiB (100-102400 GiB in supported regions): ((100 or 1024)-10239 GiB in 1 GiB increments or its multiples; 10240-102400 GiB in 2560 GiB increments or its multiples)."]
    pub fn capacity_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the fileshare (16 characters or less)"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_backup` after provisioning.\nThe resource name of the backup, in the format\nprojects/{projectId}/locations/{locationId}/backups/{backupId},\nthat this file share has been restored from."]
    pub fn source_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_backup", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_backupdr_backup` after provisioning.\nThe resource name of the BackupDR backup, in the format\n'projects/{project_id}/locations/{location_id}/backupVaults/{backupvault_id}/dataSources/{datasource_id}/backups/{backup_id}',\nthat this file share has been restored from."]
    pub fn source_backupdr_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_backupdr_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs_export_options` after provisioning.\n"]
    pub fn nfs_export_options(&self) -> terrars::ListRef<FilestoreInstance_FileShares_NfsExportOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_export_options", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_InitialReplication_Replicas {
    peer_instance: terrars::PrimField<std::string::String>,
}

impl FilestoreInstance_InitialReplication_Replicas { }

impl terrars::ToListMappable for FilestoreInstance_InitialReplication_Replicas {
    type O = terrars::BlockAssignable<FilestoreInstance_InitialReplication_Replicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_InitialReplication_Replicas {
    #[doc = "The peer instance."]
    pub peer_instance: terrars::PrimField<std::string::String>,
}

impl BuildFilestoreInstance_InitialReplication_Replicas {
    pub fn build(self) -> FilestoreInstance_InitialReplication_Replicas {
        FilestoreInstance_InitialReplication_Replicas { peer_instance: self.peer_instance }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_InitialReplication_ReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_InitialReplication_ReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FilestoreInstance_InitialReplication_ReplicasRef {
        FilestoreInstance_InitialReplication_ReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_InitialReplication_ReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `peer_instance` after provisioning.\nThe peer instance."]
    pub fn peer_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct FilestoreInstance_InitialReplicationDynamic {
    replicas: std::option::Option<terrars::DynamicBlock<FilestoreInstance_InitialReplication_Replicas>>,
}

#[derive(Serialize)]
pub struct FilestoreInstance_InitialReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<std::vec::Vec<FilestoreInstance_InitialReplication_Replicas>>,
    dynamic: FilestoreInstance_InitialReplicationDynamic,
}

impl FilestoreInstance_InitialReplication {
    #[doc =
        "Set the field `role`.\nThe replication role. Default value: \"STANDBY\" Possible values: [\"ROLE_UNSPECIFIED\", \"ACTIVE\", \"STANDBY\"]"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_InitialReplication_Replicas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replicas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replicas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_InitialReplication {
    type O = terrars::BlockAssignable<FilestoreInstance_InitialReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_InitialReplication {}

impl BuildFilestoreInstance_InitialReplication {
    pub fn build(self) -> FilestoreInstance_InitialReplication {
        FilestoreInstance_InitialReplication {
            role: core::default::Default::default(),
            replicas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_InitialReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_InitialReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_InitialReplicationRef {
        FilestoreInstance_InitialReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_InitialReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe replication role. Default value: \"STANDBY\" Possible values: [\"ROLE_UNSPECIFIED\", \"ACTIVE\", \"STANDBY\"]"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<FilestoreInstance_InitialReplication_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_Networks_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FilestoreInstance_Networks_PscConfig {
    #[doc =
        "Set the field `endpoint_project`.\nConsumer service project in which the Private Service Connect endpoint\nwould be set up. This is optional, and only relevant in case the network\nis a shared VPC. If this is not specified, the endpoint would be set up\nin the VPC host project."]
    pub fn set_endpoint_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_Networks_PscConfig {
    type O = terrars::BlockAssignable<FilestoreInstance_Networks_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_Networks_PscConfig {}

impl BuildFilestoreInstance_Networks_PscConfig {
    pub fn build(self) -> FilestoreInstance_Networks_PscConfig {
        FilestoreInstance_Networks_PscConfig { endpoint_project: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_Networks_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_Networks_PscConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_Networks_PscConfigRef {
        FilestoreInstance_Networks_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_Networks_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint_project` after provisioning.\nConsumer service project in which the Private Service Connect endpoint\nwould be set up. This is optional, and only relevant in case the network\nis a shared VPC. If this is not specified, the endpoint would be set up\nin the VPC host project."]
    pub fn endpoint_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_project", self.base))
    }
}

#[derive(Serialize, Default)]
struct FilestoreInstance_NetworksDynamic {
    psc_config: std::option::Option<terrars::DynamicBlock<FilestoreInstance_Networks_PscConfig>>,
}

#[derive(Serialize)]
pub struct FilestoreInstance_Networks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    modes: terrars::ListField<terrars::PrimField<std::string::String>>,
    network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<std::vec::Vec<FilestoreInstance_Networks_PscConfig>>,
    dynamic: FilestoreInstance_NetworksDynamic,
}

impl FilestoreInstance_Networks {
    #[doc =
        "Set the field `connect_mode`.\nThe network connect mode of the Filestore instance.\nIf not provided, the connect mode defaults to\nDIRECT_PEERING. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\", \"PRIVATE_SERVICE_CONNECT\"]"]
    pub fn set_connect_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connect_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_ip_range`.\nA /29 CIDR block that identifies the range of IP\naddresses reserved for this instance."]
    pub fn set_reserved_ip_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reserved_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_Networks_PscConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_Networks {
    type O = terrars::BlockAssignable<FilestoreInstance_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_Networks {
    #[doc =
        "IP versions for which the instance has\nIP addresses assigned. Possible values: [\"ADDRESS_MODE_UNSPECIFIED\", \"MODE_IPV4\", \"MODE_IPV6\"]"]
    pub modes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The name of the GCE VPC network to which the\ninstance is connected."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildFilestoreInstance_Networks {
    pub fn build(self) -> FilestoreInstance_Networks {
        FilestoreInstance_Networks {
            connect_mode: core::default::Default::default(),
            modes: self.modes,
            network: self.network,
            reserved_ip_range: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_NetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_NetworksRef {
        FilestoreInstance_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connect_mode` after provisioning.\nThe network connect mode of the Filestore instance.\nIf not provided, the connect mode defaults to\nDIRECT_PEERING. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\", \"PRIVATE_SERVICE_CONNECT\"]"]
    pub fn connect_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_addresses` after provisioning.\nA list of IPv4 or IPv6 addresses."]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modes` after provisioning.\nIP versions for which the instance has\nIP addresses assigned. Possible values: [\"ADDRESS_MODE_UNSPECIFIED\", \"MODE_IPV4\", \"MODE_IPV6\"]"]
    pub fn modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.modes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the GCE VPC network to which the\ninstance is connected."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_range` after provisioning.\nA /29 CIDR block that identifies the range of IP\naddresses reserved for this instance."]
    pub fn reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<FilestoreInstance_Networks_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_PerformanceConfig_FixedIops {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_iops: std::option::Option<terrars::PrimField<f64>>,
}

impl FilestoreInstance_PerformanceConfig_FixedIops {
    #[doc =
        "Set the field `max_iops`.\nThe number of IOPS to provision for the instance.\nmax_iops must be in multiple of 1000."]
    pub fn set_max_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_iops = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_PerformanceConfig_FixedIops {
    type O = terrars::BlockAssignable<FilestoreInstance_PerformanceConfig_FixedIops>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_PerformanceConfig_FixedIops {}

impl BuildFilestoreInstance_PerformanceConfig_FixedIops {
    pub fn build(self) -> FilestoreInstance_PerformanceConfig_FixedIops {
        FilestoreInstance_PerformanceConfig_FixedIops { max_iops: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_PerformanceConfig_FixedIopsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_PerformanceConfig_FixedIopsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FilestoreInstance_PerformanceConfig_FixedIopsRef {
        FilestoreInstance_PerformanceConfig_FixedIopsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_PerformanceConfig_FixedIopsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_iops` after provisioning.\nThe number of IOPS to provision for the instance.\nmax_iops must be in multiple of 1000."]
    pub fn max_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_PerformanceConfig_IopsPerTb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_iops_per_tb: std::option::Option<terrars::PrimField<f64>>,
}

impl FilestoreInstance_PerformanceConfig_IopsPerTb {
    #[doc =
        "Set the field `max_iops_per_tb`.\nThe instance max IOPS will be calculated by multiplying\nthe capacity of the instance (TB) by max_iops_per_tb,\nand rounding to the nearest 1000. The instance max IOPS\nwill be changed dynamically based on the instance\ncapacity."]
    pub fn set_max_iops_per_tb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_iops_per_tb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_PerformanceConfig_IopsPerTb {
    type O = terrars::BlockAssignable<FilestoreInstance_PerformanceConfig_IopsPerTb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_PerformanceConfig_IopsPerTb {}

impl BuildFilestoreInstance_PerformanceConfig_IopsPerTb {
    pub fn build(self) -> FilestoreInstance_PerformanceConfig_IopsPerTb {
        FilestoreInstance_PerformanceConfig_IopsPerTb { max_iops_per_tb: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_PerformanceConfig_IopsPerTbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_PerformanceConfig_IopsPerTbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FilestoreInstance_PerformanceConfig_IopsPerTbRef {
        FilestoreInstance_PerformanceConfig_IopsPerTbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_PerformanceConfig_IopsPerTbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_iops_per_tb` after provisioning.\nThe instance max IOPS will be calculated by multiplying\nthe capacity of the instance (TB) by max_iops_per_tb,\nand rounding to the nearest 1000. The instance max IOPS\nwill be changed dynamically based on the instance\ncapacity."]
    pub fn max_iops_per_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_tb", self.base))
    }
}

#[derive(Serialize, Default)]
struct FilestoreInstance_PerformanceConfigDynamic {
    fixed_iops: std::option::Option<terrars::DynamicBlock<FilestoreInstance_PerformanceConfig_FixedIops>>,
    iops_per_tb: std::option::Option<terrars::DynamicBlock<FilestoreInstance_PerformanceConfig_IopsPerTb>>,
}

#[derive(Serialize)]
pub struct FilestoreInstance_PerformanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_iops: std::option::Option<std::vec::Vec<FilestoreInstance_PerformanceConfig_FixedIops>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops_per_tb: std::option::Option<std::vec::Vec<FilestoreInstance_PerformanceConfig_IopsPerTb>>,
    dynamic: FilestoreInstance_PerformanceConfigDynamic,
}

impl FilestoreInstance_PerformanceConfig {
    #[doc = "Set the field `fixed_iops`.\n"]
    pub fn set_fixed_iops(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_PerformanceConfig_FixedIops>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_iops = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_iops = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iops_per_tb`.\n"]
    pub fn set_iops_per_tb(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FilestoreInstance_PerformanceConfig_IopsPerTb>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iops_per_tb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iops_per_tb = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FilestoreInstance_PerformanceConfig {
    type O = terrars::BlockAssignable<FilestoreInstance_PerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_PerformanceConfig {}

impl BuildFilestoreInstance_PerformanceConfig {
    pub fn build(self) -> FilestoreInstance_PerformanceConfig {
        FilestoreInstance_PerformanceConfig {
            fixed_iops: core::default::Default::default(),
            iops_per_tb: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_PerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_PerformanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_PerformanceConfigRef {
        FilestoreInstance_PerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_PerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed_iops` after provisioning.\n"]
    pub fn fixed_iops(&self) -> terrars::ListRef<FilestoreInstance_PerformanceConfig_FixedIopsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `iops_per_tb` after provisioning.\n"]
    pub fn iops_per_tb(&self) -> terrars::ListRef<FilestoreInstance_PerformanceConfig_IopsPerTbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iops_per_tb", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FilestoreInstance_Timeouts {
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

impl terrars::ToListMappable for FilestoreInstance_Timeouts {
    type O = terrars::BlockAssignable<FilestoreInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstance_Timeouts {}

impl BuildFilestoreInstance_Timeouts {
    pub fn build(self) -> FilestoreInstance_Timeouts {
        FilestoreInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FilestoreInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FilestoreInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FilestoreInstance_TimeoutsRef {
        FilestoreInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstance_TimeoutsRef {
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
