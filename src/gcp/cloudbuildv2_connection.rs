use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct Cloudbuildv2ConnectionData {
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
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_config: std::option::Option<std::vec::Vec<Cloudbuildv2ConnectionGithubConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_config: std::option::Option<std::vec::Vec<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab_config: std::option::Option<std::vec::Vec<Cloudbuildv2ConnectionGitlabConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Cloudbuildv2ConnectionTimeoutsEl>,
    dynamic: Cloudbuildv2ConnectionDynamic,
}

struct Cloudbuildv2Connection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Cloudbuildv2ConnectionData>,
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
        "Set the field `annotations`.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_config`.\n"]
    pub fn set_github_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2ConnectionGithubConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<Cloudbuildv2ConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
    pub fn installation_state(&self) -> terrars::ListRef<Cloudbuildv2ConnectionInstallationStateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.installation_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the connection, in the format `projects/{project}/locations/{location}/connections/{connection_id}`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
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

    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> terrars::ListRef<Cloudbuildv2ConnectionGithubConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::ListRef<Cloudbuildv2ConnectionGithubEnterpriseConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> terrars::ListRef<Cloudbuildv2ConnectionGitlabConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudbuildv2ConnectionTimeoutsElRef {
        Cloudbuildv2ConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
    #[doc =
        "Immutable. The resource name of the connection, in the format `projects/{project}/locations/{location}/connections/{connection_id}`."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2Connection {
    pub fn build(self, stack: &mut terrars::Stack) -> Cloudbuildv2Connection {
        let out = Cloudbuildv2Connection(std::rc::Rc::new(Cloudbuildv2Connection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Cloudbuildv2ConnectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
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
        "Get a reference to the value of field `annotations` after provisioning.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
    pub fn installation_state(&self) -> terrars::ListRef<Cloudbuildv2ConnectionInstallationStateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.installation_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the connection, in the format `projects/{project}/locations/{location}/connections/{connection_id}`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
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

    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> terrars::ListRef<Cloudbuildv2ConnectionGithubConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::ListRef<Cloudbuildv2ConnectionGithubEnterpriseConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> terrars::ListRef<Cloudbuildv2ConnectionGitlabConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudbuildv2ConnectionTimeoutsElRef {
        Cloudbuildv2ConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionInstallationStateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudbuildv2ConnectionInstallationStateEl {
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

impl terrars::ToListMappable for Cloudbuildv2ConnectionInstallationStateEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionInstallationStateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionInstallationStateEl {}

impl BuildCloudbuildv2ConnectionInstallationStateEl {
    pub fn build(self) -> Cloudbuildv2ConnectionInstallationStateEl {
        Cloudbuildv2ConnectionInstallationStateEl {
            action_uri: core::default::Default::default(),
            message: core::default::Default::default(),
            stage: core::default::Default::default(),
        }
    }
}

pub struct Cloudbuildv2ConnectionInstallationStateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionInstallationStateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2ConnectionInstallationStateElRef {
        Cloudbuildv2ConnectionInstallationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionInstallationStateElRef {
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
pub struct Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_token_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    #[doc =
        "Set the field `oauth_token_secret_version`.\nA SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`."]
    pub fn set_oauth_token_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth_token_secret_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {}

impl BuildCloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
        Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
            oauth_token_secret_version: core::default::Default::default(),
        }
    }
}

pub struct Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `oauth_token_secret_version` after provisioning.\nA SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`."]
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
struct Cloudbuildv2ConnectionGithubConfigElDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGithubConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_installation_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std::option::Option<std::vec::Vec<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>>,
    dynamic: Cloudbuildv2ConnectionGithubConfigElDynamic,
}

impl Cloudbuildv2ConnectionGithubConfigEl {
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>>,
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

impl terrars::ToListMappable for Cloudbuildv2ConnectionGithubConfigEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGithubConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGithubConfigEl {}

impl BuildCloudbuildv2ConnectionGithubConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubConfigEl {
        Cloudbuildv2ConnectionGithubConfigEl {
            app_installation_id: core::default::Default::default(),
            authorizer_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Cloudbuildv2ConnectionGithubConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGithubConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2ConnectionGithubConfigElRef {
        Cloudbuildv2ConnectionGithubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGithubConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_installation_id` after provisioning.\nGitHub App installation id."]
    pub fn app_installation_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_installation_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    service: terrars::PrimField<std::string::String>,
}

impl Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl { }

impl terrars::ToListMappable for Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    #[doc =
        "Required. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
        Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl { service: self.service }
    }
}

pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
        Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
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
struct Cloudbuildv2ConnectionGithubEnterpriseConfigElDynamic {
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
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
    ::Option<std::vec::Vec<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>>,
    dynamic: Cloudbuildv2ConnectionGithubEnterpriseConfigElDynamic,
}

impl Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
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
        "Set the field `private_key_secret_version`.\nSecretManager resource containing the private key of the GitHub App, formatted as `projects/*/secrets/*/versions/*`."]
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
        "Set the field `webhook_secret_secret_version`.\nSecretManager resource containing the webhook secret of the GitHub App, formatted as `projects/*/secrets/*/versions/*`."]
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
                        ::BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>,
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

