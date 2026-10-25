#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiOnlineEvaluatorData_ {
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
    agent_resource: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_observability: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_sources: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_MetricSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiOnlineEvaluator_Timeouts>,
    dynamic: VertexAiOnlineEvaluatorDynamic,
}

struct VertexAiOnlineEvaluator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiOnlineEvaluatorData_>,
}

#[derive(Clone)]
pub struct VertexAiOnlineEvaluator(std::rc::Rc<VertexAiOnlineEvaluator_>);

impl VertexAiOnlineEvaluator {
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

    #[doc =
        "Set the field `display_name`.\nHuman-readable name for the OnlineEvaluator. The name doesn't have to be\nunique. The name can consist of any UTF-8 characters. The maximum length\nis 63 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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
        "Set the field `region`.\nThe region of the OnlineEvaluator (e.g. us-central1). If unset, the provider's region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_observability`.\n"]
    pub fn set_cloud_observability(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_observability = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_observability = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_sources`.\n"]
    pub fn set_metric_sources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_MetricSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiOnlineEvaluator_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `agent_resource` after provisioning.\nThe name of the agent that the OnlineEvaluator evaluates periodically.\nThis value is used to filter the traces with a matching cloud.resource_id\nand link the evaluation results with relevant dashboards/UIs. This field\nis immutable; once set, it cannot be changed."]
    pub fn agent_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the OnlineEvaluator was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for the OnlineEvaluator. The name doesn't have to be\nunique. The name can consist of any UTF-8 characters. The maximum length\nis 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the OnlineEvaluator. This is the final component of\nthe resource path.\nFormat: projects/{{project}}/locations/{{region}}/onlineEvaluators/{{id}}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the OnlineEvaluator (e.g. us-central1). If unset, the provider's region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the OnlineEvaluator. Possible values are: ACTIVE, SUSPENDED,\nFAILED, WARNING."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_details` after provisioning.\nContains additional information about the state of the OnlineEvaluator.\nThis is used to provide more details in the event of a failure."]
    pub fn state_details(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_StateDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the OnlineEvaluator was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_observability` after provisioning.\n"]
    pub fn cloud_observability(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservabilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_observability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_sources` after provisioning.\n"]
    pub fn metric_sources(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_MetricSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiOnlineEvaluator_TimeoutsRef {
        VertexAiOnlineEvaluator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VertexAiOnlineEvaluator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiOnlineEvaluator { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator {
    type O = terrars::ListRef<VertexAiOnlineEvaluatorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiOnlineEvaluator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_online_evaluator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiOnlineEvaluator {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the agent that the OnlineEvaluator evaluates periodically.\nThis value is used to filter the traces with a matching cloud.resource_id\nand link the evaluation results with relevant dashboards/UIs. This field\nis immutable; once set, it cannot be changed."]
    pub agent_resource: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiOnlineEvaluator {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiOnlineEvaluator {
        let out = VertexAiOnlineEvaluator(std::rc::Rc::new(VertexAiOnlineEvaluator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiOnlineEvaluatorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_resource: self.agent_resource,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                cloud_observability: core::default::Default::default(),
                config: core::default::Default::default(),
                metric_sources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiOnlineEvaluatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluatorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiOnlineEvaluatorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_resource` after provisioning.\nThe name of the agent that the OnlineEvaluator evaluates periodically.\nThis value is used to filter the traces with a matching cloud.resource_id\nand link the evaluation results with relevant dashboards/UIs. This field\nis immutable; once set, it cannot be changed."]
    pub fn agent_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the OnlineEvaluator was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for the OnlineEvaluator. The name doesn't have to be\nunique. The name can consist of any UTF-8 characters. The maximum length\nis 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the OnlineEvaluator. This is the final component of\nthe resource path.\nFormat: projects/{{project}}/locations/{{region}}/onlineEvaluators/{{id}}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the OnlineEvaluator (e.g. us-central1). If unset, the provider's region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the OnlineEvaluator. Possible values are: ACTIVE, SUSPENDED,\nFAILED, WARNING."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_details` after provisioning.\nContains additional information about the state of the OnlineEvaluator.\nThis is used to provide more details in the event of a failure."]
    pub fn state_details(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_StateDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the OnlineEvaluator was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_observability` after provisioning.\n"]
    pub fn cloud_observability(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservabilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_observability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_sources` after provisioning.\n"]
    pub fn metric_sources(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_MetricSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiOnlineEvaluator_TimeoutsRef {
        VertexAiOnlineEvaluator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VertexAiOnlineEvaluatorDynamic {
    cloud_observability: std::option::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability>>,
    config: std::option::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_Config>>,
    metric_sources: std::option::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_MetricSources>>,
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_StateDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiOnlineEvaluator_StateDetails {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_StateDetails {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_StateDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_StateDetails {}

impl BuildVertexAiOnlineEvaluator_StateDetails {
    pub fn build(self) -> VertexAiOnlineEvaluator_StateDetails {
        VertexAiOnlineEvaluator_StateDetails { message: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_StateDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_StateDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiOnlineEvaluator_StateDetailsRef {
        VertexAiOnlineEvaluator_StateDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_StateDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry {
    semconv_version: terrars::PrimField<std::string::String>,
}

impl VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_OpenTelemetry {
    #[doc = "Defines which version of the OTel Semantic Convention the data\nfollows. Can be \"1.39.0\" or newer."]
    pub semconv_version: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_OpenTelemetry {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry {
        VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry { semconv_version: self.semconv_version }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_OpenTelemetryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_OpenTelemetryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_OpenTelemetryRef {
        VertexAiOnlineEvaluator_CloudObservability_OpenTelemetryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_OpenTelemetryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `semconv_version` after provisioning.\nDefines which version of the OTel Semantic Convention the data\nfollows. Can be \"1.39.0\" or newer."]
    pub fn semconv_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.semconv_version", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration {
    comparison_operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration {
    #[doc =
        "The comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = "The value to compare against."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration {
        VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration {
            comparison_operator: self.comparison_operator,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_DurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_DurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_DurationRef {
        VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_DurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_DurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison_operator` after provisioning.\nThe comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to compare against."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors {
    comparison_operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors {
    type O =
        terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors {
    #[doc =
        "The comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = "The value to compare against."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors {
            comparison_operator: self.comparison_operator,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrorsRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison_operator` after provisioning.\nThe comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to compare against."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls {
    comparison_operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls {
    #[doc =
        "The comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = "The value to compare against."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls {
            comparison_operator: self.comparison_operator,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallsRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison_operator` after provisioning.\nThe comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to compare against."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors {
    comparison_operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors {
    #[doc =
        "The comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = "The value to compare against."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors {
            comparison_operator: self.comparison_operator,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrorsRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison_operator` after provisioning.\nThe comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to compare against."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls {
    comparison_operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls {
    #[doc =
        "The comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = "The value to compare against."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls {
            comparison_operator: self.comparison_operator,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallsRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison_operator` after provisioning.\nThe comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to compare against."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage {
    comparison_operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage {
    #[doc =
        "The comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = "The value to compare against."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage {
        VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage {
            comparison_operator: self.comparison_operator,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsageRef {
        VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison_operator` after provisioning.\nThe comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to compare against."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns {
    comparison_operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns {
    #[doc =
        "The comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = "The value to compare against."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns {
            comparison_operator: self.comparison_operator,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurnsRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison_operator` after provisioning.\nThe comparison operator to apply. Possible values are:\nLESS, LESS_OR_EQUAL, EQUAL, NOT_EQUAL, GREATER_OR_EQUAL,\nGREATER."]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to compare against."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterDynamic {
    duration: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration>>,
    model_call_errors: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors>>,
    model_calls: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls>>,
    tool_call_errors: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors>>,
    tool_calls: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls>>,
    total_token_usage: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage>>,
    user_turns: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns>>,
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_call_errors: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_calls: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_call_errors: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_calls: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_token_usage: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_turns: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns>>,
    dynamic: VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterDynamic,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_call_errors`.\n"]
    pub fn set_model_call_errors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrors,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_call_errors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_call_errors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_calls`.\n"]
    pub fn set_model_calls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCalls>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_calls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_calls = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_call_errors`.\n"]
    pub fn set_tool_call_errors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrors,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_call_errors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_call_errors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_calls`.\n"]
    pub fn set_tool_calls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCalls>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_calls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_calls = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `total_token_usage`.\n"]
    pub fn set_total_token_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.total_token_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.total_token_usage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_turns`.\n"]
    pub fn set_user_turns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurns>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_turns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_turns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter {}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter {
            duration: core::default::Default::default(),
            model_call_errors: core::default::Default::default(),
            model_calls: core::default::Default::default(),
            tool_call_errors: core::default::Default::default(),
            tool_calls: core::default::Default::default(),
            total_token_usage: core::default::Default::default(),
            user_turns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_DurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `model_call_errors` after provisioning.\n"]
    pub fn model_call_errors(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_call_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `model_calls` after provisioning.\n"]
    pub fn model_calls(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ModelCallsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_calls", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_call_errors` after provisioning.\n"]
    pub fn tool_call_errors(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_call_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_calls` after provisioning.\n"]
    pub fn tool_calls(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_ToolCallsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_calls", self.base))
    }

    #[doc = "Get a reference to the value of field `total_token_usage` after provisioning.\n"]
    pub fn total_token_usage(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_token_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `user_turns` after provisioning.\n"]
    pub fn user_turns(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter_UserTurnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_turns", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger {
    threshold: terrars::PrimField<std::string::String>,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger {
    #[doc =
        "The amount of time that must pass with no new traces before a\nsession is considered ready for evaluation. Must be a positive\nduration no greater than 7 days (604800 seconds), e.g. \"3600s\"."]
    pub threshold: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger { threshold: self.threshold }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTriggerRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe amount of time that must pass with no new traces before a\nsession is considered ready for evaluation. Must be a positive\nduration no greater than 7 days (604800 seconds), e.g. \"3600s\"."]
    pub fn threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiOnlineEvaluator_CloudObservability_SessionScopeDynamic {
    filter: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter>>,
    inactivity_trigger: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger>>,
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inactivity_trigger: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger>>,
    dynamic: VertexAiOnlineEvaluator_CloudObservability_SessionScopeDynamic,
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScope {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inactivity_trigger`.\n"]
    pub fn set_inactivity_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTrigger>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inactivity_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inactivity_trigger = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_SessionScope {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope {}

impl BuildVertexAiOnlineEvaluator_CloudObservability_SessionScope {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_SessionScope {
        VertexAiOnlineEvaluator_CloudObservability_SessionScope {
            filter: core::default::Default::default(),
            inactivity_trigger: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_SessionScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_SessionScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_SessionScopeRef {
        VertexAiOnlineEvaluator_CloudObservability_SessionScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_SessionScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScope_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `inactivity_trigger` after provisioning.\n"]
    pub fn inactivity_trigger(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScope_InactivityTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inactivity_trigger", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterDynamic {
    duration: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration>>,
    total_token_usage: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage>>,
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_token_usage: std
    ::option
    ::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage>>,
    dynamic: VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterDynamic,
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_Duration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `total_token_usage`.\n"]
    pub fn set_total_token_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.total_token_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.total_token_usage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter {}

impl BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter {
        VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter {
            duration: core::default::Default::default(),
            total_token_usage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterRef {
        VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_DurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `total_token_usage` after provisioning.\n"]
    pub fn total_token_usage(
        &self,
    ) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter_TotalTokenUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_token_usage", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiOnlineEvaluator_CloudObservability_TraceScopeDynamic {
    filter: std::option::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter>>,
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter>>,
    dynamic: VertexAiOnlineEvaluator_CloudObservability_TraceScopeDynamic,
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScope {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability_TraceScope {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope {}

impl BuildVertexAiOnlineEvaluator_CloudObservability_TraceScope {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability_TraceScope {
        VertexAiOnlineEvaluator_CloudObservability_TraceScope {
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservability_TraceScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservability_TraceScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_CloudObservability_TraceScopeRef {
        VertexAiOnlineEvaluator_CloudObservability_TraceScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservability_TraceScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_TraceScope_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiOnlineEvaluator_CloudObservabilityDynamic {
    open_telemetry: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry>>,
    session_scope: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_SessionScope>>,
    trace_scope: std::option::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_CloudObservability_TraceScope>>,
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_CloudObservability {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_view: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trace_view: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_telemetry: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_scope: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_SessionScope>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trace_scope: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_CloudObservability_TraceScope>>,
    dynamic: VertexAiOnlineEvaluator_CloudObservabilityDynamic,
}

impl VertexAiOnlineEvaluator_CloudObservability {
    #[doc =
        "Set the field `log_view`.\nOptional log view that will be used to query logs. If empty, the\nproject's default view ('projects/{{project_id}}') will be used."]
    pub fn set_log_view(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_view = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trace_view`.\nOptional trace view that will be used to query traces. If empty, the\n'_AllSpans' view from the '_Trace' US bucket will be used."]
    pub fn set_trace_view(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trace_view = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_telemetry`.\n"]
    pub fn set_open_telemetry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_OpenTelemetry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_telemetry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_telemetry = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `session_scope`.\n"]
    pub fn set_session_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_SessionScope>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.session_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.session_scope = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trace_scope`.\n"]
    pub fn set_trace_scope(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability_TraceScope>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trace_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trace_scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_CloudObservability {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_CloudObservability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_CloudObservability {}

impl BuildVertexAiOnlineEvaluator_CloudObservability {
    pub fn build(self) -> VertexAiOnlineEvaluator_CloudObservability {
        VertexAiOnlineEvaluator_CloudObservability {
            log_view: core::default::Default::default(),
            trace_view: core::default::Default::default(),
            open_telemetry: core::default::Default::default(),
            session_scope: core::default::Default::default(),
            trace_scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_CloudObservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_CloudObservabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiOnlineEvaluator_CloudObservabilityRef {
        VertexAiOnlineEvaluator_CloudObservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_CloudObservabilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log_view` after provisioning.\nOptional log view that will be used to query logs. If empty, the\nproject's default view ('projects/{{project_id}}') will be used."]
    pub fn log_view(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_view", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trace_view` after provisioning.\nOptional trace view that will be used to query traces. If empty, the\n'_AllSpans' view from the '_Trace' US bucket will be used."]
    pub fn trace_view(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trace_view", self.base))
    }

    #[doc = "Get a reference to the value of field `open_telemetry` after provisioning.\n"]
    pub fn open_telemetry(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_OpenTelemetryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_telemetry", self.base))
    }

    #[doc = "Get a reference to the value of field `session_scope` after provisioning.\n"]
    pub fn session_scope(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_SessionScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `trace_scope` after provisioning.\n"]
    pub fn trace_scope(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_CloudObservability_TraceScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trace_scope", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_Config_RandomSampling {
    percentage: terrars::PrimField<f64>,
}

impl VertexAiOnlineEvaluator_Config_RandomSampling { }

impl terrars::ToListMappable for VertexAiOnlineEvaluator_Config_RandomSampling {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_Config_RandomSampling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_Config_RandomSampling {
    #[doc = "The percentage of traces to sample for evaluation. Must be an\ninteger between 1 and 100."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildVertexAiOnlineEvaluator_Config_RandomSampling {
    pub fn build(self) -> VertexAiOnlineEvaluator_Config_RandomSampling {
        VertexAiOnlineEvaluator_Config_RandomSampling { percentage: self.percentage }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_Config_RandomSamplingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_Config_RandomSamplingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiOnlineEvaluator_Config_RandomSamplingRef {
        VertexAiOnlineEvaluator_Config_RandomSamplingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_Config_RandomSamplingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traces to sample for evaluation. Must be an\ninteger between 1 and 100."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiOnlineEvaluator_ConfigDynamic {
    random_sampling: std::option::Option<terrars::DynamicBlock<VertexAiOnlineEvaluator_Config_RandomSampling>>,
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_evaluated_samples_per_run: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    random_sampling: std::option::Option<std::vec::Vec<VertexAiOnlineEvaluator_Config_RandomSampling>>,
    dynamic: VertexAiOnlineEvaluator_ConfigDynamic,
}

impl VertexAiOnlineEvaluator_Config {
    #[doc =
        "Set the field `max_evaluated_samples_per_run`.\nThe maximum number of evaluations to perform per run. If set to 0, the\nnumber is unbounded."]
    pub fn set_max_evaluated_samples_per_run(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_evaluated_samples_per_run = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `random_sampling`.\n"]
    pub fn set_random_sampling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiOnlineEvaluator_Config_RandomSampling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.random_sampling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.random_sampling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_Config {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_Config {}

impl BuildVertexAiOnlineEvaluator_Config {
    pub fn build(self) -> VertexAiOnlineEvaluator_Config {
        VertexAiOnlineEvaluator_Config {
            max_evaluated_samples_per_run: core::default::Default::default(),
            random_sampling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiOnlineEvaluator_ConfigRef {
        VertexAiOnlineEvaluator_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_evaluated_samples_per_run` after provisioning.\nThe maximum number of evaluations to perform per run. If set to 0, the\nnumber is unbounded."]
    pub fn max_evaluated_samples_per_run(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_evaluated_samples_per_run", self.base))
    }

    #[doc = "Get a reference to the value of field `random_sampling` after provisioning.\n"]
    pub fn random_sampling(&self) -> terrars::ListRef<VertexAiOnlineEvaluator_Config_RandomSamplingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.random_sampling", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_MetricSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_resource_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiOnlineEvaluator_MetricSources {
    #[doc =
        "Set the field `metric`.\nInline metric config, provided as a JSON-formatted string using\ncamelCase field names to match the API format (see the\n[Metric API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/Metric)).\nThe suggested predefined 'metricSpecName' values depend on the\nevaluation scope configured in 'cloud_observability'; a predefined\nmetric must match the scope or the create request will be rejected:\n  - 'trace_scope' (single trace): 'final_response_quality_v1',\n    'tool_use_quality_v1', 'hallucination_v1', 'safety_v1'.\n  - 'session_scope' (multi-turn session):\n    'multi_turn_task_success_v1', 'multi_turn_tool_use_quality_v1',\n    'multi_turn_trajectory_quality_v1'."]
    pub fn set_metric(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metric_resource_name`.\nResource name of a registered metric (a\n'google_vertex_ai_evaluation_metric'), e.g.\nprojects/{{project}}/locations/{{region}}/evaluationMetrics/{{id}}."]
    pub fn set_metric_resource_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metric_resource_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiOnlineEvaluator_MetricSources {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_MetricSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_MetricSources {}

impl BuildVertexAiOnlineEvaluator_MetricSources {
    pub fn build(self) -> VertexAiOnlineEvaluator_MetricSources {
        VertexAiOnlineEvaluator_MetricSources {
            metric: core::default::Default::default(),
            metric_resource_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_MetricSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_MetricSourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiOnlineEvaluator_MetricSourcesRef {
        VertexAiOnlineEvaluator_MetricSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_MetricSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metric` after provisioning.\nInline metric config, provided as a JSON-formatted string using\ncamelCase field names to match the API format (see the\n[Metric API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/Metric)).\nThe suggested predefined 'metricSpecName' values depend on the\nevaluation scope configured in 'cloud_observability'; a predefined\nmetric must match the scope or the create request will be rejected:\n  - 'trace_scope' (single trace): 'final_response_quality_v1',\n    'tool_use_quality_v1', 'hallucination_v1', 'safety_v1'.\n  - 'session_scope' (multi-turn session):\n    'multi_turn_task_success_v1', 'multi_turn_tool_use_quality_v1',\n    'multi_turn_trajectory_quality_v1'."]
    pub fn metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metric_resource_name` after provisioning.\nResource name of a registered metric (a\n'google_vertex_ai_evaluation_metric'), e.g.\nprojects/{{project}}/locations/{{region}}/evaluationMetrics/{{id}}."]
    pub fn metric_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_resource_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiOnlineEvaluator_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiOnlineEvaluator_Timeouts {
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

impl terrars::ToListMappable for VertexAiOnlineEvaluator_Timeouts {
    type O = terrars::BlockAssignable<VertexAiOnlineEvaluator_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiOnlineEvaluator_Timeouts {}

impl BuildVertexAiOnlineEvaluator_Timeouts {
    pub fn build(self) -> VertexAiOnlineEvaluator_Timeouts {
        VertexAiOnlineEvaluator_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiOnlineEvaluator_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiOnlineEvaluator_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiOnlineEvaluator_TimeoutsRef {
        VertexAiOnlineEvaluator_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiOnlineEvaluator_TimeoutsRef {
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
