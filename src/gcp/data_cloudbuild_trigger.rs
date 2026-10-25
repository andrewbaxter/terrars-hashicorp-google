#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudbuildTriggerData_ {
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
    trigger_id: terrars::PrimField<std::string::String>,
}

struct DataCloudbuildTrigger_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudbuildTriggerData_>,
}

#[derive(Clone)]
pub struct DataCloudbuildTrigger(std::rc::Rc<DataCloudbuildTrigger_>);

impl DataCloudbuildTrigger {
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
        "Get a reference to the value of field `approval_config` after provisioning.\nConfiguration for manual approval to start a build invocation of this BuildTrigger.\nBuilds created by this trigger will require approval before they execute.\nAny user with a Cloud Build Approver role for the project can approve a build."]
    pub fn approval_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_ApprovalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\nBitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received."]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> terrars::ListRef<DataCloudbuildTrigger_BitbucketServerTriggerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `build` after provisioning.\nContents of the build template. Either a filename or build template must be provided."]
    pub fn build(&self) -> terrars::ListRef<DataCloudbuildTrigger_BuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build", self.extract_ref()))
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
        "Get a reference to the value of field `developer_connect_event_config` after provisioning.\nConfiguration for triggers that respond to Developer Connect events."]
    pub fn developer_connect_event_config(
        &self,
    ) -> terrars::ListRef<DataCloudbuildTrigger_DeveloperConnectEventConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
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

    #[doc =
        "Get a reference to the value of field `git_file_source` after provisioning.\nThe file source describing the local or remote Build template."]
    pub fn git_file_source(&self) -> terrars::ListRef<DataCloudbuildTrigger_GitFileSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_file_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `github` after provisioning.\nDescribes the configuration of a trigger that creates a build whenever a GitHub event is received."]
    pub fn github(&self) -> terrars::ListRef<DataCloudbuildTrigger_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.extract_ref()))
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
        "Get a reference to the value of field `pubsub_config` after provisioning.\nPubsubConfig describes the configuration of a trigger that creates\na build whenever a Pub/Sub message is published."]
    pub fn pubsub_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_PubsubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_event_config` after provisioning.\nThe configuration of a trigger that creates a build whenever an event from Repo API is received."]
    pub fn repository_event_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_RepositoryEventConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_event_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_to_build` after provisioning.\nThe repo and ref of the repository from which to build.\nThis field is used only for those triggers that do not respond to SCM events.\nTriggers that respond to such events build source at whatever commit caused the event.\nThis field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers."]
    pub fn source_to_build(&self) -> terrars::ListRef<DataCloudbuildTrigger_SourceToBuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_to_build", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `trigger_template` after provisioning.\nTemplate describing the types of source changes to trigger a build.\n\nBranch and tag names in trigger templates are interpreted as regular\nexpressions. Any branch or tag change that matches that regular\nexpression will trigger a build."]
    pub fn trigger_template(&self) -> terrars::ListRef<DataCloudbuildTrigger_TriggerTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `webhook_config` after provisioning.\nWebhookConfig describes the configuration of a trigger that creates\na build whenever a webhook is sent to a trigger's webhook URL."]
    pub fn webhook_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_WebhookConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudbuildTrigger {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudbuildTrigger { }

impl terrars::ToListMappable for DataCloudbuildTrigger {
    type O = terrars::ListRef<DataCloudbuildTriggerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudbuildTrigger_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloudbuild_trigger".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudbuildTrigger {
    pub tf_id: std::string::String,
    #[doc =
        "The [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The unique identifier for the trigger."]
    pub trigger_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudbuildTrigger {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudbuildTrigger {
        let out = DataCloudbuildTrigger(std::rc::Rc::new(DataCloudbuildTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudbuildTriggerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                trigger_id: self.trigger_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudbuildTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudbuildTriggerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `approval_config` after provisioning.\nConfiguration for manual approval to start a build invocation of this BuildTrigger.\nBuilds created by this trigger will require approval before they execute.\nAny user with a Cloud Build Approver role for the project can approve a build."]
    pub fn approval_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_ApprovalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\nBitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received."]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> terrars::ListRef<DataCloudbuildTrigger_BitbucketServerTriggerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `build` after provisioning.\nContents of the build template. Either a filename or build template must be provided."]
    pub fn build(&self) -> terrars::ListRef<DataCloudbuildTrigger_BuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build", self.extract_ref()))
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
        "Get a reference to the value of field `developer_connect_event_config` after provisioning.\nConfiguration for triggers that respond to Developer Connect events."]
    pub fn developer_connect_event_config(
        &self,
    ) -> terrars::ListRef<DataCloudbuildTrigger_DeveloperConnectEventConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
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

    #[doc =
        "Get a reference to the value of field `git_file_source` after provisioning.\nThe file source describing the local or remote Build template."]
    pub fn git_file_source(&self) -> terrars::ListRef<DataCloudbuildTrigger_GitFileSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_file_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `github` after provisioning.\nDescribes the configuration of a trigger that creates a build whenever a GitHub event is received."]
    pub fn github(&self) -> terrars::ListRef<DataCloudbuildTrigger_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.extract_ref()))
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
        "Get a reference to the value of field `pubsub_config` after provisioning.\nPubsubConfig describes the configuration of a trigger that creates\na build whenever a Pub/Sub message is published."]
    pub fn pubsub_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_PubsubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_event_config` after provisioning.\nThe configuration of a trigger that creates a build whenever an event from Repo API is received."]
    pub fn repository_event_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_RepositoryEventConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_event_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_to_build` after provisioning.\nThe repo and ref of the repository from which to build.\nThis field is used only for those triggers that do not respond to SCM events.\nTriggers that respond to such events build source at whatever commit caused the event.\nThis field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers."]
    pub fn source_to_build(&self) -> terrars::ListRef<DataCloudbuildTrigger_SourceToBuildRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_to_build", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `trigger_template` after provisioning.\nTemplate describing the types of source changes to trigger a build.\n\nBranch and tag names in trigger templates are interpreted as regular\nexpressions. Any branch or tag change that matches that regular\nexpression will trigger a build."]
    pub fn trigger_template(&self) -> terrars::ListRef<DataCloudbuildTrigger_TriggerTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `webhook_config` after provisioning.\nWebhookConfig describes the configuration of a trigger that creates\na build whenever a webhook is sent to a trigger's webhook URL."]
    pub fn webhook_config(&self) -> terrars::ListRef<DataCloudbuildTrigger_WebhookConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_ApprovalConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_required: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildTrigger_ApprovalConfig {
    #[doc = "Set the field `approval_required`.\n"]
    pub fn set_approval_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.approval_required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_ApprovalConfig {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_ApprovalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_ApprovalConfig {}

impl BuildDataCloudbuildTrigger_ApprovalConfig {
    pub fn build(self) -> DataCloudbuildTrigger_ApprovalConfig {
        DataCloudbuildTrigger_ApprovalConfig { approval_required: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_ApprovalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_ApprovalConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_ApprovalConfigRef {
        DataCloudbuildTrigger_ApprovalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_ApprovalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approval_required` after provisioning.\n"]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Github_PullRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildTrigger_Github_PullRequest {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comment_control`.\n"]
    pub fn set_comment_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Github_PullRequest {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Github_PullRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Github_PullRequest {}

impl BuildDataCloudbuildTrigger_Github_PullRequest {
    pub fn build(self) -> DataCloudbuildTrigger_Github_PullRequest {
        DataCloudbuildTrigger_Github_PullRequest {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Github_PullRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Github_PullRequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Github_PullRequestRef {
        DataCloudbuildTrigger_Github_PullRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Github_PullRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\n"]
    pub fn comment_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment_control", self.base))
    }

    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Github_Push {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Github_Push {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Github_Push {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Github_Push>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Github_Push {}

impl BuildDataCloudbuildTrigger_Github_Push {
    pub fn build(self) -> DataCloudbuildTrigger_Github_Push {
        DataCloudbuildTrigger_Github_Push {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Github_PushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Github_PushRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Github_PushRef {
        DataCloudbuildTrigger_Github_PushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Github_PushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_BitbucketServerTriggerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_server_config_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_slug: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_BitbucketServerTriggerConfig {
    #[doc = "Set the field `bitbucket_server_config_resource`.\n"]
    pub fn set_bitbucket_server_config_resource(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bitbucket_server_config_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_key`.\n"]
    pub fn set_project_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
    ) -> Self {
        self.push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_slug`.\n"]
    pub fn set_repo_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_slug = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_BitbucketServerTriggerConfig {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_BitbucketServerTriggerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_BitbucketServerTriggerConfig {}

impl BuildDataCloudbuildTrigger_BitbucketServerTriggerConfig {
    pub fn build(self) -> DataCloudbuildTrigger_BitbucketServerTriggerConfig {
        DataCloudbuildTrigger_BitbucketServerTriggerConfig {
            bitbucket_server_config_resource: core::default::Default::default(),
            project_key: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            repo_slug: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_BitbucketServerTriggerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_BitbucketServerTriggerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_BitbucketServerTriggerConfigRef {
        DataCloudbuildTrigger_BitbucketServerTriggerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_BitbucketServerTriggerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitbucket_server_config_resource` after provisioning.\n"]
    pub fn bitbucket_server_config_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitbucket_server_config_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `project_key` after provisioning.\n"]
    pub fn project_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_slug` after provisioning.\n"]
    pub fn repo_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_slug", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts {
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

impl DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts {
    #[doc = "Set the field `artifact_id`.\n"]
    pub fn set_artifact_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.artifact_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Artifacts_MavenArtifacts {}

impl BuildDataCloudbuildTrigger_Build_Artifacts_MavenArtifacts {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts {
        DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts {
            artifact_id: core::default::Default::default(),
            group_id: core::default::Default::default(),
            path: core::default::Default::default(),
            repository: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
        DataCloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Artifacts_MavenArtifactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `artifact_id` after provisioning.\n"]
    pub fn artifact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_id", self.base))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Artifacts_NpmPackages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_Artifacts_NpmPackages {
    #[doc = "Set the field `package_path`.\n"]
    pub fn set_package_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.package_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Artifacts_NpmPackages {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Artifacts_NpmPackages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Artifacts_NpmPackages {}

impl BuildDataCloudbuildTrigger_Build_Artifacts_NpmPackages {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Artifacts_NpmPackages {
        DataCloudbuildTrigger_Build_Artifacts_NpmPackages {
            package_path: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
        DataCloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Artifacts_NpmPackagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `package_path` after provisioning.\n"]
    pub fn package_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_path", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Artifacts_Objects_Timing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_Artifacts_Objects_Timing {
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

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Artifacts_Objects_Timing {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Artifacts_Objects_Timing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Artifacts_Objects_Timing {}

impl BuildDataCloudbuildTrigger_Build_Artifacts_Objects_Timing {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Artifacts_Objects_Timing {
        DataCloudbuildTrigger_Build_Artifacts_Objects_Timing {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
        DataCloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Artifacts_Objects_TimingRef {
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
pub struct DataCloudbuildTrigger_Build_Artifacts_Objects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timing: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_Objects_Timing>>,
}

impl DataCloudbuildTrigger_Build_Artifacts_Objects {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.paths = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timing`.\n"]
    pub fn set_timing(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_Objects_Timing>>,
    ) -> Self {
        self.timing = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Artifacts_Objects {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Artifacts_Objects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Artifacts_Objects {}

impl BuildDataCloudbuildTrigger_Build_Artifacts_Objects {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Artifacts_Objects {
        DataCloudbuildTrigger_Build_Artifacts_Objects {
            location: core::default::Default::default(),
            paths: core::default::Default::default(),
            timing: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Artifacts_ObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Artifacts_ObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_Artifacts_ObjectsRef {
        DataCloudbuildTrigger_Build_Artifacts_ObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Artifacts_ObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc = "Get a reference to the value of field `timing` after provisioning.\n"]
    pub fn timing(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Artifacts_Objects_TimingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timing", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Artifacts_PythonPackages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_Artifacts_PythonPackages {
    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.paths = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Artifacts_PythonPackages {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Artifacts_PythonPackages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Artifacts_PythonPackages {}

impl BuildDataCloudbuildTrigger_Build_Artifacts_PythonPackages {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Artifacts_PythonPackages {
        DataCloudbuildTrigger_Build_Artifacts_PythonPackages {
            paths: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
        DataCloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Artifacts_PythonPackagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Artifacts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maven_artifacts: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    npm_packages: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_NpmPackages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_Objects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_packages: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_PythonPackages>>,
}

impl DataCloudbuildTrigger_Build_Artifacts {
    #[doc = "Set the field `images`.\n"]
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_MavenArtifacts>>,
    ) -> Self {
        self.maven_artifacts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `npm_packages`.\n"]
    pub fn set_npm_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_NpmPackages>>,
    ) -> Self {
        self.npm_packages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_Objects>>,
    ) -> Self {
        self.objects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_packages`.\n"]
    pub fn set_python_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts_PythonPackages>>,
    ) -> Self {
        self.python_packages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Artifacts {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Artifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Artifacts {}

impl BuildDataCloudbuildTrigger_Build_Artifacts {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Artifacts {
        DataCloudbuildTrigger_Build_Artifacts {
            images: core::default::Default::default(),
            maven_artifacts: core::default::Default::default(),
            npm_packages: core::default::Default::default(),
            objects: core::default::Default::default(),
            python_packages: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_ArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_ArtifactsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Build_ArtifactsRef {
        DataCloudbuildTrigger_Build_ArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_ArtifactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }

    #[doc = "Get a reference to the value of field `maven_artifacts` after provisioning.\n"]
    pub fn maven_artifacts(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Artifacts_MavenArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `npm_packages` after provisioning.\n"]
    pub fn npm_packages(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Artifacts_NpmPackagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.npm_packages", self.base))
    }

    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Artifacts_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }

    #[doc = "Get a reference to the value of field `python_packages` after provisioning.\n"]
    pub fn python_packages(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Artifacts_PythonPackagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_packages", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_name`.\n"]
    pub fn set_version_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_AvailableSecrets_SecretManager {}

impl BuildDataCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
    pub fn build(self) -> DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
        DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager {
            env: core::default::Default::default(),
            version_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
        DataCloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_AvailableSecrets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager>>,
}

impl DataCloudbuildTrigger_Build_AvailableSecrets {
    #[doc = "Set the field `secret_manager`.\n"]
    pub fn set_secret_manager(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_AvailableSecrets_SecretManager>>,
    ) -> Self {
        self.secret_manager = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_AvailableSecrets {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_AvailableSecrets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_AvailableSecrets {}

impl BuildDataCloudbuildTrigger_Build_AvailableSecrets {
    pub fn build(self) -> DataCloudbuildTrigger_Build_AvailableSecrets {
        DataCloudbuildTrigger_Build_AvailableSecrets { secret_manager: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_AvailableSecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_AvailableSecretsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_AvailableSecretsRef {
        DataCloudbuildTrigger_Build_AvailableSecretsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_AvailableSecretsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_manager` after provisioning.\n"]
    pub fn secret_manager(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_AvailableSecrets_SecretManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Step_Volumes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_Step_Volumes {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Step_Volumes {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Step_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Step_Volumes {}

impl BuildDataCloudbuildTrigger_Build_Step_Volumes {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Step_Volumes {
        DataCloudbuildTrigger_Build_Step_Volumes {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Step_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Step_VolumesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Build_Step_VolumesRef {
        DataCloudbuildTrigger_Build_Step_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Step_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Options {
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
    volumes: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Step_Volumes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_Options {
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynamic_substitutions`.\n"]
    pub fn set_dynamic_substitutions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dynamic_substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_streaming_option`.\n"]
    pub fn set_log_streaming_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_streaming_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requested_verify_option`.\n"]
    pub fn set_requested_verify_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.requested_verify_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secret_env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_provenance_hash`.\n"]
    pub fn set_source_provenance_hash(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_provenance_hash = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `substitution_option`.\n"]
    pub fn set_substitution_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.substitution_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Step_Volumes>>,
    ) -> Self {
        self.volumes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_pool`.\n"]
    pub fn set_worker_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Options {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Options>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Options {}

impl BuildDataCloudbuildTrigger_Build_Options {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Options {
        DataCloudbuildTrigger_Build_Options {
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
            volumes: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_OptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_OptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Build_OptionsRef {
        DataCloudbuildTrigger_Build_OptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_OptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamic_substitutions` after provisioning.\n"]
    pub fn dynamic_substitutions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dynamic_substitutions", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `log_streaming_option` after provisioning.\n"]
    pub fn log_streaming_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_streaming_option", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `requested_verify_option` after provisioning.\n"]
    pub fn requested_verify_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_verify_option", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\n"]
    pub fn secret_env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }

    #[doc = "Get a reference to the value of field `source_provenance_hash` after provisioning.\n"]
    pub fn source_provenance_hash(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_provenance_hash", self.base))
    }

    #[doc = "Get a reference to the value of field `substitution_option` after provisioning.\n"]
    pub fn substitution_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.substitution_option", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Step_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\n"]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Secret {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudbuildTrigger_Build_Secret {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secret_env = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Secret {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Secret {}

impl BuildDataCloudbuildTrigger_Build_Secret {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Secret {
        DataCloudbuildTrigger_Build_Secret {
            kms_key_name: core::default::Default::default(),
            secret_env: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_SecretRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Build_SecretRef {
        DataCloudbuildTrigger_Build_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_SecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\n"]
    pub fn secret_env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Source_RepoSource {
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
    substitutions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_Source_RepoSource {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `commit_sha`.\n"]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_name`.\n"]
    pub fn set_repo_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `substitutions`.\n"]
    pub fn set_substitutions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Source_RepoSource {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Source_RepoSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Source_RepoSource {}

impl BuildDataCloudbuildTrigger_Build_Source_RepoSource {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Source_RepoSource {
        DataCloudbuildTrigger_Build_Source_RepoSource {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            substitutions: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Source_RepoSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Source_RepoSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_Source_RepoSourceRef {
        DataCloudbuildTrigger_Build_Source_RepoSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Source_RepoSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\n"]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\n"]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }

    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\n"]
    pub fn substitutions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.substitutions", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Source_StorageSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build_Source_StorageSource {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generation`.\n"]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Source_StorageSource {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Source_StorageSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Source_StorageSource {}

impl BuildDataCloudbuildTrigger_Build_Source_StorageSource {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Source_StorageSource {
        DataCloudbuildTrigger_Build_Source_StorageSource {
            bucket: core::default::Default::default(),
            generation: core::default::Default::default(),
            object: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_Source_StorageSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_Source_StorageSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_Build_Source_StorageSourceRef {
        DataCloudbuildTrigger_Build_Source_StorageSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_Source_StorageSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `generation` after provisioning.\n"]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_source: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Source_RepoSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_source: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Source_StorageSource>>,
}

impl DataCloudbuildTrigger_Build_Source {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Source_RepoSource>>,
    ) -> Self {
        self.repo_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Source_StorageSource>>,
    ) -> Self {
        self.storage_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Source {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Source {}

impl BuildDataCloudbuildTrigger_Build_Source {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Source {
        DataCloudbuildTrigger_Build_Source {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Build_SourceRef {
        DataCloudbuildTrigger_Build_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Source_RepoSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Source_StorageSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build_Step {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_env: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Step_Volumes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudbuildTrigger_Build_Step {
    #[doc = "Set the field `allow_exit_codes`.\n"]
    pub fn set_allow_exit_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.allow_exit_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_failure`.\n"]
    pub fn set_allow_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `args`.\n"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entrypoint`.\n"]
    pub fn set_entrypoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script`.\n"]
    pub fn set_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secret_env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timing`.\n"]
    pub fn set_timing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Step_Volumes>>,
    ) -> Self {
        self.volumes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for`.\n"]
    pub fn set_wait_for(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.wait_for = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build_Step {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build_Step>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build_Step {}

impl BuildDataCloudbuildTrigger_Build_Step {
    pub fn build(self) -> DataCloudbuildTrigger_Build_Step {
        DataCloudbuildTrigger_Build_Step {
            allow_exit_codes: core::default::Default::default(),
            allow_failure: core::default::Default::default(),
            args: core::default::Default::default(),
            dir: core::default::Default::default(),
            entrypoint: core::default::Default::default(),
            env: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            script: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            timeout: core::default::Default::default(),
            timing: core::default::Default::default(),
            volumes: core::default::Default::default(),
            wait_for: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_Build_StepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_Build_StepRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_Build_StepRef {
        DataCloudbuildTrigger_Build_StepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_Build_StepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_exit_codes` after provisioning.\n"]
    pub fn allow_exit_codes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_exit_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_failure` after provisioning.\n"]
    pub fn allow_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entrypoint", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `script` after provisioning.\n"]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\n"]
    pub fn secret_env(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `timing` after provisioning.\n"]
    pub fn timing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timing", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_Step_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_for` after provisioning.\n"]
    pub fn wait_for(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wait_for", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Build {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifacts: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_secrets: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_AvailableSecrets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Options>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Secret>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Build_Step>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitutions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_Build {
    #[doc = "Set the field `artifacts`.\n"]
    pub fn set_artifacts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Artifacts>>,
    ) -> Self {
        self.artifacts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_secrets`.\n"]
    pub fn set_available_secrets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_AvailableSecrets>>,
    ) -> Self {
        self.available_secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `images`.\n"]
    pub fn set_images(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.images = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logs_bucket`.\n"]
    pub fn set_logs_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logs_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Options>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_ttl`.\n"]
    pub fn set_queue_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Secret>>,
    ) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Source>>,
    ) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step`.\n"]
    pub fn set_step(mut self, v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Build_Step>>) -> Self {
        self.step = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `substitutions`.\n"]
    pub fn set_substitutions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Build {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Build>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Build {}

impl BuildDataCloudbuildTrigger_Build {
    pub fn build(self) -> DataCloudbuildTrigger_Build {
        DataCloudbuildTrigger_Build {
            artifacts: core::default::Default::default(),
            available_secrets: core::default::Default::default(),
            images: core::default::Default::default(),
            logs_bucket: core::default::Default::default(),
            options: core::default::Default::default(),
            queue_ttl: core::default::Default::default(),
            secret: core::default::Default::default(),
            source: core::default::Default::default(),
            step: core::default::Default::default(),
            substitutions: core::default::Default::default(),
            tags: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_BuildRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_BuildRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_BuildRef {
        DataCloudbuildTrigger_BuildRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_BuildRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_ArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `available_secrets` after provisioning.\n"]
    pub fn available_secrets(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_AvailableSecretsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_secrets", self.base))
    }

    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }

    #[doc = "Get a reference to the value of field `logs_bucket` after provisioning.\n"]
    pub fn logs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logs_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_OptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_ttl` after provisioning.\n"]
    pub fn queue_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::ListRef<DataCloudbuildTrigger_Build_StepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step", self.base))
    }

    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\n"]
    pub fn substitutions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.substitutions", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_DeveloperConnectEventConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_repository_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_repository_link_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
}

impl DataCloudbuildTrigger_DeveloperConnectEventConfig {
    #[doc = "Set the field `git_repository_link`.\n"]
    pub fn set_git_repository_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.git_repository_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git_repository_link_type`.\n"]
    pub fn set_git_repository_link_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.git_repository_link_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
    ) -> Self {
        self.push = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_DeveloperConnectEventConfig {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_DeveloperConnectEventConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_DeveloperConnectEventConfig {}

impl BuildDataCloudbuildTrigger_DeveloperConnectEventConfig {
    pub fn build(self) -> DataCloudbuildTrigger_DeveloperConnectEventConfig {
        DataCloudbuildTrigger_DeveloperConnectEventConfig {
            git_repository_link: core::default::Default::default(),
            git_repository_link_type: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_DeveloperConnectEventConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_DeveloperConnectEventConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_DeveloperConnectEventConfigRef {
        DataCloudbuildTrigger_DeveloperConnectEventConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_DeveloperConnectEventConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `git_repository_link` after provisioning.\n"]
    pub fn git_repository_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_repository_link", self.base))
    }

    #[doc = "Get a reference to the value of field `git_repository_link_type` after provisioning.\n"]
    pub fn git_repository_link_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_repository_link_type", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_GitFileSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_server_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_GitFileSource {
    #[doc = "Set the field `bitbucket_server_config`.\n"]
    pub fn set_bitbucket_server_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bitbucket_server_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_enterprise_config`.\n"]
    pub fn set_github_enterprise_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.github_enterprise_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_type`.\n"]
    pub fn set_repo_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_GitFileSource {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_GitFileSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_GitFileSource {}

impl BuildDataCloudbuildTrigger_GitFileSource {
    pub fn build(self) -> DataCloudbuildTrigger_GitFileSource {
        DataCloudbuildTrigger_GitFileSource {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            path: core::default::Default::default(),
            repo_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            revision: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_GitFileSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_GitFileSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_GitFileSourceRef {
        DataCloudbuildTrigger_GitFileSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_GitFileSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitbucket_server_config` after provisioning.\n"]
    pub fn bitbucket_server_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitbucket_server_config", self.base))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.github_enterprise_config", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_type` after provisioning.\n"]
    pub fn repo_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enterprise_config_resource_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
}

impl DataCloudbuildTrigger_Github {
    #[doc = "Set the field `enterprise_config_resource_name`.\n"]
    pub fn set_enterprise_config_resource_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enterprise_config_resource_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
    ) -> Self {
        self.push = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_Github {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_Github {}

impl BuildDataCloudbuildTrigger_Github {
    pub fn build(self) -> DataCloudbuildTrigger_Github {
        DataCloudbuildTrigger_Github {
            enterprise_config_resource_name: core::default::Default::default(),
            name: core::default::Default::default(),
            owner: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_GithubRef {
        DataCloudbuildTrigger_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_GithubRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enterprise_config_resource_name` after provisioning.\n"]
    pub fn enterprise_config_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enterprise_config_resource_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_PubsubConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_PubsubConfig {
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscription`.\n"]
    pub fn set_subscription(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_PubsubConfig {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_PubsubConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_PubsubConfig {}

impl BuildDataCloudbuildTrigger_PubsubConfig {
    pub fn build(self) -> DataCloudbuildTrigger_PubsubConfig {
        DataCloudbuildTrigger_PubsubConfig {
            service_account_email: core::default::Default::default(),
            state: core::default::Default::default(),
            subscription: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_PubsubConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_PubsubConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_PubsubConfigRef {
        DataCloudbuildTrigger_PubsubConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_PubsubConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `subscription` after provisioning.\n"]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_RepositoryEventConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_RepositoryEventConfig {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_PullRequest>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTrigger_Github_Push>>,
    ) -> Self {
        self.push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_RepositoryEventConfig {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_RepositoryEventConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_RepositoryEventConfig {}

impl BuildDataCloudbuildTrigger_RepositoryEventConfig {
    pub fn build(self) -> DataCloudbuildTrigger_RepositoryEventConfig {
        DataCloudbuildTrigger_RepositoryEventConfig {
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_RepositoryEventConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_RepositoryEventConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTrigger_RepositoryEventConfigRef {
        DataCloudbuildTrigger_RepositoryEventConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_RepositoryEventConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<DataCloudbuildTrigger_Github_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_SourceToBuild {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_server_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_SourceToBuild {
    #[doc = "Set the field `bitbucket_server_config`.\n"]
    pub fn set_bitbucket_server_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bitbucket_server_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_enterprise_config`.\n"]
    pub fn set_github_enterprise_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.github_enterprise_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_type`.\n"]
    pub fn set_repo_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_SourceToBuild {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_SourceToBuild>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_SourceToBuild {}

impl BuildDataCloudbuildTrigger_SourceToBuild {
    pub fn build(self) -> DataCloudbuildTrigger_SourceToBuild {
        DataCloudbuildTrigger_SourceToBuild {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            ref_: core::default::Default::default(),
            repo_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_SourceToBuildRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_SourceToBuildRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_SourceToBuildRef {
        DataCloudbuildTrigger_SourceToBuildRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_SourceToBuildRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitbucket_server_config` after provisioning.\n"]
    pub fn bitbucket_server_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitbucket_server_config", self.base))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.github_enterprise_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_type` after provisioning.\n"]
    pub fn repo_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_TriggerTemplate {
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

impl DataCloudbuildTrigger_TriggerTemplate {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `commit_sha`.\n"]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_name`.\n"]
    pub fn set_repo_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_TriggerTemplate {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_TriggerTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_TriggerTemplate {}

impl BuildDataCloudbuildTrigger_TriggerTemplate {
    pub fn build(self) -> DataCloudbuildTrigger_TriggerTemplate {
        DataCloudbuildTrigger_TriggerTemplate {
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
pub struct DataCloudbuildTrigger_TriggerTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_TriggerTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_TriggerTemplateRef {
        DataCloudbuildTrigger_TriggerTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_TriggerTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\n"]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\n"]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTrigger_WebhookConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTrigger_WebhookConfig {
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTrigger_WebhookConfig {
    type O = terrars::BlockAssignable<DataCloudbuildTrigger_WebhookConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTrigger_WebhookConfig {}

impl BuildDataCloudbuildTrigger_WebhookConfig {
    pub fn build(self) -> DataCloudbuildTrigger_WebhookConfig {
        DataCloudbuildTrigger_WebhookConfig {
            secret: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildTrigger_WebhookConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTrigger_WebhookConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTrigger_WebhookConfigRef {
        DataCloudbuildTrigger_WebhookConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTrigger_WebhookConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
