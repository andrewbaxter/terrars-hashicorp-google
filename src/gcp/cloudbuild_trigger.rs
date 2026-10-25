#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudbuildTriggerData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignored_files: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_build_logs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_files: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitutions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_config: std::option::Option<std::vec::Vec<CloudbuildTrigger_ApprovalConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_server_trigger_config: std::option::Option<std::vec::Vec<CloudbuildTrigger_BitbucketServerTriggerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_connect_event_config: std::option::Option<std::vec::Vec<CloudbuildTrigger_DeveloperConnectEventConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_file_source: std::option::Option<std::vec::Vec<CloudbuildTrigger_GitFileSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<CloudbuildTrigger_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_config: std::option::Option<std::vec::Vec<CloudbuildTrigger_PubsubConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_event_config: std::option::Option<std::vec::Vec<CloudbuildTrigger_RepositoryEventConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_to_build: std::option::Option<std::vec::Vec<CloudbuildTrigger_SourceToBuild>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudbuildTrigger_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_template: std::option::Option<std::vec::Vec<CloudbuildTrigger_TriggerTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook_config: std::option::Option<std::vec::Vec<CloudbuildTrigger_WebhookConfig>>,
    dynamic: CloudbuildTriggerDynamic,
}

struct CloudbuildTrigger_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudbuildTriggerData_>,
}

#[derive(Clone)]
pub struct CloudbuildTrigger(std::rc::Rc<CloudbuildTrigger_>);

impl CloudbuildTrigger {
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

