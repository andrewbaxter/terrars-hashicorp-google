#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiEndpointWithModelGardenDeploymentData_ {
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
    hugging_face_model_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publisher_model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_config: std::option::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_DeployConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_config: std::option::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_EndpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_config: std::option::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiEndpointWithModelGardenDeployment_Timeouts>,
    dynamic: VertexAiEndpointWithModelGardenDeploymentDynamic,
}

struct VertexAiEndpointWithModelGardenDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiEndpointWithModelGardenDeploymentData_>,
}

#[derive(Clone)]
pub struct VertexAiEndpointWithModelGardenDeployment(std::rc::Rc<VertexAiEndpointWithModelGardenDeployment_>);

impl VertexAiEndpointWithModelGardenDeployment {
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
        "Set the field `hugging_face_model_id`.\nThe Hugging Face model to deploy.\nFormat: Hugging Face model ID like 'google/gemma-2-2b-it'."]
    pub fn set_hugging_face_model_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hugging_face_model_id = std::option::Option::Some(v.into());
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
        "Set the field `publisher_model_name`.\nThe Model Garden model to deploy.\nFormat:\n'publishers/{publisher}/models/{publisher_model}@{version_id}', or\n'publishers/hf-{hugging-face-author}/models/{hugging-face-model-name}@001'."]
    pub fn set_publisher_model_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().publisher_model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_config`.\n"]
    pub fn set_deploy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_DeployConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deploy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deploy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_config`.\n"]
    pub fn set_endpoint_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_EndpointConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_config`.\n"]
    pub fn set_model_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiEndpointWithModelGardenDeployment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_model_display_name` after provisioning.\nOutput only. The display name assigned to the model deployed to the endpoint.\nThis is not required to delete the resource but is used for debug logging."]
    pub fn deployed_model_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_model_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_model_id` after provisioning.\nOutput only. The unique numeric ID that Vertex AI assigns to the model at the time it is deployed to the endpoint.\nIt is required to undeploy the model from the endpoint during resource deletion as described in\nhttps://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints/undeployModel."]
    pub fn deployed_model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_model_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nResource ID segment making up resource 'endpoint'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hugging_face_model_id` after provisioning.\nThe Hugging Face model to deploy.\nFormat: Hugging Face model ID like 'google/gemma-2-2b-it'."]
    pub fn hugging_face_model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugging_face_model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'location'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `publisher_model_name` after provisioning.\nThe Model Garden model to deploy.\nFormat:\n'publishers/{publisher}/models/{publisher_model}@{version_id}', or\n'publishers/hf-{hugging-face-author}/models/{hugging-face-model-name}@001'."]
    pub fn publisher_model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publisher_model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deploy_config` after provisioning.\n"]
    pub fn deploy_config(&self) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_DeployConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(&self) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_EndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_config` after provisioning.\n"]
    pub fn model_config(&self) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpointWithModelGardenDeployment_TimeoutsRef {
        VertexAiEndpointWithModelGardenDeployment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiEndpointWithModelGardenDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiEndpointWithModelGardenDeployment { }

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment {
    type O = terrars::ListRef<VertexAiEndpointWithModelGardenDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiEndpointWithModelGardenDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_endpoint_with_model_garden_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'location'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiEndpointWithModelGardenDeployment {
        let out =
            VertexAiEndpointWithModelGardenDeployment(std::rc::Rc::new(VertexAiEndpointWithModelGardenDeployment_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(VertexAiEndpointWithModelGardenDeploymentData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    hugging_face_model_id: core::default::Default::default(),
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    publisher_model_name: core::default::Default::default(),
                    deploy_config: core::default::Default::default(),
                    endpoint_config: core::default::Default::default(),
                    model_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiEndpointWithModelGardenDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiEndpointWithModelGardenDeploymentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_model_display_name` after provisioning.\nOutput only. The display name assigned to the model deployed to the endpoint.\nThis is not required to delete the resource but is used for debug logging."]
    pub fn deployed_model_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_model_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_model_id` after provisioning.\nOutput only. The unique numeric ID that Vertex AI assigns to the model at the time it is deployed to the endpoint.\nIt is required to undeploy the model from the endpoint during resource deletion as described in\nhttps://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints/undeployModel."]
    pub fn deployed_model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_model_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nResource ID segment making up resource 'endpoint'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hugging_face_model_id` after provisioning.\nThe Hugging Face model to deploy.\nFormat: Hugging Face model ID like 'google/gemma-2-2b-it'."]
    pub fn hugging_face_model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugging_face_model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'location'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `publisher_model_name` after provisioning.\nThe Model Garden model to deploy.\nFormat:\n'publishers/{publisher}/models/{publisher_model}@{version_id}', or\n'publishers/hf-{hugging-face-author}/models/{hugging-face-model-name}@001'."]
    pub fn publisher_model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publisher_model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deploy_config` after provisioning.\n"]
    pub fn deploy_config(&self) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_DeployConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(&self) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_EndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_config` after provisioning.\n"]
    pub fn model_config(&self) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpointWithModelGardenDeployment_TimeoutsRef {
        VertexAiEndpointWithModelGardenDeployment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentDynamic {
    deploy_config: std::option::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_DeployConfig>>,
    endpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_EndpointConfig>>,
    model_config: std::option::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs {
    metric_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs {
    #[doc =
        "Set the field `target`.\nThe target resource utilization in percentage (1% - 100%) for the given\nmetric; once the real usage deviates from the target by a certain\npercentage, the machine replicas change. The default value is 60\n(representing 60%) if not provided."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs {
    #[doc =
        "The resource metric name.\nSupported metrics:\n\n* For Online Prediction:\n* 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle'\n* 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"]
    pub metric_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs {
            metric_name: self.metric_name,
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecsRef {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metric_name` after provisioning.\nThe resource metric name.\nSupported metrics:\n\n* For Online Prediction:\n* 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle'\n* 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe target resource utilization in percentage (1% - 100%) for the given\nmetric; once the real usage deviates from the target by a certain\npercentage, the machine replicas change. The default value is 60\n(representing 60%) if not provided."]
    pub fn target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    reservation_affinity_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity {
    #[doc =
        "Set the field `key`.\nCorresponds to the label key of a reservation resource. To target a\nSPECIFIC_RESERVATION by name, use 'compute.googleapis.com/reservation-name'\nas the key and specify the name of your reservation as its value."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `values`.\nCorresponds to the label values of a reservation resource. This must be the\nfull resource name of the reservation or reservation block."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity {
    #[doc =
        "Specifies the reservation affinity type.\nPossible values:\nTYPE_UNSPECIFIED\nNO_RESERVATION\nANY_RESERVATION\nSPECIFIC_RESERVATION"]
    pub reservation_affinity_type: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity {
            key: core::default::Default::default(),
            reservation_affinity_type: self.reservation_affinity_type,
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinityRef {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a\nSPECIFIC_RESERVATION by name, use 'compute.googleapis.com/reservation-name'\nas the key and specify the name of your reservation as its value."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reservation_affinity_type` after provisioning.\nSpecifies the reservation affinity type.\nPossible values:\nTYPE_UNSPECIFIED\nNO_RESERVATION\nANY_RESERVATION\nSPECIFIC_RESERVATION"]
    pub fn reservation_affinity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_affinity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource. This must be the\nfull resource name of the reservation or reservation block."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecDynamic {
    reservation_affinity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multihost_gpu_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity,
        >,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators to attach to the machine."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nPossible values:\nACCELERATOR_TYPE_UNSPECIFIED\nNVIDIA_TESLA_K80\nNVIDIA_TESLA_P100\nNVIDIA_TESLA_V100\nNVIDIA_TESLA_P4\nNVIDIA_TESLA_T4\nNVIDIA_TESLA_A100\nNVIDIA_A100_80GB\nNVIDIA_L4\nNVIDIA_H100_80GB\nNVIDIA_H100_MEGA_80GB\nNVIDIA_H200_141GB\nNVIDIA_B200\nTPU_V2\nTPU_V3\nTPU_V4_POD\nTPU_V5_LITEPOD"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe type of the machine.\n\nSee the [list of machine types supported for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom\ntraining](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor DeployedModel this field is optional, and the default\nvalue is 'n1-standard-2'. For BatchPredictionJob or as part of\nWorkerPoolSpec this field is required."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multihost_gpu_node_count`.\nThe number of nodes per replica for multihost GPU deployments."]
    pub fn set_multihost_gpu_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.multihost_gpu_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tpu_topology`.\nThe topology of the TPUs. Corresponds to the TPU topologies available from\nGKE. (Example: tpu_topology: \"2x2x1\")."]
    pub fn set_tpu_topology(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tpu_topology = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reservation_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec {
    type O =
        terrars
        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec {}

impl BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            multihost_gpu_node_count: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecRef {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators to attach to the machine."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nPossible values:\nACCELERATOR_TYPE_UNSPECIFIED\nNVIDIA_TESLA_K80\nNVIDIA_TESLA_P100\nNVIDIA_TESLA_V100\nNVIDIA_TESLA_P4\nNVIDIA_TESLA_T4\nNVIDIA_TESLA_A100\nNVIDIA_A100_80GB\nNVIDIA_L4\nNVIDIA_H100_80GB\nNVIDIA_H100_MEGA_80GB\nNVIDIA_H200_141GB\nNVIDIA_B200\nTPU_V2\nTPU_V3\nTPU_V4_POD\nTPU_V5_LITEPOD"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine.\n\nSee the [list of machine types supported for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom\ntraining](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor DeployedModel this field is optional, and the default\nvalue is 'n1-standard-2'. For BatchPredictionJob or as part of\nWorkerPoolSpec this field is required."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `multihost_gpu_node_count` after provisioning.\nThe number of nodes per replica for multihost GPU deployments."]
    pub fn multihost_gpu_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multihost_gpu_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tpu_topology` after provisioning.\nThe topology of the TPUs. Corresponds to the TPU topologies available from\nGKE. (Example: tpu_topology: \"2x2x1\")."]
    pub fn tpu_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec_ReservationAffinityRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesDynamic {
    autoscaling_metric_specs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs,
        >,
    >,
    machine_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<f64>>,
    min_replica_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_metric_specs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec>>,
    dynamic: VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources {
    #[doc =
        "Set the field `max_replica_count`.\nThe maximum number of replicas that may be deployed on when the traffic\nagainst it increases. If the requested value is too large, the deployment\nwill error, but if deployment succeeds then the ability to scale to that\nmany replicas is guaranteed (barring service outages). If traffic increases\nbeyond what its replicas at maximum may handle, a portion of the traffic\nwill be dropped. If this value is not provided, will use\nmin_replica_count as the default value.\n\nThe value of this field impacts the charge against Vertex CPU and GPU\nquotas. Specifically, you will be charged for (max_replica_count *\nnumber of cores in the selected machine type) and (max_replica_count *\nnumber of GPUs per replica in the selected machine type)."]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `required_replica_count`.\nNumber of required available replicas for the deployment to succeed.\nThis field is only needed when partial deployment/mutation is\ndesired. If set, the deploy/mutate operation will succeed once\navailable_replica_count reaches required_replica_count, and the rest of\nthe replicas will be retried. If not set, the default\nrequired_replica_count will be min_replica_count."]
    pub fn set_required_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.required_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `spot`.\nIf true, schedule the deployment workload on [spot\nVMs](https://cloud.google.com/kubernetes-engine/docs/concepts/spot-vms)."]
    pub fn set_spot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.spot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_metric_specs`.\n"]
    pub fn set_autoscaling_metric_specs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_metric_specs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_metric_specs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.machine_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources {
    #[doc =
        "The minimum number of machine replicas that will be always deployed on.\nThis value must be greater than or equal to 1.\n\nIf traffic increases, it may dynamically be deployed onto more replicas,\nand as traffic decreases, some of these extra replicas may be freed."]
    pub min_replica_count: terrars::PrimField<f64>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources {
            max_replica_count: core::default::Default::default(),
            min_replica_count: self.min_replica_count,
            required_replica_count: core::default::Default::default(),
            spot: core::default::Default::default(),
            autoscaling_metric_specs: core::default::Default::default(),
            machine_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesRef {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas that may be deployed on when the traffic\nagainst it increases. If the requested value is too large, the deployment\nwill error, but if deployment succeeds then the ability to scale to that\nmany replicas is guaranteed (barring service outages). If traffic increases\nbeyond what its replicas at maximum may handle, a portion of the traffic\nwill be dropped. If this value is not provided, will use\nmin_replica_count as the default value.\n\nThe value of this field impacts the charge against Vertex CPU and GPU\nquotas. Specifically, you will be charged for (max_replica_count *\nnumber of cores in the selected machine type) and (max_replica_count *\nnumber of GPUs per replica in the selected machine type)."]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of machine replicas that will be always deployed on.\nThis value must be greater than or equal to 1.\n\nIf traffic increases, it may dynamically be deployed onto more replicas,\nand as traffic decreases, some of these extra replicas may be freed."]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required_replica_count` after provisioning.\nNumber of required available replicas for the deployment to succeed.\nThis field is only needed when partial deployment/mutation is\ndesired. If set, the deploy/mutate operation will succeed once\navailable_replica_count reaches required_replica_count, and the rest of\nthe replicas will be retried. If not set, the default\nrequired_replica_count will be min_replica_count."]
    pub fn required_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `spot` after provisioning.\nIf true, schedule the deployment workload on [spot\nVMs](https://cloud.google.com/kubernetes-engine/docs/concepts/spot-vms)."]
    pub fn spot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_metric_specs` after provisioning.\n"]
    pub fn autoscaling_metric_specs(
        &self,
    ) -> terrars
    ::ListRef<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_AutoscalingMetricSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_metric_specs", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_DeployConfigDynamic {
    dedicated_resources: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources>>,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_tryout_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_resources: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources>>,
    dynamic: VertexAiEndpointWithModelGardenDeployment_DeployConfigDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfig {
    #[doc =
        "Set the field `fast_tryout_enabled`.\nIf true, enable the QMT fast tryout feature for this model if possible."]
    pub fn set_fast_tryout_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fast_tryout_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `system_labels`.\nSystem labels for Model Garden deployments.\nThese labels are managed by Google and for tracking purposes only."]
    pub fn set_system_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.system_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResources>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dedicated_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dedicated_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_DeployConfig {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_DeployConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig {}

impl BuildVertexAiEndpointWithModelGardenDeployment_DeployConfig {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_DeployConfig {
        VertexAiEndpointWithModelGardenDeployment_DeployConfig {
            fast_tryout_enabled: core::default::Default::default(),
            system_labels: core::default::Default::default(),
            dedicated_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_DeployConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_DeployConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_DeployConfigRef {
        VertexAiEndpointWithModelGardenDeployment_DeployConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_DeployConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fast_tryout_enabled` after provisioning.\nIf true, enable the QMT fast tryout feature for this model if possible."]
    pub fn fast_tryout_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fast_tryout_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `system_labels` after provisioning.\nSystem labels for Model Garden deployments.\nThese labels are managed by Google and for tracking purposes only."]
    pub fn system_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.system_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_DeployConfig_DedicatedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs {
    network: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs { }

impl terrars
::ToListMappable for VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs {
    #[doc =
        "Required. The full name of the Google Compute Engine network.\nFormat: projects/{project}/global/networks/{network}."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc = "Required. Project id used to create forwarding rule."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs {
        VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs {
            network: self.network,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigsRef {
        VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nOutput only. Error message if the PSC service automation failed."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nOutput only. Forwarding rule created by the PSC service automation."]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nOutput only. IP address rule created by the PSC service automation."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nRequired. The full name of the Google Compute Engine network.\nFormat: projects/{project}/global/networks/{network}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nRequired. Project id used to create forwarding rule."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the PSC service automation."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigDynamic {
    psc_automation_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig {
    enable_private_service_connect: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_allowlist: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_automation_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs,
        >,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig {
    #[doc =
        "Set the field `project_allowlist`.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn set_project_allowlist(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.project_allowlist = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_automation_configs`.\n"]
    pub fn set_psc_automation_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_automation_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_automation_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig {
    type O =
        terrars
        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig {
    #[doc = "Required. If true, expose the IndexEndpoint via private service connect."]
    pub enable_private_service_connect: terrars::PrimField<bool>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig {
        VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig {
            enable_private_service_connect: self.enable_private_service_connect,
            project_allowlist: core::default::Default::default(),
            psc_automation_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigRef {
        VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_private_service_connect` after provisioning.\nRequired. If true, expose the IndexEndpoint via private service connect."]
    pub fn enable_private_service_connect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_service_connect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_allowlist` after provisioning.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn project_allowlist(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_allowlist", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_attachment` after provisioning.\nOutput only. The name of the generated service attachment resource.\nThis is only populated if the endpoint is deployed with PrivateServiceConnect."]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_automation_configs` after provisioning.\n"]
    pub fn psc_automation_configs(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig_PscAutomationConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_automation_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_EndpointConfigDynamic {
    private_service_connect_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_EndpointConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_endpoint_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_service_connect_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig>>,
    dynamic: VertexAiEndpointWithModelGardenDeployment_EndpointConfigDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_EndpointConfig {
    #[doc =
        "Set the field `dedicated_endpoint_enabled`.\nIf true, the endpoint will be exposed through a dedicated\nDNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS\nwill be isolated from other users' traffic and will have better\nperformance and reliability. Note: Once you enabled dedicated endpoint,\nyou won't be able to send request to the shared DNS\n{region}-aiplatform.googleapis.com. The limitations will be removed soon."]
    pub fn set_dedicated_endpoint_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dedicated_endpoint_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpoint_display_name`.\nThe user-specified display name of the endpoint. If not set, a\ndefault name will be used."]
    pub fn set_endpoint_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_service_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_service_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_EndpointConfig {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_EndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_EndpointConfig {}

impl BuildVertexAiEndpointWithModelGardenDeployment_EndpointConfig {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_EndpointConfig {
        VertexAiEndpointWithModelGardenDeployment_EndpointConfig {
            dedicated_endpoint_enabled: core::default::Default::default(),
            endpoint_display_name: core::default::Default::default(),
            private_service_connect_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_EndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_EndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_EndpointConfigRef {
        VertexAiEndpointWithModelGardenDeployment_EndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_EndpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dedicated_endpoint_enabled` after provisioning.\nIf true, the endpoint will be exposed through a dedicated\nDNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS\nwill be isolated from other users' traffic and will have better\nperformance and reliability. Note: Once you enabled dedicated endpoint,\nyou won't be able to send request to the shared DNS\n{region}-aiplatform.googleapis.com. The limitations will be removed soon."]
    pub fn dedicated_endpoint_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_endpoint_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_display_name` after provisioning.\nThe user-specified display name of the endpoint. If not set, a\ndefault name will be used."]
    pub fn endpoint_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_EndpointConfig_PrivateServiceConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_service_connect_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env { }

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env {
    #[doc = "Name of the environment variable. Must be a valid C identifier."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Variables that reference a $(VAR_NAME) are expanded\nusing the previous defined environment variables in the container and\nany service environment variables. If a variable cannot be resolved,\nthe reference in the input string will be unchanged. The $(VAR_NAME)\nsyntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped\nreferences will never be expanded, regardless of whether the variable\nexists or not."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_EnvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_EnvRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_EnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a valid C identifier."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nVariables that reference a $(VAR_NAME) are expanded\nusing the previous defined environment variables in the container and\nany service environment variables. If a variable cannot be resolved,\nthe reference in the input string will be unchanged. The $(VAR_NAME)\nsyntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped\nreferences will never be expanded, regardless of whether the variable\nexists or not."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts {
    #[doc =
        "Set the field `container_port`.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts {
            container_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPortsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPortsRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_port` after provisioning.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec {
    #[doc =
        "Set the field `command`.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec {
    type O =
        terrars
        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec {
            command: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc {
    #[doc = "Set the field `port`.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc {
    type O =
        terrars
        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders {
    #[doc =
        "Set the field `name`.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe header field value"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeadersRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetDynamic {
    http_headers: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders>,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet {
    #[doc =
        "Set the field `host`.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nPath to access on the HTTP server."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scheme`.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn set_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeaders,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet {
    type O =
        terrars
        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            scheme: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheme` after provisioning.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> terrars
    ::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet_HttpHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket {
    #[doc = "Set the field `host`.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket {
    type O =
        terrars
        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeDynamic {
    exec: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec>,
    >,
    grpc: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc>,
    >,
    http_get: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet>,
    >,
    tcp_socket: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exec: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_socket: std
    ::option
    ::Option<
        std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket>,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe {
    #[doc =
        "Set the field `failure_threshold`.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_threshold`.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

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
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec,
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

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_get = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp_socket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe {
    type O =
        terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            exec: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> terrars
    ::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeDynamic {
    exec: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec>,
    >,
    grpc: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc>,
    >,
    http_get: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet>,
    >,
    tcp_socket: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exec: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_socket: std
    ::option
    ::Option<
        std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket>,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe {
    #[doc =
        "Set the field `failure_threshold`.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_threshold`.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

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
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec,
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

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_get = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp_socket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe {
    type O =
        terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            exec: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> terrars
    ::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports {
    #[doc =
        "Set the field `container_port`.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports {
            container_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_PortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_PortsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_PortsRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_PortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_PortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_port` after provisioning.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeDynamic {
    exec: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec>,
    >,
    grpc: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc>,
    >,
    http_get: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet>,
    >,
    tcp_socket: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exec: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_socket: std
    ::option
    ::Option<
        std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket>,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe {
    #[doc =
        "Set the field `failure_threshold`.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_threshold`.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

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
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Exec,
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

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_Grpc,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_get = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocket,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp_socket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe {
    type O =
        terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            exec: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_ExecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> terrars
    ::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe_TcpSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecDynamic {
    env: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env>>,
    grpc_ports: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts>>,
    health_probe: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe>>,
    liveness_probe: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe>,
    >,
    ports: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports>>,
    startup_probe: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_route: std::option::Option<terrars::PrimField<std::string::String>>,
    image_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predict_route: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_memory_size_mb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_ports: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_probe: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_probe: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_probe: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe>>,
    dynamic: VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec {
    #[doc =
        "Set the field `args`.\nSpecifies arguments for the command that runs when the container starts.\nThis overrides the container's\n['CMD'](https://docs.docker.com/engine/reference/builder/#cmd). Specify\nthis field as an array of executable and arguments, similar to a Docker\n'CMD''s \"default parameters\" form.\n\nIf you don't specify this field but do specify the\ncommand field, then the command from the\n'command' field runs without any additional arguments. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIf you don't specify this field and don't specify the 'command' field,\nthen the container's\n['ENTRYPOINT'](https://docs.docker.com/engine/reference/builder/#cmd) and\n'CMD' determine what runs based on their default behavior. See the Docker\ndocumentation about [how 'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIn this field, you can reference [environment variables\nset by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'args' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nSpecifies the command that runs when the container starts. This overrides\nthe container's\n[ENTRYPOINT](https://docs.docker.com/engine/reference/builder/#entrypoint).\nSpecify this field as an array of executable and arguments, similar to a\nDocker 'ENTRYPOINT''s \"exec\" form, not its \"shell\" form.\n\nIf you do not specify this field, then the container's 'ENTRYPOINT' runs,\nin conjunction with the args field or the\ncontainer's ['CMD'](https://docs.docker.com/engine/reference/builder/#cmd),\nif either exists. If this field is not specified and the container does not\nhave an 'ENTRYPOINT', then refer to the Docker documentation about [how\n'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIf you specify this field, then you can also specify the 'args' field to\nprovide additional arguments for this command. However, if you specify this\nfield, then the container's 'CMD' is ignored. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIn this field, you can reference [environment variables set by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'command' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_timeout`.\nDeployment timeout.\nLimit for deployment timeout is 2 hours."]
    pub fn set_deployment_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deployment_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `health_route`.\nHTTP path on the container to send health checks to. Vertex AI\nintermittently sends GET requests to this path on the container's IP\naddress and port to check that the container is healthy. Read more about\n[health\nchecks](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#health).\n\nFor example, if you set this field to '/bar', then Vertex AI\nintermittently sends a GET request to the '/bar' path on the port of your\ncontainer specified by the first value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code as the\n['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn set_health_route(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_route = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `predict_route`.\nHTTP path on the container to send prediction requests to. Vertex AI\nforwards requests sent using\nprojects.locations.endpoints.predict to this\npath on the container's IP address and port. Vertex AI then returns the\ncontainer's response in the API response.\n\nFor example, if you set this field to '/foo', then when Vertex AI\nreceives a prediction request, it forwards the request body in a POST\nrequest to the '/foo' path on the port of your container specified by the\nfirst value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code\nas the ['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn set_predict_route(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.predict_route = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_memory_size_mb`.\nThe amount of the VM memory to reserve as the shared memory for the model\nin megabytes."]
    pub fn set_shared_memory_size_mb(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.shared_memory_size_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Env>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grpc_ports`.\n"]
    pub fn set_grpc_ports(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPorts,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc_ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_ports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_probe`.\n"]
    pub fn set_health_probe(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbe,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_probe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_probe = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbe,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.liveness_probe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.liveness_probe = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_Ports>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbe,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.startup_probe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.startup_probe = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec {
    #[doc =
        "URI of the Docker image to be used as the custom container for serving\npredictions. This URI must identify an image in Artifact Registry or\nContainer Registry. Learn more about the [container publishing\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#publishing),\nincluding permissions requirements for the Vertex AI Service Agent.\n\nThe container image is ingested upon ModelService.UploadModel, stored\ninternally, and this original path is afterwards not used.\n\nTo learn about the requirements for the Docker image itself, see\n[Custom container\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#).\n\nYou can use the URI to one of Vertex AI's [pre-built container images for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/pre-built-containers)\nin this field."]
    pub image_uri: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            deployment_timeout: core::default::Default::default(),
            health_route: core::default::Default::default(),
            image_uri: self.image_uri,
            predict_route: core::default::Default::default(),
            shared_memory_size_mb: core::default::Default::default(),
            env: core::default::Default::default(),
            grpc_ports: core::default::Default::default(),
            health_probe: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            ports: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nSpecifies arguments for the command that runs when the container starts.\nThis overrides the container's\n['CMD'](https://docs.docker.com/engine/reference/builder/#cmd). Specify\nthis field as an array of executable and arguments, similar to a Docker\n'CMD''s \"default parameters\" form.\n\nIf you don't specify this field but do specify the\ncommand field, then the command from the\n'command' field runs without any additional arguments. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIf you don't specify this field and don't specify the 'command' field,\nthen the container's\n['ENTRYPOINT'](https://docs.docker.com/engine/reference/builder/#cmd) and\n'CMD' determine what runs based on their default behavior. See the Docker\ndocumentation about [how 'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIn this field, you can reference [environment variables\nset by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'args' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nSpecifies the command that runs when the container starts. This overrides\nthe container's\n[ENTRYPOINT](https://docs.docker.com/engine/reference/builder/#entrypoint).\nSpecify this field as an array of executable and arguments, similar to a\nDocker 'ENTRYPOINT''s \"exec\" form, not its \"shell\" form.\n\nIf you do not specify this field, then the container's 'ENTRYPOINT' runs,\nin conjunction with the args field or the\ncontainer's ['CMD'](https://docs.docker.com/engine/reference/builder/#cmd),\nif either exists. If this field is not specified and the container does not\nhave an 'ENTRYPOINT', then refer to the Docker documentation about [how\n'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIf you specify this field, then you can also specify the 'args' field to\nprovide additional arguments for this command. However, if you specify this\nfield, then the container's 'CMD' is ignored. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIn this field, you can reference [environment variables set by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'command' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_timeout` after provisioning.\nDeployment timeout.\nLimit for deployment timeout is 2 hours."]
    pub fn deployment_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `health_route` after provisioning.\nHTTP path on the container to send health checks to. Vertex AI\nintermittently sends GET requests to this path on the container's IP\naddress and port to check that the container is healthy. Read more about\n[health\nchecks](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#health).\n\nFor example, if you set this field to '/bar', then Vertex AI\nintermittently sends a GET request to the '/bar' path on the port of your\ncontainer specified by the first value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code as the\n['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn health_route(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_route", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_uri` after provisioning.\nURI of the Docker image to be used as the custom container for serving\npredictions. This URI must identify an image in Artifact Registry or\nContainer Registry. Learn more about the [container publishing\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#publishing),\nincluding permissions requirements for the Vertex AI Service Agent.\n\nThe container image is ingested upon ModelService.UploadModel, stored\ninternally, and this original path is afterwards not used.\n\nTo learn about the requirements for the Docker image itself, see\n[Custom container\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#).\n\nYou can use the URI to one of Vertex AI's [pre-built container images for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/pre-built-containers)\nin this field."]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `predict_route` after provisioning.\nHTTP path on the container to send prediction requests to. Vertex AI\nforwards requests sent using\nprojects.locations.endpoints.predict to this\npath on the container's IP address and port. Vertex AI then returns the\ncontainer's response in the API response.\n\nFor example, if you set this field to '/foo', then when Vertex AI\nreceives a prediction request, it forwards the request body in a POST\nrequest to the '/foo' path on the port of your container specified by the\nfirst value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code\nas the ['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn predict_route(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predict_route", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shared_memory_size_mb` after provisioning.\nThe amount of the VM memory to reserve as the shared memory for the model\nin megabytes."]
    pub fn shared_memory_size_mb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_memory_size_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_EnvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_ports` after provisioning.\n"]
    pub fn grpc_ports(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_GrpcPortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `health_probe` after provisioning.\n"]
    pub fn health_probe(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_HealthProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_LivenessProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec_StartupProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.startup_probe", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeployment_ModelConfigDynamic {
    container_spec: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accept_eula: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugging_face_access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugging_face_cache_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_spec: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec>>,
    dynamic: VertexAiEndpointWithModelGardenDeployment_ModelConfigDynamic,
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfig {
    #[doc =
        "Set the field `accept_eula`.\nWhether the user accepts the End User License Agreement (EULA)\nfor the model."]
    pub fn set_accept_eula(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.accept_eula = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hugging_face_access_token`.\nThe Hugging Face read access token used to access the model\nartifacts of gated models."]
    pub fn set_hugging_face_access_token(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hugging_face_access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hugging_face_cache_enabled`.\nIf true, the model will deploy with a cached version instead of directly\ndownloading the model artifacts from Hugging Face. This is suitable for\nVPC-SC users with limited internet access."]
    pub fn set_hugging_face_cache_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.hugging_face_cache_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model_display_name`.\nThe user-specified display name of the uploaded model. If not\nset, a default name will be used."]
    pub fn set_model_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.model_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_spec`.\n"]
    pub fn set_container_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_ModelConfig {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_ModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig {}

impl BuildVertexAiEndpointWithModelGardenDeployment_ModelConfig {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_ModelConfig {
        VertexAiEndpointWithModelGardenDeployment_ModelConfig {
            accept_eula: core::default::Default::default(),
            hugging_face_access_token: core::default::Default::default(),
            hugging_face_cache_enabled: core::default::Default::default(),
            model_display_name: core::default::Default::default(),
            container_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_ModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_ModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_ModelConfigRef {
        VertexAiEndpointWithModelGardenDeployment_ModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_ModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accept_eula` after provisioning.\nWhether the user accepts the End User License Agreement (EULA)\nfor the model."]
    pub fn accept_eula(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hugging_face_access_token` after provisioning.\nThe Hugging Face read access token used to access the model\nartifacts of gated models."]
    pub fn hugging_face_access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugging_face_access_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hugging_face_cache_enabled` after provisioning.\nIf true, the model will deploy with a cached version instead of directly\ndownloading the model artifacts from Hugging Face. This is suitable for\nVPC-SC users with limited internet access."]
    pub fn hugging_face_cache_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugging_face_cache_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_display_name` after provisioning.\nThe user-specified display name of the uploaded model. If not\nset, a default name will be used."]
    pub fn model_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_spec` after provisioning.\n"]
    pub fn container_spec(
        &self,
    ) -> terrars::ListRef<VertexAiEndpointWithModelGardenDeployment_ModelConfig_ContainerSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeployment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpointWithModelGardenDeployment_Timeouts {
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

impl terrars::ToListMappable for VertexAiEndpointWithModelGardenDeployment_Timeouts {
    type O = terrars::BlockAssignable<VertexAiEndpointWithModelGardenDeployment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointWithModelGardenDeployment_Timeouts {}

impl BuildVertexAiEndpointWithModelGardenDeployment_Timeouts {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeployment_Timeouts {
        VertexAiEndpointWithModelGardenDeployment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpointWithModelGardenDeployment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointWithModelGardenDeployment_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpointWithModelGardenDeployment_TimeoutsRef {
        VertexAiEndpointWithModelGardenDeployment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointWithModelGardenDeployment_TimeoutsRef {
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
