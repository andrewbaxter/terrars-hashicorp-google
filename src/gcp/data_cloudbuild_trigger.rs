use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudbuildTriggerData {
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
    data: std::cell::RefCell<DataCloudbuildTriggerData>,
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
    pub fn approval_config(&self) -> terrars::ListRef<DataCloudbuildTriggerApprovalConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\nBitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received."]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> terrars::ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `build` after provisioning.\nContents of the build template. Either a filename or build template must be provided."]
    pub fn build(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `git_file_source` after provisioning.\nThe file source describing the local or remote Build template."]
    pub fn git_file_source(&self) -> terrars::ListRef<DataCloudbuildTriggerGitFileSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_file_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `github` after provisioning.\nDescribes the configuration of a trigger that creates a build whenever a GitHub event is received.\n\nOne of 'trigger_template', 'github', 'pubsub_config' or 'webhook_config' must be provided."]
    pub fn github(&self) -> terrars::ListRef<DataCloudbuildTriggerGithubElRef> {
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
        "Get a reference to the value of field `pubsub_config` after provisioning.\nPubsubConfig describes the configuration of a trigger that creates\na build whenever a Pub/Sub message is published.\n\nOne of 'trigger_template', 'github', 'pubsub_config' 'webhook_config' or 'source_to_build' must be provided."]
    pub fn pubsub_config(&self) -> terrars::ListRef<DataCloudbuildTriggerPubsubConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_event_config` after provisioning.\nThe configuration of a trigger that creates a build whenever an event from Repo API is received."]
    pub fn repository_event_config(&self) -> terrars::ListRef<DataCloudbuildTriggerRepositoryEventConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_event_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_to_build` after provisioning.\nThe repo and ref of the repository from which to build.\nThis field is used only for those triggers that do not respond to SCM events.\nTriggers that respond to such events build source at whatever commit caused the event.\nThis field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers.\n\nOne of 'trigger_template', 'github', 'pubsub_config' 'webhook_config' or 'source_to_build' must be provided."]
    pub fn source_to_build(&self) -> terrars::ListRef<DataCloudbuildTriggerSourceToBuildElRef> {
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
        "Get a reference to the value of field `trigger_template` after provisioning.\nTemplate describing the types of source changes to trigger a build.\n\nBranch and tag names in trigger templates are interpreted as regular\nexpressions. Any branch or tag change that matches that regular\nexpression will trigger a build.\n\nOne of 'trigger_template', 'github', 'pubsub_config', 'webhook_config' or 'source_to_build' must be provided."]
    pub fn trigger_template(&self) -> terrars::ListRef<DataCloudbuildTriggerTriggerTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `webhook_config` after provisioning.\nWebhookConfig describes the configuration of a trigger that creates\na build whenever a webhook is sent to a trigger's webhook URL.\n\nOne of 'trigger_template', 'github', 'pubsub_config' 'webhook_config' or 'source_to_build' must be provided."]
    pub fn webhook_config(&self) -> terrars::ListRef<DataCloudbuildTriggerWebhookConfigElRef> {
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
            data: std::cell::RefCell::new(DataCloudbuildTriggerData {
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
    pub fn approval_config(&self) -> terrars::ListRef<DataCloudbuildTriggerApprovalConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\nBitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received."]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> terrars::ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `build` after provisioning.\nContents of the build template. Either a filename or build template must be provided."]
    pub fn build(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `git_file_source` after provisioning.\nThe file source describing the local or remote Build template."]
    pub fn git_file_source(&self) -> terrars::ListRef<DataCloudbuildTriggerGitFileSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_file_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `github` after provisioning.\nDescribes the configuration of a trigger that creates a build whenever a GitHub event is received.\n\nOne of 'trigger_template', 'github', 'pubsub_config' or 'webhook_config' must be provided."]
    pub fn github(&self) -> terrars::ListRef<DataCloudbuildTriggerGithubElRef> {
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
        "Get a reference to the value of field `pubsub_config` after provisioning.\nPubsubConfig describes the configuration of a trigger that creates\na build whenever a Pub/Sub message is published.\n\nOne of 'trigger_template', 'github', 'pubsub_config' 'webhook_config' or 'source_to_build' must be provided."]
    pub fn pubsub_config(&self) -> terrars::ListRef<DataCloudbuildTriggerPubsubConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_event_config` after provisioning.\nThe configuration of a trigger that creates a build whenever an event from Repo API is received."]
    pub fn repository_event_config(&self) -> terrars::ListRef<DataCloudbuildTriggerRepositoryEventConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_event_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_to_build` after provisioning.\nThe repo and ref of the repository from which to build.\nThis field is used only for those triggers that do not respond to SCM events.\nTriggers that respond to such events build source at whatever commit caused the event.\nThis field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers.\n\nOne of 'trigger_template', 'github', 'pubsub_config' 'webhook_config' or 'source_to_build' must be provided."]
    pub fn source_to_build(&self) -> terrars::ListRef<DataCloudbuildTriggerSourceToBuildElRef> {
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
        "Get a reference to the value of field `trigger_template` after provisioning.\nTemplate describing the types of source changes to trigger a build.\n\nBranch and tag names in trigger templates are interpreted as regular\nexpressions. Any branch or tag change that matches that regular\nexpression will trigger a build.\n\nOne of 'trigger_template', 'github', 'pubsub_config', 'webhook_config' or 'source_to_build' must be provided."]
    pub fn trigger_template(&self) -> terrars::ListRef<DataCloudbuildTriggerTriggerTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `webhook_config` after provisioning.\nWebhookConfig describes the configuration of a trigger that creates\na build whenever a webhook is sent to a trigger's webhook URL.\n\nOne of 'trigger_template', 'github', 'pubsub_config' 'webhook_config' or 'source_to_build' must be provided."]
    pub fn webhook_config(&self) -> terrars::ListRef<DataCloudbuildTriggerWebhookConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerApprovalConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_required: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildTriggerApprovalConfigEl {
    #[doc = "Set the field `approval_required`.\n"]
    pub fn set_approval_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.approval_required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTriggerApprovalConfigEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerApprovalConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerApprovalConfigEl {}

impl BuildDataCloudbuildTriggerApprovalConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerApprovalConfigEl {
        DataCloudbuildTriggerApprovalConfigEl { approval_required: core::default::Default::default() }
    }
}

pub struct DataCloudbuildTriggerApprovalConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerApprovalConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerApprovalConfigElRef {
        DataCloudbuildTriggerApprovalConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerApprovalConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approval_required` after provisioning.\n"]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {}

impl BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    pub fn build(self) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
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
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {}

impl BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    pub fn build(self) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
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
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket_server_config_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std
    ::option
    ::Option<terrars::ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_slug: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl>>,
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

impl terrars::ToListMappable for DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBitbucketServerTriggerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBitbucketServerTriggerConfigEl {}

impl BuildDataCloudbuildTriggerBitbucketServerTriggerConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
        DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
            bitbucket_server_config_resource: core::default::Default::default(),
            project_key: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            repo_slug: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
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
    pub fn pull_request(&self) -> terrars::ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_slug` after provisioning.\n"]
    pub fn repo_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_slug", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
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

impl DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {}

impl BuildDataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
        DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
            artifact_id: core::default::Default::default(),
            group_id: core::default::Default::default(),
            path: core::default::Default::default(),
            repository: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
        DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
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
pub struct DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {}

impl BuildDataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
        DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
            package_path: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
        DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
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
pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {}

impl BuildDataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
        DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
        DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
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
pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timing: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl>>,
}

impl DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl>>,
    ) -> Self {
        self.timing = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElArtifactsElObjectsEl {}

impl BuildDataCloudbuildTriggerBuildElArtifactsElObjectsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
        DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
            location: core::default::Default::default(),
            paths: core::default::Default::default(),
            timing: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
        DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
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
    pub fn timing(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timing", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {}

impl BuildDataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
        DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
            paths: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
        DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
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
pub struct DataCloudbuildTriggerBuildElArtifactsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maven_artifacts: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    npm_packages: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_packages: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl>>,
}

impl DataCloudbuildTriggerBuildElArtifactsEl {
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>>,
    ) -> Self {
        self.maven_artifacts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `npm_packages`.\n"]
    pub fn set_npm_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl>>,
    ) -> Self {
        self.npm_packages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsEl>>,
    ) -> Self {
        self.objects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_packages`.\n"]
    pub fn set_python_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl>>,
    ) -> Self {
        self.python_packages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElArtifactsEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElArtifactsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElArtifactsEl {}

impl BuildDataCloudbuildTriggerBuildElArtifactsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsEl {
        DataCloudbuildTriggerBuildElArtifactsEl {
            images: core::default::Default::default(),
            maven_artifacts: core::default::Default::default(),
            npm_packages: core::default::Default::default(),
            objects: core::default::Default::default(),
            python_packages: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElArtifactsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElArtifactsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerBuildElArtifactsElRef {
        DataCloudbuildTriggerBuildElArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElArtifactsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }

    #[doc = "Get a reference to the value of field `maven_artifacts` after provisioning.\n"]
    pub fn maven_artifacts(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maven_artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `npm_packages` after provisioning.\n"]
    pub fn npm_packages(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.npm_packages", self.base))
    }

    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElArtifactsElObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }

    #[doc = "Get a reference to the value of field `python_packages` after provisioning.\n"]
    pub fn python_packages(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_packages", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {}

impl BuildDataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
        DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
            env: core::default::Default::default(),
            version_name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
        DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
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
pub struct DataCloudbuildTriggerBuildElAvailableSecretsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager: std
    ::option
    ::Option<terrars::ListField<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>>,
}

impl DataCloudbuildTriggerBuildElAvailableSecretsEl {
    #[doc = "Set the field `secret_manager`.\n"]
    pub fn set_secret_manager(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>>,
    ) -> Self {
        self.secret_manager = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElAvailableSecretsEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElAvailableSecretsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElAvailableSecretsEl {}

impl BuildDataCloudbuildTriggerBuildElAvailableSecretsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElAvailableSecretsEl {
        DataCloudbuildTriggerBuildElAvailableSecretsEl { secret_manager: core::default::Default::default() }
    }
}

pub struct DataCloudbuildTriggerBuildElAvailableSecretsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElAvailableSecretsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElAvailableSecretsElRef {
        DataCloudbuildTriggerBuildElAvailableSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElAvailableSecretsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_manager` after provisioning.\n"]
    pub fn secret_manager(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElOptionsElVolumesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElOptionsElVolumesEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElOptionsElVolumesEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElOptionsElVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElOptionsElVolumesEl {}

impl BuildDataCloudbuildTriggerBuildElOptionsElVolumesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElOptionsElVolumesEl {
        DataCloudbuildTriggerBuildElOptionsElVolumesEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
        DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
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
pub struct DataCloudbuildTriggerBuildElOptionsEl {
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
    volumes: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElOptionsElVolumesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElOptionsEl {
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElOptionsElVolumesEl>>,
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElOptionsEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElOptionsEl {}

impl BuildDataCloudbuildTriggerBuildElOptionsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElOptionsEl {
        DataCloudbuildTriggerBuildElOptionsEl {
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

pub struct DataCloudbuildTriggerBuildElOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerBuildElOptionsElRef {
        DataCloudbuildTriggerBuildElOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElOptionsElRef {
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
    pub fn volumes(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElOptionsElVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\n"]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElSecretEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudbuildTriggerBuildElSecretEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElSecretEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElSecretEl {}

impl BuildDataCloudbuildTriggerBuildElSecretEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSecretEl {
        DataCloudbuildTriggerBuildElSecretEl {
            kms_key_name: core::default::Default::default(),
            secret_env: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElSecretElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerBuildElSecretElRef {
        DataCloudbuildTriggerBuildElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElSecretElRef {
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
pub struct DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
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

impl DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElSourceElRepoSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElSourceElRepoSourceEl {}

impl BuildDataCloudbuildTriggerBuildElSourceElRepoSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
        DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
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

pub struct DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
        DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
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
pub struct DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElSourceElStorageSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElSourceElStorageSourceEl {}

impl BuildDataCloudbuildTriggerBuildElSourceElStorageSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
        DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
            bucket: core::default::Default::default(),
            generation: core::default::Default::default(),
            object: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
        DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
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
pub struct DataCloudbuildTriggerBuildElSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_source: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElSourceElRepoSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_source: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElSourceElStorageSourceEl>>,
}

impl DataCloudbuildTriggerBuildElSourceEl {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElSourceElRepoSourceEl>>,
    ) -> Self {
        self.repo_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElSourceElStorageSourceEl>>,
    ) -> Self {
        self.storage_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElSourceEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElSourceEl {}

impl BuildDataCloudbuildTriggerBuildElSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSourceEl {
        DataCloudbuildTriggerBuildElSourceEl {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerBuildElSourceElRef {
        DataCloudbuildTriggerBuildElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElSourceElRepoSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElSourceElStorageSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElStepElVolumesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildElStepElVolumesEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElStepElVolumesEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElStepElVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElStepElVolumesEl {}

impl BuildDataCloudbuildTriggerBuildElStepElVolumesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElStepElVolumesEl {
        DataCloudbuildTriggerBuildElStepElVolumesEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerBuildElStepElVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElStepElVolumesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerBuildElStepElVolumesElRef {
        DataCloudbuildTriggerBuildElStepElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElStepElVolumesElRef {
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
pub struct DataCloudbuildTriggerBuildElStepEl {
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
    volumes: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElStepElVolumesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudbuildTriggerBuildElStepEl {
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElStepElVolumesEl>>,
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildElStepEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildElStepEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildElStepEl {}

impl BuildDataCloudbuildTriggerBuildElStepEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElStepEl {
        DataCloudbuildTriggerBuildElStepEl {
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

pub struct DataCloudbuildTriggerBuildElStepElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElStepElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerBuildElStepElRef {
        DataCloudbuildTriggerBuildElStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElStepElRef {
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
    pub fn volumes(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElStepElVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_for` after provisioning.\n"]
    pub fn wait_for(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wait_for", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifacts: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_secrets: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElAvailableSecretsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElSecretEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step: std::option::Option<terrars::ListField<DataCloudbuildTriggerBuildElStepEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitutions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerBuildEl {
    #[doc = "Set the field `artifacts`.\n"]
    pub fn set_artifacts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElArtifactsEl>>,
    ) -> Self {
        self.artifacts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_secrets`.\n"]
    pub fn set_available_secrets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElAvailableSecretsEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElOptionsEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElSecretEl>>,
    ) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElSourceEl>>,
    ) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step`.\n"]
    pub fn set_step(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerBuildElStepEl>>,
    ) -> Self {
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

impl terrars::ToListMappable for DataCloudbuildTriggerBuildEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerBuildEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerBuildEl {}

impl BuildDataCloudbuildTriggerBuildEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildEl {
        DataCloudbuildTriggerBuildEl {
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

pub struct DataCloudbuildTriggerBuildElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerBuildElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerBuildElRef {
        DataCloudbuildTriggerBuildElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerBuildElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElArtifactsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `available_secrets` after provisioning.\n"]
    pub fn available_secrets(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElAvailableSecretsElRef> {
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
    pub fn options(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_ttl` after provisioning.\n"]
    pub fn queue_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElSecretElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::ListRef<DataCloudbuildTriggerBuildElStepElRef> {
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
pub struct DataCloudbuildTriggerGitFileSourceEl {
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

impl DataCloudbuildTriggerGitFileSourceEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerGitFileSourceEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerGitFileSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerGitFileSourceEl {}

impl BuildDataCloudbuildTriggerGitFileSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerGitFileSourceEl {
        DataCloudbuildTriggerGitFileSourceEl {
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

pub struct DataCloudbuildTriggerGitFileSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerGitFileSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerGitFileSourceElRef {
        DataCloudbuildTriggerGitFileSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerGitFileSourceElRef {
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
pub struct DataCloudbuildTriggerGithubElPullRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildTriggerGithubElPullRequestEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerGithubElPullRequestEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerGithubElPullRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerGithubElPullRequestEl {}

impl BuildDataCloudbuildTriggerGithubElPullRequestEl {
    pub fn build(self) -> DataCloudbuildTriggerGithubElPullRequestEl {
        DataCloudbuildTriggerGithubElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerGithubElPullRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerGithubElPullRequestElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerGithubElPullRequestElRef {
        DataCloudbuildTriggerGithubElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerGithubElPullRequestElRef {
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
pub struct DataCloudbuildTriggerGithubElPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerGithubElPushEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerGithubElPushEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerGithubElPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerGithubElPushEl {}

impl BuildDataCloudbuildTriggerGithubElPushEl {
    pub fn build(self) -> DataCloudbuildTriggerGithubElPushEl {
        DataCloudbuildTriggerGithubElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerGithubElPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerGithubElPushElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerGithubElPushElRef {
        DataCloudbuildTriggerGithubElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerGithubElPushElRef {
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
pub struct DataCloudbuildTriggerGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enterprise_config_resource_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<terrars::ListField<DataCloudbuildTriggerGithubElPullRequestEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<DataCloudbuildTriggerGithubElPushEl>>,
}

impl DataCloudbuildTriggerGithubEl {
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
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerGithubElPullRequestEl>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerGithubElPushEl>>,
    ) -> Self {
        self.push = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildTriggerGithubEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerGithubEl {}

impl BuildDataCloudbuildTriggerGithubEl {
    pub fn build(self) -> DataCloudbuildTriggerGithubEl {
        DataCloudbuildTriggerGithubEl {
            enterprise_config_resource_name: core::default::Default::default(),
            name: core::default::Default::default(),
            owner: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerGithubElRef {
        DataCloudbuildTriggerGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerGithubElRef {
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
    pub fn pull_request(&self) -> terrars::ListRef<DataCloudbuildTriggerGithubElPullRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<DataCloudbuildTriggerGithubElPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerPubsubConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerPubsubConfigEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerPubsubConfigEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerPubsubConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerPubsubConfigEl {}

impl BuildDataCloudbuildTriggerPubsubConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerPubsubConfigEl {
        DataCloudbuildTriggerPubsubConfigEl {
            service_account_email: core::default::Default::default(),
            state: core::default::Default::default(),
            subscription: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerPubsubConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerPubsubConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerPubsubConfigElRef {
        DataCloudbuildTriggerPubsubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerPubsubConfigElRef {
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
pub struct DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {}

impl BuildDataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    pub fn build(self) -> DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
        DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
        DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
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
pub struct DataCloudbuildTriggerRepositoryEventConfigElPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerRepositoryEventConfigElPushEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerRepositoryEventConfigElPushEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerRepositoryEventConfigElPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerRepositoryEventConfigElPushEl {}

impl BuildDataCloudbuildTriggerRepositoryEventConfigElPushEl {
    pub fn build(self) -> DataCloudbuildTriggerRepositoryEventConfigElPushEl {
        DataCloudbuildTriggerRepositoryEventConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
        DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
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
pub struct DataCloudbuildTriggerRepositoryEventConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<terrars::ListField<DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<DataCloudbuildTriggerRepositoryEventConfigElPushEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerRepositoryEventConfigEl {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudbuildTriggerRepositoryEventConfigElPushEl>>,
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

impl terrars::ToListMappable for DataCloudbuildTriggerRepositoryEventConfigEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerRepositoryEventConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerRepositoryEventConfigEl {}

impl BuildDataCloudbuildTriggerRepositoryEventConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerRepositoryEventConfigEl {
        DataCloudbuildTriggerRepositoryEventConfigEl {
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerRepositoryEventConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerRepositoryEventConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildTriggerRepositoryEventConfigElRef {
        DataCloudbuildTriggerRepositoryEventConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerRepositoryEventConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<DataCloudbuildTriggerRepositoryEventConfigElPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildTriggerSourceToBuildEl {
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

impl DataCloudbuildTriggerSourceToBuildEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerSourceToBuildEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerSourceToBuildEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerSourceToBuildEl {}

impl BuildDataCloudbuildTriggerSourceToBuildEl {
    pub fn build(self) -> DataCloudbuildTriggerSourceToBuildEl {
        DataCloudbuildTriggerSourceToBuildEl {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            ref_: core::default::Default::default(),
            repo_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerSourceToBuildElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerSourceToBuildElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerSourceToBuildElRef {
        DataCloudbuildTriggerSourceToBuildElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerSourceToBuildElRef {
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
pub struct DataCloudbuildTriggerTriggerTemplateEl {
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

impl DataCloudbuildTriggerTriggerTemplateEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerTriggerTemplateEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerTriggerTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerTriggerTemplateEl {}

impl BuildDataCloudbuildTriggerTriggerTemplateEl {
    pub fn build(self) -> DataCloudbuildTriggerTriggerTemplateEl {
        DataCloudbuildTriggerTriggerTemplateEl {
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

pub struct DataCloudbuildTriggerTriggerTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerTriggerTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerTriggerTemplateElRef {
        DataCloudbuildTriggerTriggerTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerTriggerTemplateElRef {
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
pub struct DataCloudbuildTriggerWebhookConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildTriggerWebhookConfigEl {
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

impl terrars::ToListMappable for DataCloudbuildTriggerWebhookConfigEl {
    type O = terrars::BlockAssignable<DataCloudbuildTriggerWebhookConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildTriggerWebhookConfigEl {}

impl BuildDataCloudbuildTriggerWebhookConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerWebhookConfigEl {
        DataCloudbuildTriggerWebhookConfigEl {
            secret: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct DataCloudbuildTriggerWebhookConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildTriggerWebhookConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildTriggerWebhookConfigElRef {
        DataCloudbuildTriggerWebhookConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildTriggerWebhookConfigElRef {
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