    #[doc = "Set the field `description`.\nHuman-readable description of the trigger."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filename`.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn set_filename(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filename = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignored_files`.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn set_ignored_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ignored_files = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_build_logs`.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn set_include_build_logs(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().include_build_logs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_files`.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn set_included_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().included_files = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the trigger. Must be unique within the project."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `substitutions`.\nSubstitutions data for Build resource."]
    pub fn set_substitutions(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nTags for annotation of a BuildTrigger"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approval_config`.\n"]
    pub fn set_approval_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_ApprovalConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bitbucket_server_trigger_config`.\n"]
    pub fn set_bitbucket_server_trigger_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_BitbucketServerTriggerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bitbucket_server_trigger_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bitbucket_server_trigger_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `build`.\n"]
    pub fn set_build(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `developer_connect_event_config`.\n"]
    pub fn set_developer_connect_event_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_DeveloperConnectEventConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().developer_connect_event_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.developer_connect_event_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git_file_source`.\n"]
    pub fn set_git_file_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_GitFileSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().git_file_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.git_file_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Github>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pubsub_config`.\n"]
    pub fn set_pubsub_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_PubsubConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pubsub_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pubsub_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repository_event_config`.\n"]
    pub fn set_repository_event_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_RepositoryEventConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().repository_event_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.repository_event_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_to_build`.\n"]
    pub fn set_source_to_build(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_SourceToBuild>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_to_build = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_to_build = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudbuildTrigger_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger_template`.\n"]
    pub fn set_trigger_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_TriggerTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `webhook_config`.\n"]
    pub fn set_webhook_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_WebhookConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().webhook_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.webhook_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the trigger."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filename` after provisioning.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignored_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn ignored_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignored_files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_build_logs` after provisioning.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn include_build_logs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_build_logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `included_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn included_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the trigger. Must be unique within the project."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.substitutions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a BuildTrigger"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_id` after provisioning.\nThe unique identifier for the trigger."]
    pub fn trigger_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_config` after provisioning.\n"]
    pub fn approval_config(&self) -> terrars::ListRef<CloudbuildTrigger_ApprovalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\n"]
    pub fn bitbucket_server_trigger_config(&self) -> terrars::ListRef<CloudbuildTrigger_BitbucketServerTriggerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> terrars::ListRef<CloudbuildTrigger_BuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_connect_event_config` after provisioning.\n"]
    pub fn developer_connect_event_config(&self) -> terrars::ListRef<CloudbuildTrigger_DeveloperConnectEventConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `git_file_source` after provisioning.\n"]
    pub fn git_file_source(&self) -> terrars::ListRef<CloudbuildTrigger_GitFileSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_file_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<CloudbuildTrigger_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pubsub_config` after provisioning.\n"]
    pub fn pubsub_config(&self) -> terrars::ListRef<CloudbuildTrigger_PubsubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_event_config` after provisioning.\n"]
    pub fn repository_event_config(&self) -> terrars::ListRef<CloudbuildTrigger_RepositoryEventConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_event_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_to_build` after provisioning.\n"]
    pub fn source_to_build(&self) -> terrars::ListRef<CloudbuildTrigger_SourceToBuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_to_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildTrigger_TimeoutsRef {
        CloudbuildTrigger_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_template` after provisioning.\n"]
    pub fn trigger_template(&self) -> terrars::ListRef<CloudbuildTrigger_TriggerTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webhook_config` after provisioning.\n"]
    pub fn webhook_config(&self) -> terrars::ListRef<CloudbuildTrigger_WebhookConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudbuildTrigger {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudbuildTrigger { }

impl terrars::ToListMappable for CloudbuildTrigger {
    type O = terrars::ListRef<CloudbuildTriggerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudbuildTrigger_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloudbuild_trigger".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudbuildTrigger {
    pub tf_id: std::string::String,
}

impl BuildCloudbuildTrigger {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudbuildTrigger {
        let out = CloudbuildTrigger(std::rc::Rc::new(CloudbuildTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudbuildTriggerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                filename: core::default::Default::default(),
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                ignored_files: core::default::Default::default(),
                include_build_logs: core::default::Default::default(),
                included_files: core::default::Default::default(),
                location: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                substitutions: core::default::Default::default(),
                tags: core::default::Default::default(),
                approval_config: core::default::Default::default(),
                bitbucket_server_trigger_config: core::default::Default::default(),
                build: core::default::Default::default(),
                developer_connect_event_config: core::default::Default::default(),
                git_file_source: core::default::Default::default(),
                github: core::default::Default::default(),
                pubsub_config: core::default::Default::default(),
                repository_event_config: core::default::Default::default(),
                source_to_build: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                trigger_template: core::default::Default::default(),
                webhook_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudbuildTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudbuildTriggerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the trigger."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filename` after provisioning.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignored_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn ignored_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignored_files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_build_logs` after provisioning.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn include_build_logs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_build_logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `included_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn included_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the trigger. Must be unique within the project."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.substitutions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a BuildTrigger"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_id` after provisioning.\nThe unique identifier for the trigger."]
    pub fn trigger_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_config` after provisioning.\n"]
    pub fn approval_config(&self) -> terrars::ListRef<CloudbuildTrigger_ApprovalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\n"]
    pub fn bitbucket_server_trigger_config(&self) -> terrars::ListRef<CloudbuildTrigger_BitbucketServerTriggerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> terrars::ListRef<CloudbuildTrigger_BuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_connect_event_config` after provisioning.\n"]
    pub fn developer_connect_event_config(&self) -> terrars::ListRef<CloudbuildTrigger_DeveloperConnectEventConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `git_file_source` after provisioning.\n"]
    pub fn git_file_source(&self) -> terrars::ListRef<CloudbuildTrigger_GitFileSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_file_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<CloudbuildTrigger_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pubsub_config` after provisioning.\n"]
    pub fn pubsub_config(&self) -> terrars::ListRef<CloudbuildTrigger_PubsubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_event_config` after provisioning.\n"]
    pub fn repository_event_config(&self) -> terrars::ListRef<CloudbuildTrigger_RepositoryEventConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_event_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_to_build` after provisioning.\n"]
    pub fn source_to_build(&self) -> terrars::ListRef<CloudbuildTrigger_SourceToBuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_to_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildTrigger_TimeoutsRef {
        CloudbuildTrigger_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_template` after provisioning.\n"]
    pub fn trigger_template(&self) -> terrars::ListRef<CloudbuildTrigger_TriggerTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webhook_config` after provisioning.\n"]
    pub fn webhook_config(&self) -> terrars::ListRef<CloudbuildTrigger_WebhookConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTriggerDynamic {
    approval_config: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_ApprovalConfig>>,
    bitbucket_server_trigger_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudbuildTrigger_BitbucketServerTriggerConfig>>,
    build: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build>>,
    developer_connect_event_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudbuildTrigger_DeveloperConnectEventConfig>>,
    git_file_source: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_GitFileSource>>,
    github: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Github>>,
    pubsub_config: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_PubsubConfig>>,
    repository_event_config: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_RepositoryEventConfig>>,
    source_to_build: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_SourceToBuild>>,
    trigger_template: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_TriggerTemplate>>,
    webhook_config: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_WebhookConfig>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_ApprovalConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_required: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudbuildTrigger_ApprovalConfig {
    #[doc =
        "Set the field `approval_required`.\nWhether or not approval is needed. If this is set on a build, it will become pending when run,\nand will need to be explicitly approved to start."]
    pub fn set_approval_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.approval_required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_ApprovalConfig {
    type O = terrars::BlockAssignable<CloudbuildTrigger_ApprovalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_ApprovalConfig {}

impl BuildCloudbuildTrigger_ApprovalConfig {
    pub fn build(self) -> CloudbuildTrigger_ApprovalConfig {
        CloudbuildTrigger_ApprovalConfig { approval_required: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_ApprovalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_ApprovalConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_ApprovalConfigRef {
        CloudbuildTrigger_ApprovalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_ApprovalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `approval_required` after provisioning.\nWhether or not approval is needed. If this is set on a build, it will become pending when run,\nand will need to be explicitly approved to start."]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest {
    branch: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest {
    #[doc =
        "Set the field `comment_control`.\nConfigure builds to run whether a repository owner or collaborator need to comment /gcbrun. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest {
    type O = terrars::BlockAssignable<CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest {
    #[doc =
        "Regex of branches to match.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub branch: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest {
    pub fn build(self) -> CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest {
        CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest {
            branch: self.branch,
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequestRef {
        CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `comment_control` after provisioning.\nConfigure builds to run whether a repository owner or collaborator need to comment /gcbrun. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_BitbucketServerTriggerConfig_Push {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_BitbucketServerTriggerConfig_Push {
    #[doc = "Set the field `branch`.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_regex`.\nWhen true, only trigger a build if the revision regex does NOT match the gitRef regex."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_BitbucketServerTriggerConfig_Push {
    type O = terrars::BlockAssignable<CloudbuildTrigger_BitbucketServerTriggerConfig_Push>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_BitbucketServerTriggerConfig_Push {}

impl BuildCloudbuildTrigger_BitbucketServerTriggerConfig_Push {
    pub fn build(self) -> CloudbuildTrigger_BitbucketServerTriggerConfig_Push {
        CloudbuildTrigger_BitbucketServerTriggerConfig_Push {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_BitbucketServerTriggerConfig_PushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_BitbucketServerTriggerConfig_PushRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_BitbucketServerTriggerConfig_PushRef {
        CloudbuildTrigger_BitbucketServerTriggerConfig_PushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_BitbucketServerTriggerConfig_PushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nWhen true, only trigger a build if the revision regex does NOT match the gitRef regex."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_BitbucketServerTriggerConfigDynamic {
    pull_request: std
    ::option
    ::Option<terrars::DynamicBlock<CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest>>,
    push: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_BitbucketServerTriggerConfig_Push>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_BitbucketServerTriggerConfig {
    bitbucket_server_config_resource: terrars::PrimField<std::string::String>,
    project_key: terrars::PrimField<std::string::String>,
    repo_slug: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<std::vec::Vec<CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<std::vec::Vec<CloudbuildTrigger_BitbucketServerTriggerConfig_Push>>,
    dynamic: CloudbuildTrigger_BitbucketServerTriggerConfigDynamic,
}

impl CloudbuildTrigger_BitbucketServerTriggerConfig {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pull_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_BitbucketServerTriggerConfig_Push>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_BitbucketServerTriggerConfig {
    type O = terrars::BlockAssignable<CloudbuildTrigger_BitbucketServerTriggerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_BitbucketServerTriggerConfig {
    #[doc = "The Bitbucket server config resource that this trigger config maps to."]
    pub bitbucket_server_config_resource: terrars::PrimField<std::string::String>,
    #[doc =
        "Key of the project that the repo is in. For example: The key for https://mybitbucket.server/projects/TEST/repos/test-repo is \"TEST\"."]
    pub project_key: terrars::PrimField<std::string::String>,
    #[doc =
        "Slug of the repository. A repository slug is a URL-friendly version of a repository name, automatically generated by Bitbucket for use in the URL.\nFor example, if the repository name is 'test repo', in the URL it would become 'test-repo' as in https://mybitbucket.server/projects/TEST/repos/test-repo."]
    pub repo_slug: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_BitbucketServerTriggerConfig {
    pub fn build(self) -> CloudbuildTrigger_BitbucketServerTriggerConfig {
        CloudbuildTrigger_BitbucketServerTriggerConfig {
            bitbucket_server_config_resource: self.bitbucket_server_config_resource,
            project_key: self.project_key,
            repo_slug: self.repo_slug,
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_BitbucketServerTriggerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_BitbucketServerTriggerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_BitbucketServerTriggerConfigRef {
        CloudbuildTrigger_BitbucketServerTriggerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_BitbucketServerTriggerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bitbucket_server_config_resource` after provisioning.\nThe Bitbucket server config resource that this trigger config maps to."]
    pub fn bitbucket_server_config_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitbucket_server_config_resource", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_key` after provisioning.\nKey of the project that the repo is in. For example: The key for https://mybitbucket.server/projects/TEST/repos/test-repo is \"TEST\"."]
    pub fn project_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo_slug` after provisioning.\nSlug of the repository. A repository slug is a URL-friendly version of a repository name, automatically generated by Bitbucket for use in the URL.\nFor example, if the repository name is 'test repo', in the URL it would become 'test-repo' as in https://mybitbucket.server/projects/TEST/repos/test-repo."]
    pub fn repo_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<CloudbuildTrigger_BitbucketServerTriggerConfig_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<CloudbuildTrigger_BitbucketServerTriggerConfig_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Artifacts_MavenArtifacts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Build_Artifacts_MavenArtifacts {
    #[doc =
        "Set the field `artifact_id`.\nMaven artifactId value used when uploading the artifact to Artifact Registry."]
    pub fn set_artifact_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.artifact_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_id`.\nMaven groupId value used when uploading the artifact to Artifact Registry."]
    pub fn set_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nPath to an artifact in the build's workspace to be uploaded to Artifact Registry. This can be either an absolute path, e.g. /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar or a relative path from /workspace, e.g. my-app/target/my-app-1.0.SNAPSHOT.jar."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repository`.\nArtifact Registry repository, in the form \"https://$REGION-maven.pkg.dev/$PROJECT/$REPOSITORY\"\n\nArtifact in the workspace specified by path will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nMaven version value used when uploading the artifact to Artifact Registry."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Artifacts_MavenArtifacts {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_MavenArtifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Artifacts_MavenArtifacts {}

impl BuildCloudbuildTrigger_Build_Artifacts_MavenArtifacts {
    pub fn build(self) -> CloudbuildTrigger_Build_Artifacts_MavenArtifacts {
        CloudbuildTrigger_Build_Artifacts_MavenArtifacts {
            artifact_id: core::default::Default::default(),
            group_id: core::default::Default::default(),
            path: core::default::Default::default(),
            repository: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
        CloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `artifact_id` after provisioning.\nMaven artifactId value used when uploading the artifact to Artifact Registry."]
    pub fn artifact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group_id` after provisioning.\nMaven groupId value used when uploading the artifact to Artifact Registry."]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath to an artifact in the build's workspace to be uploaded to Artifact Registry. This can be either an absolute path, e.g. /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar or a relative path from /workspace, e.g. my-app/target/my-app-1.0.SNAPSHOT.jar."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nArtifact Registry repository, in the form \"https://$REGION-maven.pkg.dev/$PROJECT/$REPOSITORY\"\n\nArtifact in the workspace specified by path will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nMaven version value used when uploading the artifact to Artifact Registry."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Artifacts_NpmPackages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Build_Artifacts_NpmPackages {
    #[doc = "Set the field `package_path`.\nPath to the package.json. e.g. workspace/path/to/package"]
    pub fn set_package_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.package_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repository`.\nArtifact Registry repository, in the form \"https://$REGION-npm.pkg.dev/$PROJECT/$REPOSITORY\"\n\nNpm package in the workspace specified by path will be zipped and uploaded to Artifact Registry with this location as a prefix."]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Artifacts_NpmPackages {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_NpmPackages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Artifacts_NpmPackages {}

impl BuildCloudbuildTrigger_Build_Artifacts_NpmPackages {
    pub fn build(self) -> CloudbuildTrigger_Build_Artifacts_NpmPackages {
        CloudbuildTrigger_Build_Artifacts_NpmPackages {
            package_path: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
        CloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `package_path` after provisioning.\nPath to the package.json. e.g. workspace/path/to/package"]
    pub fn package_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nArtifact Registry repository, in the form \"https://$REGION-npm.pkg.dev/$PROJECT/$REPOSITORY\"\n\nNpm package in the workspace specified by path will be zipped and uploaded to Artifact Registry with this location as a prefix."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Artifacts_Objects_Timing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Build_Artifacts_Objects_Timing {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Artifacts_Objects_Timing {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_Objects_Timing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Artifacts_Objects_Timing {}

impl BuildCloudbuildTrigger_Build_Artifacts_Objects_Timing {
    pub fn build(self) -> CloudbuildTrigger_Build_Artifacts_Objects_Timing {
        CloudbuildTrigger_Build_Artifacts_Objects_Timing {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
        CloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Artifacts_Objects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudbuildTrigger_Build_Artifacts_Objects {
    #[doc =
        "Set the field `location`.\nCloud Storage bucket and optional object path, in the form \"gs://bucket/path/to/somewhere/\".\n\nFiles in the workspace matching any path pattern will be uploaded to Cloud Storage with\nthis location as a prefix."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `paths`.\nPath globs used to match files in the build's workspace."]
    pub fn set_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.paths = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Artifacts_Objects {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_Objects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Artifacts_Objects {}

impl BuildCloudbuildTrigger_Build_Artifacts_Objects {
    pub fn build(self) -> CloudbuildTrigger_Build_Artifacts_Objects {
        CloudbuildTrigger_Build_Artifacts_Objects {
            location: core::default::Default::default(),
            paths: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Artifacts_ObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Artifacts_ObjectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_Artifacts_ObjectsRef {
        CloudbuildTrigger_Build_Artifacts_ObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Artifacts_ObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nCloud Storage bucket and optional object path, in the form \"gs://bucket/path/to/somewhere/\".\n\nFiles in the workspace matching any path pattern will be uploaded to Cloud Storage with\nthis location as a prefix."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `paths` after provisioning.\nPath globs used to match files in the build's workspace."]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timing` after provisioning.\nOutput only. Stores timing information for pushing all artifact objects."]
    pub fn timing(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Artifacts_Objects_TimingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timing", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Artifacts_PythonPackages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Build_Artifacts_PythonPackages {
    #[doc =
        "Set the field `paths`.\nPath globs used to match files in the build's workspace. For Python/ Twine, this is usually dist/*, and sometimes additionally an .asc file."]
    pub fn set_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.paths = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repository`.\nArtifact Registry repository, in the form \"https://$REGION-python.pkg.dev/$PROJECT/$REPOSITORY\"\n\nFiles in the workspace matching any path pattern will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Artifacts_PythonPackages {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_PythonPackages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Artifacts_PythonPackages {}

impl BuildCloudbuildTrigger_Build_Artifacts_PythonPackages {
    pub fn build(self) -> CloudbuildTrigger_Build_Artifacts_PythonPackages {
        CloudbuildTrigger_Build_Artifacts_PythonPackages {
            paths: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
        CloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `paths` after provisioning.\nPath globs used to match files in the build's workspace. For Python/ Twine, this is usually dist/*, and sometimes additionally an .asc file."]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nArtifact Registry repository, in the form \"https://$REGION-python.pkg.dev/$PROJECT/$REPOSITORY\"\n\nFiles in the workspace matching any path pattern will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_Build_ArtifactsDynamic {
    maven_artifacts: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Artifacts_MavenArtifacts>>,
    npm_packages: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Artifacts_NpmPackages>>,
    objects: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Artifacts_Objects>>,
    python_packages: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Artifacts_PythonPackages>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Artifacts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maven_artifacts: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Artifacts_MavenArtifacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    npm_packages: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Artifacts_NpmPackages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Artifacts_Objects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_packages: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Artifacts_PythonPackages>>,
    dynamic: CloudbuildTrigger_Build_ArtifactsDynamic,
}

impl CloudbuildTrigger_Build_Artifacts {
    #[doc =
        "Set the field `images`.\nA list of images to be pushed upon the successful completion of all build steps.\n\nThe images will be pushed using the builder service account's credentials.\n\nThe digests of the pushed images will be stored in the Build resource's results field.\n\nIf any of the images fail to be pushed, the build is marked FAILURE."]
    pub fn set_images(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.images = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maven_artifacts`.\n"]
    pub fn set_maven_artifacts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_MavenArtifacts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maven_artifacts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maven_artifacts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `npm_packages`.\n"]
    pub fn set_npm_packages(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_NpmPackages>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.npm_packages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.npm_packages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_Objects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `python_packages`.\n"]
    pub fn set_python_packages(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts_PythonPackages>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.python_packages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.python_packages = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Artifacts {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Artifacts {}

impl BuildCloudbuildTrigger_Build_Artifacts {
    pub fn build(self) -> CloudbuildTrigger_Build_Artifacts {
        CloudbuildTrigger_Build_Artifacts {
            images: core::default::Default::default(),
            maven_artifacts: core::default::Default::default(),
            npm_packages: core::default::Default::default(),
            objects: core::default::Default::default(),
            python_packages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_ArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_ArtifactsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_ArtifactsRef {
        CloudbuildTrigger_Build_ArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_ArtifactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `images` after provisioning.\nA list of images to be pushed upon the successful completion of all build steps.\n\nThe images will be pushed using the builder service account's credentials.\n\nThe digests of the pushed images will be stored in the Build resource's results field.\n\nIf any of the images fail to be pushed, the build is marked FAILURE."]
    pub fn images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }

    #[doc = "Get a reference to the value of field `maven_artifacts` after provisioning.\n"]
    pub fn maven_artifacts(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Artifacts_MavenArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `npm_packages` after provisioning.\n"]
    pub fn npm_packages(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Artifacts_NpmPackagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.npm_packages", self.base))
    }

    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Artifacts_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }

    #[doc = "Get a reference to the value of field `python_packages` after provisioning.\n"]
    pub fn python_packages(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Artifacts_PythonPackagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_packages", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    env: terrars::PrimField<std::string::String>,
    version_name: terrars::PrimField<std::string::String>,
}

impl CloudbuildTrigger_Build_AvailableSecrets_SecretManager { }

impl terrars::ToListMappable for CloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_AvailableSecrets_SecretManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    #[doc =
        "Environment variable name to associate with the secret. Secret environment\nvariables must be unique across all of a build's secrets, and must be used\nby at least one build step."]
    pub env: terrars::PrimField<std::string::String>,
    #[doc = "Resource name of the SecretVersion. In format: projects/*/secrets/*/versions/*"]
    pub version_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    pub fn build(self) -> CloudbuildTrigger_Build_AvailableSecrets_SecretManager {
        CloudbuildTrigger_Build_AvailableSecrets_SecretManager {
            env: self.env,
            version_name: self.version_name,
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
        CloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nEnvironment variable name to associate with the secret. Secret environment\nvariables must be unique across all of a build's secrets, and must be used\nby at least one build step."]
    pub fn env(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version_name` after provisioning.\nResource name of the SecretVersion. In format: projects/*/secrets/*/versions/*"]
    pub fn version_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_Build_AvailableSecretsDynamic {
    secret_manager: std
    ::option
    ::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_AvailableSecrets_SecretManager>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_AvailableSecrets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_AvailableSecrets_SecretManager>>,
    dynamic: CloudbuildTrigger_Build_AvailableSecretsDynamic,
}

impl CloudbuildTrigger_Build_AvailableSecrets {
    #[doc = "Set the field `secret_manager`.\n"]
    pub fn set_secret_manager(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_AvailableSecrets_SecretManager>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_manager = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_manager = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_AvailableSecrets {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_AvailableSecrets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_AvailableSecrets {}

impl BuildCloudbuildTrigger_Build_AvailableSecrets {
    pub fn build(self) -> CloudbuildTrigger_Build_AvailableSecrets {
        CloudbuildTrigger_Build_AvailableSecrets {
            secret_manager: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_AvailableSecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_AvailableSecretsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_AvailableSecretsRef {
        CloudbuildTrigger_Build_AvailableSecretsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_AvailableSecretsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_manager` after provisioning.\n"]
    pub fn secret_manager(&self) -> terrars::ListRef<CloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Options_Volumes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Build_Options_Volumes {
    #[doc =
        "Set the field `name`.\nName of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for Docker volumes.\nEach named volume must be used by at least two build steps."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nPath at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on the same\nbuild step or with certain reserved volume paths."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Options_Volumes {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Options_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Options_Volumes {}

impl BuildCloudbuildTrigger_Build_Options_Volumes {
    pub fn build(self) -> CloudbuildTrigger_Build_Options_Volumes {
        CloudbuildTrigger_Build_Options_Volumes {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Options_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Options_VolumesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_Options_VolumesRef {
        CloudbuildTrigger_Build_Options_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Options_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for Docker volumes.\nEach named volume must be used by at least two build steps."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on the same\nbuild step or with certain reserved volume paths."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_Build_OptionsDynamic {
    volumes: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Options_Volumes>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Options {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_substitutions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_streaming_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requested_verify_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_env: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_provenance_hash: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitution_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Options_Volumes>>,
    dynamic: CloudbuildTrigger_Build_OptionsDynamic,
}

impl CloudbuildTrigger_Build_Options {
    #[doc =
        "Set the field `disk_size_gb`.\nRequested disk size for the VM that runs the build. Note that this is NOT \"disk free\";\nsome of the space will be used by the operating system and build utilities.\nAlso note that this is the minimum disk size that will be allocated for the build --\nthe build may run with a larger disk than requested. At present, the maximum disk size\nis 1000GB; builds that request more than the maximum are rejected with an error."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dynamic_substitutions`.\nOption to specify whether or not to apply bash style string operations to the substitutions.\n\nNOTE this is always enabled for triggered builds and cannot be overridden in the build configuration file."]
    pub fn set_dynamic_substitutions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dynamic_substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `env`.\nA list of global environment variable definitions that will exist for all build steps\nin this build. If a variable is defined in both globally and in a build step,\nthe variable will use the build step value.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `log_streaming_option`.\nOption to define build log streaming behavior to Google Cloud Storage. Possible values: [\"STREAM_DEFAULT\", \"STREAM_ON\", \"STREAM_OFF\"]"]
    pub fn set_log_streaming_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_streaming_option = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `logging`.\nOption to specify the logging mode, which determines if and where build logs are stored. Possible values: [\"LOGGING_UNSPECIFIED\", \"LEGACY\", \"GCS_ONLY\", \"STACKDRIVER_ONLY\", \"CLOUD_LOGGING_ONLY\", \"NONE\"]"]
    pub fn set_logging(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nCompute Engine machine type on which to run the build."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `requested_verify_option`.\nRequested verifiability options. Possible values: [\"NOT_VERIFIED\", \"VERIFIED\"]"]
    pub fn set_requested_verify_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.requested_verify_option = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_env`.\nA list of global environment variables, which are encrypted using a Cloud Key Management\nService crypto key. These values must be specified in the build's Secret. These variables\nwill be available to all build steps in this build."]
    pub fn set_secret_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secret_env = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_provenance_hash`.\nRequested hash for SourceProvenance. Possible values: [\"NONE\", \"SHA256\", \"MD5\"]"]
    pub fn set_source_provenance_hash(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_provenance_hash = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `substitution_option`.\nOption to specify behavior when there is an error in the substitution checks.\n\nNOTE this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden\nin the build configuration file. Possible values: [\"MUST_MATCH\", \"ALLOW_LOOSE\"]"]
    pub fn set_substitution_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.substitution_option = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_pool`.\nOption to specify a WorkerPool for the build. Format projects/{project}/workerPools/{workerPool}\n\nThis field is experimental."]
    pub fn set_worker_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Options_Volumes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Options {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Options>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Options {}

impl BuildCloudbuildTrigger_Build_Options {
    pub fn build(self) -> CloudbuildTrigger_Build_Options {
        CloudbuildTrigger_Build_Options {
            disk_size_gb: core::default::Default::default(),
            dynamic_substitutions: core::default::Default::default(),
            env: core::default::Default::default(),
            log_streaming_option: core::default::Default::default(),
            logging: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            requested_verify_option: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            source_provenance_hash: core::default::Default::default(),
            substitution_option: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_OptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_OptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_OptionsRef {
        CloudbuildTrigger_Build_OptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_OptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nRequested disk size for the VM that runs the build. Note that this is NOT \"disk free\";\nsome of the space will be used by the operating system and build utilities.\nAlso note that this is the minimum disk size that will be allocated for the build --\nthe build may run with a larger disk than requested. At present, the maximum disk size\nis 1000GB; builds that request more than the maximum are rejected with an error."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dynamic_substitutions` after provisioning.\nOption to specify whether or not to apply bash style string operations to the substitutions.\n\nNOTE this is always enabled for triggered builds and cannot be overridden in the build configuration file."]
    pub fn dynamic_substitutions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dynamic_substitutions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nA list of global environment variable definitions that will exist for all build steps\nin this build. If a variable is defined in both globally and in a build step,\nthe variable will use the build step value.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_streaming_option` after provisioning.\nOption to define build log streaming behavior to Google Cloud Storage. Possible values: [\"STREAM_DEFAULT\", \"STREAM_ON\", \"STREAM_OFF\"]"]
    pub fn log_streaming_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_streaming_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logging` after provisioning.\nOption to specify the logging mode, which determines if and where build logs are stored. Possible values: [\"LOGGING_UNSPECIFIED\", \"LEGACY\", \"GCS_ONLY\", \"STACKDRIVER_ONLY\", \"CLOUD_LOGGING_ONLY\", \"NONE\"]"]
    pub fn logging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nCompute Engine machine type on which to run the build."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requested_verify_option` after provisioning.\nRequested verifiability options. Possible values: [\"NOT_VERIFIED\", \"VERIFIED\"]"]
    pub fn requested_verify_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_verify_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_env` after provisioning.\nA list of global environment variables, which are encrypted using a Cloud Key Management\nService crypto key. These values must be specified in the build's Secret. These variables\nwill be available to all build steps in this build."]
    pub fn secret_env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_provenance_hash` after provisioning.\nRequested hash for SourceProvenance. Possible values: [\"NONE\", \"SHA256\", \"MD5\"]"]
    pub fn source_provenance_hash(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_provenance_hash", self.base))
    }

    #[doc =
        "Get a reference to the value of field `substitution_option` after provisioning.\nOption to specify behavior when there is an error in the substitution checks.\n\nNOTE this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden\nin the build configuration file. Possible values: [\"MUST_MATCH\", \"ALLOW_LOOSE\"]"]
    pub fn substitution_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.substitution_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_pool` after provisioning.\nOption to specify a WorkerPool for the build. Format projects/{project}/workerPools/{workerPool}\n\nThis field is experimental."]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Options_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Secret {
    kms_key_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CloudbuildTrigger_Build_Secret {
    #[doc =
        "Set the field `secret_env`.\nMap of environment variable name to its encrypted value.\nSecret environment variables must be unique across all of a build's secrets,\nand must be used by at least one build step. Values can be at most 64 KB in size.\nThere can be at most 100 secret values across all of a build's secrets."]
    pub fn set_secret_env(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secret_env = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Secret {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Secret {
    #[doc = "Cloud KMS key name to use to decrypt these envs."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_Build_Secret {
    pub fn build(self) -> CloudbuildTrigger_Build_Secret {
        CloudbuildTrigger_Build_Secret {
            kms_key_name: self.kms_key_name,
            secret_env: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_SecretRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_SecretRef {
        CloudbuildTrigger_Build_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_SecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nCloud KMS key name to use to decrypt these envs."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_env` after provisioning.\nMap of environment variable name to its encrypted value.\nSecret environment variables must be unique across all of a build's secrets,\nand must be used by at least one build step. Values can be at most 64 KB in size.\nThere can be at most 100 secret values across all of a build's secrets."]
    pub fn secret_env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Source_RepoSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    repo_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitutions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Build_Source_RepoSource {
    #[doc =
        "Set the field `branch_name`.\nRegex matching branches to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `commit_sha`.\nExplicit commit SHA to build. Exactly one a of branch name, tag, or commit SHA must be provided."]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dir`.\nDirectory, relative to the source root, in which to run the build.\nThis must be a relative path. If a step's dir is specified and is an absolute path,\nthis value is ignored for that step's execution."]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_regex`.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nID of the project that owns the Cloud Source Repository.\nIf omitted, the project ID requesting the build is assumed."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `substitutions`.\nSubstitutions to use in a triggered build. Should only be used with triggers.run"]
    pub fn set_substitutions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag_name`.\nRegex matching tags to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_tag_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Source_RepoSource {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Source_RepoSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Source_RepoSource {
    #[doc = "Name of the Cloud Source Repository."]
    pub repo_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_Build_Source_RepoSource {
    pub fn build(self) -> CloudbuildTrigger_Build_Source_RepoSource {
        CloudbuildTrigger_Build_Source_RepoSource {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: self.repo_name,
            substitutions: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Source_RepoSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Source_RepoSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_Source_RepoSourceRef {
        CloudbuildTrigger_Build_Source_RepoSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Source_RepoSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch_name` after provisioning.\nRegex matching branches to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit_sha` after provisioning.\nExplicit commit SHA to build. Exactly one a of branch name, tag, or commit SHA must be provided."]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build.\nThis must be a relative path. If a step's dir is specified and is an absolute path,\nthis value is ignored for that step's execution."]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nID of the project that owns the Cloud Source Repository.\nIf omitted, the project ID requesting the build is assumed."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo_name` after provisioning.\nName of the Cloud Source Repository."]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions to use in a triggered build. Should only be used with triggers.run"]
    pub fn substitutions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.substitutions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag_name` after provisioning.\nRegex matching tags to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn tag_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Source_StorageSource {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<std::string::String>>,
    object: terrars::PrimField<std::string::String>,
}

impl CloudbuildTrigger_Build_Source_StorageSource {
    #[doc =
        "Set the field `generation`.\nGoogle Cloud Storage generation for the object.\nIf the generation is omitted, the latest generation will be used"]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Source_StorageSource {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Source_StorageSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Source_StorageSource {
    #[doc = "Google Cloud Storage bucket containing the source."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc =
        "Google Cloud Storage object containing the source.\nThis object must be a gzipped archive file (.tar.gz) containing source to build."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_Build_Source_StorageSource {
    pub fn build(self) -> CloudbuildTrigger_Build_Source_StorageSource {
        CloudbuildTrigger_Build_Source_StorageSource {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Source_StorageSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Source_StorageSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_Build_Source_StorageSourceRef {
        CloudbuildTrigger_Build_Source_StorageSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Source_StorageSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nGoogle Cloud Storage bucket containing the source."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGoogle Cloud Storage generation for the object.\nIf the generation is omitted, the latest generation will be used"]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nGoogle Cloud Storage object containing the source.\nThis object must be a gzipped archive file (.tar.gz) containing source to build."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_Build_SourceDynamic {
    repo_source: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Source_RepoSource>>,
    storage_source: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Source_StorageSource>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_source: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Source_RepoSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_source: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Source_StorageSource>>,
    dynamic: CloudbuildTrigger_Build_SourceDynamic,
}

impl CloudbuildTrigger_Build_Source {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Source_RepoSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repo_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repo_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Source_StorageSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Source {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Source {}

impl BuildCloudbuildTrigger_Build_Source {
    pub fn build(self) -> CloudbuildTrigger_Build_Source {
        CloudbuildTrigger_Build_Source {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_SourceRef {
        CloudbuildTrigger_Build_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Source_RepoSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Source_StorageSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Step_Volumes {
    name: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
}

impl CloudbuildTrigger_Build_Step_Volumes { }

impl terrars::ToListMappable for CloudbuildTrigger_Build_Step_Volumes {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Step_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Step_Volumes {
    #[doc =
        "Name of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for\nDocker volumes. Each named volume must be used by at least two build steps."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Path at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on\nthe same build step or with certain reserved volume paths."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_Build_Step_Volumes {
    pub fn build(self) -> CloudbuildTrigger_Build_Step_Volumes {
        CloudbuildTrigger_Build_Step_Volumes {
            name: self.name,
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_Step_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_Step_VolumesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_Step_VolumesRef {
        CloudbuildTrigger_Build_Step_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_Step_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for\nDocker volumes. Each named volume must be used by at least two build steps."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on\nthe same build step or with certain reserved volume paths."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_Build_StepDynamic {
    volumes: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Step_Volumes>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build_Step {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_exit_codes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_env: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Step_Volumes>>,
    dynamic: CloudbuildTrigger_Build_StepDynamic,
}

impl CloudbuildTrigger_Build_Step {
    #[doc =
        "Set the field `allow_exit_codes`.\nAllow this build step to fail without failing the entire build if and\nonly if the exit code is one of the specified codes.\n\nIf 'allowFailure' is also specified, this field will take precedence."]
    pub fn set_allow_exit_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.allow_exit_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_failure`.\nAllow this build step to fail without failing the entire build.\nIf false, the entire build will fail if this step fails. Otherwise, the\nbuild will succeed, but this step will still have a failure status.\nError information will be reported in the 'failureDetail' field.\n\n'allowExitCodes' takes precedence over this field."]
    pub fn set_allow_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nA list of arguments that will be presented to the step when it is started.\n\nIf the image used to run the step's container has an entrypoint, the args\nare used as arguments to that entrypoint. If the image does not define an\nentrypoint, the first element in args is used as the entrypoint, and the\nremainder will be used as arguments."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dir`.\nWorking directory to use when running this step's container.\n\nIf this value is a relative path, it is relative to the build's working\ndirectory. If this value is absolute, it may be outside the build's working\ndirectory, in which case the contents of the path may not be persisted\nacross build step executions, unless a 'volume' for that path is specified.\n\nIf the build specifies a 'RepoSource' with 'dir' and a step with a\n'dir',\nwhich specifies an absolute path, the 'RepoSource' 'dir' is ignored\nfor the step's execution."]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entrypoint`.\nEntrypoint to be used instead of the build step image's\ndefault entrypoint.\nIf unset, the image's default entrypoint is used"]
    pub fn set_entrypoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `env`.\nA list of environment variable definitions to be used when\nrunning a step.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable\n\"KEY\" being given the value \"VALUE\"."]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `id`.\nUnique identifier for this build step, used in 'wait_for' to\nreference this build step as a dependency."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script`.\nA shell script to be executed in the step.\nWhen script is provided, the user cannot specify the entrypoint or args."]
    pub fn set_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.script = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_env`.\nA list of environment variables which are encrypted using\na Cloud Key\nManagement Service crypto key. These values must be specified in\nthe build's 'Secret'."]
    pub fn set_secret_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secret_env = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout`.\nTime limit for executing this build step. If not defined,\nthe step has no\ntime limit and will be allowed to continue to run until either it\ncompletes or the build itself times out."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timing`.\nOutput only. Stores timing information for executing this\nbuild step."]
    pub fn set_timing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for`.\nThe ID(s) of the step(s) that this build step depends on.\n\nThis build step will not start until all the build steps in 'wait_for'\nhave completed successfully. If 'wait_for' is empty, this build step\nwill start when all previous build steps in the 'Build.Steps' list\nhave completed successfully."]
    pub fn set_wait_for(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.wait_for = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Step_Volumes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build_Step {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build_Step>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build_Step {
    #[doc =
        "The name of the container image that will run this particular build step.\n\nIf the image is available in the host's Docker daemon's cache, it will be\nrun directly. If not, the host will attempt to pull the image first, using\nthe builder service account's credentials if necessary.\n\nThe Docker daemon's cache will already have the latest versions of all of\nthe officially supported build steps (see https://github.com/GoogleCloudPlatform/cloud-builders\nfor images and examples).\nThe Docker daemon will also have cached many of the layers for some popular\nimages, like \"ubuntu\", \"debian\", but they will be refreshed at the time\nyou attempt to use them.\n\nIf you built an image in a previous build step, it will be stored in the\nhost's Docker daemon's cache and is available to use as the name for a\nlater build step."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_Build_Step {
    pub fn build(self) -> CloudbuildTrigger_Build_Step {
        CloudbuildTrigger_Build_Step {
            allow_exit_codes: core::default::Default::default(),
            allow_failure: core::default::Default::default(),
            args: core::default::Default::default(),
            dir: core::default::Default::default(),
            entrypoint: core::default::Default::default(),
            env: core::default::Default::default(),
            id: core::default::Default::default(),
            name: self.name,
            script: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            timeout: core::default::Default::default(),
            timing: core::default::Default::default(),
            wait_for: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Build_StepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Build_StepRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Build_StepRef {
        CloudbuildTrigger_Build_StepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Build_StepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_exit_codes` after provisioning.\nAllow this build step to fail without failing the entire build if and\nonly if the exit code is one of the specified codes.\n\nIf 'allowFailure' is also specified, this field will take precedence."]
    pub fn allow_exit_codes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_exit_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_failure` after provisioning.\nAllow this build step to fail without failing the entire build.\nIf false, the entire build will fail if this step fails. Otherwise, the\nbuild will succeed, but this step will still have a failure status.\nError information will be reported in the 'failureDetail' field.\n\n'allowExitCodes' takes precedence over this field."]
    pub fn allow_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nA list of arguments that will be presented to the step when it is started.\n\nIf the image used to run the step's container has an entrypoint, the args\nare used as arguments to that entrypoint. If the image does not define an\nentrypoint, the first element in args is used as the entrypoint, and the\nremainder will be used as arguments."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dir` after provisioning.\nWorking directory to use when running this step's container.\n\nIf this value is a relative path, it is relative to the build's working\ndirectory. If this value is absolute, it may be outside the build's working\ndirectory, in which case the contents of the path may not be persisted\nacross build step executions, unless a 'volume' for that path is specified.\n\nIf the build specifies a 'RepoSource' with 'dir' and a step with a\n'dir',\nwhich specifies an absolute path, the 'RepoSource' 'dir' is ignored\nfor the step's execution."]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entrypoint` after provisioning.\nEntrypoint to be used instead of the build step image's\ndefault entrypoint.\nIf unset, the image's default entrypoint is used"]
    pub fn entrypoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entrypoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nA list of environment variable definitions to be used when\nrunning a step.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable\n\"KEY\" being given the value \"VALUE\"."]
    pub fn env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for this build step, used in 'wait_for' to\nreference this build step as a dependency."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the container image that will run this particular build step.\n\nIf the image is available in the host's Docker daemon's cache, it will be\nrun directly. If not, the host will attempt to pull the image first, using\nthe builder service account's credentials if necessary.\n\nThe Docker daemon's cache will already have the latest versions of all of\nthe officially supported build steps (see https://github.com/GoogleCloudPlatform/cloud-builders\nfor images and examples).\nThe Docker daemon will also have cached many of the layers for some popular\nimages, like \"ubuntu\", \"debian\", but they will be refreshed at the time\nyou attempt to use them.\n\nIf you built an image in a previous build step, it will be stored in the\nhost's Docker daemon's cache and is available to use as the name for a\nlater build step."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nA shell script to be executed in the step.\nWhen script is provided, the user cannot specify the entrypoint or args."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_env` after provisioning.\nA list of environment variables which are encrypted using\na Cloud Key\nManagement Service crypto key. These values must be specified in\nthe build's 'Secret'."]
    pub fn secret_env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nTime limit for executing this build step. If not defined,\nthe step has no\ntime limit and will be allowed to continue to run until either it\ncompletes or the build itself times out."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timing` after provisioning.\nOutput only. Stores timing information for executing this\nbuild step."]
    pub fn timing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wait_for` after provisioning.\nThe ID(s) of the step(s) that this build step depends on.\n\nThis build step will not start until all the build steps in 'wait_for'\nhave completed successfully. If 'wait_for' is empty, this build step\nwill start when all previous build steps in the 'Build.Steps' list\nhave completed successfully."]
    pub fn wait_for(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wait_for", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<CloudbuildTrigger_Build_Step_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_BuildDynamic {
    artifacts: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Artifacts>>,
    available_secrets: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_AvailableSecrets>>,
    options: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Options>>,
    secret: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Secret>>,
    source: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Source>>,
    step: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Build_Step>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Build {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitutions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifacts: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Artifacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_secrets: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_AvailableSecrets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Options>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Secret>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step: std::option::Option<std::vec::Vec<CloudbuildTrigger_Build_Step>>,
    dynamic: CloudbuildTrigger_BuildDynamic,
}

impl CloudbuildTrigger_Build {
    #[doc =
        "Set the field `images`.\nA list of images to be pushed upon the successful completion of all build steps.\nThe images are pushed using the builder service account's credentials.\nThe digests of the pushed images will be stored in the Build resource's results field.\nIf any of the images fail to be pushed, the build status is marked FAILURE."]
    pub fn set_images(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.images = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `logs_bucket`.\nGoogle Cloud Storage bucket where logs should be written.\nLogs file names will be of the format ${logsBucket}/log-${build_id}.txt."]
    pub fn set_logs_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logs_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `queue_ttl`.\nTTL in queue for this build. If provided and the build is enqueued longer than this value,\nthe build will expire and the build status will be EXPIRED.\nThe TTL starts ticking from createTime.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_queue_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `substitutions`.\nSubstitutions data for Build resource."]
    pub fn set_substitutions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nTags for annotation of a Build. These are not docker tags."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout`.\nAmount of time that this build should be allowed to run, to second granularity.\nIf this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.\nThis timeout must be equal to or greater than the sum of the timeouts for build steps within the build.\nThe expected format is the number of seconds followed by s.\nDefault time is ten minutes (600s)."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `artifacts`.\n"]
    pub fn set_artifacts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Artifacts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.artifacts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.artifacts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `available_secrets`.\n"]
    pub fn set_available_secrets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_AvailableSecrets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.available_secrets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.available_secrets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Options>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Secret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step`.\n"]
    pub fn set_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Build_Step>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.step = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Build {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Build>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Build {}

impl BuildCloudbuildTrigger_Build {
    pub fn build(self) -> CloudbuildTrigger_Build {
        CloudbuildTrigger_Build {
            images: core::default::Default::default(),
            logs_bucket: core::default::Default::default(),
            queue_ttl: core::default::Default::default(),
            substitutions: core::default::Default::default(),
            tags: core::default::Default::default(),
            timeout: core::default::Default::default(),
            artifacts: core::default::Default::default(),
            available_secrets: core::default::Default::default(),
            options: core::default::Default::default(),
            secret: core::default::Default::default(),
            source: core::default::Default::default(),
            step: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_BuildRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_BuildRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_BuildRef {
        CloudbuildTrigger_BuildRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_BuildRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `images` after provisioning.\nA list of images to be pushed upon the successful completion of all build steps.\nThe images are pushed using the builder service account's credentials.\nThe digests of the pushed images will be stored in the Build resource's results field.\nIf any of the images fail to be pushed, the build status is marked FAILURE."]
    pub fn images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logs_bucket` after provisioning.\nGoogle Cloud Storage bucket where logs should be written.\nLogs file names will be of the format ${logsBucket}/log-${build_id}.txt."]
    pub fn logs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logs_bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queue_ttl` after provisioning.\nTTL in queue for this build. If provided and the build is enqueued longer than this value,\nthe build will expire and the build status will be EXPIRED.\nThe TTL starts ticking from createTime.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn queue_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.substitutions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a Build. These are not docker tags."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nAmount of time that this build should be allowed to run, to second granularity.\nIf this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.\nThis timeout must be equal to or greater than the sum of the timeouts for build steps within the build.\nThe expected format is the number of seconds followed by s.\nDefault time is ten minutes (600s)."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> terrars::ListRef<CloudbuildTrigger_Build_ArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `available_secrets` after provisioning.\n"]
    pub fn available_secrets(&self) -> terrars::ListRef<CloudbuildTrigger_Build_AvailableSecretsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_secrets", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<CloudbuildTrigger_Build_OptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<CloudbuildTrigger_Build_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<CloudbuildTrigger_Build_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::ListRef<CloudbuildTrigger_Build_StepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest {
    #[doc = "Set the field `branch`.\nRegex of branches to match."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `comment_control`.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest {
    type O = terrars::BlockAssignable<CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_DeveloperConnectEventConfig_PullRequest {}

impl BuildCloudbuildTrigger_DeveloperConnectEventConfig_PullRequest {
    pub fn build(self) -> CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest {
        CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_DeveloperConnectEventConfig_PullRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_DeveloperConnectEventConfig_PullRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_DeveloperConnectEventConfig_PullRequestRef {
        CloudbuildTrigger_DeveloperConnectEventConfig_PullRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_DeveloperConnectEventConfig_PullRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `comment_control` after provisioning.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_DeveloperConnectEventConfig_Push {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_DeveloperConnectEventConfig_Push {
    #[doc = "Set the field `branch`.\nRegex of branches to match."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_regex`.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nRegex of tags to match."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_DeveloperConnectEventConfig_Push {
    type O = terrars::BlockAssignable<CloudbuildTrigger_DeveloperConnectEventConfig_Push>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_DeveloperConnectEventConfig_Push {}

impl BuildCloudbuildTrigger_DeveloperConnectEventConfig_Push {
    pub fn build(self) -> CloudbuildTrigger_DeveloperConnectEventConfig_Push {
        CloudbuildTrigger_DeveloperConnectEventConfig_Push {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_DeveloperConnectEventConfig_PushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_DeveloperConnectEventConfig_PushRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_DeveloperConnectEventConfig_PushRef {
        CloudbuildTrigger_DeveloperConnectEventConfig_PushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_DeveloperConnectEventConfig_PushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_DeveloperConnectEventConfigDynamic {
    pull_request: std
    ::option
    ::Option<terrars::DynamicBlock<CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest>>,
    push: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_DeveloperConnectEventConfig_Push>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_DeveloperConnectEventConfig {
    git_repository_link: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<std::vec::Vec<CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<std::vec::Vec<CloudbuildTrigger_DeveloperConnectEventConfig_Push>>,
    dynamic: CloudbuildTrigger_DeveloperConnectEventConfigDynamic,
}

impl CloudbuildTrigger_DeveloperConnectEventConfig {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudbuildTrigger_DeveloperConnectEventConfig_PullRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pull_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_DeveloperConnectEventConfig_Push>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_DeveloperConnectEventConfig {
    type O = terrars::BlockAssignable<CloudbuildTrigger_DeveloperConnectEventConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_DeveloperConnectEventConfig {
    #[doc =
        "The Developer Connect Git repository link, formatted as 'projects/*/locations/*/connections/*/gitRepositoryLink/*'."]
    pub git_repository_link: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_DeveloperConnectEventConfig {
    pub fn build(self) -> CloudbuildTrigger_DeveloperConnectEventConfig {
        CloudbuildTrigger_DeveloperConnectEventConfig {
            git_repository_link: self.git_repository_link,
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_DeveloperConnectEventConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_DeveloperConnectEventConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_DeveloperConnectEventConfigRef {
        CloudbuildTrigger_DeveloperConnectEventConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_DeveloperConnectEventConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `git_repository_link` after provisioning.\nThe Developer Connect Git repository link, formatted as 'projects/*/locations/*/connections/*/gitRepositoryLink/*'."]
    pub fn git_repository_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_repository_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `git_repository_link_type` after provisioning.\nThe type of DeveloperConnect GitRepositoryLink."]
    pub fn git_repository_link_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_repository_link_type", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<CloudbuildTrigger_DeveloperConnectEventConfig_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<CloudbuildTrigger_DeveloperConnectEventConfig_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_GitFileSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_server_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_config: std::option::Option<terrars::PrimField<std::string::String>>,
    path: terrars::PrimField<std::string::String>,
    repo_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_GitFileSource {
    #[doc =
        "Set the field `bitbucket_server_config`.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn set_bitbucket_server_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bitbucket_server_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `github_enterprise_config`.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn set_github_enterprise_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.github_enterprise_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repository`.\nThe fully qualified resource name of the Repo API repository. The fully qualified resource name of the Repo API repository.\nIf unspecified, the repo from which the trigger invocation originated is assumed to be the repo from which to read the specified path."]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `revision`.\nThe branch, tag, arbitrary ref, or SHA version of the repo to use when resolving the\nfilename (optional). This field respects the same syntax/resolution as described here: https://git-scm.com/docs/gitrevisions\nIf unspecified, the revision from which the trigger invocation originated is assumed to be the revision from which to read the specified path."]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `uri`.\nThe URI of the repo (optional). If unspecified, the repo from which the trigger\ninvocation originated is assumed to be the repo from which to read the specified path."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_GitFileSource {
    type O = terrars::BlockAssignable<CloudbuildTrigger_GitFileSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_GitFileSource {
    #[doc = "The path of the file, with the repo root as the root of the path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub repo_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_GitFileSource {
    pub fn build(self) -> CloudbuildTrigger_GitFileSource {
        CloudbuildTrigger_GitFileSource {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            path: self.path,
            repo_type: self.repo_type,
            repository: core::default::Default::default(),
            revision: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_GitFileSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_GitFileSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_GitFileSourceRef {
        CloudbuildTrigger_GitFileSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_GitFileSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bitbucket_server_config` after provisioning.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn bitbucket_server_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitbucket_server_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `github_enterprise_config` after provisioning.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn github_enterprise_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.github_enterprise_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe path of the file, with the repo root as the root of the path."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo_type` after provisioning.\nThe type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub fn repo_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nThe fully qualified resource name of the Repo API repository. The fully qualified resource name of the Repo API repository.\nIf unspecified, the repo from which the trigger invocation originated is assumed to be the repo from which to read the specified path."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nThe branch, tag, arbitrary ref, or SHA version of the repo to use when resolving the\nfilename (optional). This field respects the same syntax/resolution as described here: https://git-scm.com/docs/gitrevisions\nIf unspecified, the revision from which the trigger invocation originated is assumed to be the revision from which to read the specified path."]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe URI of the repo (optional). If unspecified, the repo from which the trigger\ninvocation originated is assumed to be the repo from which to read the specified path."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Github_PullRequest {
    branch: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudbuildTrigger_Github_PullRequest {
    #[doc =
        "Set the field `comment_control`.\nWhether to block builds on a \"/gcbrun\" comment from a repository owner or collaborator. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Github_PullRequest {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Github_PullRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Github_PullRequest {
    #[doc = "Regex of branches to match."]
    pub branch: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_Github_PullRequest {
    pub fn build(self) -> CloudbuildTrigger_Github_PullRequest {
        CloudbuildTrigger_Github_PullRequest {
            branch: self.branch,
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Github_PullRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Github_PullRequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Github_PullRequestRef {
        CloudbuildTrigger_Github_PullRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Github_PullRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `comment_control` after provisioning.\nWhether to block builds on a \"/gcbrun\" comment from a repository owner or collaborator. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Github_Push {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Github_Push {
    #[doc = "Set the field `branch`.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_regex`.\nWhen true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Github_Push {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Github_Push>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Github_Push {}

impl BuildCloudbuildTrigger_Github_Push {
    pub fn build(self) -> CloudbuildTrigger_Github_Push {
        CloudbuildTrigger_Github_Push {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_Github_PushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_Github_PushRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_Github_PushRef {
        CloudbuildTrigger_Github_PushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_Github_PushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nWhen true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_GithubDynamic {
    pull_request: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Github_PullRequest>>,
    push: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_Github_Push>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enterprise_config_resource_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<std::vec::Vec<CloudbuildTrigger_Github_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<std::vec::Vec<CloudbuildTrigger_Github_Push>>,
    dynamic: CloudbuildTrigger_GithubDynamic,
}

impl CloudbuildTrigger_Github {
    #[doc =
        "Set the field `enterprise_config_resource_name`.\nThe resource name of the github enterprise config that should be applied to this installation.\nFor example: \"projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}\""]
    pub fn set_enterprise_config_resource_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enterprise_config_resource_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nName of the repository. For example: The name for\nhttps://github.com/googlecloudplatform/cloud-builders is \"cloud-builders\"."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `owner`.\nOwner of the repository. For example: The owner for\nhttps://github.com/googlecloudplatform/cloud-builders is \"googlecloudplatform\"."]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Github_PullRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pull_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_Github_Push>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_Github {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Github {}

impl BuildCloudbuildTrigger_Github {
    pub fn build(self) -> CloudbuildTrigger_Github {
        CloudbuildTrigger_Github {
            enterprise_config_resource_name: core::default::Default::default(),
            name: core::default::Default::default(),
            owner: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_GithubRef {
        CloudbuildTrigger_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_GithubRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enterprise_config_resource_name` after provisioning.\nThe resource name of the github enterprise config that should be applied to this installation.\nFor example: \"projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}\""]
    pub fn enterprise_config_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enterprise_config_resource_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the repository. For example: The name for\nhttps://github.com/googlecloudplatform/cloud-builders is \"cloud-builders\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `owner` after provisioning.\nOwner of the repository. For example: The owner for\nhttps://github.com/googlecloudplatform/cloud-builders is \"googlecloudplatform\"."]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<CloudbuildTrigger_Github_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<CloudbuildTrigger_Github_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_PubsubConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    topic: terrars::PrimField<std::string::String>,
}

impl CloudbuildTrigger_PubsubConfig {
    #[doc = "Set the field `service_account_email`.\nService account that will make the push request."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_PubsubConfig {
    type O = terrars::BlockAssignable<CloudbuildTrigger_PubsubConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_PubsubConfig {
    #[doc = "The name of the topic from which this subscription is receiving messages."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_PubsubConfig {
    pub fn build(self) -> CloudbuildTrigger_PubsubConfig {
        CloudbuildTrigger_PubsubConfig {
            service_account_email: core::default::Default::default(),
            topic: self.topic,
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_PubsubConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_PubsubConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_PubsubConfigRef {
        CloudbuildTrigger_PubsubConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_PubsubConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nService account that will make the push request."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nPotential issues with the underlying Pub/Sub subscription configuration.\nOnly populated on get requests."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subscription` after provisioning.\nOutput only. Name of the subscription."]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nThe name of the topic from which this subscription is receiving messages."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_RepositoryEventConfig_PullRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudbuildTrigger_RepositoryEventConfig_PullRequest {
    #[doc =
        "Set the field `branch`.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `comment_control`.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_RepositoryEventConfig_PullRequest {
    type O = terrars::BlockAssignable<CloudbuildTrigger_RepositoryEventConfig_PullRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_RepositoryEventConfig_PullRequest {}

impl BuildCloudbuildTrigger_RepositoryEventConfig_PullRequest {
    pub fn build(self) -> CloudbuildTrigger_RepositoryEventConfig_PullRequest {
        CloudbuildTrigger_RepositoryEventConfig_PullRequest {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_RepositoryEventConfig_PullRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_RepositoryEventConfig_PullRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_RepositoryEventConfig_PullRequestRef {
        CloudbuildTrigger_RepositoryEventConfig_PullRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_RepositoryEventConfig_PullRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `comment_control` after provisioning.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_RepositoryEventConfig_Push {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_RepositoryEventConfig_Push {
    #[doc =
        "Set the field `branch`.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_regex`.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nRegex of tags to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_RepositoryEventConfig_Push {
    type O = terrars::BlockAssignable<CloudbuildTrigger_RepositoryEventConfig_Push>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_RepositoryEventConfig_Push {}

impl BuildCloudbuildTrigger_RepositoryEventConfig_Push {
    pub fn build(self) -> CloudbuildTrigger_RepositoryEventConfig_Push {
        CloudbuildTrigger_RepositoryEventConfig_Push {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_RepositoryEventConfig_PushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_RepositoryEventConfig_PushRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudbuildTrigger_RepositoryEventConfig_PushRef {
        CloudbuildTrigger_RepositoryEventConfig_PushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_RepositoryEventConfig_PushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudbuildTrigger_RepositoryEventConfigDynamic {
    pull_request: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_RepositoryEventConfig_PullRequest>>,
    push: std::option::Option<terrars::DynamicBlock<CloudbuildTrigger_RepositoryEventConfig_Push>>,
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_RepositoryEventConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<std::vec::Vec<CloudbuildTrigger_RepositoryEventConfig_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<std::vec::Vec<CloudbuildTrigger_RepositoryEventConfig_Push>>,
    dynamic: CloudbuildTrigger_RepositoryEventConfigDynamic,
}

impl CloudbuildTrigger_RepositoryEventConfig {
    #[doc = "Set the field `repository`.\nThe resource name of the Repo API resource."]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_RepositoryEventConfig_PullRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pull_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudbuildTrigger_RepositoryEventConfig_Push>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_RepositoryEventConfig {
    type O = terrars::BlockAssignable<CloudbuildTrigger_RepositoryEventConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_RepositoryEventConfig {}

impl BuildCloudbuildTrigger_RepositoryEventConfig {
    pub fn build(self) -> CloudbuildTrigger_RepositoryEventConfig {
        CloudbuildTrigger_RepositoryEventConfig {
            repository: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_RepositoryEventConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_RepositoryEventConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_RepositoryEventConfigRef {
        CloudbuildTrigger_RepositoryEventConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_RepositoryEventConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nThe resource name of the Repo API resource."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<CloudbuildTrigger_RepositoryEventConfig_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<CloudbuildTrigger_RepositoryEventConfig_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_SourceToBuild {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_server_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref")]
    ref_: terrars::PrimField<std::string::String>,
    repo_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_SourceToBuild {
    #[doc =
        "Set the field `bitbucket_server_config`.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn set_bitbucket_server_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bitbucket_server_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `github_enterprise_config`.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn set_github_enterprise_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.github_enterprise_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repository`.\nThe qualified resource name of the Repo API repository.\nEither uri or repository can be specified and is required."]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\nThe URI of the repo."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_SourceToBuild {
    type O = terrars::BlockAssignable<CloudbuildTrigger_SourceToBuild>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_SourceToBuild {
    #[doc = "The branch or tag to use. Must start with \"refs/\" (required)."]
    pub ref_: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub repo_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_SourceToBuild {
    pub fn build(self) -> CloudbuildTrigger_SourceToBuild {
        CloudbuildTrigger_SourceToBuild {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            ref_: self.ref_,
            repo_type: self.repo_type,
            repository: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_SourceToBuildRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_SourceToBuildRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_SourceToBuildRef {
        CloudbuildTrigger_SourceToBuildRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_SourceToBuildRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bitbucket_server_config` after provisioning.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn bitbucket_server_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitbucket_server_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `github_enterprise_config` after provisioning.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn github_enterprise_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.github_enterprise_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nThe branch or tag to use. Must start with \"refs/\" (required)."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo_type` after provisioning.\nThe type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub fn repo_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nThe qualified resource name of the Repo API repository.\nEither uri or repository can be specified and is required."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the repo."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_Timeouts {
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

impl terrars::ToListMappable for CloudbuildTrigger_Timeouts {
    type O = terrars::BlockAssignable<CloudbuildTrigger_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_Timeouts {}

impl BuildCloudbuildTrigger_Timeouts {
    pub fn build(self) -> CloudbuildTrigger_Timeouts {
        CloudbuildTrigger_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_TimeoutsRef {
        CloudbuildTrigger_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_TimeoutsRef {
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
pub struct CloudbuildTrigger_TriggerTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudbuildTrigger_TriggerTemplate {
    #[doc =
        "Set the field `branch_name`.\nName of the branch to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `commit_sha`.\nExplicit commit SHA to build. Exactly one of a branch name, tag, or commit SHA must be provided."]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dir`.\nDirectory, relative to the source root, in which to run the build.\n\nThis must be a relative path. If a step's dir is specified and\nis an absolute path, this value is ignored for that step's\nexecution."]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_regex`.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nID of the project that owns the Cloud Source Repository. If\nomitted, the project ID requesting the build is assumed."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repo_name`.\nName of the Cloud Source Repository. If omitted, the name \"default\" is assumed."]
    pub fn set_repo_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag_name`.\nName of the tag to build. Exactly one of a branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn set_tag_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudbuildTrigger_TriggerTemplate {
    type O = terrars::BlockAssignable<CloudbuildTrigger_TriggerTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_TriggerTemplate {}

impl BuildCloudbuildTrigger_TriggerTemplate {
    pub fn build(self) -> CloudbuildTrigger_TriggerTemplate {
        CloudbuildTrigger_TriggerTemplate {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_TriggerTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_TriggerTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_TriggerTemplateRef {
        CloudbuildTrigger_TriggerTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_TriggerTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch_name` after provisioning.\nName of the branch to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit_sha` after provisioning.\nExplicit commit SHA to build. Exactly one of a branch name, tag, or commit SHA must be provided."]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build.\n\nThis must be a relative path. If a step's dir is specified and\nis an absolute path, this value is ignored for that step's\nexecution."]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nID of the project that owns the Cloud Source Repository. If\nomitted, the project ID requesting the build is assumed."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo_name` after provisioning.\nName of the Cloud Source Repository. If omitted, the name \"default\" is assumed."]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag_name` after provisioning.\nName of the tag to build. Exactly one of a branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn tag_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudbuildTrigger_WebhookConfig {
    secret: terrars::PrimField<std::string::String>,
}

impl CloudbuildTrigger_WebhookConfig { }

impl terrars::ToListMappable for CloudbuildTrigger_WebhookConfig {
    type O = terrars::BlockAssignable<CloudbuildTrigger_WebhookConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudbuildTrigger_WebhookConfig {
    #[doc = "Resource name for the secret required as a URL parameter."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudbuildTrigger_WebhookConfig {
    pub fn build(self) -> CloudbuildTrigger_WebhookConfig {
        CloudbuildTrigger_WebhookConfig { secret: self.secret }
    }
}

#[allow(dead_code)]
pub struct CloudbuildTrigger_WebhookConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudbuildTrigger_WebhookConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudbuildTrigger_WebhookConfigRef {
        CloudbuildTrigger_WebhookConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudbuildTrigger_WebhookConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nResource name for the secret required as a URL parameter."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nPotential issues with the underlying Pub/Sub subscription configuration.\nOnly populated on get requests."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
