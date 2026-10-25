#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataFilestoreInstanceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataFilestoreInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFilestoreInstanceData_>,
}

#[derive(Clone)]
pub struct DataFilestoreInstance(std::rc::Rc<DataFilestoreInstance_>);

impl DataFilestoreInstance {
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
        "Get a reference to the value of field `directory_services` after provisioning.\nDirectory Services configuration.\nShould only be set if protocol is \"NFS_V4_1\"."]
    pub fn directory_services(&self) -> terrars::ListRef<DataFilestoreInstance_DirectoryServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.directory_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_replication` after provisioning.\nOutput only fields for replication configuration."]
    pub fn effective_replication(&self) -> terrars::ListRef<DataFilestoreInstance_EffectiveReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_shares` after provisioning.\nFile system shares on the instance. For this version, only a\nsingle file share is supported."]
    pub fn file_shares(&self) -> terrars::ListRef<DataFilestoreInstance_FileSharesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_shares", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_replication` after provisioning.\nReplication configuration, once set, this cannot be updated.\nAdditionally this should be specified on the replica instance only, indicating the active as the peer_instance"]
    pub fn initial_replication(&self) -> terrars::ListRef<DataFilestoreInstance_InitialReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_replication", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nVPC networks to which the instance is connected. For this version,\nonly a single network is supported."]
    pub fn networks(&self) -> terrars::ListRef<DataFilestoreInstance_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `performance_config` after provisioning.\nPerformance configuration for the instance. If not provided,\nthe default performance settings will be used."]
    pub fn performance_config(&self) -> terrars::ListRef<DataFilestoreInstance_PerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_config", self.extract_ref()))
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
}

impl terrars::Referable for DataFilestoreInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataFilestoreInstance { }

