use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<VertexAiEndpointEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiEndpointTimeoutsEl>,
    dynamic: VertexAiEndpointDynamic,
}

struct VertexAiEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiEndpointData>,
}

#[derive(Clone)]
pub struct VertexAiEndpoint(Rc<VertexAiEndpoint_>);

impl VertexAiEndpoint {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\nThe description of the Endpoint."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nThe labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'. Where '{project}' is a project number, as in '12345', and '{network}' is network name."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe region for the resource"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(self, v: impl Into<BlockAssignable<VertexAiEndpointEncryptionSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when this Endpoint was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployed_models` after provisioning.\nOutput only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. Models can also be deployed and undeployed using the [Cloud Console](https://console.cloud.google.com/vertex-ai/)."]
    pub fn deployed_models(&self) -> ListRef<VertexAiEndpointDeployedModelsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployed_models", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the Endpoint."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates. If not set, a blind \"overwrite\" update happens."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `model_deployment_monitoring_job` after provisioning.\nOutput only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by CreateModelDeploymentMonitoringJob. Format: 'projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}'"]
    pub fn model_deployment_monitoring_job(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_deployment_monitoring_job", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'. Where '{project}' is a project number, as in '12345', and '{network}' is network name."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region for the resource"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this Endpoint was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiEndpointEncryptionSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpointTimeoutsElRef {
        VertexAiEndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VertexAiEndpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VertexAiEndpoint { }

impl ToListMappable for VertexAiEndpoint {
    type O = ListRef<VertexAiEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VertexAiEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiEndpoint {
    pub tf_id: String,
    #[doc= "Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub display_name: PrimField<String>,
    #[doc= "The location for the resource"]
    pub location: PrimField<String>,
    #[doc= "The resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits."]
    pub name: PrimField<String>,
}

impl BuildVertexAiEndpoint {
    pub fn build(self, stack: &mut Stack) -> VertexAiEndpoint {
        let out = VertexAiEndpoint(Rc::new(VertexAiEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when this Endpoint was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployed_models` after provisioning.\nOutput only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. Models can also be deployed and undeployed using the [Cloud Console](https://console.cloud.google.com/vertex-ai/)."]
    pub fn deployed_models(&self) -> ListRef<VertexAiEndpointDeployedModelsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployed_models", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the Endpoint."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates. If not set, a blind \"overwrite\" update happens."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `model_deployment_monitoring_job` after provisioning.\nOutput only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by CreateModelDeploymentMonitoringJob. Format: 'projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}'"]
    pub fn model_deployment_monitoring_job(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_deployment_monitoring_job", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'. Where '{project}' is a project number, as in '12345', and '{network}' is network name."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region for the resource"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this Endpoint was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiEndpointEncryptionSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpointTimeoutsElRef {
        VertexAiEndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointDeployedModelsElAutomaticResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_replica_count: Option<PrimField<f64>>,
}

impl VertexAiEndpointDeployedModelsElAutomaticResourcesEl {
    #[doc= "Set the field `max_replica_count`.\n"]
    pub fn set_max_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_replica_count = Some(v.into());
        self
    }

    #[doc= "Set the field `min_replica_count`.\n"]
    pub fn set_min_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_replica_count = Some(v.into());
        self
    }
}

impl ToListMappable for VertexAiEndpointDeployedModelsElAutomaticResourcesEl {
    type O = BlockAssignable<VertexAiEndpointDeployedModelsElAutomaticResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointDeployedModelsElAutomaticResourcesEl {}

impl BuildVertexAiEndpointDeployedModelsElAutomaticResourcesEl {
    pub fn build(self) -> VertexAiEndpointDeployedModelsElAutomaticResourcesEl {
        VertexAiEndpointDeployedModelsElAutomaticResourcesEl {
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}

pub struct VertexAiEndpointDeployedModelsElAutomaticResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointDeployedModelsElAutomaticResourcesElRef {
    fn new(shared: StackShared, base: String) -> VertexAiEndpointDeployedModelsElAutomaticResourcesElRef {
        VertexAiEndpointDeployedModelsElAutomaticResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointDeployedModelsElAutomaticResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `max_replica_count` after provisioning.\n"]
    pub fn max_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc= "Get a reference to the value of field `min_replica_count` after provisioning.\n"]
    pub fn min_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<f64>>,
}

impl VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl {
    #[doc= "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_name = Some(v.into());
        self
    }

    #[doc= "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target = Some(v.into());
        self
    }
}

impl ToListMappable for VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl {
    type O = BlockAssignable<VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl {}

impl BuildVertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl {
    pub fn build(self) -> VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl {
        VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl {
            metric_name: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

pub struct VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsElRef {
        VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}

impl VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl {
    #[doc= "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }

    #[doc= "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }

    #[doc= "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}

impl ToListMappable for VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl {
    type O = BlockAssignable<VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl {}

impl BuildVertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl {
    pub fn build(self) -> VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl {
        VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

pub struct VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecElRef {
        VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc= "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointDeployedModelsElDedicatedResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_metric_specs: Option<
        ListField<VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_spec: Option<ListField<VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_replica_count: Option<PrimField<f64>>,
}

impl VertexAiEndpointDeployedModelsElDedicatedResourcesEl {
    #[doc= "Set the field `autoscaling_metric_specs`.\n"]
    pub fn set_autoscaling_metric_specs(
        mut self,
        v: impl Into<ListField<VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsEl>>,
    ) -> Self {
        self.autoscaling_metric_specs = Some(v.into());
        self
    }

    #[doc= "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v: impl Into<ListField<VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecEl>>,
    ) -> Self {
        self.machine_spec = Some(v.into());
        self
    }

    #[doc= "Set the field `max_replica_count`.\n"]
    pub fn set_max_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_replica_count = Some(v.into());
        self
    }

    #[doc= "Set the field `min_replica_count`.\n"]
    pub fn set_min_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_replica_count = Some(v.into());
        self
    }
}

impl ToListMappable for VertexAiEndpointDeployedModelsElDedicatedResourcesEl {
    type O = BlockAssignable<VertexAiEndpointDeployedModelsElDedicatedResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointDeployedModelsElDedicatedResourcesEl {}

impl BuildVertexAiEndpointDeployedModelsElDedicatedResourcesEl {
    pub fn build(self) -> VertexAiEndpointDeployedModelsElDedicatedResourcesEl {
        VertexAiEndpointDeployedModelsElDedicatedResourcesEl {
            autoscaling_metric_specs: core::default::Default::default(),
            machine_spec: core::default::Default::default(),
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}

pub struct VertexAiEndpointDeployedModelsElDedicatedResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointDeployedModelsElDedicatedResourcesElRef {
    fn new(shared: StackShared, base: String) -> VertexAiEndpointDeployedModelsElDedicatedResourcesElRef {
        VertexAiEndpointDeployedModelsElDedicatedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointDeployedModelsElDedicatedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `autoscaling_metric_specs` after provisioning.\n"]
    pub fn autoscaling_metric_specs(
        &self,
    ) -> ListRef<VertexAiEndpointDeployedModelsElDedicatedResourcesElAutoscalingMetricSpecsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling_metric_specs", self.base))
    }

    #[doc= "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> ListRef<VertexAiEndpointDeployedModelsElDedicatedResourcesElMachineSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }

    #[doc= "Get a reference to the value of field `max_replica_count` after provisioning.\n"]
    pub fn max_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc= "Get a reference to the value of field `min_replica_count` after provisioning.\n"]
    pub fn min_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointDeployedModelsElPrivateEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    explain_http_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_http_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predict_http_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}

impl VertexAiEndpointDeployedModelsElPrivateEndpointsEl {
    #[doc= "Set the field `explain_http_uri`.\n"]
    pub fn set_explain_http_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.explain_http_uri = Some(v.into());
        self
    }

    #[doc= "Set the field `health_http_uri`.\n"]
    pub fn set_health_http_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_http_uri = Some(v.into());
        self
    }

    #[doc= "Set the field `predict_http_uri`.\n"]
    pub fn set_predict_http_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predict_http_uri = Some(v.into());
        self
    }

    #[doc= "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}

impl ToListMappable for VertexAiEndpointDeployedModelsElPrivateEndpointsEl {
    type O = BlockAssignable<VertexAiEndpointDeployedModelsElPrivateEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointDeployedModelsElPrivateEndpointsEl {}

impl BuildVertexAiEndpointDeployedModelsElPrivateEndpointsEl {
    pub fn build(self) -> VertexAiEndpointDeployedModelsElPrivateEndpointsEl {
        VertexAiEndpointDeployedModelsElPrivateEndpointsEl {
            explain_http_uri: core::default::Default::default(),
            health_http_uri: core::default::Default::default(),
            predict_http_uri: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

pub struct VertexAiEndpointDeployedModelsElPrivateEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointDeployedModelsElPrivateEndpointsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiEndpointDeployedModelsElPrivateEndpointsElRef {
        VertexAiEndpointDeployedModelsElPrivateEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointDeployedModelsElPrivateEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `explain_http_uri` after provisioning.\n"]
    pub fn explain_http_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.explain_http_uri", self.base))
    }

    #[doc= "Get a reference to the value of field `health_http_uri` after provisioning.\n"]
    pub fn health_http_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_http_uri", self.base))
    }

    #[doc= "Get a reference to the value of field `predict_http_uri` after provisioning.\n"]
    pub fn predict_http_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predict_http_uri", self.base))
    }

    #[doc= "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointDeployedModelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_resources: Option<ListField<VertexAiEndpointDeployedModelsElAutomaticResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_resources: Option<ListField<VertexAiEndpointDeployedModelsElDedicatedResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_access_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_container_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_endpoints: Option<ListField<VertexAiEndpointDeployedModelsElPrivateEndpointsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_resources: Option<PrimField<String>>,
}

impl VertexAiEndpointDeployedModelsEl {
    #[doc= "Set the field `automatic_resources`.\n"]
    pub fn set_automatic_resources(
        mut self,
        v: impl Into<ListField<VertexAiEndpointDeployedModelsElAutomaticResourcesEl>>,
    ) -> Self {
        self.automatic_resources = Some(v.into());
        self
    }

    #[doc= "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }

    #[doc= "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        mut self,
        v: impl Into<ListField<VertexAiEndpointDeployedModelsElDedicatedResourcesEl>>,
    ) -> Self {
        self.dedicated_resources = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_access_logging`.\n"]
    pub fn set_enable_access_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_access_logging = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_container_logging`.\n"]
    pub fn set_enable_container_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_container_logging = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }

    #[doc= "Set the field `model_version_id`.\n"]
    pub fn set_model_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_version_id = Some(v.into());
        self
    }

    #[doc= "Set the field `private_endpoints`.\n"]
    pub fn set_private_endpoints(
        mut self,
        v: impl Into<ListField<VertexAiEndpointDeployedModelsElPrivateEndpointsEl>>,
    ) -> Self {
        self.private_endpoints = Some(v.into());
        self
    }

    #[doc= "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_resources`.\n"]
    pub fn set_shared_resources(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shared_resources = Some(v.into());
        self
    }
}

impl ToListMappable for VertexAiEndpointDeployedModelsEl {
    type O = BlockAssignable<VertexAiEndpointDeployedModelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointDeployedModelsEl {}

impl BuildVertexAiEndpointDeployedModelsEl {
    pub fn build(self) -> VertexAiEndpointDeployedModelsEl {
        VertexAiEndpointDeployedModelsEl {
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

pub struct VertexAiEndpointDeployedModelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointDeployedModelsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiEndpointDeployedModelsElRef {
        VertexAiEndpointDeployedModelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointDeployedModelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `automatic_resources` after provisioning.\n"]
    pub fn automatic_resources(&self) -> ListRef<VertexAiEndpointDeployedModelsElAutomaticResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.automatic_resources", self.base))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc= "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(&self) -> ListRef<VertexAiEndpointDeployedModelsElDedicatedResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dedicated_resources", self.base))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_access_logging` after provisioning.\n"]
    pub fn enable_access_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_access_logging", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_container_logging` after provisioning.\n"]
    pub fn enable_container_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_container_logging", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc= "Get a reference to the value of field `model_version_id` after provisioning.\n"]
    pub fn model_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_version_id", self.base))
    }

    #[doc= "Get a reference to the value of field `private_endpoints` after provisioning.\n"]
    pub fn private_endpoints(&self) -> ListRef<VertexAiEndpointDeployedModelsElPrivateEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_endpoints", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc= "Get a reference to the value of field `shared_resources` after provisioning.\n"]
    pub fn shared_resources(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointEncryptionSpecEl {
    kms_key_name: PrimField<String>,
}

impl VertexAiEndpointEncryptionSpecEl { }

impl ToListMappable for VertexAiEndpointEncryptionSpecEl {
    type O = BlockAssignable<VertexAiEndpointEncryptionSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointEncryptionSpecEl {
    #[doc= "Required. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub kms_key_name: PrimField<String>,
}

impl BuildVertexAiEndpointEncryptionSpecEl {
    pub fn build(self) -> VertexAiEndpointEncryptionSpecEl {
        VertexAiEndpointEncryptionSpecEl { kms_key_name: self.kms_key_name }
    }
}

pub struct VertexAiEndpointEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiEndpointEncryptionSpecElRef {
        VertexAiEndpointEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VertexAiEndpointTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for VertexAiEndpointTimeoutsEl {
    type O = BlockAssignable<VertexAiEndpointTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiEndpointTimeoutsEl {}

impl BuildVertexAiEndpointTimeoutsEl {
    pub fn build(self) -> VertexAiEndpointTimeoutsEl {
        VertexAiEndpointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VertexAiEndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VertexAiEndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiEndpointTimeoutsElRef {
        VertexAiEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiEndpointTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiEndpointDynamic {
    encryption_spec: Option<DynamicBlock<VertexAiEndpointEncryptionSpecEl>>,
}
