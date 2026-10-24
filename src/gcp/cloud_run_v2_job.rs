use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudRunV2JobData {
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
    client: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_stage: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<CloudRunV2JobBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudRunV2JobTimeoutsEl>,
    dynamic: CloudRunV2JobDynamic,
}

struct CloudRunV2Job_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudRunV2JobData>,
}

#[derive(Clone)]
pub struct CloudRunV2Job(std::rc::Rc<CloudRunV2Job_>);

impl CloudRunV2Job {
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
        "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client`.\nArbitrary identifier for the API client."]
    pub fn set_client(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_version`.\nArbitrary version identifier for the API client."]
    pub fn set_client_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Job.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `launch_stage`.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn set_launch_stage(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().launch_stage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobBinaryAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().binary_authorization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.binary_authorization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template`.\n"]
    pub fn set_template(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudRunV2JobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
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
    pub fn conditions(&self) -> terrars::ListRef<CloudRunV2JobConditionsElRef> {
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
    pub fn latest_created_execution(&self) -> terrars::ListRef<CloudRunV2JobLatestCreatedExecutionElRef> {
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
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> terrars::ListRef<CloudRunV2JobTerminalConditionElRef> {
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

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<CloudRunV2JobBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<CloudRunV2JobTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2JobTimeoutsElRef {
        CloudRunV2JobTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudRunV2Job {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudRunV2Job { }

impl terrars::ToListMappable for CloudRunV2Job {
    type O = terrars::ListRef<CloudRunV2JobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudRunV2Job_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_run_v2_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudRunV2Job {
    pub tf_id: std::string::String,
    #[doc = "The location of the cloud run job"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Job."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Job {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudRunV2Job {
        let out = CloudRunV2Job(std::rc::Rc::new(CloudRunV2Job_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudRunV2JobData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                client: core::default::Default::default(),
                client_version: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                launch_stage: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudRunV2JobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudRunV2JobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new resources.\nAll system annotations in v1 now have a corresponding field in v2 Job.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
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
    pub fn conditions(&self) -> terrars::ListRef<CloudRunV2JobConditionsElRef> {
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
    pub fn latest_created_execution(&self) -> terrars::ListRef<CloudRunV2JobLatestCreatedExecutionElRef> {
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
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state"]
    pub fn terminal_condition(&self) -> terrars::ListRef<CloudRunV2JobTerminalConditionElRef> {
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

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<CloudRunV2JobBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<CloudRunV2JobTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2JobTimeoutsElRef {
        CloudRunV2JobTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobConditionsEl {
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

impl CloudRunV2JobConditionsEl {
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

impl terrars::ToListMappable for CloudRunV2JobConditionsEl {
    type O = terrars::BlockAssignable<CloudRunV2JobConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobConditionsEl {}

impl BuildCloudRunV2JobConditionsEl {
    pub fn build(self) -> CloudRunV2JobConditionsEl {
        CloudRunV2JobConditionsEl {
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

pub struct CloudRunV2JobConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobConditionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobConditionsElRef {
        CloudRunV2JobConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobConditionsElRef {
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
pub struct CloudRunV2JobLatestCreatedExecutionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completion_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2JobLatestCreatedExecutionEl {
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

impl terrars::ToListMappable for CloudRunV2JobLatestCreatedExecutionEl {
    type O = terrars::BlockAssignable<CloudRunV2JobLatestCreatedExecutionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobLatestCreatedExecutionEl {}

impl BuildCloudRunV2JobLatestCreatedExecutionEl {
    pub fn build(self) -> CloudRunV2JobLatestCreatedExecutionEl {
        CloudRunV2JobLatestCreatedExecutionEl {
            completion_time: core::default::Default::default(),
            create_time: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobLatestCreatedExecutionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobLatestCreatedExecutionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobLatestCreatedExecutionElRef {
        CloudRunV2JobLatestCreatedExecutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobLatestCreatedExecutionElRef {
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
pub struct CloudRunV2JobTerminalConditionEl {
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

impl CloudRunV2JobTerminalConditionEl {
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

impl terrars::ToListMappable for CloudRunV2JobTerminalConditionEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTerminalConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTerminalConditionEl {}

impl BuildCloudRunV2JobTerminalConditionEl {
    pub fn build(self) -> CloudRunV2JobTerminalConditionEl {
        CloudRunV2JobTerminalConditionEl {
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

pub struct CloudRunV2JobTerminalConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTerminalConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobTerminalConditionElRef {
        CloudRunV2JobTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTerminalConditionElRef {
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
pub struct CloudRunV2JobBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    breakglass_justification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_default: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudRunV2JobBinaryAuthorizationEl {
    #[doc =
        "Set the field `breakglass_justification`.\nIf present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass"]
    pub fn set_breakglass_justification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.breakglass_justification = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_default`.\nIf True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled."]
    pub fn set_use_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_default = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobBinaryAuthorizationEl {
    type O = terrars::BlockAssignable<CloudRunV2JobBinaryAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobBinaryAuthorizationEl {}

impl BuildCloudRunV2JobBinaryAuthorizationEl {
    pub fn build(self) -> CloudRunV2JobBinaryAuthorizationEl {
        CloudRunV2JobBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobBinaryAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobBinaryAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobBinaryAuthorizationElRef {
        CloudRunV2JobBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobBinaryAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `breakglass_justification` after provisioning.\nIf present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass"]
    pub fn breakglass_justification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.breakglass_justification", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_default` after provisioning.\nIf True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled."]
    pub fn use_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    secret: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl { }

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc =
        "The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub secret: terrars::PrimField<std::string::String>,
    #[doc =
        "The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: self.secret,
            version: self.version,
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElDynamic {
    secret_key_ref: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key_ref: std
    ::option
    ::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElDynamic,
}

impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_key_ref = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_key_ref = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {}

impl BuildCloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_key_ref", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElContainersElEnvElDynamic {
    value_source: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_source: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElEnvElDynamic,
}

impl CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    #[doc =
        "Set the field `value`.\nVariable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any route environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to \"\", and the maximum length is 32768 bytes"]
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
                    ::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER, and mnay not exceed 32768 characters."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2JobTemplateElTemplateElContainersElEnvEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
        CloudRunV2JobTemplateElTemplateElContainersElEnvEl {
            name: self.name,
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
        CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElContainersElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a C_IDENTIFIER, and mnay not exceed 32768 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nVariable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any route environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to \"\", and the maximum length is 32768 bytes"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElContainersElEnvElValueSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    #[doc =
        "Set the field `container_port`.\nPort number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536."]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" and \"h2c\"."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElPortsEl {}

impl BuildCloudRunV2JobTemplateElTemplateElContainersElPortsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
        CloudRunV2JobTemplateElTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
        CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElContainersElPortsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_port` after provisioning.\nPort number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536."]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" and \"h2c\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    #[doc =
        "Set the field `limits`.\nOnly memory and CPU are supported. Use key 'cpu' for CPU limit and 'memory' for memory limit. Note: The only supported values for CPU are '1', '2', '4', and '8'. Setting 4 CPU requires at least 2Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {}

impl BuildCloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElResourcesEl {
        CloudRunV2JobTemplateElTemplateElContainersElResourcesEl { limits: core::default::Default::default() }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
        CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `limits` after provisioning.\nOnly memory and CPU are supported. Use key 'cpu' for CPU limit and 'memory' for memory limit. Note: The only supported values for CPU are '1', '2', '4', and '8'. Setting 4 CPU requires at least 2Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    mount_path: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl { }

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    #[doc =
        "Path within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub mount_path: terrars::PrimField<std::string::String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
        CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl {
            mount_path: self.mount_path,
            name: self.name,
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
        CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mount_path` after provisioning.\nPath within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThis must match the Name of a Volume."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElContainersElDynamic {
    env: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    ports: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    resources: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    volume_mounts: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElContainersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElContainersElDynamic,
}

impl CloudRunV2JobTemplateElTemplateElContainersEl {
    #[doc =
        "Set the field `args`.\nArguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nEntrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the container specified as a DNS_LABEL."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `working_dir`.\nContainer's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image."]
    pub fn set_working_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.working_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElEnvEl>>,
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

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElPortsEl>>,
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

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElResourcesEl>>,
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

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volume_mounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElContainersEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElContainersEl {
    #[doc =
        "URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2JobTemplateElTemplateElContainersEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElContainersEl {
        CloudRunV2JobTemplateElTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            image: self.image,
            name: core::default::Default::default(),
            working_dir: core::default::Default::default(),
            env: core::default::Default::default(),
            ports: core::default::Default::default(),
            resources: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElContainersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElContainersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElContainersElRef {
        CloudRunV2JobTemplateElTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElContainersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nArguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nEntrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nURL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the container specified as a DNS_LABEL."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `working_dir` after provisioning.\nContainer's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image."]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElContainersElEnvElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElContainersElPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElContainersElResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElContainersElVolumeMountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    #[doc =
        "Set the field `instances`.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn set_instances(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instances = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {}

impl BuildCloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl {
        CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl { instances: core::default::Default::default() }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instances` after provisioning.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn instances(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    path: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[doc =
        "Set the field `mode`.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    #[doc = "The relative path of the secret in the container."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc =
        "The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: self.path,
            version: self.version,
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe relative path of the secret in the container."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElDynamic {
    items: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_mode: std::option::Option<terrars::PrimField<f64>>,
    secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElVolumesElSecretElDynamic,
}

impl CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[doc =
        "Set the field `default_mode`.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
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
                    ::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    #[doc =
        "The name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            secret: self.secret,
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_mode` after provisioning.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn default_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElVolumesElSecretElItemsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElVolumesElDynamic {
    cloud_sql_instance: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    secret: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVolumesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElVolumesElDynamic,
}

impl CloudRunV2JobTemplateElTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_sql_instance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_sql_instance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesElSecretEl>>,
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
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElVolumesEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElVolumesEl {
    #[doc = "Volume's name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2JobTemplateElTemplateElVolumesEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVolumesEl {
        CloudRunV2JobTemplateElTemplateElVolumesEl {
            name: self.name,
            cloud_sql_instance: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElVolumesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobTemplateElTemplateElVolumesElRef {
        CloudRunV2JobTemplateElTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElVolumesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nVolume's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(
        &self,
    ) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElVolumesElCloudSqlInstanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElVolumesElSecretElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    #[doc =
        "Set the field `network`.\nThe VPC network that the Cloud Run resource will be able to send traffic to. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If network is not specified, it will be\nlooked up from the subnetwork."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe VPC subnetwork that the Cloud Run resource will get IPs from. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the\nsubnetwork with the same name with the network will be used."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nNetwork tags applied to this Cloud Run job."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {}

impl BuildCloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
        CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
        CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe VPC network that the Cloud Run resource will be able to send traffic to. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If network is not specified, it will be\nlooked up from the subnetwork."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe VPC subnetwork that the Cloud Run resource will get IPs from. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the\nsubnetwork with the same name with the network will be used."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nNetwork tags applied to this Cloud Run job."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElVpcAccessElDynamic {
    network_interfaces: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std
    ::option
    ::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElVpcAccessElDynamic,
}

impl CloudRunV2JobTemplateElTemplateElVpcAccessEl {
    #[doc =
        "Set the field `connector`.\nVPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number."]
    pub fn set_connector(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connector = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `egress`.\nTraffic VPC egress settings. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
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
                    ::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_interfaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interfaces = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateElVpcAccessEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateElVpcAccessEl {}

impl BuildCloudRunV2JobTemplateElTemplateElVpcAccessEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateElVpcAccessEl {
        CloudRunV2JobTemplateElTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
        CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElVpcAccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connector` after provisioning.\nVPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number."]
    pub fn connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector", self.base))
    }

    #[doc =
        "Get a reference to the value of field `egress` after provisioning.\nTraffic VPC egress settings. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
    pub fn egress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(
        &self,
    ) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElVpcAccessElNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElTemplateElDynamic {
    containers: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElContainersEl>>,
    volumes: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElVolumesEl>>,
    vpc_access: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateElTemplateEl {
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
    containers: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_access: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
    dynamic: CloudRunV2JobTemplateElTemplateElDynamic,
}

impl CloudRunV2JobTemplateElTemplateEl {
    #[doc =
        "Set the field `encryption_key`.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn set_encryption_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `execution_environment`.\nThe execution environment being used to host this Task. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn set_execution_environment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\nNumber of retries allowed per Task, before marking this Task failed."]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nEmail address of the IAM service account associated with the Task of a Job. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout`.\nMax allowed time duration the Task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElContainersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.containers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVolumesEl>>,
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

    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateElVpcAccessEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_access = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateElTemplateEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateElTemplateEl {}

impl BuildCloudRunV2JobTemplateElTemplateEl {
    pub fn build(self) -> CloudRunV2JobTemplateElTemplateEl {
        CloudRunV2JobTemplateElTemplateEl {
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            service_account: core::default::Default::default(),
            timeout: core::default::Default::default(),
            containers: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobTemplateElTemplateElRef {
        CloudRunV2JobTemplateElTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encryption_key` after provisioning.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_environment` after provisioning.\nThe execution environment being used to host this Task. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn execution_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_retries` after provisioning.\nNumber of retries allowed per Task, before marking this Task failed."]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the IAM service account associated with the Task of a Job. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nMax allowed time duration the Task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElContainersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElVpcAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2JobTemplateElDynamic {
    template: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateElTemplateEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2JobTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template: std::option::Option<std::vec::Vec<CloudRunV2JobTemplateElTemplateEl>>,
    dynamic: CloudRunV2JobTemplateElDynamic,
}

impl CloudRunV2JobTemplateEl {
    #[doc =
        "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 ExecutionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter,\nor break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or\nhttps://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 ExecutionTemplate."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parallelism`.\nSpecifies the maximum desired number of tasks the execution should run at given time. Must be <= taskCount. When the job is run, if this field is 0 or unset, the maximum possible value will be used for that execution. The actual number of tasks running in steady state will be less than this number when there are fewer tasks waiting to be completed remaining, i.e. when the work left to do is less than max parallelism."]
    pub fn set_parallelism(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelism = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `task_count`.\nSpecifies the desired number of tasks the execution should run. Setting to 1 means that parallelism is limited to 1 and the success of that task signals the success of the execution. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/"]
    pub fn set_task_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.task_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `template`.\n"]
    pub fn set_template(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2JobTemplateElTemplateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.template = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunV2JobTemplateEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTemplateEl {}

impl BuildCloudRunV2JobTemplateEl {
    pub fn build(self) -> CloudRunV2JobTemplateEl {
        CloudRunV2JobTemplateEl {
            annotations: core::default::Default::default(),
            labels: core::default::Default::default(),
            parallelism: core::default::Default::default(),
            task_count: core::default::Default::default(),
            template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobTemplateElRef {
        CloudRunV2JobTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 ExecutionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter,\nor break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or\nhttps://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 ExecutionTemplate."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parallelism` after provisioning.\nSpecifies the maximum desired number of tasks the execution should run at given time. Must be <= taskCount. When the job is run, if this field is 0 or unset, the maximum possible value will be used for that execution. The actual number of tasks running in steady state will be less than this number when there are fewer tasks waiting to be completed remaining, i.e. when the work left to do is less than max parallelism."]
    pub fn parallelism(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }

    #[doc =
        "Get a reference to the value of field `task_count` after provisioning.\nSpecifies the desired number of tasks the execution should run. Setting to 1 means that parallelism is limited to 1 and the success of that task signals the success of the execution. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/"]
    pub fn task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<CloudRunV2JobTemplateElTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2JobTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2JobTimeoutsEl {
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

impl terrars::ToListMappable for CloudRunV2JobTimeoutsEl {
    type O = terrars::BlockAssignable<CloudRunV2JobTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2JobTimeoutsEl {}

impl BuildCloudRunV2JobTimeoutsEl {
    pub fn build(self) -> CloudRunV2JobTimeoutsEl {
        CloudRunV2JobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2JobTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2JobTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2JobTimeoutsElRef {
        CloudRunV2JobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2JobTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct CloudRunV2JobDynamic {
    binary_authorization: std::option::Option<terrars::DynamicBlock<CloudRunV2JobBinaryAuthorizationEl>>,
    template: std::option::Option<terrars::DynamicBlock<CloudRunV2JobTemplateEl>>,
}
