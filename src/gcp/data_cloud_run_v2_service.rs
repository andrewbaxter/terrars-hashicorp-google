use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudRunV2ServiceData {
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

struct DataCloudRunV2Service_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudRunV2ServiceData>,
}

#[derive(Clone)]
pub struct DataCloudRunV2Service(std::rc::Rc<DataCloudRunV2Service_>);

impl DataCloudRunV2Service {
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

    #[doc = "Set the field `location`.\nThe location of the cloud run service"]
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
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> terrars::ListRef<DataCloudRunV2ServiceBinaryAuthorizationElRef> {
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
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<DataCloudRunV2ServiceConditionsElRef> {
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

    #[doc =
        "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_audiences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the Service. This field currently has a 512-character limit."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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
        "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permamently deleted."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_created_revision` after provisioning.\nName of the last created revision. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_created_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_created_revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_ready_revision` after provisioning.\nName of the latest revision that is serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_ready_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_ready_revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run service"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Service."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Service currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observed_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Service is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template` after provisioning.\nThe template used to create revisions for this Service."]
    pub fn template(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> terrars::ListRef<DataCloudRunV2ServiceTerminalConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic` after provisioning.\nSpecifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not provided, defaults to 100% traffic to the latest Ready Revision."]
    pub fn traffic(&self) -> terrars::ListRef<DataCloudRunV2ServiceTrafficElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> terrars::ListRef<DataCloudRunV2ServiceTrafficStatusesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_statuses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe main URI in which this Service is serving traffic."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudRunV2Service {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudRunV2Service { }

impl terrars::ToListMappable for DataCloudRunV2Service {
    type O = terrars::ListRef<DataCloudRunV2ServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudRunV2Service_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_run_v2_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudRunV2Service {
    pub tf_id: std::string::String,
    #[doc = "Name of the Service."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudRunV2Service {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudRunV2Service {
        let out = DataCloudRunV2Service(std::rc::Rc::new(DataCloudRunV2Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudRunV2ServiceData {
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

pub struct DataCloudRunV2ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudRunV2ServiceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `binary_authorization` after provisioning.\nSettings for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> terrars::ListRef<DataCloudRunV2ServiceBinaryAuthorizationElRef> {
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
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<DataCloudRunV2ServiceConditionsElRef> {
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

    #[doc =
        "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_audiences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the Service. This field currently has a 512-character limit."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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
        "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permamently deleted."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_created_revision` after provisioning.\nName of the last created revision. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_created_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_created_revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_ready_revision` after provisioning.\nName of the latest revision that is serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_ready_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_ready_revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run service"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Service."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Service currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observed_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Service is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template` after provisioning.\nThe template used to create revisions for this Service."]
    pub fn template(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> terrars::ListRef<DataCloudRunV2ServiceTerminalConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic` after provisioning.\nSpecifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not provided, defaults to 100% traffic to the latest Ready Revision."]
    pub fn traffic(&self) -> terrars::ListRef<DataCloudRunV2ServiceTrafficElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> terrars::ListRef<DataCloudRunV2ServiceTrafficStatusesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_statuses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe main URI in which this Service is serving traffic."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    breakglass_justification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_default: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudRunV2ServiceBinaryAuthorizationEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceBinaryAuthorizationEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceBinaryAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceBinaryAuthorizationEl {}

impl BuildDataCloudRunV2ServiceBinaryAuthorizationEl {
    pub fn build(self) -> DataCloudRunV2ServiceBinaryAuthorizationEl {
        DataCloudRunV2ServiceBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceBinaryAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceBinaryAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceBinaryAuthorizationElRef {
        DataCloudRunV2ServiceBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceBinaryAuthorizationElRef {
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
pub struct DataCloudRunV2ServiceConditionsEl {
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

impl DataCloudRunV2ServiceConditionsEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceConditionsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceConditionsEl {}

impl BuildDataCloudRunV2ServiceConditionsEl {
    pub fn build(self) -> DataCloudRunV2ServiceConditionsEl {
        DataCloudRunV2ServiceConditionsEl {
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

pub struct DataCloudRunV2ServiceConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceConditionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceConditionsElRef {
        DataCloudRunV2ServiceConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceConditionsElRef {
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
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
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
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key_ref: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
                    >,
    ) -> Self {
        self.secret_key_ref = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
        DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_key_ref", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_source: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElEnvEl {
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
                    ::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        self.value_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElEnvEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElEnvEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElEnvEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElEnvEl {
        DataCloudRunV2ServiceTemplateElContainersElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
        DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElEnvElRef {
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
    pub fn value_source(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
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
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    type O =
        terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
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
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>,
                    >,
    ) -> Self {
        self.http_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
            http_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    ) -> Self {
        self.http_get = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_delay_seconds`.\n"]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_seconds`.\n"]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
            failure_threshold: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
        DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\n"]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\n"]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElPortsEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElPortsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElPortsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElPortsEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElPortsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElPortsEl {
        DataCloudRunV2ServiceTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
        DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElPortsElRef {
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
pub struct DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_idle: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_cpu_boost: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[doc = "Set the field `cpu_idle`.\n"]
    pub fn set_cpu_idle(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cpu_idle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startup_cpu_boost`.\n"]
    pub fn set_startup_cpu_boost(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.startup_cpu_boost = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElResourcesEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElResourcesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
        DataCloudRunV2ServiceTemplateElContainersElResourcesEl {
            cpu_idle: core::default::Default::default(),
            limits: core::default::Default::default(),
            startup_cpu_boost: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
        DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_idle` after provisioning.\n"]
    pub fn cpu_idle(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_idle", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_cpu_boost` after provisioning.\n"]
    pub fn startup_cpu_boost(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_cpu_boost", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
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
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    type O =
        terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
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
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>,
                    >,
    ) -> Self {
        self.http_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
            http_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_socket: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    ) -> Self {
        self.grpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    ) -> Self {
        self.http_get = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_delay_seconds`.\n"]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_seconds`.\n"]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
    ) -> Self {
        self.tcp_socket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
            failure_threshold: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
        DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\n"]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\n"]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
        DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
        DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
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
pub struct DataCloudRunV2ServiceTemplateElContainersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    depends_on: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_probe: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_probe: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElContainersEl {
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

    #[doc = "Set the field `depends_on`.\n"]
    pub fn set_depends_on(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.depends_on = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElEnvEl>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    ) -> Self {
        self.liveness_probe = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElPortsEl>>,
    ) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    ) -> Self {
        self.startup_probe = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElContainersEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElContainersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElContainersEl {}

impl BuildDataCloudRunV2ServiceTemplateElContainersEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElContainersEl {
        DataCloudRunV2ServiceTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            name: core::default::Default::default(),
            ports: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            working_dir: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElContainersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElContainersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElContainersElRef {
        DataCloudRunV2ServiceTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElContainersElRef {
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

    #[doc = "Get a reference to the value of field `depends_on` after provisioning.\n"]
    pub fn depends_on(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElEnvElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElLivenessProbeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElStartupProbeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.startup_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElVolumeMountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }

    #[doc = "Get a reference to the value of field `working_dir` after provisioning.\n"]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElScalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instance_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudRunV2ServiceTemplateElScalingEl {
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instance_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElScalingEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElScalingEl {}

impl BuildDataCloudRunV2ServiceTemplateElScalingEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElScalingEl {
        DataCloudRunV2ServiceTemplateElScalingEl {
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElScalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceTemplateElScalingElRef {
        DataCloudRunV2ServiceTemplateElScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElScalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instances = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {}

impl BuildDataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
        DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl { instances: core::default::Default::default() }
    }
}

pub struct DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
        DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {}

impl BuildDataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
        DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
        DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
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
pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\n"]
    pub fn set_default_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesElSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElVolumesElSecretEl {}

impl BuildDataCloudRunV2ServiceTemplateElVolumesElSecretEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
        DataCloudRunV2ServiceTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            items: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
        DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_mode` after provisioning.\n"]
    pub fn default_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVolumesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretEl>>,
}

impl DataCloudRunV2ServiceTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesElSecretEl>>,
    ) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElVolumesEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElVolumesEl {}

impl BuildDataCloudRunV2ServiceTemplateElVolumesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVolumesEl {
        DataCloudRunV2ServiceTemplateElVolumesEl {
            cloud_sql_instance: core::default::Default::default(),
            name: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElVolumesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceTemplateElVolumesElRef {
        DataCloudRunV2ServiceTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElVolumesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElVolumesElSecretElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {}

impl BuildDataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
        DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
        DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
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
pub struct DataCloudRunV2ServiceTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std
    ::option
    ::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
}

impl DataCloudRunV2ServiceTemplateElVpcAccessEl {
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
                    ::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        self.network_interfaces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateElVpcAccessEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateElVpcAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateElVpcAccessEl {}

impl BuildDataCloudRunV2ServiceTemplateElVpcAccessEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateElVpcAccessEl {
        DataCloudRunV2ServiceTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElVpcAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElVpcAccessElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceTemplateElVpcAccessElRef {
        DataCloudRunV2ServiceTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElVpcAccessElRef {
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
    ) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_request_concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElScalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_access: std::option::Option<terrars::ListField<DataCloudRunV2ServiceTemplateElVpcAccessEl>>,
}

impl DataCloudRunV2ServiceTemplateEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElContainersEl>>,
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

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_instance_request_concurrency`.\n"]
    pub fn set_max_instance_request_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling`.\n"]
    pub fn set_scaling(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElScalingEl>>,
    ) -> Self {
        self.scaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_affinity`.\n"]
    pub fn set_session_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.session_affinity = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElVolumesEl>>,
    ) -> Self {
        self.volumes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudRunV2ServiceTemplateElVpcAccessEl>>,
    ) -> Self {
        self.vpc_access = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTemplateEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTemplateEl {}

impl BuildDataCloudRunV2ServiceTemplateEl {
    pub fn build(self) -> DataCloudRunV2ServiceTemplateEl {
        DataCloudRunV2ServiceTemplateEl {
            annotations: core::default::Default::default(),
            containers: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            labels: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            revision: core::default::Default::default(),
            scaling: core::default::Default::default(),
            service_account: core::default::Default::default(),
            session_affinity: core::default::Default::default(),
            timeout: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceTemplateElRef {
        DataCloudRunV2ServiceTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElContainersElRef> {
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

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\n"]
    pub fn max_instance_request_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_request_concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\n"]
    pub fn session_affinity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> terrars::ListRef<DataCloudRunV2ServiceTemplateElVpcAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTerminalConditionEl {
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

impl DataCloudRunV2ServiceTerminalConditionEl {
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

impl terrars::ToListMappable for DataCloudRunV2ServiceTerminalConditionEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTerminalConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTerminalConditionEl {}

impl BuildDataCloudRunV2ServiceTerminalConditionEl {
    pub fn build(self) -> DataCloudRunV2ServiceTerminalConditionEl {
        DataCloudRunV2ServiceTerminalConditionEl {
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

pub struct DataCloudRunV2ServiceTerminalConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTerminalConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceTerminalConditionElRef {
        DataCloudRunV2ServiceTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTerminalConditionElRef {
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
pub struct DataCloudRunV2ServiceTrafficEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTrafficEl {
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTrafficEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTrafficEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTrafficEl {}

impl BuildDataCloudRunV2ServiceTrafficEl {
    pub fn build(self) -> DataCloudRunV2ServiceTrafficEl {
        DataCloudRunV2ServiceTrafficEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTrafficElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTrafficElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceTrafficElRef {
        DataCloudRunV2ServiceTrafficElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTrafficElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudRunV2ServiceTrafficStatusesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudRunV2ServiceTrafficStatusesEl {
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudRunV2ServiceTrafficStatusesEl {
    type O = terrars::BlockAssignable<DataCloudRunV2ServiceTrafficStatusesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudRunV2ServiceTrafficStatusesEl {}

impl BuildDataCloudRunV2ServiceTrafficStatusesEl {
    pub fn build(self) -> DataCloudRunV2ServiceTrafficStatusesEl {
        DataCloudRunV2ServiceTrafficStatusesEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

pub struct DataCloudRunV2ServiceTrafficStatusesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudRunV2ServiceTrafficStatusesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudRunV2ServiceTrafficStatusesElRef {
        DataCloudRunV2ServiceTrafficStatusesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudRunV2ServiceTrafficStatusesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
