#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OsConfigV2PolicyOrchestratorData_ {
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
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    policy_orchestrator_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orchestrated_resource: std::option::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestratedResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orchestration_scope: std::option::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestrationScope>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OsConfigV2PolicyOrchestrator_Timeouts>,
    dynamic: OsConfigV2PolicyOrchestratorDynamic,
}

struct OsConfigV2PolicyOrchestrator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OsConfigV2PolicyOrchestratorData_>,
}

#[derive(Clone)]
pub struct OsConfigV2PolicyOrchestrator(std::rc::Rc<OsConfigV2PolicyOrchestrator_>);

impl OsConfigV2PolicyOrchestrator {
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

    #[doc = "Set the field `description`.\nOptional. Freeform text describing the purpose of the resource."]
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

    #[doc =
        "Set the field `state`.\nOptional. State of the orchestrator. Can be updated to change orchestrator behaviour.\nAllowed values:\n- 'ACTIVE' - orchestrator is actively looking for actions to be taken.\n- 'STOPPED' - orchestrator won't make any changes.\n\nNote: There might be more states added in the future. We use string here\ninstead of an enum, to avoid the need of propagating new states to all the\nclient code."]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orchestrated_resource`.\n"]
    pub fn set_orchestrated_resource(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestratedResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().orchestrated_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.orchestrated_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `orchestration_scope`.\n"]
    pub fn set_orchestration_scope(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationScope>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().orchestration_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.orchestration_scope = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OsConfigV2PolicyOrchestrator_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nRequired. Action to be done by the orchestrator in\n'projects/{project_id}/zones/{zone_id}' locations defined by the\n'orchestration_scope'. Allowed values:\n- 'UPSERT' - Orchestrator will create or update target resources.\n- 'DELETE' - Orchestrator will delete target resources, if they exist"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Freeform text describing the purpose of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. Identifier. In form of\n* 'organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `orchestration_state` after provisioning.\nDescribes the state of the orchestration process."]
    pub fn orchestration_state(&self) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestration_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_orchestrator_id` after provisioning.\nRequired. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub fn policy_orchestrator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_orchestrator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true, if the there are ongoing changes being applied by the\norchestrator."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOptional. State of the orchestrator. Can be updated to change orchestrator behaviour.\nAllowed values:\n- 'ACTIVE' - orchestrator is actively looking for actions to be taken.\n- 'STOPPED' - orchestrator won't make any changes.\n\nNote: There might be more states added in the future. We use string here\ninstead of an enum, to avoid the need of propagating new states to all the\nclient code."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was last modified."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `orchestrated_resource` after provisioning.\n"]
    pub fn orchestrated_resource(&self) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestratedResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestrated_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `orchestration_scope` after provisioning.\n"]
    pub fn orchestration_scope(&self) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestration_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigV2PolicyOrchestrator_TimeoutsRef {
        OsConfigV2PolicyOrchestrator_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OsConfigV2PolicyOrchestrator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OsConfigV2PolicyOrchestrator { }

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator {
    type O = terrars::ListRef<OsConfigV2PolicyOrchestratorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OsConfigV2PolicyOrchestrator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_os_config_v2_policy_orchestrator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator {
    pub tf_id: std::string::String,
    #[doc =
        "Required. Action to be done by the orchestrator in\n'projects/{project_id}/zones/{zone_id}' locations defined by the\n'orchestration_scope'. Allowed values:\n- 'UPSERT' - Orchestrator will create or update target resources.\n- 'DELETE' - Orchestrator will delete target resources, if they exist"]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub policy_orchestrator_id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator {
    pub fn build(self, stack: &mut terrars::Stack) -> OsConfigV2PolicyOrchestrator {
        let out = OsConfigV2PolicyOrchestrator(std::rc::Rc::new(OsConfigV2PolicyOrchestrator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OsConfigV2PolicyOrchestratorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                policy_orchestrator_id: self.policy_orchestrator_id,
                project: core::default::Default::default(),
                state: core::default::Default::default(),
                orchestrated_resource: core::default::Default::default(),
                orchestration_scope: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OsConfigV2PolicyOrchestratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OsConfigV2PolicyOrchestratorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nRequired. Action to be done by the orchestrator in\n'projects/{project_id}/zones/{zone_id}' locations defined by the\n'orchestration_scope'. Allowed values:\n- 'UPSERT' - Orchestrator will create or update target resources.\n- 'DELETE' - Orchestrator will delete target resources, if they exist"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Freeform text describing the purpose of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. Identifier. In form of\n* 'organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `orchestration_state` after provisioning.\nDescribes the state of the orchestration process."]
    pub fn orchestration_state(&self) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestration_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_orchestrator_id` after provisioning.\nRequired. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub fn policy_orchestrator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_orchestrator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true, if the there are ongoing changes being applied by the\norchestrator."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOptional. State of the orchestrator. Can be updated to change orchestrator behaviour.\nAllowed values:\n- 'ACTIVE' - orchestrator is actively looking for actions to be taken.\n- 'STOPPED' - orchestrator won't make any changes.\n\nNote: There might be more states added in the future. We use string here\ninstead of an enum, to avoid the need of propagating new states to all the\nclient code."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was last modified."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `orchestrated_resource` after provisioning.\n"]
    pub fn orchestrated_resource(&self) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestratedResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestrated_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `orchestration_scope` after provisioning.\n"]
    pub fn orchestration_scope(&self) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestration_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigV2PolicyOrchestrator_TimeoutsRef {
        OsConfigV2PolicyOrchestrator_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorDynamic {
    orchestrated_resource: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestratedResource>>,
    orchestration_scope: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestrationScope>>,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details {
    #[doc = "Set the field `type_url`.\n"]
    pub fn set_type_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details {
    type O =
        terrars
        ::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details {
        OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details {
            type_url: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_DetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_DetailsRef {
        OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_url` after provisioning.\n"]
    pub fn type_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_url", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std
    ::option
    ::Option<terrars::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_Details,
                        >,
                    >,
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

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error {
        OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_ErrorRef {
        OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std
    ::option
    ::Option<terrars::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failed_actions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performed_actions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    progress: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error>,
                    >,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failed_actions`.\n"]
    pub fn set_failed_actions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failed_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finish_time`.\n"]
    pub fn set_finish_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finish_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performed_actions`.\n"]
    pub fn set_performed_actions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.performed_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `progress`.\n"]
    pub fn set_progress(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.progress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout_resource`.\n"]
    pub fn set_rollout_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState {
        OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState {
            error: core::default::Default::default(),
            failed_actions: core::default::Default::default(),
            finish_time: core::default::Default::default(),
            performed_actions: core::default::Default::default(),
            progress: core::default::Default::default(),
            rollout_resource: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationStateRef {
        OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `failed_actions` after provisioning.\n"]
    pub fn failed_actions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failed_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `finish_time` after provisioning.\n"]
    pub fn finish_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finish_time", self.base))
    }

    #[doc = "Get a reference to the value of field `performed_actions` after provisioning.\n"]
    pub fn performed_actions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performed_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `progress` after provisioning.\n"]
    pub fn progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.progress", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout_resource` after provisioning.\n"]
    pub fn rollout_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std
    ::option
    ::Option<terrars::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failed_actions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performed_actions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    progress: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_Error>,
                    >,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failed_actions`.\n"]
    pub fn set_failed_actions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failed_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finish_time`.\n"]
    pub fn set_finish_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finish_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performed_actions`.\n"]
    pub fn set_performed_actions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.performed_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `progress`.\n"]
    pub fn set_progress(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.progress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout_resource`.\n"]
    pub fn set_rollout_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState {
        OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState {
            error: core::default::Default::default(),
            failed_actions: core::default::Default::default(),
            finish_time: core::default::Default::default(),
            performed_actions: core::default::Default::default(),
            progress: core::default::Default::default(),
            rollout_resource: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationStateRef {
        OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `failed_actions` after provisioning.\n"]
    pub fn failed_actions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failed_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `finish_time` after provisioning.\n"]
    pub fn finish_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finish_time", self.base))
    }

    #[doc = "Get a reference to the value of field `performed_actions` after provisioning.\n"]
    pub fn performed_actions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performed_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `progress` after provisioning.\n"]
    pub fn progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.progress", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout_resource` after provisioning.\n"]
    pub fn rollout_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_iteration_state: std
    ::option
    ::Option<terrars::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    previous_iteration_state: std
    ::option
    ::Option<terrars::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationState {
    #[doc = "Set the field `current_iteration_state`.\n"]
    pub fn set_current_iteration_state(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationState>>,
    ) -> Self {
        self.current_iteration_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `previous_iteration_state`.\n"]
    pub fn set_previous_iteration_state(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationState>,
                    >,
    ) -> Self {
        self.previous_iteration_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationState {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationState {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationState {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationState {
        OsConfigV2PolicyOrchestrator_OrchestrationState {
            current_iteration_state: core::default::Default::default(),
            previous_iteration_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationStateRef {
        OsConfigV2PolicyOrchestrator_OrchestrationStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_iteration_state` after provisioning.\n"]
    pub fn current_iteration_state(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationState_CurrentIterationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_iteration_state", self.base))
    }

    #[doc = "Get a reference to the value of field `previous_iteration_state` after provisioning.\n"]
    pub fn previous_iteration_state(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationState_PreviousIterationStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.previous_iteration_state", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels {
    #[doc =
        "Set the field `labels`.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels {
            labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabelsRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels {
    #[doc =
        "Set the field `labels`.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels {
            labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabelsRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories {
    os_short_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories {
    #[doc =
        "Set the field `os_version`.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."]
    pub fn set_os_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories {
    #[doc = "Required. The OS short name"]
    pub os_short_name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories {
            os_short_name: self.os_short_name,
            os_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InventoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InventoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InventoriesRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InventoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InventoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `os_short_name` after provisioning.\nRequired. The OS short name"]
    pub fn os_short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_short_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."]
    pub fn os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterDynamic {
    exclusion_labels: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels,
        >,
    >,
    inclusion_labels: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels,
        >,
    >,
    inventories: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_labels: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_labels: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inventories: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter {
    #[doc = "Set the field `all`.\nTarget all VMs in the project. If true, no other criteria is\npermitted."]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusion_labels`.\n"]
    pub fn set_exclusion_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabels,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inclusion_labels`.\n"]
    pub fn set_inclusion_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabels,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inclusion_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inclusion_labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inventories`.\n"]
    pub fn set_inventories(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_Inventories,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inventories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inventories = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter {
            all: core::default::Default::default(),
            exclusion_labels: core::default::Default::default(),
            inclusion_labels: core::default::Default::default(),
            inventories: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all` after provisioning.\nTarget all VMs in the project. If true, no other criteria is\npermitted."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_labels` after provisioning.\n"]
    pub fn exclusion_labels(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_ExclusionLabelsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_labels` after provisioning.\n"]
    pub fn inclusion_labels(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InclusionLabelsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inclusion_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `inventories` after provisioning.\n"]
    pub fn inventories(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter_InventoriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inventories", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters {
    os_short_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters {
    #[doc =
        "Set the field `os_version`.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."]
    pub fn set_os_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters {
    #[doc = "Required. The OS short name"]
    pub os_short_name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters {
            os_short_name: self.os_short_name,
            os_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFiltersRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `os_short_name` after provisioning.\nRequired. The OS short name"]
    pub fn os_short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_short_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."]
    pub fn os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<std::string::String>>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs {
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_GcsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_GcsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_GcsRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_GcsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_GcsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_checksum: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_checksum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote {
    #[doc =
        "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_RemoteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_RemoteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_RemoteRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_RemoteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_RemoteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_checksum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileDynamic {
    gcs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs,
        >,
    >,
    remote: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_GcsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_RemoteRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceDynamic {
    file: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    interpreter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_file_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce {
    #[doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_file_path`.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."]
    pub fn set_output_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_file_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script`.\nAn inline script.\nThe size of the script is limited to 32KiB."]
    pub fn set_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce {
    #[doc =
        "Required. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub interpreter: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce {
            args: core::default::Default::default(),
            interpreter: self.interpreter,
            output_file_path: core::default::Default::default(),
            script: core::default::Default::default(),
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nRequired. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."]
    pub fn output_file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_file_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nAn inline script.\nThe size of the script is limited to 32KiB."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateDynamic {
    file: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    interpreter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_file_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate {
    #[doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_file_path`.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."]
    pub fn set_output_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_file_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script`.\nAn inline script.\nThe size of the script is limited to 32KiB."]
    pub fn set_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate {
    #[doc =
        "Required. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub interpreter: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate {
            args: core::default::Default::default(),
            interpreter: self.interpreter,
            output_file_path: core::default::Default::default(),
            script: core::default::Default::default(),
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nRequired. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."]
    pub fn output_file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_file_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nAn inline script.\nThe size of the script is limited to 32KiB."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecDynamic {
    enforce: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce,
        >,
    >,
    validate: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validate: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec {
    #[doc = "Set the field `enforce`.\n"]
    pub fn set_enforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Enforce,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validate`.\n"]
    pub fn set_validate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_Validate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec {
            enforce: core::default::Default::default(),
            validate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_EnforceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `validate` after provisioning.\n"]
    pub fn validate(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec_ValidateRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validate", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileDynamic {
    file: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::PrimField<std::string::String>>,
    state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File {
    #[doc = "Set the field `content`.\nA a file with this content.\nThe size of the content is limited to 32KiB."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permissions`.\nConsists of three octal digits which represent, in\norder, the permissions of the owner, group, and other users for the\nfile (similarly to the numeric mode used in the linux chmod\nutility). Each digit represents a three bit number with the 4 bit\ncorresponding to the read permissions, the 2 bit corresponds to the\nwrite bit, and the one bit corresponds to the execute permission.\nDefault behavior is 755.\n\nBelow are some examples of permissions and their associated values:\nread, write, and execute: 7\nread and execute: 5\nread and write: 6\nread only: 4"]
    pub fn set_permissions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File {
    #[doc = "Required. The absolute path of the file within the VM."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Desired state of the file.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nPRESENT\nABSENT\nCONTENTS_MATCH"]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File {
            content: core::default::Default::default(),
            path: self.path,
            permissions: core::default::Default::default(),
            state: self.state,
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nA a file with this content.\nThe size of the content is limited to 32KiB."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRequired. The absolute path of the file within the VM."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nConsists of three octal digits which represent, in\norder, the permissions of the owner, group, and other users for the\nfile (similarly to the numeric mode used in the linux chmod\nutility). Each digit represents a three bit number with the 4 bit\ncorresponding to the read permissions, the 2 bit corresponds to the\nwrite bit, and the one bit corresponds to the execute permission.\nDefault behavior is 755.\n\nBelow are some examples of permissions and their associated values:\nread, write, and execute: 7\nread and execute: 5\nread and write: 6\nread only: 4"]
    pub fn permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nRequired. Desired state of the file.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nPRESENT\nABSENT\nCONTENTS_MATCH"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_FileRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt {

}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt {
    #[doc = "Required. Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_AptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_AptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_AptRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_AptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_AptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceDynamic {
    gcs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs,
        >,
    >,
    remote: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Gcs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_Remote,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_GcsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File_File_RemoteRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebDynamic {
    source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_deps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb {
    #[doc =
        "Set the field `pull_deps`.\nWhether dependencies should also be installed.\n- install when false: 'dpkg -i package'\n- install when true: 'apt-get update && apt-get -y install\npackage.deb'"]
    pub fn set_pull_deps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pull_deps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
                        >,
                    >,
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
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb {
            pull_deps: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed.\n- install when false: 'dpkg -i package'\n- install when true: 'apt-get update && apt-get -y install\npackage.deb'"]
    pub fn pull_deps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pull_deps", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet {

}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet {
    #[doc = "Required. Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_GoogetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_GoogetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_GoogetRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_GoogetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_GoogetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiDynamic {
    source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi {
    #[doc =
        "Set the field `properties`.\nAdditional properties to use during installation.\nThis should be in the format of Property=Setting.\nAppended to the defaults of 'ACTION=INSTALL\nREBOOT=ReallySuppress'."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
                        >,
                    >,
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
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi {
            properties: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nAdditional properties to use during installation.\nThis should be in the format of Property=Setting.\nAppended to the defaults of 'ACTION=INSTALL\nREBOOT=ReallySuppress'."]
    pub fn properties(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmDynamic {
    source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_deps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm {
    #[doc =
        "Set the field `pull_deps`.\nWhether dependencies should also be installed.\n- install when false: 'rpm --upgrade --replacepkgs package.rpm'\n- install when true: 'yum -y install package.rpm' or\n'zypper -y install package.rpm'"]
    pub fn set_pull_deps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pull_deps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_Source,
                        >,
                    >,
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
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm {
            pull_deps: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed.\n- install when false: 'rpm --upgrade --replacepkgs package.rpm'\n- install when true: 'yum -y install package.rpm' or\n'zypper -y install package.rpm'"]
    pub fn pull_deps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pull_deps", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb_SourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum {

}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum {
    #[doc = "Required. Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_YumRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_YumRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_YumRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_YumRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_YumRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper {

}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper {
    #[doc = "Required. Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_ZypperRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_ZypperRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_ZypperRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_ZypperRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_ZypperRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgDynamic {
    apt: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt,
        >,
    >,
    deb: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb,
        >,
    >,
    googet: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet,
        >,
    >,
    msi: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi,
        >,
    >,
    rpm: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm,
        >,
    >,
    yum: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum,
        >,
    >,
    zypper: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg {
    desired_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deb: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    googet: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msi: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpm: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zypper: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg {
    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Apt,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.apt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.apt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deb`.\n"]
    pub fn set_deb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Deb,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `googet`.\n"]
    pub fn set_googet(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Googet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.googet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.googet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `msi`.\n"]
    pub fn set_msi(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Msi,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.msi = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.msi = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rpm`.\n"]
    pub fn set_rpm(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Rpm,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rpm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rpm = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Yum,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.yum = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.yum = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zypper`.\n"]
    pub fn set_zypper(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_Zypper,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zypper = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zypper = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg {
    #[doc =
        "Required. The desired state the agent should maintain for this package.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nINSTALLED\nREMOVED"]
    pub desired_state: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg {
            desired_state: self.desired_state,
            apt: core::default::Default::default(),
            deb: core::default::Default::default(),
            googet: core::default::Default::default(),
            msi: core::default::Default::default(),
            rpm: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nRequired. The desired state the agent should maintain for this package.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nINSTALLED\nREMOVED"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.base))
    }

    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_AptRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }

    #[doc = "Get a reference to the value of field `deb` after provisioning.\n"]
    pub fn deb(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_DebRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deb", self.base))
    }

    #[doc = "Get a reference to the value of field `googet` after provisioning.\n"]
    pub fn googet(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_GoogetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.googet", self.base))
    }

    #[doc = "Get a reference to the value of field `msi` after provisioning.\n"]
    pub fn msi(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_MsiRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.msi", self.base))
    }

    #[doc = "Get a reference to the value of field `rpm` after provisioning.\n"]
    pub fn rpm(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_RpmRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rpm", self.base))
    }

    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_YumRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }

    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg_ZypperRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt {
    archive_type: terrars::PrimField<std::string::String>,
    components: terrars::ListField<terrars::PrimField<std::string::String>>,
    distribution: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpg_key: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt {
    #[doc =
        "Set the field `gpg_key`.\nURI of the key file for this repository. The agent maintains a\nkeyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."]
    pub fn set_gpg_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gpg_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt {
    #[doc =
        "Required. Type of archive files in this repository.\nPossible values:\nARCHIVE_TYPE_UNSPECIFIED\nDEB\nDEB_SRC"]
    pub archive_type: terrars::PrimField<std::string::String>,
    #[doc = "Required. List of components for this repository. Must contain at least one\nitem."]
    pub components: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Required. Distribution of this repository."]
    pub distribution: terrars::PrimField<std::string::String>,
    #[doc = "Required. URI for this repository."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt {
            archive_type: self.archive_type,
            components: self.components,
            distribution: self.distribution,
            gpg_key: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_AptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_AptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_AptRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_AptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_AptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_type` after provisioning.\nRequired. Type of archive files in this repository.\nPossible values:\nARCHIVE_TYPE_UNSPECIFIED\nDEB\nDEB_SRC"]
    pub fn archive_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `components` after provisioning.\nRequired. List of components for this repository. Must contain at least one\nitem."]
    pub fn components(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.components", self.base))
    }

    #[doc =
        "Get a reference to the value of field `distribution` after provisioning.\nRequired. Distribution of this repository."]
    pub fn distribution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gpg_key` after provisioning.\nURI of the key file for this repository. The agent maintains a\nkeyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."]
    pub fn gpg_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpg_key", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI for this repository."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo {
    name: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo {

}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo {
    #[doc = "Required. The name of the repository."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Required. The url of the repository."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo {
            name: self.name,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_GooRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_GooRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_GooRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_GooRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_GooRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The name of the repository."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nRequired. The url of the repository."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum {
    base_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpg_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    id: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum {
    #[doc = "Set the field `display_name`.\nThe display name of the repository."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpg_keys`.\nURIs of GPG keys."]
    pub fn set_gpg_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.gpg_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum {
    #[doc = "Required. The location of the repository directory."]
    pub base_url: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. A one word, unique name for this repository. This is  the 'repo\nid' in the yum config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for resource conflicts."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum {
            base_url: self.base_url,
            display_name: core::default::Default::default(),
            gpg_keys: core::default::Default::default(),
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_YumRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_YumRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_YumRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_YumRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_YumRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_url` after provisioning.\nRequired. The location of the repository directory."]
    pub fn base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."]
    pub fn gpg_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. A one word, unique name for this repository. This is  the 'repo\nid' in the yum config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for resource conflicts."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper {
    base_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpg_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    id: terrars::PrimField<std::string::String>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper {
    #[doc = "Set the field `display_name`.\nThe display name of the repository."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpg_keys`.\nURIs of GPG keys."]
    pub fn set_gpg_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.gpg_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper {
    #[doc = "Required. The location of the repository directory."]
    pub base_url: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. A one word, unique name for this repository. This is the 'repo\nid' in the zypper config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for GuestPolicy conflicts."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper {
            base_url: self.base_url,
            display_name: core::default::Default::default(),
            gpg_keys: core::default::Default::default(),
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_ZypperRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_ZypperRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_ZypperRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_ZypperRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_ZypperRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_url` after provisioning.\nRequired. The location of the repository directory."]
    pub fn base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."]
    pub fn gpg_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. A one word, unique name for this repository. This is the 'repo\nid' in the zypper config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for GuestPolicy conflicts."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryDynamic {
    apt: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt,
        >,
    >,
    goo: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo,
        >,
    >,
    yum: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum,
        >,
    >,
    zypper: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    goo: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zypper: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository {
    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Apt,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.apt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.apt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `goo`.\n"]
    pub fn set_goo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Goo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.goo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.goo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Yum,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.yum = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.yum = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zypper`.\n"]
    pub fn set_zypper(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_Zypper,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zypper = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zypper = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository {
            apt: core::default::Default::default(),
            goo: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_AptRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }

    #[doc = "Get a reference to the value of field `goo` after provisioning.\n"]
    pub fn goo(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_GooRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.goo", self.base))
    }

    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_YumRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }

    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository_ZypperRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesDynamic {
    exec: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec,
        >,
    >,
    file: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File,
        >,
    >,
    pkg: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg,
        >,
    >,
    repository: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exec: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pkg: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources {
    #[doc = "Set the field `exec`.\n"]
    pub fn set_exec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Exec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_File,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pkg`.\n"]
    pub fn set_pkg(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Pkg,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pkg = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pkg = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_Repository,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources {
    #[doc =
        "Required. The id of the resource with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources {
            id: self.id,
            exec: core::default::Default::default(),
            file: core::default::Default::default(),
            pkg: core::default::Default::default(),
            repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. The id of the resource with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_ExecRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_FileRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `pkg` after provisioning.\n"]
    pub fn pkg(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_PkgRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pkg", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources_RepositoryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsDynamic {
    inventory_filters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters,
        >,
    >,
    resources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inventory_filters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources,
        >,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups {
    #[doc = "Set the field `inventory_filters`.\n"]
    pub fn set_inventory_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFilters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inventory_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inventory_filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_Resources,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups {
            inventory_filters: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inventory_filters` after provisioning.\n"]
    pub fn inventory_filters(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_InventoryFiltersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inventory_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups_ResourcesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesDynamic {
    resource_groups: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_no_resource_group_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_groups: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups>,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies {
    #[doc =
        "Set the field `allow_no_resource_group_match`.\nThis flag determines the OS policy compliance status when none of the\nresource groups within the policy are applicable for a VM. Set this value\nto 'true' if the policy needs to be reported as compliant even if the\npolicy has nothing to validate or enforce."]
    pub fn set_allow_no_resource_group_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_no_resource_group_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nPolicy description.\nLength of the description is limited to 1024 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_groups`.\n"]
    pub fn set_resource_groups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroups,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_groups = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies {
    type O =
        terrars
        ::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies {
    #[doc =
        "Required. The id of the OS policy with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "Required. Policy mode\nPossible values:\nMODE_UNSPECIFIED\nVALIDATION\nENFORCEMENT"]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies {
            allow_no_resource_group_match: core::default::Default::default(),
            description: core::default::Default::default(),
            id: self.id,
            mode: self.mode,
            resource_groups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_no_resource_group_match` after provisioning.\nThis flag determines the OS policy compliance status when none of the\nresource groups within the policy are applicable for a VM. Set this value\nto 'true' if the policy needs to be reported as compliant even if the\npolicy has nothing to validate or enforce."]
    pub fn allow_no_resource_group_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_no_resource_group_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nPolicy description.\nLength of the description is limited to 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. The id of the OS policy with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nRequired. Policy mode\nPossible values:\nMODE_UNSPECIFIED\nVALIDATION\nENFORCEMENT"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_groups` after provisioning.\n"]
    pub fn resource_groups(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies_ResourceGroupsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget {
    #[doc = "Set the field `fixed`.\nSpecifies a fixed value."]
    pub fn set_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percent`.\nSpecifies the relative value defined as a percentage, which will be\nmultiplied by a reference value."]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudgetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudgetRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudgetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudgetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed value."]
    pub fn fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nSpecifies the relative value defined as a percentage, which will be\nmultiplied by a reference value."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutDynamic {
    disruption_budget: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout {
    min_wait_duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disruption_budget: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget>,
    >,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout {
    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudget,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disruption_budget = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disruption_budget = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout {
    type O =
        terrars
        ::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout {
    #[doc =
        "Required. This determines the minimum duration of time to wait after the\nconfiguration changes are applied through the current rollout. A\nVM continues to count towards the 'disruption_budget' at least\nuntil this duration of time has passed after configuration changes are\napplied."]
    pub min_wait_duration: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout {
            min_wait_duration: self.min_wait_duration,
            disruption_budget: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_wait_duration` after provisioning.\nRequired. This determines the minimum duration of time to wait after the\nconfiguration changes are applied through the current rollout. A\nVM continues to count towards the 'disruption_budget' at least\nuntil this duration of time has passed after configuration changes are\napplied."]
    pub fn min_wait_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_wait_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(
        &self,
    ) -> terrars
    ::ListRef<
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout_DisruptionBudgetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disruption_budget", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadDynamic {
    instance_filter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter>,
    >,
    os_policies: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies>,
    >,
    rollout: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_filter: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_policies: std
    ::option
    ::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout: std
    ::option
    ::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout>>,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload {
    #[doc =
        "Set the field `description`.\nOS policy assignment description.\nLength of the description is limited to 1024 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nResource name.\n\nFormat:\n'projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}'\n\nThis field is ignored when you create an OS policy assignment."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_filter`.\n"]
    pub fn set_instance_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `os_policies`.\n"]
    pub fn set_os_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPolicies,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.os_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.os_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_Rollout,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rollout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            instance_filter: core::default::Default::default(),
            os_policies: core::default::Default::default(),
            rollout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `baseline` after provisioning.\nOutput only. Indicates that this revision has been successfully rolled out in this zone\nand new VMs will be assigned OS policies from this revision.\n\nFor a given OS policy assignment, there is only one revision with a value\nof 'true' for this field."]
    pub fn baseline(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deleted` after provisioning.\nOutput only. Indicates that this revision deletes the OS policy assignment."]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOS policy assignment description.\nLength of the description is limited to 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name.\n\nFormat:\n'projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}'\n\nThis field is ignored when you create an OS policy assignment."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Indicates that reconciliation is in progress for the revision.\nThis value is 'true' when the 'rollout_state' is one of:\n* IN_PROGRESS\n* CANCELLING"]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.base))
    }

    #[doc =
        "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp that the revision was created."]
    pub fn revision_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The assignment revision ID\nA new revision is committed whenever a rollout is triggered for a OS policy\nassignment"]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rollout_state` after provisioning.\nOutput only. OS policy assignment rollout state\nPossible values:\nROLLOUT_STATE_UNSPECIFIED\nIN_PROGRESS\nCANCELLING\nCANCELLED\nSUCCEEDED"]
    pub fn rollout_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server generated unique id for the OS policy assignment resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(
        &self,
    ) -> terrars
    ::ListRef<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_InstanceFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `os_policies` after provisioning.\n"]
    pub fn os_policies(
        &self,
    ) -> terrars
    ::ListRef<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_OsPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload_RolloutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestratedResourceDynamic {
    os_policy_assignment_v1_payload: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload>>,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_policy_assignment_v1_payload: std
    ::option
    ::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload>>,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestratedResourceDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResource {
    #[doc =
        "Set the field `id`.\nOptional. ID of the resource to be used while generating set of affected resources.\n\nFor UPSERT action the value is auto-generated during PolicyOrchestrator\ncreation when not set. When the value is set it should following next\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project.\n\nFor DELETE action, ID must be specified explicitly during\nPolicyOrchestrator creation."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os_policy_assignment_v1_payload`.\n"]
    pub fn set_os_policy_assignment_v1_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1Payload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.os_policy_assignment_v1_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.os_policy_assignment_v1_payload = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestratedResource {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestratedResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestratedResource {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestratedResource {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestratedResource {
        OsConfigV2PolicyOrchestrator_OrchestratedResource {
            id: core::default::Default::default(),
            os_policy_assignment_v1_payload: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestratedResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestratedResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestratedResourceRef {
        OsConfigV2PolicyOrchestrator_OrchestratedResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestratedResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nOptional. ID of the resource to be used while generating set of affected resources.\n\nFor UPSERT action the value is auto-generated during PolicyOrchestrator\ncreation when not set. When the value is set it should following next\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project.\n\nFor DELETE action, ID must be specified explicitly during\nPolicyOrchestrator creation."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `os_policy_assignment_v1_payload` after provisioning.\n"]
    pub fn os_policy_assignment_v1_payload(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestratedResource_OsPolicyAssignmentV1PayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_policy_assignment_v1_payload", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector {
    #[doc = "Set the field `included_locations`.\nOptional. Names of the locations in scope.\nFormat: 'us-central1-a'"]
    pub fn set_included_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector {
        OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector {
            included_locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelectorRef {
        OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `included_locations` after provisioning.\nOptional. Names of the locations in scope.\nFormat: 'us-central1-a'"]
    pub fn included_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_folders: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector {
    #[doc =
        "Set the field `included_folders`.\nOptional. Names of the folders in scope.\nFormat: 'folders/{folder_id}'"]
    pub fn set_included_folders(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_folders = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_projects`.\nOptional. Names of the projects in scope.\nFormat: 'projects/{project_number}'"]
    pub fn set_included_projects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_projects = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector {
    type O =
        terrars
        ::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector {
        OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector {
            included_folders: core::default::Default::default(),
            included_projects: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelectorRef {
        OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `included_folders` after provisioning.\nOptional. Names of the folders in scope.\nFormat: 'folders/{folder_id}'"]
    pub fn included_folders(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_folders", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_projects` after provisioning.\nOptional. Names of the projects in scope.\nFormat: 'projects/{project_number}'"]
    pub fn included_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_projects", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsDynamic {
    location_selector: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector>>,
    resource_hierarchy_selector: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_selector: std
    ::option
    ::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_hierarchy_selector: std
    ::option
    ::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector>>,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors {
    #[doc = "Set the field `location_selector`.\n"]
    pub fn set_location_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelector>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.location_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.location_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_hierarchy_selector`.\n"]
    pub fn set_resource_hierarchy_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelector,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_hierarchy_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_hierarchy_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors {
        OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors {
            location_selector: core::default::Default::default(),
            resource_hierarchy_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsRef {
        OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location_selector` after provisioning.\n"]
    pub fn location_selector(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_LocationSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_hierarchy_selector` after provisioning.\n"]
    pub fn resource_hierarchy_selector(
        &self,
    ) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors_ResourceHierarchySelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_hierarchy_selector", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestrator_OrchestrationScopeDynamic {
    selectors: std::option::Option<terrars::DynamicBlock<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors>>,
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selectors: std::option::Option<std::vec::Vec<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors>>,
    dynamic: OsConfigV2PolicyOrchestrator_OrchestrationScopeDynamic,
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScope {
    #[doc = "Set the field `selectors`.\n"]
    pub fn set_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationScope_Selectors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_OrchestrationScope {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_OrchestrationScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_OrchestrationScope {}

impl BuildOsConfigV2PolicyOrchestrator_OrchestrationScope {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_OrchestrationScope {
        OsConfigV2PolicyOrchestrator_OrchestrationScope {
            selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_OrchestrationScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_OrchestrationScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigV2PolicyOrchestrator_OrchestrationScopeRef {
        OsConfigV2PolicyOrchestrator_OrchestrationScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_OrchestrationScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(&self) -> terrars::ListRef<OsConfigV2PolicyOrchestrator_OrchestrationScope_SelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selectors", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestrator_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigV2PolicyOrchestrator_Timeouts {
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

impl terrars::ToListMappable for OsConfigV2PolicyOrchestrator_Timeouts {
    type O = terrars::BlockAssignable<OsConfigV2PolicyOrchestrator_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigV2PolicyOrchestrator_Timeouts {}

impl BuildOsConfigV2PolicyOrchestrator_Timeouts {
    pub fn build(self) -> OsConfigV2PolicyOrchestrator_Timeouts {
        OsConfigV2PolicyOrchestrator_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsConfigV2PolicyOrchestrator_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigV2PolicyOrchestrator_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigV2PolicyOrchestrator_TimeoutsRef {
        OsConfigV2PolicyOrchestrator_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigV2PolicyOrchestrator_TimeoutsRef {
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
