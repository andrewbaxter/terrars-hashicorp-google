#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataArtifactRegistryRepositoryData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    repository_id: terrars::PrimField<std::string::String>,
}

struct DataArtifactRegistryRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataArtifactRegistryRepositoryData_>,
}

#[derive(Clone)]
pub struct DataArtifactRegistryRepository(std::rc::Rc<DataArtifactRegistryRepository_>);

impl DataArtifactRegistryRepository {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cleanup_policies` after provisioning.\nCleanup policies for this repository. Cleanup policies indicate when\ncertain package versions can be automatically deleted.\nMap keys are policy IDs supplied by users during policy creation. They must\nunique within a repository and be under 128 characters in length."]
    pub fn cleanup_policies(&self) -> terrars::SetRef<DataArtifactRegistryRepository_CleanupPoliciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cleanup_policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cleanup_policy_dry_run` after provisioning.\nIf true, the cleanup pipeline is prevented from deleting versions in this\nrepository."]
    pub fn cleanup_policy_dry_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cleanup_policy_dry_run", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the repository."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_config` after provisioning.\nDocker repository config contains repository level configuration for the repositories of docker type."]
    pub fn docker_config(&self) -> terrars::ListRef<DataArtifactRegistryRepository_DockerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nThe format of packages that are stored in the repository. Supported formats\ncan be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).\nYou can only create alpha formats if you are a member of the\n[alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access)."]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS resource name of the customer managed encryption key that’s\nused to encrypt the contents of the Repository. Has the form:\n'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'.\nThis value may not be changed after the Repository has been created."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\nThis field may contain up to 64 entries. Label keys and values may be no\nlonger than 63 characters. Label keys must begin with a lowercase letter\nand may only contain lowercase letters, numeric characters, underscores,\nand dashes.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maven_config` after provisioning.\nMavenRepositoryConfig is maven related repository details.\nProvides additional configuration details for repositories of the maven\nformat type."]
    pub fn maven_config(&self) -> terrars::ListRef<DataArtifactRegistryRepository_MavenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode configures the repository to serve artifacts from different sources. Default value: \"STANDARD_REPOSITORY\" Possible values: [\"STANDARD_REPOSITORY\", \"VIRTUAL_REPOSITORY\", \"REMOTE_REPOSITORY\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the repository, for example:\n\"repo1\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_uri` after provisioning.\nThe repository endpoint, for example: us-docker.pkg.dev/my-proj/my-repo."]
    pub fn registry_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_repository_config` after provisioning.\nConfiguration specific for a Remote Repository."]
    pub fn remote_repository_config(&self) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_repository_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_id` after provisioning.\nThe last part of the repository name, for example:\n\"repo1\""]
    pub fn repository_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_repository_config` after provisioning.\nConfiguration specific for a Virtual Repository."]
    pub fn virtual_repository_config(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_VirtualRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_repository_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vulnerability_scanning_config` after provisioning.\nConfiguration for vulnerability scanning of artifacts stored in this repository."]
    pub fn vulnerability_scanning_config(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_VulnerabilityScanningConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vulnerability_scanning_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataArtifactRegistryRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataArtifactRegistryRepository { }

impl terrars::ToListMappable for DataArtifactRegistryRepository {
    type O = terrars::ListRef<DataArtifactRegistryRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataArtifactRegistryRepository_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_artifact_registry_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataArtifactRegistryRepository {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The last part of the repository name, for example:\n\"repo1\""]
    pub repository_id: terrars::PrimField<std::string::String>,
}

impl BuildDataArtifactRegistryRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> DataArtifactRegistryRepository {
        let out = DataArtifactRegistryRepository(std::rc::Rc::new(DataArtifactRegistryRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataArtifactRegistryRepositoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                repository_id: self.repository_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataArtifactRegistryRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataArtifactRegistryRepositoryRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `cleanup_policies` after provisioning.\nCleanup policies for this repository. Cleanup policies indicate when\ncertain package versions can be automatically deleted.\nMap keys are policy IDs supplied by users during policy creation. They must\nunique within a repository and be under 128 characters in length."]
    pub fn cleanup_policies(&self) -> terrars::SetRef<DataArtifactRegistryRepository_CleanupPoliciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cleanup_policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cleanup_policy_dry_run` after provisioning.\nIf true, the cleanup pipeline is prevented from deleting versions in this\nrepository."]
    pub fn cleanup_policy_dry_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cleanup_policy_dry_run", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the repository."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_config` after provisioning.\nDocker repository config contains repository level configuration for the repositories of docker type."]
    pub fn docker_config(&self) -> terrars::ListRef<DataArtifactRegistryRepository_DockerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nThe format of packages that are stored in the repository. Supported formats\ncan be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).\nYou can only create alpha formats if you are a member of the\n[alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access)."]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS resource name of the customer managed encryption key that’s\nused to encrypt the contents of the Repository. Has the form:\n'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'.\nThis value may not be changed after the Repository has been created."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\nThis field may contain up to 64 entries. Label keys and values may be no\nlonger than 63 characters. Label keys must begin with a lowercase letter\nand may only contain lowercase letters, numeric characters, underscores,\nand dashes.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maven_config` after provisioning.\nMavenRepositoryConfig is maven related repository details.\nProvides additional configuration details for repositories of the maven\nformat type."]
    pub fn maven_config(&self) -> terrars::ListRef<DataArtifactRegistryRepository_MavenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode configures the repository to serve artifacts from different sources. Default value: \"STANDARD_REPOSITORY\" Possible values: [\"STANDARD_REPOSITORY\", \"VIRTUAL_REPOSITORY\", \"REMOTE_REPOSITORY\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the repository, for example:\n\"repo1\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_uri` after provisioning.\nThe repository endpoint, for example: us-docker.pkg.dev/my-proj/my-repo."]
    pub fn registry_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_repository_config` after provisioning.\nConfiguration specific for a Remote Repository."]
    pub fn remote_repository_config(&self) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_repository_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_id` after provisioning.\nThe last part of the repository name, for example:\n\"repo1\""]
    pub fn repository_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_repository_config` after provisioning.\nConfiguration specific for a Virtual Repository."]
    pub fn virtual_repository_config(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_VirtualRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_repository_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vulnerability_scanning_config` after provisioning.\nConfiguration for vulnerability scanning of artifacts stored in this repository."]
    pub fn vulnerability_scanning_config(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_VulnerabilityScanningConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vulnerability_scanning_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_CleanupPolicies_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    newer_than: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    older_than: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_name_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_name_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataArtifactRegistryRepository_CleanupPolicies_Condition {
    #[doc = "Set the field `newer_than`.\n"]
    pub fn set_newer_than(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.newer_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `older_than`.\n"]
    pub fn set_older_than(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.older_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_name_prefixes`.\n"]
    pub fn set_package_name_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.package_name_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_prefixes`.\n"]
    pub fn set_tag_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_state`.\n"]
    pub fn set_tag_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_name_prefixes`.\n"]
    pub fn set_version_name_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.version_name_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_CleanupPolicies_Condition {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_CleanupPolicies_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_CleanupPolicies_Condition {}

impl BuildDataArtifactRegistryRepository_CleanupPolicies_Condition {
    pub fn build(self) -> DataArtifactRegistryRepository_CleanupPolicies_Condition {
        DataArtifactRegistryRepository_CleanupPolicies_Condition {
            newer_than: core::default::Default::default(),
            older_than: core::default::Default::default(),
            package_name_prefixes: core::default::Default::default(),
            tag_prefixes: core::default::Default::default(),
            tag_state: core::default::Default::default(),
            version_name_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_CleanupPolicies_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_CleanupPolicies_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_CleanupPolicies_ConditionRef {
        DataArtifactRegistryRepository_CleanupPolicies_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_CleanupPolicies_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `newer_than` after provisioning.\n"]
    pub fn newer_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.newer_than", self.base))
    }

    #[doc = "Get a reference to the value of field `older_than` after provisioning.\n"]
    pub fn older_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.older_than", self.base))
    }

    #[doc = "Get a reference to the value of field `package_name_prefixes` after provisioning.\n"]
    pub fn package_name_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_name_prefixes", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_prefixes` after provisioning.\n"]
    pub fn tag_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_prefixes", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_state` after provisioning.\n"]
    pub fn tag_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_state", self.base))
    }

    #[doc = "Get a reference to the value of field `version_name_prefixes` after provisioning.\n"]
    pub fn version_name_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_name_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_name_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    #[doc = "Set the field `keep_count`.\n"]
    pub fn set_keep_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_name_prefixes`.\n"]
    pub fn set_package_name_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.package_name_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {}

impl BuildDataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    pub fn build(self) -> DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
        DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
            keep_count: core::default::Default::default(),
            package_name_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
        DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keep_count` after provisioning.\n"]
    pub fn keep_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_count", self.base))
    }

    #[doc = "Get a reference to the value of field `package_name_prefixes` after provisioning.\n"]
    pub fn package_name_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_name_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_CleanupPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::ListField<DataArtifactRegistryRepository_CleanupPolicies_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_recent_versions: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions>>,
}

impl DataArtifactRegistryRepository_CleanupPolicies {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataArtifactRegistryRepository_CleanupPolicies_Condition>>,
    ) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `most_recent_versions`.\n"]
    pub fn set_most_recent_versions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersions>>,
    ) -> Self {
        self.most_recent_versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_CleanupPolicies {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_CleanupPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_CleanupPolicies {}

impl BuildDataArtifactRegistryRepository_CleanupPolicies {
    pub fn build(self) -> DataArtifactRegistryRepository_CleanupPolicies {
        DataArtifactRegistryRepository_CleanupPolicies {
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
            id: core::default::Default::default(),
            most_recent_versions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_CleanupPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_CleanupPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_CleanupPoliciesRef {
        DataArtifactRegistryRepository_CleanupPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_CleanupPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<DataArtifactRegistryRepository_CleanupPolicies_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `most_recent_versions` after provisioning.\n"]
    pub fn most_recent_versions(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.most_recent_versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_DockerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    immutable_tags: std::option::Option<terrars::PrimField<bool>>,
}

impl DataArtifactRegistryRepository_DockerConfig {
    #[doc = "Set the field `immutable_tags`.\n"]
    pub fn set_immutable_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.immutable_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_DockerConfig {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_DockerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_DockerConfig {}

impl BuildDataArtifactRegistryRepository_DockerConfig {
    pub fn build(self) -> DataArtifactRegistryRepository_DockerConfig {
        DataArtifactRegistryRepository_DockerConfig { immutable_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_DockerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_DockerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_DockerConfigRef {
        DataArtifactRegistryRepository_DockerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_DockerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `immutable_tags` after provisioning.\n"]
    pub fn immutable_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.immutable_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_MavenConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_snapshot_overwrites: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_MavenConfig {
    #[doc = "Set the field `allow_snapshot_overwrites`.\n"]
    pub fn set_allow_snapshot_overwrites(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_snapshot_overwrites = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_policy`.\n"]
    pub fn set_version_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_MavenConfig {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_MavenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_MavenConfig {}

impl BuildDataArtifactRegistryRepository_MavenConfig {
    pub fn build(self) -> DataArtifactRegistryRepository_MavenConfig {
        DataArtifactRegistryRepository_MavenConfig {
            allow_snapshot_overwrites: core::default::Default::default(),
            version_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_MavenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_MavenConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataArtifactRegistryRepository_MavenConfigRef {
        DataArtifactRegistryRepository_MavenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_MavenConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_snapshot_overwrites` after provisioning.\n"]
    pub fn allow_snapshot_overwrites(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_snapshot_overwrites", self.base))
    }

    #[doc = "Get a reference to the value of field `version_policy` after provisioning.\n"]
    pub fn version_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    #[doc = "Set the field `repository_base`.\n"]
    pub fn set_repository_base(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository_base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository_path`.\n"]
    pub fn set_repository_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    type O =
        terrars
        ::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
            repository_base: core::default::Default::default(),
            repository_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_base` after provisioning.\n"]
    pub fn repository_base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_base", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_path` after provisioning.\n"]
    pub fn repository_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository>,
    >,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository,
                        >,
                    >,
    ) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
            public_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    type O =
        terrars
        ::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        self.custom_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
            custom_repository: core::default::Default::default(),
            public_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        self.custom_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
            custom_repository: core::default::Default::default(),
            public_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache { }

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {}
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        self.custom_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
            custom_repository: core::default::Default::default(),
            public_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        self.custom_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
            custom_repository: core::default::Default::default(),
            public_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    #[doc = "Set the field `password_secret_version`.\n"]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    type O =
        terrars
        ::BlockAssignable<
            DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    pub fn build(
        self,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
            password_secret_version: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_secret_version` after provisioning.\n"]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_password_credentials: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials,
        >,
    >,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    #[doc = "Set the field `username_password_credentials`.\n"]
    pub fn set_username_password_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials,
                        >,
                    >,
    ) -> Self {
        self.username_password_credentials = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
            username_password_credentials: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username_password_credentials` after provisioning.\n"]
    pub fn username_password_credentials(
        &self,
    ) -> terrars
    ::ListRef<
        DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_password_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository>,
    >,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository,
                        >,
                    >,
    ) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
            public_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_upstream_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maven_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_cache: std::option::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    npm_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream_credentials: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository>>,
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfig {
    #[doc = "Set the field `apt_repository`.\n"]
    pub fn set_apt_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository>>,
    ) -> Self {
        self.apt_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_repository`.\n"]
    pub fn set_common_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository>,
                    >,
    ) -> Self {
        self.common_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_upstream_validation`.\n"]
    pub fn set_disable_upstream_validation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_upstream_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docker_repository`.\n"]
    pub fn set_docker_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository>,
                    >,
    ) -> Self {
        self.docker_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maven_repository`.\n"]
    pub fn set_maven_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository>>,
    ) -> Self {
        self.maven_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_cache`.\n"]
    pub fn set_no_cache(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCache>>,
    ) -> Self {
        self.no_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `npm_repository`.\n"]
    pub fn set_npm_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository>>,
    ) -> Self {
        self.npm_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_repository`.\n"]
    pub fn set_python_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository>,
                    >,
    ) -> Self {
        self.python_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upstream_credentials`.\n"]
    pub fn set_upstream_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials>,
                    >,
    ) -> Self {
        self.upstream_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `yum_repository`.\n"]
    pub fn set_yum_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository>>,
    ) -> Self {
        self.yum_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_RemoteRepositoryConfig {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_RemoteRepositoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_RemoteRepositoryConfig {}

impl BuildDataArtifactRegistryRepository_RemoteRepositoryConfig {
    pub fn build(self) -> DataArtifactRegistryRepository_RemoteRepositoryConfig {
        DataArtifactRegistryRepository_RemoteRepositoryConfig {
            apt_repository: core::default::Default::default(),
            common_repository: core::default::Default::default(),
            description: core::default::Default::default(),
            disable_upstream_validation: core::default::Default::default(),
            docker_repository: core::default::Default::default(),
            maven_repository: core::default::Default::default(),
            no_cache: core::default::Default::default(),
            npm_repository: core::default::Default::default(),
            python_repository: core::default::Default::default(),
            upstream_credentials: core::default::Default::default(),
            yum_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_RemoteRepositoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_RemoteRepositoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_RemoteRepositoryConfigRef {
        DataArtifactRegistryRepository_RemoteRepositoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_RemoteRepositoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apt_repository` after provisioning.\n"]
    pub fn apt_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `common_repository` after provisioning.\n"]
    pub fn common_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_upstream_validation` after provisioning.\n"]
    pub fn disable_upstream_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_upstream_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\n"]
    pub fn docker_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `maven_repository` after provisioning.\n"]
    pub fn maven_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `no_cache` after provisioning.\n"]
    pub fn no_cache(&self) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_cache", self.base))
    }

    #[doc = "Get a reference to the value of field `npm_repository` after provisioning.\n"]
    pub fn npm_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.npm_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `python_repository` after provisioning.\n"]
    pub fn python_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `upstream_credentials` after provisioning.\n"]
    pub fn upstream_credentials(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `yum_repository` after provisioning.\n"]
    pub fn yum_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {}

impl BuildDataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    pub fn build(self) -> DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
        DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
            id: core::default::Default::default(),
            priority: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
        DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_VirtualRepositoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream_policies: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies>>,
}

impl DataArtifactRegistryRepository_VirtualRepositoryConfig {
    #[doc = "Set the field `upstream_policies`.\n"]
    pub fn set_upstream_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies>,
                    >,
    ) -> Self {
        self.upstream_policies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_VirtualRepositoryConfig {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_VirtualRepositoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_VirtualRepositoryConfig {}

impl BuildDataArtifactRegistryRepository_VirtualRepositoryConfig {
    pub fn build(self) -> DataArtifactRegistryRepository_VirtualRepositoryConfig {
        DataArtifactRegistryRepository_VirtualRepositoryConfig {
            upstream_policies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_VirtualRepositoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_VirtualRepositoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_VirtualRepositoryConfigRef {
        DataArtifactRegistryRepository_VirtualRepositoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_VirtualRepositoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `upstream_policies` after provisioning.\n"]
    pub fn upstream_policies(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream_policies", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepository_VulnerabilityScanningConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enablement_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enablement_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enablement_state_reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepository_VulnerabilityScanningConfig {
    #[doc = "Set the field `enablement_config`.\n"]
    pub fn set_enablement_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enablement_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enablement_state`.\n"]
    pub fn set_enablement_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enablement_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enablement_state_reason`.\n"]
    pub fn set_enablement_state_reason(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enablement_state_reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepository_VulnerabilityScanningConfig {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepository_VulnerabilityScanningConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepository_VulnerabilityScanningConfig {}

impl BuildDataArtifactRegistryRepository_VulnerabilityScanningConfig {
    pub fn build(self) -> DataArtifactRegistryRepository_VulnerabilityScanningConfig {
        DataArtifactRegistryRepository_VulnerabilityScanningConfig {
            enablement_config: core::default::Default::default(),
            enablement_state: core::default::Default::default(),
            enablement_state_reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArtifactRegistryRepository_VulnerabilityScanningConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepository_VulnerabilityScanningConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepository_VulnerabilityScanningConfigRef {
        DataArtifactRegistryRepository_VulnerabilityScanningConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepository_VulnerabilityScanningConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enablement_config` after provisioning.\n"]
    pub fn enablement_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_config", self.base))
    }

    #[doc = "Get a reference to the value of field `enablement_state` after provisioning.\n"]
    pub fn enablement_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_state", self.base))
    }

    #[doc = "Get a reference to the value of field `enablement_state_reason` after provisioning.\n"]
    pub fn enablement_state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_state_reason", self.base))
    }
}