impl terrars::ToListMappable for Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGithubEnterpriseConfigEl {
    #[doc = "Required. The URI of the GitHub Enterprise host this connection is for."]
    pub host_uri: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2ConnectionGithubEnterpriseConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
        Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
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

pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
        Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
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
        "Get a reference to the value of field `private_key_secret_version` after provisioning.\nSecretManager resource containing the private key of the GitHub App, formatted as `projects/*/secrets/*/versions/*`."]
    pub fn private_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to GitHub Enterprise."]
    pub fn ssl_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nSecretManager resource containing the webhook secret of the GitHub App, formatted as `projects/*/secrets/*/versions/*`."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl { }

impl terrars::ToListMappable for Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    #[doc =
        "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`."]
    pub user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
        Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}

pub struct Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`."]
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
pub struct Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl { }

impl terrars::ToListMappable for Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    #[doc =
        "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`."]
    pub user_token_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
        Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}

pub struct Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`."]
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
pub struct Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    service: terrars::PrimField<std::string::String>,
}

impl Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl { }

impl terrars::ToListMappable for Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    #[doc =
        "Required. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
        Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl { service: self.service }
    }
}

pub struct Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
        Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
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
struct Cloudbuildv2ConnectionGitlabConfigElDynamic {
    authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>>,
    read_authorizer_credential: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>>,
}

#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGitlabConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    webhook_secret_secret_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_credential: std::option::Option<std::vec::Vec<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_authorizer_credential: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>>,
    dynamic: Cloudbuildv2ConnectionGitlabConfigElDynamic,
}

impl Cloudbuildv2ConnectionGitlabConfigEl {
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>>,
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
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
                    ::Into<terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>>,
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

impl terrars::ToListMappable for Cloudbuildv2ConnectionGitlabConfigEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionGitlabConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionGitlabConfigEl {
    #[doc =
        "Required. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as `projects/*/secrets/*/versions/*`."]
    pub webhook_secret_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildv2ConnectionGitlabConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigEl {
        Cloudbuildv2ConnectionGitlabConfigEl {
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

pub struct Cloudbuildv2ConnectionGitlabConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionGitlabConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2ConnectionGitlabConfigElRef {
        Cloudbuildv2ConnectionGitlabConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionGitlabConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nThe URI of the GitLab Enterprise host this connection is for. If not specified, the default value is https://gitlab.com."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the GitLab Enterprise server running on the `host_uri`."]
    pub fn server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to GitLab Enterprise."]
    pub fn ssl_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as `projects/*/secrets/*/versions/*`."]
    pub fn webhook_secret_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_secret_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_authorizer_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudbuildv2ConnectionTimeoutsEl {
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

impl terrars::ToListMappable for Cloudbuildv2ConnectionTimeoutsEl {
    type O = terrars::BlockAssignable<Cloudbuildv2ConnectionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildv2ConnectionTimeoutsEl {}

impl BuildCloudbuildv2ConnectionTimeoutsEl {
    pub fn build(self) -> Cloudbuildv2ConnectionTimeoutsEl {
        Cloudbuildv2ConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Cloudbuildv2ConnectionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudbuildv2ConnectionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudbuildv2ConnectionTimeoutsElRef {
        Cloudbuildv2ConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudbuildv2ConnectionTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct Cloudbuildv2ConnectionDynamic {
    github_config: std::option::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGithubConfigEl>>,
    github_enterprise_config: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>>,
    gitlab_config: std::option::Option<terrars::DynamicBlock<Cloudbuildv2ConnectionGitlabConfigEl>>,
}