impl terrars::ToListMappable for DataFilestoreInstance {
    type O = terrars::ListRef<DataFilestoreInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataFilestoreInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_filestore_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFilestoreInstance {
    pub tf_id: std::string::String,
    #[doc = "The resource name of the instance."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataFilestoreInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataFilestoreInstance {
        let out = DataFilestoreInstance(std::rc::Rc::new(DataFilestoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataFilestoreInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFilestoreInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFilestoreInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `directory_services` after provisioning.\nDirectory Services configuration.\nShould only be set if protocol is \"NFS_V4_1\"."]
    pub fn directory_services(&self) -> terrars::ListRef<DataFilestoreInstance_DirectoryServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.directory_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_replication` after provisioning.\nOutput only fields for replication configuration."]
    pub fn effective_replication(&self) -> terrars::ListRef<DataFilestoreInstance_EffectiveReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_shares` after provisioning.\nFile system shares on the instance. For this version, only a\nsingle file share is supported."]
    pub fn file_shares(&self) -> terrars::ListRef<DataFilestoreInstance_FileSharesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_shares", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_replication` after provisioning.\nReplication configuration, once set, this cannot be updated.\nAdditionally this should be specified on the replica instance only, indicating the active as the peer_instance"]
    pub fn initial_replication(&self) -> terrars::ListRef<DataFilestoreInstance_InitialReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_replication", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nVPC networks to which the instance is connected. For this version,\nonly a single network is supported."]
    pub fn networks(&self) -> terrars::ListRef<DataFilestoreInstance_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `performance_config` after provisioning.\nPerformance configuration for the instance. If not provided,\nthe default performance settings will be used."]
    pub fn performance_config(&self) -> terrars::ListRef<DataFilestoreInstance_PerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_config", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_DirectoryServices_Ldap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups_ou: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    users_ou: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFilestoreInstance_DirectoryServices_Ldap {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `groups_ou`.\n"]
    pub fn set_groups_ou(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.groups_ou = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `servers`.\n"]
    pub fn set_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `users_ou`.\n"]
    pub fn set_users_ou(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.users_ou = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_DirectoryServices_Ldap {
    type O = terrars::BlockAssignable<DataFilestoreInstance_DirectoryServices_Ldap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_DirectoryServices_Ldap {}

impl BuildDataFilestoreInstance_DirectoryServices_Ldap {
    pub fn build(self) -> DataFilestoreInstance_DirectoryServices_Ldap {
        DataFilestoreInstance_DirectoryServices_Ldap {
            domain: core::default::Default::default(),
            groups_ou: core::default::Default::default(),
            servers: core::default::Default::default(),
            users_ou: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_DirectoryServices_LdapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_DirectoryServices_LdapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFilestoreInstance_DirectoryServices_LdapRef {
        DataFilestoreInstance_DirectoryServices_LdapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_DirectoryServices_LdapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `groups_ou` after provisioning.\n"]
    pub fn groups_ou(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.groups_ou", self.base))
    }

    #[doc = "Get a reference to the value of field `servers` after provisioning.\n"]
    pub fn servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }

    #[doc = "Get a reference to the value of field `users_ou` after provisioning.\n"]
    pub fn users_ou(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.users_ou", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_DirectoryServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ldap: std::option::Option<terrars::ListField<DataFilestoreInstance_DirectoryServices_Ldap>>,
}

impl DataFilestoreInstance_DirectoryServices {
    #[doc = "Set the field `ldap`.\n"]
    pub fn set_ldap(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFilestoreInstance_DirectoryServices_Ldap>>,
    ) -> Self {
        self.ldap = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_DirectoryServices {
    type O = terrars::BlockAssignable<DataFilestoreInstance_DirectoryServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_DirectoryServices {}

impl BuildDataFilestoreInstance_DirectoryServices {
    pub fn build(self) -> DataFilestoreInstance_DirectoryServices {
        DataFilestoreInstance_DirectoryServices { ldap: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_DirectoryServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_DirectoryServicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFilestoreInstance_DirectoryServicesRef {
        DataFilestoreInstance_DirectoryServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_DirectoryServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ldap` after provisioning.\n"]
    pub fn ldap(&self) -> terrars::ListRef<DataFilestoreInstance_DirectoryServices_LdapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_EffectiveReplication_Replicas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_active_sync_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataFilestoreInstance_EffectiveReplication_Replicas {
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

impl terrars::ToListMappable for DataFilestoreInstance_EffectiveReplication_Replicas {
    type O = terrars::BlockAssignable<DataFilestoreInstance_EffectiveReplication_Replicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_EffectiveReplication_Replicas {}

impl BuildDataFilestoreInstance_EffectiveReplication_Replicas {
    pub fn build(self) -> DataFilestoreInstance_EffectiveReplication_Replicas {
        DataFilestoreInstance_EffectiveReplication_Replicas {
            last_active_sync_time: core::default::Default::default(),
            peer_instance: core::default::Default::default(),
            state: core::default::Default::default(),
            state_reasons: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_EffectiveReplication_ReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_EffectiveReplication_ReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFilestoreInstance_EffectiveReplication_ReplicasRef {
        DataFilestoreInstance_EffectiveReplication_ReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_EffectiveReplication_ReplicasRef {
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
pub struct DataFilestoreInstance_EffectiveReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::ListField<DataFilestoreInstance_EffectiveReplication_Replicas>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFilestoreInstance_EffectiveReplication {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFilestoreInstance_EffectiveReplication_Replicas>>,
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

impl terrars::ToListMappable for DataFilestoreInstance_EffectiveReplication {
    type O = terrars::BlockAssignable<DataFilestoreInstance_EffectiveReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_EffectiveReplication {}

impl BuildDataFilestoreInstance_EffectiveReplication {
    pub fn build(self) -> DataFilestoreInstance_EffectiveReplication {
        DataFilestoreInstance_EffectiveReplication {
            replicas: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_EffectiveReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_EffectiveReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFilestoreInstance_EffectiveReplicationRef {
        DataFilestoreInstance_EffectiveReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_EffectiveReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<DataFilestoreInstance_EffectiveReplication_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_FileShares_NfsExportOptions {
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

impl DataFilestoreInstance_FileShares_NfsExportOptions {
    #[doc = "Set the field `access_mode`.\n"]
    pub fn set_access_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anon_gid`.\n"]
    pub fn set_anon_gid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.anon_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anon_uid`.\n"]
    pub fn set_anon_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.anon_uid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_ranges`.\n"]
    pub fn set_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `squash_mode`.\n"]
    pub fn set_squash_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.squash_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_FileShares_NfsExportOptions {
    type O = terrars::BlockAssignable<DataFilestoreInstance_FileShares_NfsExportOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_FileShares_NfsExportOptions {}

impl BuildDataFilestoreInstance_FileShares_NfsExportOptions {
    pub fn build(self) -> DataFilestoreInstance_FileShares_NfsExportOptions {
        DataFilestoreInstance_FileShares_NfsExportOptions {
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
pub struct DataFilestoreInstance_FileShares_NfsExportOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_FileShares_NfsExportOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFilestoreInstance_FileShares_NfsExportOptionsRef {
        DataFilestoreInstance_FileShares_NfsExportOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_FileShares_NfsExportOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\n"]
    pub fn access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `anon_gid` after provisioning.\n"]
    pub fn anon_gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anon_gid", self.base))
    }

    #[doc = "Get a reference to the value of field `anon_uid` after provisioning.\n"]
    pub fn anon_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anon_uid", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_ranges` after provisioning.\n"]
    pub fn ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `squash_mode` after provisioning.\n"]
    pub fn squash_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_FileShares {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_export_options: std::option::Option<terrars::ListField<DataFilestoreInstance_FileShares_NfsExportOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_backup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_backupdr_backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFilestoreInstance_FileShares {
    #[doc = "Set the field `capacity_gb`.\n"]
    pub fn set_capacity_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.capacity_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs_export_options`.\n"]
    pub fn set_nfs_export_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFilestoreInstance_FileShares_NfsExportOptions>>,
    ) -> Self {
        self.nfs_export_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_backup`.\n"]
    pub fn set_source_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_backupdr_backup`.\n"]
    pub fn set_source_backupdr_backup(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_backupdr_backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_FileShares {
    type O = terrars::BlockAssignable<DataFilestoreInstance_FileShares>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_FileShares {}

impl BuildDataFilestoreInstance_FileShares {
    pub fn build(self) -> DataFilestoreInstance_FileShares {
        DataFilestoreInstance_FileShares {
            capacity_gb: core::default::Default::default(),
            name: core::default::Default::default(),
            nfs_export_options: core::default::Default::default(),
            source_backup: core::default::Default::default(),
            source_backupdr_backup: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_FileSharesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_FileSharesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFilestoreInstance_FileSharesRef {
        DataFilestoreInstance_FileSharesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_FileSharesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_gb` after provisioning.\n"]
    pub fn capacity_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs_export_options` after provisioning.\n"]
    pub fn nfs_export_options(&self) -> terrars::ListRef<DataFilestoreInstance_FileShares_NfsExportOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_export_options", self.base))
    }

    #[doc = "Get a reference to the value of field `source_backup` after provisioning.\n"]
    pub fn source_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `source_backupdr_backup` after provisioning.\n"]
    pub fn source_backupdr_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_backupdr_backup", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_InitialReplication_Replicas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFilestoreInstance_InitialReplication_Replicas {
    #[doc = "Set the field `peer_instance`.\n"]
    pub fn set_peer_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_InitialReplication_Replicas {
    type O = terrars::BlockAssignable<DataFilestoreInstance_InitialReplication_Replicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_InitialReplication_Replicas {}

impl BuildDataFilestoreInstance_InitialReplication_Replicas {
    pub fn build(self) -> DataFilestoreInstance_InitialReplication_Replicas {
        DataFilestoreInstance_InitialReplication_Replicas { peer_instance: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_InitialReplication_ReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_InitialReplication_ReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFilestoreInstance_InitialReplication_ReplicasRef {
        DataFilestoreInstance_InitialReplication_ReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_InitialReplication_ReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `peer_instance` after provisioning.\n"]
    pub fn peer_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_InitialReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::ListField<DataFilestoreInstance_InitialReplication_Replicas>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFilestoreInstance_InitialReplication {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFilestoreInstance_InitialReplication_Replicas>>,
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

impl terrars::ToListMappable for DataFilestoreInstance_InitialReplication {
    type O = terrars::BlockAssignable<DataFilestoreInstance_InitialReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_InitialReplication {}

impl BuildDataFilestoreInstance_InitialReplication {
    pub fn build(self) -> DataFilestoreInstance_InitialReplication {
        DataFilestoreInstance_InitialReplication {
            replicas: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_InitialReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_InitialReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFilestoreInstance_InitialReplicationRef {
        DataFilestoreInstance_InitialReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_InitialReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<DataFilestoreInstance_InitialReplication_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_Networks_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFilestoreInstance_Networks_PscConfig {
    #[doc = "Set the field `endpoint_project`.\n"]
    pub fn set_endpoint_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_Networks_PscConfig {
    type O = terrars::BlockAssignable<DataFilestoreInstance_Networks_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_Networks_PscConfig {}

impl BuildDataFilestoreInstance_Networks_PscConfig {
    pub fn build(self) -> DataFilestoreInstance_Networks_PscConfig {
        DataFilestoreInstance_Networks_PscConfig { endpoint_project: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_Networks_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_Networks_PscConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFilestoreInstance_Networks_PscConfigRef {
        DataFilestoreInstance_Networks_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_Networks_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_project` after provisioning.\n"]
    pub fn endpoint_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_Networks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<terrars::ListField<DataFilestoreInstance_Networks_PscConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFilestoreInstance_Networks {
    #[doc = "Set the field `connect_mode`.\n"]
    pub fn set_connect_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connect_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modes`.\n"]
    pub fn set_modes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.modes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFilestoreInstance_Networks_PscConfig>>,
    ) -> Self {
        self.psc_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reserved_ip_range`.\n"]
    pub fn set_reserved_ip_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reserved_ip_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_Networks {
    type O = terrars::BlockAssignable<DataFilestoreInstance_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_Networks {}

impl BuildDataFilestoreInstance_Networks {
    pub fn build(self) -> DataFilestoreInstance_Networks {
        DataFilestoreInstance_Networks {
            connect_mode: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
            modes: core::default::Default::default(),
            network: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            reserved_ip_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_NetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFilestoreInstance_NetworksRef {
        DataFilestoreInstance_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connect_mode` after provisioning.\n"]
    pub fn connect_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `modes` after provisioning.\n"]
    pub fn modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.modes", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<DataFilestoreInstance_Networks_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }

    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\n"]
    pub fn reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_ip_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_PerformanceConfig_FixedIops {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_iops: std::option::Option<terrars::PrimField<f64>>,
}

impl DataFilestoreInstance_PerformanceConfig_FixedIops {
    #[doc = "Set the field `max_iops`.\n"]
    pub fn set_max_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_iops = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_PerformanceConfig_FixedIops {
    type O = terrars::BlockAssignable<DataFilestoreInstance_PerformanceConfig_FixedIops>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_PerformanceConfig_FixedIops {}

impl BuildDataFilestoreInstance_PerformanceConfig_FixedIops {
    pub fn build(self) -> DataFilestoreInstance_PerformanceConfig_FixedIops {
        DataFilestoreInstance_PerformanceConfig_FixedIops { max_iops: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_PerformanceConfig_FixedIopsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_PerformanceConfig_FixedIopsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFilestoreInstance_PerformanceConfig_FixedIopsRef {
        DataFilestoreInstance_PerformanceConfig_FixedIopsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_PerformanceConfig_FixedIopsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_iops` after provisioning.\n"]
    pub fn max_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_PerformanceConfig_IopsPerTb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_iops_per_tb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataFilestoreInstance_PerformanceConfig_IopsPerTb {
    #[doc = "Set the field `max_iops_per_tb`.\n"]
    pub fn set_max_iops_per_tb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_iops_per_tb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_PerformanceConfig_IopsPerTb {
    type O = terrars::BlockAssignable<DataFilestoreInstance_PerformanceConfig_IopsPerTb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_PerformanceConfig_IopsPerTb {}

impl BuildDataFilestoreInstance_PerformanceConfig_IopsPerTb {
    pub fn build(self) -> DataFilestoreInstance_PerformanceConfig_IopsPerTb {
        DataFilestoreInstance_PerformanceConfig_IopsPerTb { max_iops_per_tb: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_PerformanceConfig_IopsPerTbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_PerformanceConfig_IopsPerTbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFilestoreInstance_PerformanceConfig_IopsPerTbRef {
        DataFilestoreInstance_PerformanceConfig_IopsPerTbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_PerformanceConfig_IopsPerTbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_iops_per_tb` after provisioning.\n"]
    pub fn max_iops_per_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_tb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFilestoreInstance_PerformanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_iops: std::option::Option<terrars::ListField<DataFilestoreInstance_PerformanceConfig_FixedIops>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops_per_tb: std::option::Option<terrars::ListField<DataFilestoreInstance_PerformanceConfig_IopsPerTb>>,
}

impl DataFilestoreInstance_PerformanceConfig {
    #[doc = "Set the field `fixed_iops`.\n"]
    pub fn set_fixed_iops(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFilestoreInstance_PerformanceConfig_FixedIops>>,
    ) -> Self {
        self.fixed_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops_per_tb`.\n"]
    pub fn set_iops_per_tb(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFilestoreInstance_PerformanceConfig_IopsPerTb>>,
    ) -> Self {
        self.iops_per_tb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFilestoreInstance_PerformanceConfig {
    type O = terrars::BlockAssignable<DataFilestoreInstance_PerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFilestoreInstance_PerformanceConfig {}

impl BuildDataFilestoreInstance_PerformanceConfig {
    pub fn build(self) -> DataFilestoreInstance_PerformanceConfig {
        DataFilestoreInstance_PerformanceConfig {
            fixed_iops: core::default::Default::default(),
            iops_per_tb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFilestoreInstance_PerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFilestoreInstance_PerformanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFilestoreInstance_PerformanceConfigRef {
        DataFilestoreInstance_PerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFilestoreInstance_PerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed_iops` after provisioning.\n"]
    pub fn fixed_iops(&self) -> terrars::ListRef<DataFilestoreInstance_PerformanceConfig_FixedIopsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `iops_per_tb` after provisioning.\n"]
    pub fn iops_per_tb(&self) -> terrars::ListRef<DataFilestoreInstance_PerformanceConfig_IopsPerTbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iops_per_tb", self.base))
    }
}
