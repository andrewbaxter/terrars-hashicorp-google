#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ArtifactRegistryRepositoryData_ {
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
    cleanup_policy_dry_run: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    repository_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cleanup_policies: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_CleanupPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_config: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_DockerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maven_config: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_MavenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_repository_config: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ArtifactRegistryRepository_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_repository_config: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_VirtualRepositoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerability_scanning_config: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_VulnerabilityScanningConfig>>,
    dynamic: ArtifactRegistryRepositoryDynamic,
}

struct ArtifactRegistryRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ArtifactRegistryRepositoryData_>,
}

#[derive(Clone)]
pub struct ArtifactRegistryRepository(std::rc::Rc<ArtifactRegistryRepository_>);

impl ArtifactRegistryRepository {
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
        "Set the field `cleanup_policy_dry_run`.\nIf true, the cleanup pipeline is prevented from deleting versions in this\nrepository."]
    pub fn set_cleanup_policy_dry_run(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cleanup_policy_dry_run = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe user-provided description of the repository."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nThe Cloud KMS resource name of the customer managed encryption key that’s\nused to encrypt the contents of the Repository. Has the form:\n'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'.\nThis value may not be changed after the Repository has been created."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels with user-defined metadata.\nThis field may contain up to 64 entries. Label keys and values may be no\nlonger than 63 characters. Label keys must begin with a lowercase letter\nand may only contain lowercase letters, numeric characters, underscores,\nand dashes.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nThe mode configures the repository to serve artifacts from different sources. Default value: \"STANDARD_REPOSITORY\" Possible values: [\"STANDARD_REPOSITORY\", \"VIRTUAL_REPOSITORY\", \"REMOTE_REPOSITORY\"]"]
    pub fn set_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cleanup_policies`.\n"]
    pub fn set_cleanup_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryRepository_CleanupPolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cleanup_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cleanup_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `docker_config`.\n"]
    pub fn set_docker_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryRepository_DockerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().docker_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.docker_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maven_config`.\n"]
    pub fn set_maven_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryRepository_MavenConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maven_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maven_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_repository_config`.\n"]
    pub fn set_remote_repository_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_repository_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_repository_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ArtifactRegistryRepository_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_repository_config`.\n"]
    pub fn set_virtual_repository_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryRepository_VirtualRepositoryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_repository_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_repository_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vulnerability_scanning_config`.\n"]
    pub fn set_vulnerability_scanning_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryRepository_VulnerabilityScanningConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vulnerability_scanning_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vulnerability_scanning_config = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `docker_config` after provisioning.\n"]
    pub fn docker_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_DockerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maven_config` after provisioning.\n"]
    pub fn maven_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_MavenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_repository_config` after provisioning.\n"]
    pub fn remote_repository_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_repository_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryRepository_TimeoutsRef {
        ArtifactRegistryRepository_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_repository_config` after provisioning.\n"]
    pub fn virtual_repository_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_VirtualRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_repository_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vulnerability_scanning_config` after provisioning.\n"]
    pub fn vulnerability_scanning_config(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_VulnerabilityScanningConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vulnerability_scanning_config", self.extract_ref()))
    }
}

impl terrars::Referable for ArtifactRegistryRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ArtifactRegistryRepository { }

impl terrars::ToListMappable for ArtifactRegistryRepository {
    type O = terrars::ListRef<ArtifactRegistryRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ArtifactRegistryRepository_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_artifact_registry_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildArtifactRegistryRepository {
    pub tf_id: std::string::String,
    #[doc =
        "The format of packages that are stored in the repository. Supported formats\ncan be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).\nYou can only create alpha formats if you are a member of the\n[alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access)."]
    pub format: terrars::PrimField<std::string::String>,
    #[doc = "The last part of the repository name, for example:\n\"repo1\""]
    pub repository_id: terrars::PrimField<std::string::String>,
}

impl BuildArtifactRegistryRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> ArtifactRegistryRepository {
        let out = ArtifactRegistryRepository(std::rc::Rc::new(ArtifactRegistryRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ArtifactRegistryRepositoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cleanup_policy_dry_run: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                format: self.format,
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                mode: core::default::Default::default(),
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                cleanup_policies: core::default::Default::default(),
                docker_config: core::default::Default::default(),
                maven_config: core::default::Default::default(),
                remote_repository_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_repository_config: core::default::Default::default(),
                vulnerability_scanning_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ArtifactRegistryRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ArtifactRegistryRepositoryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `docker_config` after provisioning.\n"]
    pub fn docker_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_DockerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maven_config` after provisioning.\n"]
    pub fn maven_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_MavenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_repository_config` after provisioning.\n"]
    pub fn remote_repository_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_repository_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryRepository_TimeoutsRef {
        ArtifactRegistryRepository_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_repository_config` after provisioning.\n"]
    pub fn virtual_repository_config(&self) -> terrars::ListRef<ArtifactRegistryRepository_VirtualRepositoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_repository_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vulnerability_scanning_config` after provisioning.\n"]
    pub fn vulnerability_scanning_config(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_VulnerabilityScanningConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vulnerability_scanning_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryDynamic {
    cleanup_policies: std::option::Option<terrars::DynamicBlock<ArtifactRegistryRepository_CleanupPolicies>>,
    docker_config: std::option::Option<terrars::DynamicBlock<ArtifactRegistryRepository_DockerConfig>>,
    maven_config: std::option::Option<terrars::DynamicBlock<ArtifactRegistryRepository_MavenConfig>>,
    remote_repository_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig>>,
    virtual_repository_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_VirtualRepositoryConfig>>,
    vulnerability_scanning_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_VulnerabilityScanningConfig>>,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_CleanupPolicies_Condition {
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

impl ArtifactRegistryRepository_CleanupPolicies_Condition {
    #[doc = "Set the field `newer_than`.\nMatch versions newer than a duration."]
    pub fn set_newer_than(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.newer_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `older_than`.\nMatch versions older than a duration."]
    pub fn set_older_than(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.older_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_name_prefixes`.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn set_package_name_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.package_name_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_prefixes`.\nMatch versions by tag prefix. Applied on any prefix match."]
    pub fn set_tag_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag_state`.\nMatch versions by tag status. Default value: \"ANY\" Possible values: [\"TAGGED\", \"UNTAGGED\", \"ANY\"]"]
    pub fn set_tag_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_name_prefixes`.\nMatch versions by version name prefix. Applied on any prefix match."]
    pub fn set_version_name_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.version_name_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_CleanupPolicies_Condition {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_CleanupPolicies_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_CleanupPolicies_Condition {}

impl BuildArtifactRegistryRepository_CleanupPolicies_Condition {
    pub fn build(self) -> ArtifactRegistryRepository_CleanupPolicies_Condition {
        ArtifactRegistryRepository_CleanupPolicies_Condition {
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
pub struct ArtifactRegistryRepository_CleanupPolicies_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_CleanupPolicies_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_CleanupPolicies_ConditionRef {
        ArtifactRegistryRepository_CleanupPolicies_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_CleanupPolicies_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `newer_than` after provisioning.\nMatch versions newer than a duration."]
    pub fn newer_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.newer_than", self.base))
    }

    #[doc =
        "Get a reference to the value of field `older_than` after provisioning.\nMatch versions older than a duration."]
    pub fn older_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.older_than", self.base))
    }

    #[doc =
        "Get a reference to the value of field `package_name_prefixes` after provisioning.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn package_name_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_name_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag_prefixes` after provisioning.\nMatch versions by tag prefix. Applied on any prefix match."]
    pub fn tag_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag_state` after provisioning.\nMatch versions by tag status. Default value: \"ANY\" Possible values: [\"TAGGED\", \"UNTAGGED\", \"ANY\"]"]
    pub fn tag_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version_name_prefixes` after provisioning.\nMatch versions by version name prefix. Applied on any prefix match."]
    pub fn version_name_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_name_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_name_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    #[doc = "Set the field `keep_count`.\nMinimum number of versions to keep."]
    pub fn set_keep_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_name_prefixes`.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn set_package_name_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.package_name_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {}

impl BuildArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
    pub fn build(self) -> ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
        ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions {
            keep_count: core::default::Default::default(),
            package_name_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
        ArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `keep_count` after provisioning.\nMinimum number of versions to keep."]
    pub fn keep_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `package_name_prefixes` after provisioning.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn package_name_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_name_prefixes", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_CleanupPoliciesDynamic {
    condition: std::option::Option<terrars::DynamicBlock<ArtifactRegistryRepository_CleanupPolicies_Condition>>,
    most_recent_versions: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions>>,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_CleanupPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_CleanupPolicies_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_recent_versions: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions>>,
    dynamic: ArtifactRegistryRepository_CleanupPoliciesDynamic,
}

impl ArtifactRegistryRepository_CleanupPolicies {
    #[doc = "Set the field `action`.\nPolicy action. Possible values: [\"DELETE\", \"KEEP\"]"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryRepository_CleanupPolicies_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `most_recent_versions`.\n"]
    pub fn set_most_recent_versions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArtifactRegistryRepository_CleanupPolicies_MostRecentVersions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.most_recent_versions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.most_recent_versions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_CleanupPolicies {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_CleanupPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_CleanupPolicies {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildArtifactRegistryRepository_CleanupPolicies {
    pub fn build(self) -> ArtifactRegistryRepository_CleanupPolicies {
        ArtifactRegistryRepository_CleanupPolicies {
            action: core::default::Default::default(),
            id: self.id,
            condition: core::default::Default::default(),
            most_recent_versions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_CleanupPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_CleanupPoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArtifactRegistryRepository_CleanupPoliciesRef {
        ArtifactRegistryRepository_CleanupPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_CleanupPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nPolicy action. Possible values: [\"DELETE\", \"KEEP\"]"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<ArtifactRegistryRepository_CleanupPolicies_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `most_recent_versions` after provisioning.\n"]
    pub fn most_recent_versions(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_CleanupPolicies_MostRecentVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.most_recent_versions", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_DockerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    immutable_tags: std::option::Option<terrars::PrimField<bool>>,
}

impl ArtifactRegistryRepository_DockerConfig {
    #[doc =
        "Set the field `immutable_tags`.\nThe repository which enabled this flag prevents all tags from being modified, moved or deleted. This does not prevent tags from being created."]
    pub fn set_immutable_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.immutable_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_DockerConfig {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_DockerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_DockerConfig {}

impl BuildArtifactRegistryRepository_DockerConfig {
    pub fn build(self) -> ArtifactRegistryRepository_DockerConfig {
        ArtifactRegistryRepository_DockerConfig { immutable_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_DockerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_DockerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArtifactRegistryRepository_DockerConfigRef {
        ArtifactRegistryRepository_DockerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_DockerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `immutable_tags` after provisioning.\nThe repository which enabled this flag prevents all tags from being modified, moved or deleted. This does not prevent tags from being created."]
    pub fn immutable_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.immutable_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_MavenConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_snapshot_overwrites: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_MavenConfig {
    #[doc =
        "Set the field `allow_snapshot_overwrites`.\nThe repository with this flag will allow publishing the same\nsnapshot versions."]
    pub fn set_allow_snapshot_overwrites(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_snapshot_overwrites = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_policy`.\nVersion policy defines the versions that the registry will accept. Default value: \"VERSION_POLICY_UNSPECIFIED\" Possible values: [\"VERSION_POLICY_UNSPECIFIED\", \"RELEASE\", \"SNAPSHOT\"]"]
    pub fn set_version_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_MavenConfig {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_MavenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_MavenConfig {}

impl BuildArtifactRegistryRepository_MavenConfig {
    pub fn build(self) -> ArtifactRegistryRepository_MavenConfig {
        ArtifactRegistryRepository_MavenConfig {
            allow_snapshot_overwrites: core::default::Default::default(),
            version_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_MavenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_MavenConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArtifactRegistryRepository_MavenConfigRef {
        ArtifactRegistryRepository_MavenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_MavenConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_snapshot_overwrites` after provisioning.\nThe repository with this flag will allow publishing the same\nsnapshot versions."]
    pub fn allow_snapshot_overwrites(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_snapshot_overwrites", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version_policy` after provisioning.\nVersion policy defines the versions that the registry will accept. Default value: \"VERSION_POLICY_UNSPECIFIED\" Possible values: [\"VERSION_POLICY_UNSPECIFIED\", \"RELEASE\", \"SNAPSHOT\"]"]
    pub fn version_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    repository_base: terrars::PrimField<std::string::String>,
    repository_path: terrars::PrimField<std::string::String>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository { }

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    type O =
        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    #[doc =
        "A common public repository base for Apt, e.g. '\"debian/dists/stable\"' Possible values: [\"DEBIAN\", \"UBUNTU\", \"DEBIAN_SNAPSHOT\"]"]
    pub repository_base: terrars::PrimField<std::string::String>,
    #[doc = "Specific repository from the base."]
    pub repository_path: terrars::PrimField<std::string::String>,
}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository {
            repository_base: self.repository_base,
            repository_path: self.repository_path,
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `repository_base` after provisioning.\nA common public repository base for Apt, e.g. '\"debian/dists/stable\"' Possible values: [\"DEBIAN\", \"UBUNTU\", \"DEBIAN_SNAPSHOT\"]"]
    pub fn repository_base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_base", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository_path` after provisioning.\nSpecific repository from the base."]
    pub fn repository_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryDynamic {
    public_repository: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository>,
    >,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository>>,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepository,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository {
            public_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository_PublicRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    uri: terrars::PrimField<std::string::String>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository { }

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    #[doc =
        "One of:\na. Artifact Registry Repository resource, e.g. 'projects/UPSTREAM_PROJECT_ID/locations/REGION/repositories/UPSTREAM_REPOSITORY'\nb. URI to the registry, e.g. '\"https://registry-1.docker.io\"'\nc. URI to Artifact Registry Repository, e.g. '\"https://REGION-docker.pkg.dev/UPSTREAM_PROJECT_ID/UPSTREAM_REPOSITORY\"'"]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository { uri: self.uri }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nOne of:\na. Artifact Registry Repository resource, e.g. 'projects/UPSTREAM_PROJECT_ID/locations/REGION/repositories/UPSTREAM_REPOSITORY'\nb. URI to the registry, e.g. '\"https://registry-1.docker.io\"'\nc. URI to Artifact Registry Repository, e.g. '\"https://REGION-docker.pkg.dev/UPSTREAM_PROJECT_ID/UPSTREAM_REPOSITORY\"'"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://registry-1.docker.io\"'"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository {
    type O =
        terrars
        ::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://registry-1.docker.io\"'"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryDynamic {
    custom_repository: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository>,
    >,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository>>,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    #[doc = "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"DOCKER_HUB\"]"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"DOCKER_HUB\"]"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://repo.maven.apache.org/maven2\"'"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository {
    type O =
        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://repo.maven.apache.org/maven2\"'"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryDynamic {
    custom_repository: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository>,
    >,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository>>,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    #[doc =
        "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"MAVEN_CENTRAL\"]"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"MAVEN_CENTRAL\"]"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache { }

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {
        ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache {}
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://registry.npmjs.org\"'"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    type O =
        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://registry.npmjs.org\"'"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryDynamic {
    custom_repository: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>,
    >,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository>>,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    #[doc = "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"NPMJS\"]"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"NPMJS\"]"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://pypi.io\"'"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository {
    type O =
        terrars
        ::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://pypi.io\"'"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryDynamic {
    custom_repository: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository>,
    >,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository>>,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    #[doc = "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"PYPI\"]"]
    pub fn set_public_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepository,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"PYPI\"]"]
    pub fn public_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository_CustomRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    #[doc =
        "Set the field `password_secret_version`.\nThe Secret Manager key version that holds the password to access the\nremote repository. Must be in the format of\n'projects/{project}/secrets/{secret}/versions/{version}'."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username to access the remote repository."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    type O =
        terrars
        ::BlockAssignable<
            ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
    pub fn build(
        self,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
        ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials {
            password_secret_version: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nThe Secret Manager key version that holds the password to access the\nremote repository. Must be in the format of\n'projects/{project}/secrets/{secret}/versions/{version}'."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username to access the remote repository."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsDynamic {
    username_password_credentials: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials,
        >,
    >,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_password_credentials: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials>,
    >,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    #[doc = "Set the field `username_password_credentials`.\n"]
    pub fn set_username_password_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentials,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_password_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_password_credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
        ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials {
            username_password_credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username_password_credentials` after provisioning.\n"]
    pub fn username_password_credentials(
        &self,
    ) -> terrars
    ::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials_UsernamePasswordCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_password_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository {
    repository_base: terrars::PrimField<std::string::String>,
    repository_path: terrars::PrimField<std::string::String>,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository { }

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository {
    type O =
        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository {
    #[doc =
        "A common public repository base for Yum. Possible values: [\"CENTOS\", \"CENTOS_DEBUG\", \"CENTOS_VAULT\", \"CENTOS_STREAM\", \"ROCKY\", \"EPEL\"]"]
    pub repository_base: terrars::PrimField<std::string::String>,
    #[doc = "Specific repository from the base, e.g. '\"pub/rocky/9/BaseOS/x86_64/os\"'"]
    pub repository_path: terrars::PrimField<std::string::String>,
}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository {
            repository_base: self.repository_base,
            repository_path: self.repository_path,
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `repository_base` after provisioning.\nA common public repository base for Yum. Possible values: [\"CENTOS\", \"CENTOS_DEBUG\", \"CENTOS_VAULT\", \"CENTOS_STREAM\", \"ROCKY\", \"EPEL\"]"]
    pub fn repository_base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_base", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository_path` after provisioning.\nSpecific repository from the base, e.g. '\"pub/rocky/9/BaseOS/x86_64/os\"'"]
    pub fn repository_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryDynamic {
    public_repository: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository>,
    >,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository>>,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepository,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
        ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository {
            public_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
        ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository_PublicRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_RemoteRepositoryConfigDynamic {
    apt_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository>>,
    common_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository>>,
    docker_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository>>,
    maven_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository>>,
    no_cache: std::option::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache>>,
    npm_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository>>,
    python_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository>>,
    upstream_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials>>,
    yum_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository>>,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_upstream_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt_repository: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maven_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_cache: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    npm_repository: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_repository: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream_credentials: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum_repository: std::option::Option<std::vec::Vec<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository>>,
    dynamic: ArtifactRegistryRepository_RemoteRepositoryConfigDynamic,
}

impl ArtifactRegistryRepository_RemoteRepositoryConfig {
    #[doc = "Set the field `description`.\nThe description of the remote source."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_upstream_validation`.\nIf true, the remote repository upstream and upstream credentials will\nnot be validated."]
    pub fn set_disable_upstream_validation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_upstream_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `apt_repository`.\n"]
    pub fn set_apt_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepository>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.apt_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.apt_repository = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.common_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.common_repository = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.docker_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_repository = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maven_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maven_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `no_cache`.\n"]
    pub fn set_no_cache(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_NoCache>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.no_cache = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.no_cache = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `npm_repository`.\n"]
    pub fn set_npm_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepository>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.npm_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.npm_repository = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.python_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.python_repository = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upstream_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upstream_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `yum_repository`.\n"]
    pub fn set_yum_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepository>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.yum_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.yum_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_RemoteRepositoryConfig {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_RemoteRepositoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_RemoteRepositoryConfig {}

impl BuildArtifactRegistryRepository_RemoteRepositoryConfig {
    pub fn build(self) -> ArtifactRegistryRepository_RemoteRepositoryConfig {
        ArtifactRegistryRepository_RemoteRepositoryConfig {
            description: core::default::Default::default(),
            disable_upstream_validation: core::default::Default::default(),
            apt_repository: core::default::Default::default(),
            common_repository: core::default::Default::default(),
            docker_repository: core::default::Default::default(),
            maven_repository: core::default::Default::default(),
            no_cache: core::default::Default::default(),
            npm_repository: core::default::Default::default(),
            python_repository: core::default::Default::default(),
            upstream_credentials: core::default::Default::default(),
            yum_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_RemoteRepositoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_RemoteRepositoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_RemoteRepositoryConfigRef {
        ArtifactRegistryRepository_RemoteRepositoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_RemoteRepositoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the remote source."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_upstream_validation` after provisioning.\nIf true, the remote repository upstream and upstream credentials will\nnot be validated."]
    pub fn disable_upstream_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_upstream_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `apt_repository` after provisioning.\n"]
    pub fn apt_repository(&self) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_AptRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `common_repository` after provisioning.\n"]
    pub fn common_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_CommonRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\n"]
    pub fn docker_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_DockerRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `maven_repository` after provisioning.\n"]
    pub fn maven_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_MavenRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `no_cache` after provisioning.\n"]
    pub fn no_cache(&self) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_NoCacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_cache", self.base))
    }

    #[doc = "Get a reference to the value of field `npm_repository` after provisioning.\n"]
    pub fn npm_repository(&self) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_NpmRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.npm_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `python_repository` after provisioning.\n"]
    pub fn python_repository(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_PythonRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `upstream_credentials` after provisioning.\n"]
    pub fn upstream_credentials(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_UpstreamCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `yum_repository` after provisioning.\n"]
    pub fn yum_repository(&self) -> terrars::ListRef<ArtifactRegistryRepository_RemoteRepositoryConfig_YumRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_Timeouts {
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

impl terrars::ToListMappable for ArtifactRegistryRepository_Timeouts {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_Timeouts {}

impl BuildArtifactRegistryRepository_Timeouts {
    pub fn build(self) -> ArtifactRegistryRepository_Timeouts {
        ArtifactRegistryRepository_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArtifactRegistryRepository_TimeoutsRef {
        ArtifactRegistryRepository_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_TimeoutsRef {
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

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    #[doc = "Set the field `id`.\nThe user-provided ID of the upstream policy."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\nEntries with a greater priority value take precedence in the pull order."]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repository`.\nA reference to the repository resource, for example:\n\"projects/p1/locations/us-central1/repository/repo1\"."]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {}

impl BuildArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
    pub fn build(self) -> ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
        ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies {
            id: core::default::Default::default(),
            priority: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
        ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe user-provided ID of the upstream policy."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nEntries with a greater priority value take precedence in the pull order."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nA reference to the repository resource, for example:\n\"projects/p1/locations/us-central1/repository/repo1\"."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryRepository_VirtualRepositoryConfigDynamic {
    upstream_policies: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies>>,
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_VirtualRepositoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream_policies: std
    ::option
    ::Option<std::vec::Vec<ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies>>,
    dynamic: ArtifactRegistryRepository_VirtualRepositoryConfigDynamic,
}

impl ArtifactRegistryRepository_VirtualRepositoryConfig {
    #[doc = "Set the field `upstream_policies`.\n"]
    pub fn set_upstream_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPolicies>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upstream_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upstream_policies = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_VirtualRepositoryConfig {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_VirtualRepositoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_VirtualRepositoryConfig {}

impl BuildArtifactRegistryRepository_VirtualRepositoryConfig {
    pub fn build(self) -> ArtifactRegistryRepository_VirtualRepositoryConfig {
        ArtifactRegistryRepository_VirtualRepositoryConfig {
            upstream_policies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_VirtualRepositoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_VirtualRepositoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_VirtualRepositoryConfigRef {
        ArtifactRegistryRepository_VirtualRepositoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_VirtualRepositoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `upstream_policies` after provisioning.\n"]
    pub fn upstream_policies(
        &self,
    ) -> terrars::ListRef<ArtifactRegistryRepository_VirtualRepositoryConfig_UpstreamPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream_policies", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryRepository_VulnerabilityScanningConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enablement_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryRepository_VulnerabilityScanningConfig {
    #[doc =
        "Set the field `enablement_config`.\nThis configures whether vulnerability scanning is automatically performed for artifacts pushed to this repository. Possible values: [\"INHERITED\", \"DISABLED\"]"]
    pub fn set_enablement_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enablement_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryRepository_VulnerabilityScanningConfig {
    type O = terrars::BlockAssignable<ArtifactRegistryRepository_VulnerabilityScanningConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryRepository_VulnerabilityScanningConfig {}

impl BuildArtifactRegistryRepository_VulnerabilityScanningConfig {
    pub fn build(self) -> ArtifactRegistryRepository_VulnerabilityScanningConfig {
        ArtifactRegistryRepository_VulnerabilityScanningConfig {
            enablement_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryRepository_VulnerabilityScanningConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryRepository_VulnerabilityScanningConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryRepository_VulnerabilityScanningConfigRef {
        ArtifactRegistryRepository_VulnerabilityScanningConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryRepository_VulnerabilityScanningConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enablement_config` after provisioning.\nThis configures whether vulnerability scanning is automatically performed for artifacts pushed to this repository. Possible values: [\"INHERITED\", \"DISABLED\"]"]
    pub fn enablement_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enablement_state` after provisioning.\nThis field returns whether scanning is active for this repository."]
    pub fn enablement_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enablement_state_reason` after provisioning.\nThis provides an explanation for the state of scanning on this repository."]
    pub fn enablement_state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enablement_state_reason", self.base))
    }
}
