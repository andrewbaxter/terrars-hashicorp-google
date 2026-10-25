#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct Cloudbuildv2ConnectionData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_cloud_config: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_BitbucketCloudConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_data_center_config: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_BitbucketDataCenterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_config: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_GithubConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_config: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_GithubEnterpriseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab_config: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Cloudbuildv2Connection_Timeouts>,
    dynamic: Cloudbuildv2ConnectionDynamic,
}

struct Cloudbuildv2Connection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Cloudbuildv2ConnectionData_>,
}

#[derive(Clone)]
pub struct Cloudbuildv2Connection(std::rc::Rc<Cloudbuildv2Connection_>);

impl Cloudbuildv2Connection {
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
        "Set the field `annotations`.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
        "Set the field `disabled`.\nIf disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `bitbucket_cloud_config`.\n"]
    pub fn set_bitbucket_cloud_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2Connection_BitbucketCloudConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2Connection_BitbucketDataCenterConfig>>,
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

    #[doc = "Set the field `github_config`.\n"]
    pub fn set_github_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GithubConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GithubEnterpriseConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Cloudbuildv2Connection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `installation_state` after provisioning.\nOutput only. Installation state of the Connection."]
    pub fn installation_state(&self) -> terrars::ListRef<Cloudbuildv2Connection_InstallationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.installation_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the connection."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the background."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_BitbucketCloudConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_cloud_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_BitbucketDataCenterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_data_center_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_GithubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_GithubEnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudbuildv2Connection_TimeoutsRef {
        Cloudbuildv2Connection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Cloudbuildv2Connection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Cloudbuildv2Connection { }

impl terrars::ToListMappable for Cloudbuildv2Connection {
    type O = terrars::ListRef<Cloudbuildv2ConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Cloudbuildv2Connection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloudbuildv2_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudbuildv2Connection {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Immutable. The resource name of the connection."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection {
    pub fn build(self, stack: &mut terrars::Stack) -> Cloudbuildv2Connection {
        let out = Cloudbuildv2Connection(std::rc::Rc::new(Cloudbuildv2Connection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Cloudbuildv2ConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                bitbucket_cloud_config: core::default::Default::default(),
                bitbucket_data_center_config: core::default::Default::default(),
                github_config: core::default::Default::default(),
                github_enterprise_config: core::default::Default::default(),
                gitlab_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Cloudbuildv2ConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Cloudbuildv2ConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `installation_state` after provisioning.\nOutput only. Installation state of the Connection."]
    pub fn installation_state(&self) -> terrars::ListRef<Cloudbuildv2Connection_InstallationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.installation_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the connection."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the background."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_BitbucketCloudConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_cloud_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_BitbucketDataCenterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket_data_center_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_GithubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_GithubEnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudbuildv2Connection_TimeoutsRef {
        Cloudbuildv2Connection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Cloudbuildv2ConnectionDynamic {
    bitbucket_cloud_config: std::option::Option<terrars::DynamicBlock<Cloudbuildv2Connection_BitbucketCloudConfig>>,
    bitbucket_data_center_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_BitbucketDataCenterConfig>>,
    github_config: std::option::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GithubConfig>>,
    github_enterprise_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GithubEnterpriseConfig>>,
    gitlab_config: std::option::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_InstallationState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudbuildv2Connection_InstallationState {
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

impl terrars::ToListMappable for Cloudbuildv2Connection_InstallationState {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_InstallationState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_InstallationState {}

impl BuildCloudbuildv2Connection_InstallationState {
    pub fn build(self) -> Cloudbuildv2Connection_InstallationState {
        Cloudbuildv2Connection_InstallationState {
            action_uri: core::default::Default::default(),
            message: core::default::Default::default(),
            stage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_InstallationStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_InstallationStateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2Connection_InstallationStateRef {
        Cloudbuildv2Connection_InstallationStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_InstallationStateRef {
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
pub struct Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential {
    user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential { }

impl terrars::ToListMappable for Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_GitlabConfig_AuthorizerCredential {
    #[doc =
        "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection_GitlabConfig_AuthorizerCredential {
    pub fn build(self) -> Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential {
        Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef {
        Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_token_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential {
    user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential { }

impl terrars::ToListMappable for Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential {
    #[doc =
        "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential {
    pub fn build(self) -> Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential {
        Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef {
        Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_token_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudbuildv2Connection_BitbucketCloudConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_BitbucketCloudConfig {
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    workspace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
    dynamic: Cloudbuildv2Connection_BitbucketCloudConfigDynamic,
}

impl Cloudbuildv2Connection_BitbucketCloudConfig {
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
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

impl terrars::ToListMappable for Cloudbuildv2Connection_BitbucketCloudConfig {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_BitbucketCloudConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_BitbucketCloudConfig {
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[doc = "The Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub workspace: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection_BitbucketCloudConfig {
    pub fn build(self) -> Cloudbuildv2Connection_BitbucketCloudConfig {
        Cloudbuildv2Connection_BitbucketCloudConfig {
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            workspace: self.workspace,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_BitbucketCloudConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_BitbucketCloudConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2Connection_BitbucketCloudConfigRef {
        Cloudbuildv2Connection_BitbucketCloudConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_BitbucketCloudConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workspace` after provisioning.\nThe Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub fn workspace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(&self) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig { }

impl terrars::ToListMappable for Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig {
    #[doc =
        "Required. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig {
    pub fn build(self) -> Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig {
        Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef {
        Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudbuildv2Connection_BitbucketDataCenterConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_BitbucketDataCenterConfig {
    host_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
    dynamic: Cloudbuildv2Connection_BitbucketDataCenterConfigDynamic,
}

impl Cloudbuildv2Connection_BitbucketDataCenterConfig {
    #[doc = "Set the field `ssl_ca`.\nSSL certificate to use for requests to the Bitbucket Data Center."]
    pub fn set_ssl_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
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

impl terrars::ToListMappable for Cloudbuildv2Connection_BitbucketDataCenterConfig {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_BitbucketDataCenterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_BitbucketDataCenterConfig {
    #[doc = "The URI of the Bitbucket Data Center host this connection is for."]
    pub host_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection_BitbucketDataCenterConfig {
    pub fn build(self) -> Cloudbuildv2Connection_BitbucketDataCenterConfig {
        Cloudbuildv2Connection_BitbucketDataCenterConfig {
            host_uri: self.host_uri,
            ssl_ca: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_BitbucketDataCenterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_BitbucketDataCenterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2Connection_BitbucketDataCenterConfigRef {
        Cloudbuildv2Connection_BitbucketDataCenterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_BitbucketDataCenterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nThe URI of the Bitbucket Data Center host this connection is for."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the Bitbucket Data Center running on the 'host_uri'."]
    pub fn server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to the Bitbucket Data Center."]
    pub fn ssl_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(&self) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_GithubConfig_AuthorizerCredential {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_token_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudbuildv2Connection_GithubConfig_AuthorizerCredential {
    #[doc =
        "Set the field `oauth_token_secret_version`.\nA SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn set_oauth_token_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth_token_secret_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Cloudbuildv2Connection_GithubConfig_AuthorizerCredential {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_GithubConfig_AuthorizerCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_GithubConfig_AuthorizerCredential {}

impl BuildCloudbuildv2Connection_GithubConfig_AuthorizerCredential {
    pub fn build(self) -> Cloudbuildv2Connection_GithubConfig_AuthorizerCredential {
        Cloudbuildv2Connection_GithubConfig_AuthorizerCredential {
            oauth_token_secret_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_GithubConfig_AuthorizerCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_GithubConfig_AuthorizerCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2Connection_GithubConfig_AuthorizerCredentialRef {
        Cloudbuildv2Connection_GithubConfig_AuthorizerCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_GithubConfig_AuthorizerCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `oauth_token_secret_version` after provisioning.\nA SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn oauth_token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_token_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudbuildv2Connection_GithubConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GithubConfig_AuthorizerCredential>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_GithubConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_installation_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_GithubConfig_AuthorizerCredential>>,
    dynamic: Cloudbuildv2Connection_GithubConfigDynamic,
}

impl Cloudbuildv2Connection_GithubConfig {
    #[doc = "Set the field `app_installation_id`.\nGitHub App installation id."]
    pub fn set_app_installation_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GithubConfig_AuthorizerCredential>>,
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

impl terrars::ToListMappable for Cloudbuildv2Connection_GithubConfig {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_GithubConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_GithubConfig {}

impl BuildCloudbuildv2Connection_GithubConfig {
    pub fn build(self) -> Cloudbuildv2Connection_GithubConfig {
        Cloudbuildv2Connection_GithubConfig {
            app_installation_id: core::default::Default::default(),
            authorizer_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_GithubConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_GithubConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2Connection_GithubConfigRef {
        Cloudbuildv2Connection_GithubConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_GithubConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_installation_id` after provisioning.\nGitHub App installation id."]
    pub fn app_installation_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_installation_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(&self) -> terrars::ListRef<Cloudbuildv2Connection_GithubConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudbuildv2Connection_GithubEnterpriseConfigDynamic {
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_GithubEnterpriseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_installation_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    host_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook_secret_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
    dynamic: Cloudbuildv2Connection_GithubEnterpriseConfigDynamic,
}

impl Cloudbuildv2Connection_GithubEnterpriseConfig {
    #[doc = "Set the field `app_id`.\nId of the GitHub App created from the manifest."]
    pub fn set_app_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.app_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_installation_id`.\nID of the installation of the GitHub App."]
    pub fn set_app_installation_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.app_installation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_slug`.\nThe URL-friendly name of the GitHub App."]
    pub fn set_app_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_key_secret_version`.\nSecretManager resource containing the private key of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn set_private_key_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_key_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_ca`.\nSSL certificate to use for requests to GitHub Enterprise."]
    pub fn set_ssl_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webhook_secret_secret_version`.\nSecretManager resource containing the webhook secret of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
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

impl terrars::ToListMappable for Cloudbuildv2Connection_GithubEnterpriseConfig {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_GithubEnterpriseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_GithubEnterpriseConfig {
    #[doc = "Required. The URI of the GitHub Enterprise host this connection is for."]
    pub host_uri: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection_GithubEnterpriseConfig {
    pub fn build(self) -> Cloudbuildv2Connection_GithubEnterpriseConfig {
        Cloudbuildv2Connection_GithubEnterpriseConfig {
            app_id: core::default::Default::default(),
            app_installation_id: core::default::Default::default(),
            app_slug: core::default::Default::default(),
            host_uri: self.host_uri,
            private_key_secret_version: core::default::Default::default(),
            ssl_ca: core::default::Default::default(),
            webhook_secret_secret_version: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_GithubEnterpriseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_GithubEnterpriseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2Connection_GithubEnterpriseConfigRef {
        Cloudbuildv2Connection_GithubEnterpriseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_GithubEnterpriseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nId of the GitHub App created from the manifest."]
    pub fn app_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `app_installation_id` after provisioning.\nID of the installation of the GitHub App."]
    pub fn app_installation_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_installation_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `app_slug` after provisioning.\nThe URL-friendly name of the GitHub App."]
    pub fn app_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the GitHub Enterprise host this connection is for."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_secret_version` after provisioning.\nSecretManager resource containing the private key of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn private_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to GitHub Enterprise."]
    pub fn ssl_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nSecretManager resource containing the webhook secret of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudbuildv2Connection_GitlabConfigDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_GitlabConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std::option::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
    dynamic: Cloudbuildv2Connection_GitlabConfigDynamic,
}

impl Cloudbuildv2Connection_GitlabConfig {
    #[doc =
        "Set the field `host_uri`.\nThe URI of the GitLab Enterprise host this connection is for. If not specified, the default value is https://gitlab.com."]
    pub fn set_host_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_ca`.\nSSL certificate to use for requests to GitLab Enterprise."]
    pub fn set_ssl_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredential>>,
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredential>>,
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfig>>,
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

impl terrars::ToListMappable for Cloudbuildv2Connection_GitlabConfig {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_GitlabConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_GitlabConfig {
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as 'projects/*/secrets/*/versions/*'."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection_GitlabConfig {
    pub fn build(self) -> Cloudbuildv2Connection_GitlabConfig {
        Cloudbuildv2Connection_GitlabConfig {
            host_uri: core::default::Default::default(),
            ssl_ca: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_GitlabConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_GitlabConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2Connection_GitlabConfigRef {
        Cloudbuildv2Connection_GitlabConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_GitlabConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nThe URI of the GitLab Enterprise host this connection is for. If not specified, the default value is https://gitlab.com."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the GitLab Enterprise server running on the 'host_uri'."]
    pub fn server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to GitLab Enterprise."]
    pub fn ssl_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(&self) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_AuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_ReadAuthorizerCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2Connection_GitlabConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2Connection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudbuildv2Connection_Timeouts {
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

impl terrars::ToListMappable for Cloudbuildv2Connection_Timeouts {
    type O = terrars::BlockAssignable<Cloudbuildv2Connection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2Connection_Timeouts {}

impl BuildCloudbuildv2Connection_Timeouts {
    pub fn build(self) -> Cloudbuildv2Connection_Timeouts {
        Cloudbuildv2Connection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudbuildv2Connection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2Connection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2Connection_TimeoutsRef {
        Cloudbuildv2Connection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2Connection_TimeoutsRef {
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
