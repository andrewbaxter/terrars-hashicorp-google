#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiEndpointData_ {
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
    dedicated_endpoint_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_split: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<VertexAiEndpoint_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predict_request_response_logging_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpoint_PredictRequestResponseLoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_service_connect_config: std::option::Option<std::vec::Vec<VertexAiEndpoint_PrivateServiceConnectConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiEndpoint_Timeouts>,
    dynamic: VertexAiEndpointDynamic,
}

struct VertexAiEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiEndpointData_>,
}

#[derive(Clone)]
pub struct VertexAiEndpoint(std::rc::Rc<VertexAiEndpoint_>);

impl VertexAiEndpoint {
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
        "Set the field `dedicated_endpoint_enabled`.\nIf true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon."]
    pub fn set_dedicated_endpoint_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dedicated_endpoint_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the Endpoint."]
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
        "Set the field `labels`.\nThe labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'. Where '{project}' is a project number, as in '12345', and '{network}' is network name. Only one of the fields, 'network' or 'privateServiceConnectConfig', can be set."]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region for the resource"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `traffic_split`.\nA map from a DeployedModel's id to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel.\nIf a DeployedModel's id is not listed in this map, then it receives no traffic.\nThe traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. See\nthe 'deployModel' [example](https://cloud.google.com/vertex-ai/docs/general/deployment#deploy_a_model_to_an_endpoint) and\n[documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints/deployModel) for more information.\n\n~> **Note:** To set the map to empty, set '\"{}\"', apply, and then remove the field from your config."]
    pub fn set_traffic_split(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().traffic_split = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiEndpoint_EncryptionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predict_request_response_logging_config`.\n"]
    pub fn set_predict_request_response_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiEndpoint_PredictRequestResponseLoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predict_request_response_logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predict_request_response_logging_config =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiEndpoint_PrivateServiceConnectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_service_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_service_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiEndpoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when this Endpoint was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dedicated_endpoint_dns` after provisioning.\nOutput only. DNS of the dedicated endpoint. Will only be populated if dedicatedEndpointEnabled is true. Format: 'https://{endpointId}.{region}-{projectNumber}.prediction.vertexai.goog'."]
    pub fn dedicated_endpoint_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_endpoint_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dedicated_endpoint_enabled` after provisioning.\nIf true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon."]
    pub fn dedicated_endpoint_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_endpoint_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_models` after provisioning.\nOutput only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. Models can also be deployed and undeployed using the [Cloud Console](https://console.cloud.google.com/vertex-ai/)."]
    pub fn deployed_models(&self) -> terrars::ListRef<VertexAiEndpoint_DeployedModelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Endpoint."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates. If not set, a blind \"overwrite\" update happens."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `model_deployment_monitoring_job` after provisioning.\nOutput only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by CreateModelDeploymentMonitoringJob. Format: 'projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}'"]
    pub fn model_deployment_monitoring_job(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_deployment_monitoring_job", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'. Where '{project}' is a project number, as in '12345', and '{network}' is network name. Only one of the fields, 'network' or 'privateServiceConnectConfig', can be set."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region for the resource"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_split` after provisioning.\nA map from a DeployedModel's id to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel.\nIf a DeployedModel's id is not listed in this map, then it receives no traffic.\nThe traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. See\nthe 'deployModel' [example](https://cloud.google.com/vertex-ai/docs/general/deployment#deploy_a_model_to_an_endpoint) and\n[documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints/deployModel) for more information.\n\n~> **Note:** To set the map to empty, set '\"{}\"', apply, and then remove the field from your config."]
    pub fn traffic_split(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_split", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this Endpoint was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiEndpoint_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predict_request_response_logging_config` after provisioning.\n"]
    pub fn predict_request_response_logging_config(
        &self,
    ) -> terrars::ListRef<VertexAiEndpoint_PredictRequestResponseLoggingConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predict_request_response_logging_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(&self) -> terrars::ListRef<VertexAiEndpoint_PrivateServiceConnectConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpoint_TimeoutsRef {
        VertexAiEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VertexAiEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiEndpoint { }

impl terrars::ToListMappable for VertexAiEndpoint {
    type O = terrars::ListRef<VertexAiEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiEndpoint {
    pub tf_id: std::string::String,
    #[doc =
        "Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiEndpoint {
        let out = VertexAiEndpoint(std::rc::Rc::new(VertexAiEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dedicated_endpoint_enabled: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                traffic_split: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                predict_request_response_logging_config: core::default::Default::default(),
                private_service_connect_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiEndpointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when this Endpoint was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dedicated_endpoint_dns` after provisioning.\nOutput only. DNS of the dedicated endpoint. Will only be populated if dedicatedEndpointEnabled is true. Format: 'https://{endpointId}.{region}-{projectNumber}.prediction.vertexai.goog'."]
    pub fn dedicated_endpoint_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_endpoint_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dedicated_endpoint_enabled` after provisioning.\nIf true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon."]
    pub fn dedicated_endpoint_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_endpoint_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_models` after provisioning.\nOutput only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. Models can also be deployed and undeployed using the [Cloud Console](https://console.cloud.google.com/vertex-ai/)."]
    pub fn deployed_models(&self) -> terrars::ListRef<VertexAiEndpoint_DeployedModelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Endpoint."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates. If not set, a blind \"overwrite\" update happens."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `model_deployment_monitoring_job` after provisioning.\nOutput only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by CreateModelDeploymentMonitoringJob. Format: 'projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}'"]
    pub fn model_deployment_monitoring_job(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_deployment_monitoring_job", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'. Where '{project}' is a project number, as in '12345', and '{network}' is network name. Only one of the fields, 'network' or 'privateServiceConnectConfig', can be set."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region for the resource"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_split` after provisioning.\nA map from a DeployedModel's id to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel.\nIf a DeployedModel's id is not listed in this map, then it receives no traffic.\nThe traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. See\nthe 'deployModel' [example](https://cloud.google.com/vertex-ai/docs/general/deployment#deploy_a_model_to_an_endpoint) and\n[documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints/deployModel) for more information.\n\n~> **Note:** To set the map to empty, set '\"{}\"', apply, and then remove the field from your config."]
    pub fn traffic_split(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_split", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this Endpoint was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiEndpoint_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predict_request_response_logging_config` after provisioning.\n"]
    pub fn predict_request_response_logging_config(
        &self,
    ) -> terrars::ListRef<VertexAiEndpoint_PredictRequestResponseLoggingConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predict_request_response_logging_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(&self) -> terrars::ListRef<VertexAiEndpoint_PrivateServiceConnectConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpoint_TimeoutsRef {
        VertexAiEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointDynamic {
    encryption_spec: std::option::Option<terrars::DynamicBlock<VertexAiEndpoint_EncryptionSpec>>,
    predict_request_response_logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpoint_PredictRequestResponseLoggingConfig>>,
    private_service_connect_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpoint_PrivateServiceConnectConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_DeployedModels_AutomaticResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_replica_count: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiEndpoint_DeployedModels_AutomaticResources {
    #[doc = "Set the field `max_replica_count`.\n"]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_replica_count`.\n"]
    pub fn set_min_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_replica_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_DeployedModels_AutomaticResources {
    type O = terrars::BlockAssignable<VertexAiEndpoint_DeployedModels_AutomaticResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_DeployedModels_AutomaticResources {}

impl BuildVertexAiEndpoint_DeployedModels_AutomaticResources {
    pub fn build(self) -> VertexAiEndpoint_DeployedModels_AutomaticResources {
        VertexAiEndpoint_DeployedModels_AutomaticResources {
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_DeployedModels_AutomaticResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_DeployedModels_AutomaticResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_DeployedModels_AutomaticResourcesRef {
        VertexAiEndpoint_DeployedModels_AutomaticResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_DeployedModels_AutomaticResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_replica_count` after provisioning.\n"]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_replica_count` after provisioning.\n"]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs {
    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs {
    type O = terrars::BlockAssignable<VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs {}

impl BuildVertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs {
    pub fn build(self) -> VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs {
        VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs {
            metric_name: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecsRef {
        VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec {
    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec {
    type O = terrars::BlockAssignable<VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec {}

impl BuildVertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec {
    pub fn build(self) -> VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec {
        VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpecRef {
        VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_DeployedModels_DedicatedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_metric_specs: std
    ::option
    ::Option<terrars::ListField<VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std::option::Option<terrars::ListField<VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_replica_count: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiEndpoint_DeployedModels_DedicatedResources {
    #[doc = "Set the field `autoscaling_metric_specs`.\n"]
    pub fn set_autoscaling_metric_specs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecs>,
                    >,
    ) -> Self {
        self.autoscaling_metric_specs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpec>>,
    ) -> Self {
        self.machine_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_replica_count`.\n"]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_replica_count`.\n"]
    pub fn set_min_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_replica_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_DeployedModels_DedicatedResources {
    type O = terrars::BlockAssignable<VertexAiEndpoint_DeployedModels_DedicatedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_DeployedModels_DedicatedResources {}

impl BuildVertexAiEndpoint_DeployedModels_DedicatedResources {
    pub fn build(self) -> VertexAiEndpoint_DeployedModels_DedicatedResources {
        VertexAiEndpoint_DeployedModels_DedicatedResources {
            autoscaling_metric_specs: core::default::Default::default(),
            machine_spec: core::default::Default::default(),
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_DeployedModels_DedicatedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_DeployedModels_DedicatedResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_DeployedModels_DedicatedResourcesRef {
        VertexAiEndpoint_DeployedModels_DedicatedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_DeployedModels_DedicatedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_metric_specs` after provisioning.\n"]
    pub fn autoscaling_metric_specs(
        &self,
    ) -> terrars::ListRef<VertexAiEndpoint_DeployedModels_DedicatedResources_AutoscalingMetricSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_metric_specs", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> terrars::ListRef<VertexAiEndpoint_DeployedModels_DedicatedResources_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `max_replica_count` after provisioning.\n"]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_replica_count` after provisioning.\n"]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_DeployedModels_PrivateEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explain_http_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_http_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predict_http_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpoint_DeployedModels_PrivateEndpoints {
    #[doc = "Set the field `explain_http_uri`.\n"]
    pub fn set_explain_http_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.explain_http_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_http_uri`.\n"]
    pub fn set_health_http_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_http_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predict_http_uri`.\n"]
    pub fn set_predict_http_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.predict_http_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_DeployedModels_PrivateEndpoints {
    type O = terrars::BlockAssignable<VertexAiEndpoint_DeployedModels_PrivateEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_DeployedModels_PrivateEndpoints {}

impl BuildVertexAiEndpoint_DeployedModels_PrivateEndpoints {
    pub fn build(self) -> VertexAiEndpoint_DeployedModels_PrivateEndpoints {
        VertexAiEndpoint_DeployedModels_PrivateEndpoints {
            explain_http_uri: core::default::Default::default(),
            health_http_uri: core::default::Default::default(),
            predict_http_uri: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_DeployedModels_PrivateEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_DeployedModels_PrivateEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_DeployedModels_PrivateEndpointsRef {
        VertexAiEndpoint_DeployedModels_PrivateEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_DeployedModels_PrivateEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `explain_http_uri` after provisioning.\n"]
    pub fn explain_http_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.explain_http_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `health_http_uri` after provisioning.\n"]
    pub fn health_http_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_http_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `predict_http_uri` after provisioning.\n"]
    pub fn predict_http_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predict_http_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_DeployedModels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_resources: std::option::Option<terrars::ListField<VertexAiEndpoint_DeployedModels_AutomaticResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_resources: std::option::Option<terrars::ListField<VertexAiEndpoint_DeployedModels_DedicatedResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_access_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_container_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoints: std::option::Option<terrars::ListField<VertexAiEndpoint_DeployedModels_PrivateEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_resources: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpoint_DeployedModels {
    #[doc = "Set the field `automatic_resources`.\n"]
    pub fn set_automatic_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<VertexAiEndpoint_DeployedModels_AutomaticResources>>,
    ) -> Self {
        self.automatic_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<VertexAiEndpoint_DeployedModels_DedicatedResources>>,
    ) -> Self {
        self.dedicated_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_access_logging`.\n"]
    pub fn set_enable_access_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_access_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_container_logging`.\n"]
    pub fn set_enable_container_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_container_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_version_id`.\n"]
    pub fn set_model_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoints`.\n"]
    pub fn set_private_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<VertexAiEndpoint_DeployedModels_PrivateEndpoints>>,
    ) -> Self {
        self.private_endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shared_resources`.\n"]
    pub fn set_shared_resources(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shared_resources = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_DeployedModels {
    type O = terrars::BlockAssignable<VertexAiEndpoint_DeployedModels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_DeployedModels {}

impl BuildVertexAiEndpoint_DeployedModels {
    pub fn build(self) -> VertexAiEndpoint_DeployedModels {
        VertexAiEndpoint_DeployedModels {
            automatic_resources: core::default::Default::default(),
            create_time: core::default::Default::default(),
            dedicated_resources: core::default::Default::default(),
            display_name: core::default::Default::default(),
            enable_access_logging: core::default::Default::default(),
            enable_container_logging: core::default::Default::default(),
            id: core::default::Default::default(),
            model: core::default::Default::default(),
            model_version_id: core::default::Default::default(),
            private_endpoints: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shared_resources: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_DeployedModelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_DeployedModelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiEndpoint_DeployedModelsRef {
        VertexAiEndpoint_DeployedModelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_DeployedModelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatic_resources` after provisioning.\n"]
    pub fn automatic_resources(&self) -> terrars::ListRef<VertexAiEndpoint_DeployedModels_AutomaticResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(&self) -> terrars::ListRef<VertexAiEndpoint_DeployedModels_DedicatedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_access_logging` after provisioning.\n"]
    pub fn enable_access_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_access_logging", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_container_logging` after provisioning.\n"]
    pub fn enable_container_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_container_logging", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `model_version_id` after provisioning.\n"]
    pub fn model_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_version_id", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoints` after provisioning.\n"]
    pub fn private_endpoints(&self) -> terrars::ListRef<VertexAiEndpoint_DeployedModels_PrivateEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `shared_resources` after provisioning.\n"]
    pub fn shared_resources(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_EncryptionSpec {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl VertexAiEndpoint_EncryptionSpec { }

impl terrars::ToListMappable for VertexAiEndpoint_EncryptionSpec {
    type O = terrars::BlockAssignable<VertexAiEndpoint_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_EncryptionSpec {
    #[doc =
        "Required. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpoint_EncryptionSpec {
    pub fn build(self) -> VertexAiEndpoint_EncryptionSpec {
        VertexAiEndpoint_EncryptionSpec { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiEndpoint_EncryptionSpecRef {
        VertexAiEndpoint_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination {
    #[doc =
        "Set the field `output_uri`.\nBigQuery URI to a project or table, up to 2000 characters long. When only the project is specified, the Dataset and Table is created. When the full table reference is specified, the Dataset must exist and table must not exist. Accepted forms: - BigQuery path. For example: 'bq://projectId' or 'bq://projectId.bqDatasetId' or 'bq://projectId.bqDatasetId.bqTableId'."]
    pub fn set_output_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination {
    type O = terrars::BlockAssignable<VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination {}

impl BuildVertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination {
    pub fn build(self) -> VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination {
        VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination {
            output_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestinationRef {
        VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_uri` after provisioning.\nBigQuery URI to a project or table, up to 2000 characters long. When only the project is specified, the Dataset and Table is created. When the full table reference is specified, the Dataset must exist and table must not exist. Accepted forms: - BigQuery path. For example: 'bq://projectId' or 'bq://projectId.bqDatasetId' or 'bq://projectId.bqDatasetId.bqTableId'."]
    pub fn output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpoint_PredictRequestResponseLoggingConfigDynamic {
    bigquery_destination: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination>>,
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_PredictRequestResponseLoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_destination: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination>>,
    dynamic: VertexAiEndpoint_PredictRequestResponseLoggingConfigDynamic,
}

impl VertexAiEndpoint_PredictRequestResponseLoggingConfig {
    #[doc = "Set the field `enabled`.\nIf logging is enabled or not."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sampling_rate`.\nPercentage of requests to be logged, expressed as a fraction in range(0,1]"]
    pub fn set_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_destination`.\n"]
    pub fn set_bigquery_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestination>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiEndpoint_PredictRequestResponseLoggingConfig {
    type O = terrars::BlockAssignable<VertexAiEndpoint_PredictRequestResponseLoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_PredictRequestResponseLoggingConfig {}

impl BuildVertexAiEndpoint_PredictRequestResponseLoggingConfig {
    pub fn build(self) -> VertexAiEndpoint_PredictRequestResponseLoggingConfig {
        VertexAiEndpoint_PredictRequestResponseLoggingConfig {
            enabled: core::default::Default::default(),
            sampling_rate: core::default::Default::default(),
            bigquery_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_PredictRequestResponseLoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_PredictRequestResponseLoggingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_PredictRequestResponseLoggingConfigRef {
        VertexAiEndpoint_PredictRequestResponseLoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_PredictRequestResponseLoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIf logging is enabled or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sampling_rate` after provisioning.\nPercentage of requests to be logged, expressed as a fraction in range(0,1]"]
    pub fn sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_destination` after provisioning.\n"]
    pub fn bigquery_destination(
        &self,
    ) -> terrars::ListRef<VertexAiEndpoint_PredictRequestResponseLoggingConfig_BigqueryDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs {
    network: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs { }

impl terrars::ToListMappable for VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs {
    type O = terrars::BlockAssignable<VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs {
    #[doc =
        "The full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/get): projects/{project}/global/networks/{network}."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc = "Project id used to create forwarding rule."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs {
    pub fn build(self) -> VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs {
        VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs {
            network: self.network,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigsRef {
        VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nError message if the PSC service automation failed."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nForwarding rule created by the PSC service automation."]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nIP address rule created by the PSC service automation."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/get): projects/{project}/global/networks/{network}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nProject id used to create forwarding rule."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the PSC service automation."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpoint_PrivateServiceConnectConfigDynamic {
    psc_automation_configs: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs>>,
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_PrivateServiceConnectConfig {
    enable_private_service_connect: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_allowlist: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_automation_configs: std
    ::option
    ::Option<std::vec::Vec<VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs>>,
    dynamic: VertexAiEndpoint_PrivateServiceConnectConfigDynamic,
}

impl VertexAiEndpoint_PrivateServiceConnectConfig {
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
                        terrars::BlockAssignable<VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigs>,
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

impl terrars::ToListMappable for VertexAiEndpoint_PrivateServiceConnectConfig {
    type O = terrars::BlockAssignable<VertexAiEndpoint_PrivateServiceConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_PrivateServiceConnectConfig {
    #[doc = "Required. If true, expose the IndexEndpoint via private service connect."]
    pub enable_private_service_connect: terrars::PrimField<bool>,
}

impl BuildVertexAiEndpoint_PrivateServiceConnectConfig {
    pub fn build(self) -> VertexAiEndpoint_PrivateServiceConnectConfig {
        VertexAiEndpoint_PrivateServiceConnectConfig {
            enable_private_service_connect: self.enable_private_service_connect,
            project_allowlist: core::default::Default::default(),
            psc_automation_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_PrivateServiceConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_PrivateServiceConnectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiEndpoint_PrivateServiceConnectConfigRef {
        VertexAiEndpoint_PrivateServiceConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_PrivateServiceConnectConfigRef {
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

    #[doc = "Get a reference to the value of field `psc_automation_configs` after provisioning.\n"]
    pub fn psc_automation_configs(
        &self,
    ) -> terrars::ListRef<VertexAiEndpoint_PrivateServiceConnectConfig_PscAutomationConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_automation_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiEndpoint_Timeouts {
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

impl terrars::ToListMappable for VertexAiEndpoint_Timeouts {
    type O = terrars::BlockAssignable<VertexAiEndpoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpoint_Timeouts {}

impl BuildVertexAiEndpoint_Timeouts {
    pub fn build(self) -> VertexAiEndpoint_Timeouts {
        VertexAiEndpoint_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiEndpoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiEndpoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiEndpoint_TimeoutsRef {
        VertexAiEndpoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpoint_TimeoutsRef {
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
