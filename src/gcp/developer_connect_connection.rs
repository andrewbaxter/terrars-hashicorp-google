#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DeveloperConnectConnectionData_ {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    connection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_cloud_config: std::option::Option<std::vec::Vec<DeveloperConnectConnection_BitbucketCloudConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_data_center_config: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_BitbucketDataCenterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crypto_key_config: std::option::Option<std::vec::Vec<DeveloperConnectConnection_CryptoKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_config: std::option::Option<std::vec::Vec<DeveloperConnectConnection_GithubConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_config: std::option::Option<std::vec::Vec<DeveloperConnectConnection_GithubEnterpriseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab_config: std::option::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab_enterprise_config: std::option::Option<std::vec::Vec<DeveloperConnectConnection_GitlabEnterpriseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_config: std::option::Option<std::vec::Vec<DeveloperConnectConnection_HttpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DeveloperConnectConnection_Timeouts>,
    dynamic: DeveloperConnectConnectionDynamic,
}

struct DeveloperConnectConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DeveloperConnectConnectionData_>,
}

#[derive(Clone)]
pub struct DeveloperConnectConnection(std::rc::Rc<DeveloperConnectConnection_>);

impl DeveloperConnectConnection {
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
        "Set the field `annotations`.\nOptional. Allows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nOptional. If disabled is set to true, functionality is disabled for this connection.\nRepository based API methods and webhooks processing for repositories in\nthis connection will be disabled."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `etag`.\nOptional. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitbucket_cloud_config`.\n"]
    pub fn set_bitbucket_cloud_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_BitbucketCloudConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bitbucket_cloud_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bitbucket_cloud_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bitbucket_data_center_config`.\n"]
    pub fn set_bitbucket_data_center_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_BitbucketDataCenterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bitbucket_data_center_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bitbucket_data_center_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `crypto_key_config`.\n"]
    pub fn set_crypto_key_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_CryptoKeyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().crypto_key_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.crypto_key_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github_config`.\n"]
    pub fn set_github_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_GithubConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().github_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.github_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github_enterprise_config`.\n"]
    pub fn set_github_enterprise_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_GithubEnterpriseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().github_enterprise_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.github_enterprise_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gitlab_config`.\n"]
    pub fn set_gitlab_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gitlab_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gitlab_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gitlab_enterprise_config`.\n"]
    pub fn set_gitlab_enterprise_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_GitlabEnterpriseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gitlab_enterprise_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gitlab_enterprise_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_config`.\n"]
    pub fn set_http_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DeveloperConnectConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Allows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nRequired. Id of the requesting object\nIf auto-generating Id server-side, remove this field and\nconnection_id from the method_signature of Create RPC"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. [Output only] Create timestamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. [Output only] Delete timestamp"]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nOptional. If disabled is set to true, functionality is disabled for this connection.\nRepository based API methods and webhooks processing for repositories in\nthis connection will be disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `installation_state` after provisioning.\nDescribes stage and necessary actions to be taken by the\nuser to complete the installation. Used for GitHub and GitHub Enterprise\nbased connections."]
    pub fn installation_state(&self) -> terrars::ListRef<DeveloperConnectConnection_InstallationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.installation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the connection, in the format\n'projects/{project}/locations/{location}/connections/{connection_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the\nbackground."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A system-assigned unique identifier for a the GitRepositoryLink."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. [Output only] Update timestamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(&self) -> terrars::ListRef<DeveloperConnectConnection_BitbucketCloudConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_cloud_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_BitbucketDataCenterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_data_center_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> terrars::ListRef<DeveloperConnectConnection_CryptoKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crypto_key_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GithubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GithubEnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_enterprise_config` after provisioning.\n"]
    pub fn gitlab_enterprise_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GitlabEnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> terrars::ListRef<DeveloperConnectConnection_HttpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectConnection_TimeoutsRef {
        DeveloperConnectConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DeveloperConnectConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DeveloperConnectConnection { }

impl terrars::ToListMappable for DeveloperConnectConnection {
    type O = terrars::ListRef<DeveloperConnectConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DeveloperConnectConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_developer_connect_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDeveloperConnectConnection {
    pub tf_id: std::string::String,
    #[doc =
        "Required. Id of the requesting object\nIf auto-generating Id server-side, remove this field and\nconnection_id from the method_signature of Create RPC"]
    pub connection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> DeveloperConnectConnection {
        let out = DeveloperConnectConnection(std::rc::Rc::new(DeveloperConnectConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DeveloperConnectConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                connection_id: self.connection_id,
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                bitbucket_cloud_config: core::default::Default::default(),
                bitbucket_data_center_config: core::default::Default::default(),
                crypto_key_config: core::default::Default::default(),
                github_config: core::default::Default::default(),
                github_enterprise_config: core::default::Default::default(),
                gitlab_config: core::default::Default::default(),
                gitlab_enterprise_config: core::default::Default::default(),
                http_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DeveloperConnectConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DeveloperConnectConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Allows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nRequired. Id of the requesting object\nIf auto-generating Id server-side, remove this field and\nconnection_id from the method_signature of Create RPC"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. [Output only] Create timestamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. [Output only] Delete timestamp"]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nOptional. If disabled is set to true, functionality is disabled for this connection.\nRepository based API methods and webhooks processing for repositories in\nthis connection will be disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `installation_state` after provisioning.\nDescribes stage and necessary actions to be taken by the\nuser to complete the installation. Used for GitHub and GitHub Enterprise\nbased connections."]
    pub fn installation_state(&self) -> terrars::ListRef<DeveloperConnectConnection_InstallationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.installation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the connection, in the format\n'projects/{project}/locations/{location}/connections/{connection_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the\nbackground."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A system-assigned unique identifier for a the GitRepositoryLink."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. [Output only] Update timestamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(&self) -> terrars::ListRef<DeveloperConnectConnection_BitbucketCloudConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_cloud_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_BitbucketDataCenterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_data_center_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> terrars::ListRef<DeveloperConnectConnection_CryptoKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crypto_key_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GithubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GithubEnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_enterprise_config` after provisioning.\n"]
    pub fn gitlab_enterprise_config(&self) -> terrars::ListRef<DeveloperConnectConnection_GitlabEnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> terrars::ListRef<DeveloperConnectConnection_HttpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectConnection_TimeoutsRef {
        DeveloperConnectConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnectionDynamic {
    bitbucket_cloud_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_BitbucketCloudConfig>>,
    bitbucket_data_center_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_BitbucketDataCenterConfig>>,
    crypto_key_config: std::option::Option<terrars::DynamicBlock<DeveloperConnectConnection_CryptoKeyConfig>>,
    github_config: std::option::Option<terrars::DynamicBlock<DeveloperConnectConnection_GithubConfig>>,
    github_enterprise_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GithubEnterpriseConfig>>,
    gitlab_config: std::option::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig>>,
    gitlab_enterprise_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabEnterpriseConfig>>,
    http_config: std::option::Option<terrars::DynamicBlock<DeveloperConnectConnection_HttpConfig>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_InstallationState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectConnection_InstallationState {
    #[doc = "Set the field `action_uri`.\n"]
    pub fn set_action_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_InstallationState {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_InstallationState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_InstallationState {}

impl BuildDeveloperConnectConnection_InstallationState {
    pub fn build(self) -> DeveloperConnectConnection_InstallationState {
        DeveloperConnectConnection_InstallationState {
            action_uri: core::default::Default::default(),
            message: core::default::Default::default(),
            stage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_InstallationStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_InstallationStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_InstallationStateRef {
        DeveloperConnectConnection_InstallationStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_InstallationStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_uri` after provisioning.\n"]
    pub fn action_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GitlabConfig_AuthorizerCredential {
    user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectConnection_GitlabConfig_AuthorizerCredential { }

impl terrars::ToListMappable for DeveloperConnectConnection_GitlabConfig_AuthorizerCredential {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GitlabConfig_AuthorizerCredential {
    #[doc =
        "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GitlabConfig_AuthorizerCredential {
    pub fn build(self) -> DeveloperConnectConnection_GitlabConfig_AuthorizerCredential {
        DeveloperConnectConnection_GitlabConfig_AuthorizerCredential {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef {
        DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_token_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential {
    user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential { }

impl terrars::ToListMappable for DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential {
    #[doc =
        "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential {
    pub fn build(self) -> DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential {
        DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef {
        DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_token_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnection_BitbucketCloudConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_BitbucketCloudConfig {
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    workspace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
    dynamic: DeveloperConnectConnection_BitbucketCloudConfigDynamic,
}

impl DeveloperConnectConnection_BitbucketCloudConfig {
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `read_authorizer_credential`.\n"]
    pub fn set_read_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.read_authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.read_authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_BitbucketCloudConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_BitbucketCloudConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_BitbucketCloudConfig {
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate and create webhooks."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[doc = "Required. The Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub workspace: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_BitbucketCloudConfig {
    pub fn build(self) -> DeveloperConnectConnection_BitbucketCloudConfig {
        DeveloperConnectConnection_BitbucketCloudConfig {
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            workspace: self.workspace,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_BitbucketCloudConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_BitbucketCloudConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_BitbucketCloudConfigRef {
        DeveloperConnectConnection_BitbucketCloudConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_BitbucketCloudConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate and create webhooks."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workspace` after provisioning.\nRequired. The Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub fn workspace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig { }

impl terrars::ToListMappable for DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig {
    #[doc =
        "Required. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig {
    pub fn build(self) -> DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig {
        DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef {
        DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnection_BitbucketDataCenterConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_BitbucketDataCenterConfig {
    host_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>>,
    dynamic: DeveloperConnectConnection_BitbucketDataCenterConfigDynamic,
}

impl DeveloperConnectConnection_BitbucketDataCenterConfig {
    #[doc =
        "Set the field `ssl_ca_certificate`.\nOptional. SSL certificate authority to trust when making requests to Bitbucket Data\nCenter."]
    pub fn set_ssl_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `read_authorizer_credential`.\n"]
    pub fn set_read_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.read_authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.read_authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_directory_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_BitbucketDataCenterConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_BitbucketDataCenterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_BitbucketDataCenterConfig {
    #[doc = "Required. The URI of the Bitbucket Data Center host this connection is for."]
    pub host_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate webhooks."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_BitbucketDataCenterConfig {
    pub fn build(self) -> DeveloperConnectConnection_BitbucketDataCenterConfig {
        DeveloperConnectConnection_BitbucketDataCenterConfig {
            host_uri: self.host_uri,
            ssl_ca_certificate: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_BitbucketDataCenterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_BitbucketDataCenterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_BitbucketDataCenterConfigRef {
        DeveloperConnectConnection_BitbucketDataCenterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_BitbucketDataCenterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the Bitbucket Data Center host this connection is for."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the Bitbucket Data Center server running on the 'host_uri'."]
    pub fn server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nOptional. SSL certificate authority to trust when making requests to Bitbucket Data\nCenter."]
    pub fn ssl_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate webhooks."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_CryptoKeyConfig {
    key_reference: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectConnection_CryptoKeyConfig { }

impl terrars::ToListMappable for DeveloperConnectConnection_CryptoKeyConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_CryptoKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_CryptoKeyConfig {
    #[doc =
        "Required. The name of the key which is used to encrypt/decrypt customer data. For key\nin Cloud KMS, the key should be in the format of\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub key_reference: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_CryptoKeyConfig {
    pub fn build(self) -> DeveloperConnectConnection_CryptoKeyConfig {
        DeveloperConnectConnection_CryptoKeyConfig { key_reference: self.key_reference }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_CryptoKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_CryptoKeyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectConnection_CryptoKeyConfigRef {
        DeveloperConnectConnection_CryptoKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_CryptoKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_reference` after provisioning.\nRequired. The name of the key which is used to encrypt/decrypt customer data. For key\nin Cloud KMS, the key should be in the format of\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub fn key_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GithubConfig_AuthorizerCredential {
    oauth_token_secret_version: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectConnection_GithubConfig_AuthorizerCredential { }

impl terrars::ToListMappable for DeveloperConnectConnection_GithubConfig_AuthorizerCredential {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GithubConfig_AuthorizerCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GithubConfig_AuthorizerCredential {
    #[doc =
        "Required. A SecretManager resource containing the OAuth token that authorizes\nthe connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub oauth_token_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GithubConfig_AuthorizerCredential {
    pub fn build(self) -> DeveloperConnectConnection_GithubConfig_AuthorizerCredential {
        DeveloperConnectConnection_GithubConfig_AuthorizerCredential {
            oauth_token_secret_version: self.oauth_token_secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GithubConfig_AuthorizerCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GithubConfig_AuthorizerCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_GithubConfig_AuthorizerCredentialRef {
        DeveloperConnectConnection_GithubConfig_AuthorizerCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GithubConfig_AuthorizerCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `oauth_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the OAuth token that authorizes\nthe connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn oauth_token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_token_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnection_GithubConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GithubConfig_AuthorizerCredential>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GithubConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_installation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    github_app: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GithubConfig_AuthorizerCredential>>,
    dynamic: DeveloperConnectConnection_GithubConfigDynamic,
}

impl DeveloperConnectConnection_GithubConfig {
    #[doc = "Set the field `app_installation_id`.\nOptional. GitHub App installation id."]
    pub fn set_app_installation_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.app_installation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_GithubConfig_AuthorizerCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_GithubConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GithubConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GithubConfig {
    #[doc =
        "Required. Immutable. The GitHub Application that was installed to the GitHub user or\norganization.\nPossible values:\nGIT_HUB_APP_UNSPECIFIED\nDEVELOPER_CONNECT\nFIREBASE"]
    pub github_app: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GithubConfig {
    pub fn build(self) -> DeveloperConnectConnection_GithubConfig {
        DeveloperConnectConnection_GithubConfig {
            app_installation_id: core::default::Default::default(),
            github_app: self.github_app,
            authorizer_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GithubConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GithubConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectConnection_GithubConfigRef {
        DeveloperConnectConnection_GithubConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GithubConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_installation_id` after provisioning.\nOptional. GitHub App installation id."]
    pub fn app_installation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_installation_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `github_app` after provisioning.\nRequired. Immutable. The GitHub Application that was installed to the GitHub user or\norganization.\nPossible values:\nGIT_HUB_APP_UNSPECIFIED\nDEVELOPER_CONNECT\nFIREBASE"]
    pub fn github_app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.github_app", self.base))
    }

    #[doc =
        "Get a reference to the value of field `installation_uri` after provisioning.\nOutput only. The URI to navigate to in order to manage the installation associated\nwith this GitHubConfig."]
    pub fn installation_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.installation_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GithubConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnection_GithubEnterpriseConfigDynamic {
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GithubEnterpriseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_installation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    host_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook_secret_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>>,
    dynamic: DeveloperConnectConnection_GithubEnterpriseConfigDynamic,
}

impl DeveloperConnectConnection_GithubEnterpriseConfig {
    #[doc = "Set the field `app_id`.\nOptional. ID of the GitHub App created from the manifest."]
    pub fn set_app_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_installation_id`.\nOptional. ID of the installation of the GitHub App."]
    pub fn set_app_installation_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.app_installation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_key_secret_version`.\nOptional. SecretManager resource containing the private key of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn set_private_key_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_key_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_ca_certificate`.\nOptional. SSL certificate to use for requests to GitHub Enterprise."]
    pub fn set_ssl_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webhook_secret_secret_version`.\nOptional. SecretManager resource containing the webhook secret of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn set_webhook_secret_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.webhook_secret_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_directory_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_GithubEnterpriseConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GithubEnterpriseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GithubEnterpriseConfig {
    #[doc = "Required. The URI of the GitHub Enterprise host this connection is for."]
    pub host_uri: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GithubEnterpriseConfig {
    pub fn build(self) -> DeveloperConnectConnection_GithubEnterpriseConfig {
        DeveloperConnectConnection_GithubEnterpriseConfig {
            app_id: core::default::Default::default(),
            app_installation_id: core::default::Default::default(),
            host_uri: self.host_uri,
            private_key_secret_version: core::default::Default::default(),
            ssl_ca_certificate: core::default::Default::default(),
            webhook_secret_secret_version: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GithubEnterpriseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GithubEnterpriseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_GithubEnterpriseConfigRef {
        DeveloperConnectConnection_GithubEnterpriseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GithubEnterpriseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nOptional. ID of the GitHub App created from the manifest."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `app_installation_id` after provisioning.\nOptional. ID of the installation of the GitHub App."]
    pub fn app_installation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_installation_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `app_slug` after provisioning.\nOutput only. The URL-friendly name of the GitHub App."]
    pub fn app_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the GitHub Enterprise host this connection is for."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `installation_uri` after provisioning.\nOutput only. The URI to navigate to in order to manage the installation associated\nwith this GitHubEnterpriseConfig."]
    pub fn installation_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.installation_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_secret_version` after provisioning.\nOptional. SecretManager resource containing the private key of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn private_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_version` after provisioning.\nOutput only. GitHub Enterprise version installed at the host_uri."]
    pub fn server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nOptional. SSL certificate to use for requests to GitHub Enterprise."]
    pub fn ssl_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nOptional. SecretManager resource containing the webhook secret of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnection_GitlabConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GitlabConfig {
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
    dynamic: DeveloperConnectConnection_GitlabConfigDynamic,
}

impl DeveloperConnectConnection_GitlabConfig {
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `read_authorizer_credential`.\n"]
    pub fn set_read_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.read_authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.read_authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_GitlabConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GitlabConfig {
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GitlabConfig {
    pub fn build(self) -> DeveloperConnectConnection_GitlabConfig {
        DeveloperConnectConnection_GitlabConfig {
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GitlabConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GitlabConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectConnection_GitlabConfigRef {
        DeveloperConnectConnection_GitlabConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GitlabConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnection_GitlabEnterpriseConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_GitlabEnterpriseConfig {
    host_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>>,
    dynamic: DeveloperConnectConnection_GitlabEnterpriseConfigDynamic,
}

impl DeveloperConnectConnection_GitlabEnterpriseConfig {
    #[doc =
        "Set the field `ssl_ca_certificate`.\nOptional. SSL Certificate Authority certificate to use for requests to GitLab\nEnterprise instance."]
    pub fn set_ssl_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_AuthorizerCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `read_authorizer_credential`.\n"]
    pub fn set_read_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredential>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.read_authorizer_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.read_authorizer_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_directory_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_GitlabEnterpriseConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_GitlabEnterpriseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_GitlabEnterpriseConfig {
    #[doc = "Required. The URI of the GitLab Enterprise host this connection is for."]
    pub host_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_GitlabEnterpriseConfig {
    pub fn build(self) -> DeveloperConnectConnection_GitlabEnterpriseConfig {
        DeveloperConnectConnection_GitlabEnterpriseConfig {
            host_uri: self.host_uri,
            ssl_ca_certificate: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_GitlabEnterpriseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_GitlabEnterpriseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_GitlabEnterpriseConfigRef {
        DeveloperConnectConnection_GitlabEnterpriseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_GitlabEnterpriseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the GitLab Enterprise host this connection is for."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the GitLab Enterprise server running on the 'host_uri'."]
    pub fn server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nOptional. SSL Certificate Authority certificate to use for requests to GitLab\nEnterprise instance."]
    pub fn ssl_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GitlabConfig_ReadAuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_GithubEnterpriseConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_HttpConfig_BasicAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectConnection_HttpConfig_BasicAuthentication {
    #[doc = "Set the field `password_secret_version`.\nThe password SecretManager secret version to authenticate as."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_HttpConfig_BasicAuthentication {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig_BasicAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_HttpConfig_BasicAuthentication {
    #[doc = "The username to authenticate as."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_HttpConfig_BasicAuthentication {
    pub fn build(self) -> DeveloperConnectConnection_HttpConfig_BasicAuthentication {
        DeveloperConnectConnection_HttpConfig_BasicAuthentication {
            password_secret_version: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_HttpConfig_BasicAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_HttpConfig_BasicAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_HttpConfig_BasicAuthenticationRef {
        DeveloperConnectConnection_HttpConfig_BasicAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_HttpConfig_BasicAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nThe password SecretManager secret version to authenticate as."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username to authenticate as."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication {
    #[doc = "Set the field `token_secret_version`.\nThe token SecretManager secret version to authenticate as."]
    pub fn set_token_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.token_secret_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_HttpConfig_BearerTokenAuthentication {}

impl BuildDeveloperConnectConnection_HttpConfig_BearerTokenAuthentication {
    pub fn build(self) -> DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication {
        DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication {
            token_secret_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_HttpConfig_BearerTokenAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_HttpConfig_BearerTokenAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_HttpConfig_BearerTokenAuthenticationRef {
        DeveloperConnectConnection_HttpConfig_BearerTokenAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_HttpConfig_BearerTokenAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `token_secret_version` after provisioning.\nThe token SecretManager secret version to authenticate as."]
    pub fn token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig { }

impl terrars::ToListMappable for DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig {
    #[doc =
        "The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig {
    pub fn build(self) -> DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig {
        DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfigRef {
        DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectConnection_HttpConfigDynamic {
    basic_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_HttpConfig_BasicAuthentication>>,
    bearer_token_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_HttpConfig {
    host_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_authentication: std::option::Option<std::vec::Vec<DeveloperConnectConnection_HttpConfig_BasicAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_authentication: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig>>,
    dynamic: DeveloperConnectConnection_HttpConfigDynamic,
}

impl DeveloperConnectConnection_HttpConfig {
    #[doc =
        "Set the field `ssl_ca_certificate`.\nThe SSL certificate to use for requests to the HTTP service provider."]
    pub fn set_ssl_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_authentication`.\n"]
    pub fn set_basic_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig_BasicAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bearer_token_authentication`.\n"]
    pub fn set_bearer_token_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig_BearerTokenAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bearer_token_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bearer_token_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_directory_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectConnection_HttpConfig {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_HttpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_HttpConfig {
    #[doc = "The service provider's https endpoint."]
    pub host_uri: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectConnection_HttpConfig {
    pub fn build(self) -> DeveloperConnectConnection_HttpConfig {
        DeveloperConnectConnection_HttpConfig {
            host_uri: self.host_uri,
            ssl_ca_certificate: core::default::Default::default(),
            basic_authentication: core::default::Default::default(),
            bearer_token_authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_HttpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_HttpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectConnection_HttpConfigRef {
        DeveloperConnectConnection_HttpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_HttpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nThe service provider's https endpoint."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nThe SSL certificate to use for requests to the HTTP service provider."]
    pub fn ssl_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_authentication` after provisioning.\n"]
    pub fn basic_authentication(&self) -> terrars::ListRef<DeveloperConnectConnection_HttpConfig_BasicAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_authentication` after provisioning.\n"]
    pub fn bearer_token_authentication(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_HttpConfig_BearerTokenAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<DeveloperConnectConnection_HttpConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectConnection_Timeouts {
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

impl terrars::ToListMappable for DeveloperConnectConnection_Timeouts {
    type O = terrars::BlockAssignable<DeveloperConnectConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectConnection_Timeouts {}

impl BuildDeveloperConnectConnection_Timeouts {
    pub fn build(self) -> DeveloperConnectConnection_Timeouts {
        DeveloperConnectConnection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectConnection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectConnection_TimeoutsRef {
        DeveloperConnectConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectConnection_TimeoutsRef {
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
