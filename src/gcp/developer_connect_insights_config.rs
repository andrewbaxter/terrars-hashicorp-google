#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DeveloperConnectInsightsConfigData_ {
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
    app_hub_application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    insights_config_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_configs: std::option::Option<std::vec::Vec<DeveloperConnectInsightsConfig_ArtifactConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_projects: std::option::Option<std::vec::Vec<DeveloperConnectInsightsConfig_TargetProjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DeveloperConnectInsightsConfig_Timeouts>,
    dynamic: DeveloperConnectInsightsConfigDynamic,
}

struct DeveloperConnectInsightsConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DeveloperConnectInsightsConfigData_>,
}

#[derive(Clone)]
pub struct DeveloperConnectInsightsConfig(std::rc::Rc<DeveloperConnectInsightsConfig_>);

impl DeveloperConnectInsightsConfig {
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
        "Set the field `annotations`.\nUser specified annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `app_hub_application`.\nThe name of the App Hub Application.\nFormat:\nprojects/{project}/locations/{location}/applications/{application}"]
    pub fn set_app_hub_application(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().app_hub_application = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nSet of labels associated with an InsightsConfig.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `artifact_configs`.\n"]
    pub fn set_artifact_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectInsightsConfig_ArtifactConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_projects`.\n"]
    pub fn set_target_projects(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectInsightsConfig_TargetProjects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_projects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_projects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DeveloperConnectInsightsConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_hub_application` after provisioning.\nThe name of the App Hub Application.\nFormat:\nprojects/{project}/locations/{location}/applications/{application}"]
    pub fn app_hub_application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_hub_application", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `errors` after provisioning.\nAny errors that occurred while setting up the InsightsConfig.\nEach error will be in the format: 'field_name: error_message', e.g.\nGetAppHubApplication: Permission denied while getting App Hub\napplication. Please grant permissions to the P4SA."]
    pub fn errors(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `insights_config_id` after provisioning.\nID of the requesting InsightsConfig."]
    pub fn insights_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insights_config_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with an InsightsConfig.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the InsightsConfig.\nFormat:\nprojects/{project}/locations/{location}/insightsConfigs/{insightsConfig}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of InsightsConfig does not match the\nuser's intended state, and the service is actively updating the resource to\nreconcile them. This can happen due to user-triggered updates or\nsystem actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_configs` after provisioning.\nThe runtime configurations where the application is deployed."]
    pub fn runtime_configs(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_RuntimeConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the InsightsConfig.\nPossible values:\nSTATE_UNSPECIFIED\nPENDING\nCOMPLETE\nERROR"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_configs` after provisioning.\n"]
    pub fn artifact_configs(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_ArtifactConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifact_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_projects` after provisioning.\n"]
    pub fn target_projects(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_TargetProjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_projects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectInsightsConfig_TimeoutsRef {
        DeveloperConnectInsightsConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DeveloperConnectInsightsConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DeveloperConnectInsightsConfig { }

impl terrars::ToListMappable for DeveloperConnectInsightsConfig {
    type O = terrars::ListRef<DeveloperConnectInsightsConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DeveloperConnectInsightsConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_developer_connect_insights_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDeveloperConnectInsightsConfig {
    pub tf_id: std::string::String,
    #[doc = "ID of the requesting InsightsConfig."]
    pub insights_config_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectInsightsConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DeveloperConnectInsightsConfig {
        let out = DeveloperConnectInsightsConfig(std::rc::Rc::new(DeveloperConnectInsightsConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DeveloperConnectInsightsConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                app_hub_application: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                insights_config_id: self.insights_config_id,
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                artifact_configs: core::default::Default::default(),
                target_projects: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DeveloperConnectInsightsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DeveloperConnectInsightsConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_hub_application` after provisioning.\nThe name of the App Hub Application.\nFormat:\nprojects/{project}/locations/{location}/applications/{application}"]
    pub fn app_hub_application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_hub_application", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `errors` after provisioning.\nAny errors that occurred while setting up the InsightsConfig.\nEach error will be in the format: 'field_name: error_message', e.g.\nGetAppHubApplication: Permission denied while getting App Hub\napplication. Please grant permissions to the P4SA."]
    pub fn errors(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `insights_config_id` after provisioning.\nID of the requesting InsightsConfig."]
    pub fn insights_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insights_config_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with an InsightsConfig.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the InsightsConfig.\nFormat:\nprojects/{project}/locations/{location}/insightsConfigs/{insightsConfig}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of InsightsConfig does not match the\nuser's intended state, and the service is actively updating the resource to\nreconcile them. This can happen due to user-triggered updates or\nsystem actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_configs` after provisioning.\nThe runtime configurations where the application is deployed."]
    pub fn runtime_configs(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_RuntimeConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the InsightsConfig.\nPossible values:\nSTATE_UNSPECIFIED\nPENDING\nCOMPLETE\nERROR"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_configs` after provisioning.\n"]
    pub fn artifact_configs(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_ArtifactConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifact_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_projects` after provisioning.\n"]
    pub fn target_projects(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_TargetProjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_projects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectInsightsConfig_TimeoutsRef {
        DeveloperConnectInsightsConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectInsightsConfigDynamic {
    artifact_configs: std::option::Option<terrars::DynamicBlock<DeveloperConnectInsightsConfig_ArtifactConfigs>>,
    target_projects: std::option::Option<terrars::DynamicBlock<DeveloperConnectInsightsConfig_TargetProjects>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_Errors_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detail_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectInsightsConfig_Errors_Details {
    #[doc = "Set the field `detail_message`.\n"]
    pub fn set_detail_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.detail_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_Errors_Details {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_Errors_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_Errors_Details {}

impl BuildDeveloperConnectInsightsConfig_Errors_Details {
    pub fn build(self) -> DeveloperConnectInsightsConfig_Errors_Details {
        DeveloperConnectInsightsConfig_Errors_Details { detail_message: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_Errors_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_Errors_DetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_Errors_DetailsRef {
        DeveloperConnectInsightsConfig_Errors_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_Errors_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `detail_message` after provisioning.\n"]
    pub fn detail_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detail_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<DeveloperConnectInsightsConfig_Errors_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectInsightsConfig_Errors {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DeveloperConnectInsightsConfig_Errors_Details>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_Errors {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_Errors {}

impl BuildDeveloperConnectInsightsConfig_Errors {
    pub fn build(self) -> DeveloperConnectInsightsConfig_Errors {
        DeveloperConnectInsightsConfig_Errors {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_ErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectInsightsConfig_ErrorsRef {
        DeveloperConnectInsightsConfig_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_ErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_Errors_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criticality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload {
    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.criticality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workload`.\n"]
    pub fn set_workload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload {}

impl BuildDeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload {
    pub fn build(self) -> DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload {
        DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload {
            criticality: core::default::Default::default(),
            environment: core::default::Default::default(),
            workload: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkloadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkloadRef {
        DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkloadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.criticality", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `workload` after provisioning.\n"]
    pub fn workload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload {}

impl BuildDeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload {
    pub fn build(self) -> DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload {
        DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload {
            cluster: core::default::Default::default(),
            deployment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkloadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkloadRef {
        DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkloadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_RuntimeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_hub_workload: std
    ::option
    ::Option<terrars::ListField<DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_workload: std::option::Option<terrars::ListField<DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectInsightsConfig_RuntimeConfigs {
    #[doc = "Set the field `app_hub_workload`.\n"]
    pub fn set_app_hub_workload(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkload>>,
    ) -> Self {
        self.app_hub_workload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_workload`.\n"]
    pub fn set_gke_workload(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkload>>,
    ) -> Self {
        self.gke_workload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_RuntimeConfigs {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_RuntimeConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_RuntimeConfigs {}

impl BuildDeveloperConnectInsightsConfig_RuntimeConfigs {
    pub fn build(self) -> DeveloperConnectInsightsConfig_RuntimeConfigs {
        DeveloperConnectInsightsConfig_RuntimeConfigs {
            app_hub_workload: core::default::Default::default(),
            gke_workload: core::default::Default::default(),
            state: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_RuntimeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_RuntimeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_RuntimeConfigsRef {
        DeveloperConnectInsightsConfig_RuntimeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_RuntimeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_hub_workload` after provisioning.\n"]
    pub fn app_hub_workload(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_RuntimeConfigs_AppHubWorkloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_hub_workload", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_workload` after provisioning.\n"]
    pub fn gke_workload(&self) -> terrars::ListRef<DeveloperConnectInsightsConfig_RuntimeConfigs_GkeWorkloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_workload", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis {
    project_id: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis { }

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis {
    #[doc = "The project id of the project where the provenance is stored."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis {
    pub fn build(self) -> DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis {
        DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis { project_id: self.project_id }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysisRef {
        DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project id of the project where the provenance is stored."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry {
    artifact_registry_package: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry { }

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry {
    #[doc = "The name of the artifact registry package."]
    pub artifact_registry_package: terrars::PrimField<std::string::String>,
    #[doc = "The host project of Artifact Registry."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry {
    pub fn build(self) -> DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry {
        DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry {
            artifact_registry_package: self.artifact_registry_package,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistryRef {
        DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `artifact_registry_package` after provisioning.\nThe name of the artifact registry package."]
    pub fn artifact_registry_package(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_registry_package", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe host project of Artifact Registry."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectInsightsConfig_ArtifactConfigsDynamic {
    google_artifact_analysis: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis>>,
    google_artifact_registry: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_ArtifactConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_artifact_analysis: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_artifact_registry: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry>>,
    dynamic: DeveloperConnectInsightsConfig_ArtifactConfigsDynamic,
}

impl DeveloperConnectInsightsConfig_ArtifactConfigs {
    #[doc =
        "Set the field `uri`.\nThe URI of the artifact that is deployed.\ne.g. 'us-docker.pkg.dev/my-project/my-repo/image'.\nThe URI does not include the tag / digest because it captures a lineage of\nartifacts."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_artifact_analysis`.\n"]
    pub fn set_google_artifact_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysis>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_artifact_analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_artifact_analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_artifact_registry`.\n"]
    pub fn set_google_artifact_registry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistry>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_artifact_registry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_artifact_registry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_ArtifactConfigs {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_ArtifactConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_ArtifactConfigs {}

impl BuildDeveloperConnectInsightsConfig_ArtifactConfigs {
    pub fn build(self) -> DeveloperConnectInsightsConfig_ArtifactConfigs {
        DeveloperConnectInsightsConfig_ArtifactConfigs {
            uri: core::default::Default::default(),
            google_artifact_analysis: core::default::Default::default(),
            google_artifact_registry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_ArtifactConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_ArtifactConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_ArtifactConfigsRef {
        DeveloperConnectInsightsConfig_ArtifactConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_ArtifactConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe URI of the artifact that is deployed.\ne.g. 'us-docker.pkg.dev/my-project/my-repo/image'.\nThe URI does not include the tag / digest because it captures a lineage of\nartifacts."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `google_artifact_analysis` after provisioning.\n"]
    pub fn google_artifact_analysis(
        &self,
    ) -> terrars::ListRef<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactAnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_artifact_analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `google_artifact_registry` after provisioning.\n"]
    pub fn google_artifact_registry(
        &self,
    ) -> terrars::ListRef<DeveloperConnectInsightsConfig_ArtifactConfigs_GoogleArtifactRegistryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_artifact_registry", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_TargetProjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DeveloperConnectInsightsConfig_TargetProjects {
    #[doc = "Set the field `project_ids`.\nThe project IDs. Format {project}."]
    pub fn set_project_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.project_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_TargetProjects {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_TargetProjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_TargetProjects {}

impl BuildDeveloperConnectInsightsConfig_TargetProjects {
    pub fn build(self) -> DeveloperConnectInsightsConfig_TargetProjects {
        DeveloperConnectInsightsConfig_TargetProjects { project_ids: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_TargetProjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_TargetProjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectInsightsConfig_TargetProjectsRef {
        DeveloperConnectInsightsConfig_TargetProjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_TargetProjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project_ids` after provisioning.\nThe project IDs. Format {project}."]
    pub fn project_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectInsightsConfig_Timeouts {
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

impl terrars::ToListMappable for DeveloperConnectInsightsConfig_Timeouts {
    type O = terrars::BlockAssignable<DeveloperConnectInsightsConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectInsightsConfig_Timeouts {}

impl BuildDeveloperConnectInsightsConfig_Timeouts {
    pub fn build(self) -> DeveloperConnectInsightsConfig_Timeouts {
        DeveloperConnectInsightsConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectInsightsConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectInsightsConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectInsightsConfig_TimeoutsRef {
        DeveloperConnectInsightsConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectInsightsConfig_TimeoutsRef {
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
