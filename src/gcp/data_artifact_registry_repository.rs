use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataArtifactRegistryRepositoryData {
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
    data: std::cell::RefCell<DataArtifactRegistryRepositoryData>,
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
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the repository."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_config` after provisioning.\nDocker repository config contains repository level configuration for the repositories of docker type."]
    pub fn docker_config(&self) -> terrars::ListRef<DataArtifactRegistryRepositoryDockerConfigElRef> {
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
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this repository is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maven_config` after provisioning.\nMavenRepositoryConfig is maven related repository details.\nProvides additional configuration details for repositories of the maven\nformat type."]
    pub fn maven_config(&self) -> terrars::ListRef<DataArtifactRegistryRepositoryMavenConfigElRef> {
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
        "Get a reference to the value of field `remote_repository_config` after provisioning.\nConfiguration specific for a Remote Repository."]
    pub fn remote_repository_config(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElRef> {
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
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryVirtualRepositoryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_repository_config", self.extract_ref()))
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
    #[doc = "The name of the location this repository is located in."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The last part of the repository name, for example:\n\"repo1\""]
    pub repository_id: terrars::PrimField<std::string::String>,
}

impl BuildDataArtifactRegistryRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> DataArtifactRegistryRepository {
        let out = DataArtifactRegistryRepository(std::rc::Rc::new(DataArtifactRegistryRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataArtifactRegistryRepositoryData {
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
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the repository."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_config` after provisioning.\nDocker repository config contains repository level configuration for the repositories of docker type."]
    pub fn docker_config(&self) -> terrars::ListRef<DataArtifactRegistryRepositoryDockerConfigElRef> {
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
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this repository is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maven_config` after provisioning.\nMavenRepositoryConfig is maven related repository details.\nProvides additional configuration details for repositories of the maven\nformat type."]
    pub fn maven_config(&self) -> terrars::ListRef<DataArtifactRegistryRepositoryMavenConfigElRef> {
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
        "Get a reference to the value of field `remote_repository_config` after provisioning.\nConfiguration specific for a Remote Repository."]
    pub fn remote_repository_config(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElRef> {
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
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryVirtualRepositoryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_repository_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryDockerConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    immutable_tags: std::option::Option<terrars::PrimField<bool>>,
}

impl DataArtifactRegistryRepositoryDockerConfigEl {
    #[doc = "Set the field `immutable_tags`.\n"]
    pub fn set_immutable_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.immutable_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryDockerConfigEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryDockerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryDockerConfigEl {}

impl BuildDataArtifactRegistryRepositoryDockerConfigEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryDockerConfigEl {
        DataArtifactRegistryRepositoryDockerConfigEl { immutable_tags: core::default::Default::default() }
    }
}

pub struct DataArtifactRegistryRepositoryDockerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryDockerConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryDockerConfigElRef {
        DataArtifactRegistryRepositoryDockerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryDockerConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `immutable_tags` after provisioning.\n"]
    pub fn immutable_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.immutable_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryMavenConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_snapshot_overwrites: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryMavenConfigEl {
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

impl terrars::ToListMappable for DataArtifactRegistryRepositoryMavenConfigEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryMavenConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryMavenConfigEl {}

impl BuildDataArtifactRegistryRepositoryMavenConfigEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryMavenConfigEl {
        DataArtifactRegistryRepositoryMavenConfigEl {
            allow_snapshot_overwrites: core::default::Default::default(),
            version_policy: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryMavenConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryMavenConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryMavenConfigElRef {
        DataArtifactRegistryRepositoryMavenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryMavenConfigElRef {
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
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
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

impl terrars
::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
    type O =
        terrars
        ::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
            repository_base: core::default::Default::default(),
            repository_path: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
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
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl>,
    >,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
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
                            DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl,
                        >,
                    >,
    ) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
            public_repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
            public_repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
            public_repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
            public_repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
            public_repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl {
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
::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl {
    type O =
        terrars
        ::BlockAssignable<
            DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl {
    pub fn build(
        self,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl {
            password_secret_version: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef {
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
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_password_credentials: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl,
        >,
    >,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
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
                            DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl,
                        >,
                    >,
    ) -> Self {
        self.username_password_credentials = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
            username_password_credentials: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username_password_credentials` after provisioning.\n"]
    pub fn username_password_credentials(
        &self,
    ) -> terrars
    ::ListRef<
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_password_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
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

impl terrars
::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
    type O =
        terrars
        ::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
            repository_base: core::default::Default::default(),
            repository_path: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
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
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std
    ::option
    ::Option<
        terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl>,
    >,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
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
                            DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl,
                        >,
                    >,
    ) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
            public_repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maven_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    npm_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream_credentials: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum_repository: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl>>,
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    #[doc = "Set the field `apt_repository`.\n"]
    pub fn set_apt_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl>>,
    ) -> Self {
        self.apt_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl>,
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
                    ::Into<
                        terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl>,
                    >,
    ) -> Self {
        self.maven_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `npm_repository`.\n"]
    pub fn set_npm_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl>>,
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
                        terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl>,
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
                        terrars
                        ::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl>,
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
                    ::Into<terrars::ListField<DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl>>,
    ) -> Self {
        self.yum_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryRemoteRepositoryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigEl {}

impl BuildDataArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigEl {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigEl {
            apt_repository: core::default::Default::default(),
            description: core::default::Default::default(),
            docker_repository: core::default::Default::default(),
            maven_repository: core::default::Default::default(),
            npm_repository: core::default::Default::default(),
            python_repository: core::default::Default::default(),
            upstream_credentials: core::default::Default::default(),
            yum_repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
        DataArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apt_repository` after provisioning.\n"]
    pub fn apt_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\n"]
    pub fn docker_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `maven_repository` after provisioning.\n"]
    pub fn maven_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `npm_repository` after provisioning.\n"]
    pub fn npm_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.npm_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `python_repository` after provisioning.\n"]
    pub fn python_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `upstream_credentials` after provisioning.\n"]
    pub fn upstream_credentials(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `yum_repository` after provisioning.\n"]
    pub fn yum_repository(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
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

impl terrars::ToListMappable for DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {}

impl BuildDataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
        DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
            id: core::default::Default::default(),
            priority: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
        DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
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
pub struct DataArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream_policies: std
    ::option
    ::Option<terrars::ListField<DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl>>,
}

impl DataArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    #[doc = "Set the field `upstream_policies`.\n"]
    pub fn set_upstream_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl>,
                    >,
    ) -> Self {
        self.upstream_policies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    type O = terrars::BlockAssignable<DataArtifactRegistryRepositoryVirtualRepositoryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArtifactRegistryRepositoryVirtualRepositoryConfigEl {}

impl BuildDataArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    pub fn build(self) -> DataArtifactRegistryRepositoryVirtualRepositoryConfigEl {
        DataArtifactRegistryRepositoryVirtualRepositoryConfigEl {
            upstream_policies: core::default::Default::default(),
        }
    }
}

pub struct DataArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
        DataArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `upstream_policies` after provisioning.\n"]
    pub fn upstream_policies(
        &self,
    ) -> terrars::ListRef<DataArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream_policies", self.base))
    }
}
