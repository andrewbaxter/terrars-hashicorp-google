use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudRunV2JobData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudRunV2Job_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudRunV2JobData>,
}

#[derive(Clone)]
pub struct DataCloudRunV2Job(std::rc::Rc<DataCloudRunV2Job_>);

impl DataCloudRunV2Job {
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

    #[doc = "Set the field `location`.\nThe location of the cloud run job"]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> terrars::ListRef<DataCloudRunV2JobBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client` after provisioning.\nArbitrary identifier for the API client."]
    pub fn client(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_version` after provisioning.\nArbitrary version identifier for the API client."]
    pub fn client_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on 'reconciliation' process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<DataCloudRunV2JobConditionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nEmail address of the authenticated creator."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
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
        "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_count` after provisioning.\nNumber of executions created for this job."]
    pub fn execution_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permamently deleted."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Job.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_created_execution` after provisioning.\nName of the last created execution."]
    pub fn latest_created_execution(&self) -> terrars::ListRef<DataCloudRunV2JobLatestCreatedExecutionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latest_created_execution", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run job"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Job. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn observed_generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observed_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Job is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, observedGeneration and latest_succeeded_execution, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: observedGeneration and generation, latest_succeeded_execution and latestCreatedExecution.\n\nIf reconciliation failed, observedGeneration and latest_succeeded_execution will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in terminalCondition and conditions"]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template` after provisioning.\nThe template used to create executions for this Job."]
    pub fn template(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> terrars::ListRef<DataCloudRunV2JobTerminalConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudRunV2Job {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudRunV2Job { }

impl terrars::ToListMappable for DataCloudRunV2Job {
    type O = terrars::ListRef<DataCloudRunV2JobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudRunV2Job_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_run_v2_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudRunV2Job {
    pub tf_id: std::string::String,
    #[doc = "Name of the Job."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudRunV2Job {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudRunV2Job {
        let out = DataCloudRunV2Job(std::rc::Rc::new(DataCloudRunV2Job_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudRunV2JobData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudRunV2JobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudRunV2JobRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> terrars::ListRef<DataCloudRunV2JobBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client` after provisioning.\nArbitrary identifier for the API client."]
    pub fn client(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_version` after provisioning.\nArbitrary version identifier for the API client."]
    pub fn client_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on 'reconciliation' process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<DataCloudRunV2JobConditionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nEmail address of the authenticated creator."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
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
        "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_count` after provisioning.\nNumber of executions created for this job."]
    pub fn execution_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permamently deleted."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Job.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_created_execution` after provisioning.\nName of the last created execution."]
    pub fn latest_created_execution(&self) -> terrars::ListRef<DataCloudRunV2JobLatestCreatedExecutionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latest_created_execution", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run job"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Job. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn observed_generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observed_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Job is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, observedGeneration and latest_succeeded_execution, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: observedGeneration and generation, latest_succeeded_execution and latestCreatedExecution.\n\nIf reconciliation failed, observedGeneration and latest_succeeded_execution will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in terminalCondition and conditions"]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template` after provisioning.\nThe template used to create executions for this Job."]
    pub fn template(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> terrars::ListRef<DataCloudRunV2JobTerminalConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    breakglass_justification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_default: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudRunV2JobBinaryAuthorizationEl {
    #[doc = "Set the field `breakglass_justification`.\n"]
    pub fn set_breakglass_justification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.breakglass_justification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_default`.\n"]
    pub fn set_use_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_default = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobBinaryAuthorizationEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobBinaryAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobBinaryAuthorizationEl {}

impl BuildDataCloudRunV2JobBinaryAuthorizationEl {
    pub fn build(self) -> DataCloudRunV2JobBinaryAuthorizationEl {
        DataCloudRunV2JobBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobBinaryAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobBinaryAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2JobBinaryAuthorizationElRef {
        DataCloudRunV2JobBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobBinaryAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `breakglass_justification` after provisioning.\n"]
    pub fn breakglass_justification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.breakglass_justification", self.base))
    }

    #[doc = "Get a reference to the value of field `use_default` after provisioning.\n"]
    pub fn use_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobConditionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transition_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobConditionsEl {
    #[doc = "Set the field `execution_reason`.\n"]
    pub fn set_execution_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_transition_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision_reason`.\n"]
    pub fn set_revision_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobConditionsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobConditionsEl {}

impl BuildDataCloudRunV2JobConditionsEl {
    pub fn build(self) -> DataCloudRunV2JobConditionsEl {
        DataCloudRunV2JobConditionsEl {
            execution_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            revision_reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobConditionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2JobConditionsElRef {
        DataCloudRunV2JobConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobConditionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_reason` after provisioning.\n"]
    pub fn execution_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_transition_time", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `revision_reason` after provisioning.\n"]
    pub fn revision_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobLatestCreatedExecutionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completion_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobLatestCreatedExecutionEl {
    #[doc = "Set the field `completion_time`.\n"]
    pub fn set_completion_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.completion_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobLatestCreatedExecutionEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobLatestCreatedExecutionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobLatestCreatedExecutionEl {}

impl BuildDataCloudRunV2JobLatestCreatedExecutionEl {
    pub fn build(self) -> DataCloudRunV2JobLatestCreatedExecutionEl {
        DataCloudRunV2JobLatestCreatedExecutionEl {
            completion_time: core::default::Default::default(),
            create_time: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobLatestCreatedExecutionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobLatestCreatedExecutionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2JobLatestCreatedExecutionElRef {
        DataCloudRunV2JobLatestCreatedExecutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobLatestCreatedExecutionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completion_time` after provisioning.\n"]
    pub fn completion_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completion_time", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    type O =
        terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key_ref: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
                    >,
    ) -> Self {
        self.secret_key_ref = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_key_ref", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_source: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_source`.\n"]
    pub fn set_value_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        self.value_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl { limits: core::default::Default::default() }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
        DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElContainersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobTemplateElTemplateElContainersEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    ) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
    ) -> Self {
        self.volume_mounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_dir`.\n"]
    pub fn set_working_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.working_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElContainersEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElContainersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElContainersEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElContainersEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElContainersEl {
        DataCloudRunV2JobTemplateElTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            name: core::default::Default::default(),
            ports: core::default::Default::default(),
            resources: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            working_dir: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElContainersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElContainersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElContainersElRef {
        DataCloudRunV2JobTemplateElTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElContainersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElEnvElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }

    #[doc = "Get a reference to the value of field `working_dir` after provisioning.\n"]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instances = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
            instances: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\n"]
    pub fn set_default_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            items: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_mode` after provisioning.\n"]
    pub fn default_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVolumesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    ) -> Self {
        self.cloud_sql_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
    ) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElVolumesEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElVolumesEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElVolumesEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVolumesEl {
        DataCloudRunV2JobTemplateElTemplateElVolumesEl {
            cloud_sql_instance: core::default::Default::default(),
            name: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
        DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElVolumesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElSecretElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
}

impl DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    #[doc = "Set the field `connector`.\n"]
    pub fn set_connector(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.egress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        self.network_interfaces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateElVpcAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
        DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector` after provisioning.\n"]
    pub fn connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateElTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_access: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
}

impl DataCloudRunV2JobTemplateElTemplateEl {
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElContainersEl>>,
    ) -> Self {
        self.containers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_environment`.\n"]
    pub fn set_execution_environment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVolumesEl>>,
    ) -> Self {
        self.volumes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
    ) -> Self {
        self.vpc_access = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateElTemplateEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateElTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateElTemplateEl {}

impl BuildDataCloudRunV2JobTemplateElTemplateEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateElTemplateEl {
        DataCloudRunV2JobTemplateElTemplateEl {
            containers: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            service_account: core::default::Default::default(),
            timeout: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2JobTemplateElTemplateElRef {
        DataCloudRunV2JobTemplateElTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElContainersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_environment` after provisioning.\n"]
    pub fn execution_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_environment", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElVpcAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template: std::option::Option<terrars::ListField<DataCloudRunV2JobTemplateElTemplateEl>>,
}

impl DataCloudRunV2JobTemplateEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelism`.\n"]
    pub fn set_parallelism(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelism = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_count`.\n"]
    pub fn set_task_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.task_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `template`.\n"]
    pub fn set_template(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2JobTemplateElTemplateEl>>,
    ) -> Self {
        self.template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTemplateEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTemplateEl {}

impl BuildDataCloudRunV2JobTemplateEl {
    pub fn build(self) -> DataCloudRunV2JobTemplateEl {
        DataCloudRunV2JobTemplateEl {
            annotations: core::default::Default::default(),
            labels: core::default::Default::default(),
            parallelism: core::default::Default::default(),
            task_count: core::default::Default::default(),
            template: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2JobTemplateElRef {
        DataCloudRunV2JobTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\n"]
    pub fn parallelism(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<DataCloudRunV2JobTemplateElTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2JobTerminalConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transition_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2JobTerminalConditionEl {
    #[doc = "Set the field `execution_reason`.\n"]
    pub fn set_execution_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_transition_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision_reason`.\n"]
    pub fn set_revision_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2JobTerminalConditionEl {
    type O = terrars::BlockAssignable<DataCloudRunV2JobTerminalConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2JobTerminalConditionEl {}

impl BuildDataCloudRunV2JobTerminalConditionEl {
    pub fn build(self) -> DataCloudRunV2JobTerminalConditionEl {
        DataCloudRunV2JobTerminalConditionEl {
            execution_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            revision_reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2JobTerminalConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2JobTerminalConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2JobTerminalConditionElRef {
        DataCloudRunV2JobTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2JobTerminalConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_reason` after provisioning.\n"]
    pub fn execution_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_transition_time", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `revision_reason` after provisioning.\n"]
    pub fn revision_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
