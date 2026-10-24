use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudRunV2ServiceData {
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
    custom_audiences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_stage: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<CloudRunV2ServiceBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudRunV2ServiceTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic: std::option::Option<std::vec::Vec<CloudRunV2ServiceTrafficEl>>,
    dynamic: CloudRunV2ServiceDynamic,
}

struct CloudRunV2Service_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudRunV2ServiceData>,
}

#[derive(Clone)]
pub struct CloudRunV2Service(std::rc::Rc<CloudRunV2Service_>);

impl CloudRunV2Service {
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
        "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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

    #[doc =
        "Set the field `custom_audiences`.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn set_custom_audiences(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_audiences = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nUser-provided description of the Service. This field currently has a 512-character limit."]
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
        "Set the field `ingress`.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn set_ingress(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceBinaryAuthorizationEl>>,
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
    pub fn set_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateEl>>,
    ) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudRunV2ServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic`.\n"]
    pub fn set_traffic(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTrafficEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<CloudRunV2ServiceConditionsElRef> {
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
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> terrars::ListRef<CloudRunV2ServiceTerminalConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> terrars::ListRef<CloudRunV2ServiceTrafficStatusesElRef> {
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

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<CloudRunV2ServiceBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2ServiceTimeoutsElRef {
        CloudRunV2ServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic` after provisioning.\n"]
    pub fn traffic(&self) -> terrars::ListRef<CloudRunV2ServiceTrafficElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }
}

impl terrars::Referable for CloudRunV2Service {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudRunV2Service { }

impl terrars::ToListMappable for CloudRunV2Service {
    type O = terrars::ListRef<CloudRunV2ServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudRunV2Service_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_run_v2_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudRunV2Service {
    pub tf_id: std::string::String,
    #[doc = "The location of the cloud run service"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Service."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Service {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudRunV2Service {
        let out = CloudRunV2Service(std::rc::Rc::new(CloudRunV2Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudRunV2ServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                client: core::default::Default::default(),
                client_version: core::default::Default::default(),
                custom_audiences: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress: core::default::Default::default(),
                labels: core::default::Default::default(),
                launch_stage: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudRunV2ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudRunV2ServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<CloudRunV2ServiceConditionsElRef> {
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
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> terrars::ListRef<CloudRunV2ServiceTerminalConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> terrars::ListRef<CloudRunV2ServiceTrafficStatusesElRef> {
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

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<CloudRunV2ServiceBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2ServiceTimeoutsElRef {
        CloudRunV2ServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic` after provisioning.\n"]
    pub fn traffic(&self) -> terrars::ListRef<CloudRunV2ServiceTrafficElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceConditionsEl {
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

impl CloudRunV2ServiceConditionsEl {
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

impl terrars::ToListMappable for CloudRunV2ServiceConditionsEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceConditionsEl {}

impl BuildCloudRunV2ServiceConditionsEl {
    pub fn build(self) -> CloudRunV2ServiceConditionsEl {
        CloudRunV2ServiceConditionsEl {
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

pub struct CloudRunV2ServiceConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceConditionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceConditionsElRef {
        CloudRunV2ServiceConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceConditionsElRef {
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
pub struct CloudRunV2ServiceTerminalConditionEl {
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

impl CloudRunV2ServiceTerminalConditionEl {
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

impl terrars::ToListMappable for CloudRunV2ServiceTerminalConditionEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTerminalConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTerminalConditionEl {}

impl BuildCloudRunV2ServiceTerminalConditionEl {
    pub fn build(self) -> CloudRunV2ServiceTerminalConditionEl {
        CloudRunV2ServiceTerminalConditionEl {
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

pub struct CloudRunV2ServiceTerminalConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTerminalConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTerminalConditionElRef {
        CloudRunV2ServiceTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTerminalConditionElRef {
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
pub struct CloudRunV2ServiceTrafficStatusesEl {
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

impl CloudRunV2ServiceTrafficStatusesEl {
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

impl terrars::ToListMappable for CloudRunV2ServiceTrafficStatusesEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTrafficStatusesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTrafficStatusesEl {}

impl BuildCloudRunV2ServiceTrafficStatusesEl {
    pub fn build(self) -> CloudRunV2ServiceTrafficStatusesEl {
        CloudRunV2ServiceTrafficStatusesEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTrafficStatusesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTrafficStatusesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTrafficStatusesElRef {
        CloudRunV2ServiceTrafficStatusesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTrafficStatusesElRef {
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

#[derive(Serialize)]
pub struct CloudRunV2ServiceBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    breakglass_justification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_default: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudRunV2ServiceBinaryAuthorizationEl {
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

impl terrars::ToListMappable for CloudRunV2ServiceBinaryAuthorizationEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceBinaryAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceBinaryAuthorizationEl {}

impl BuildCloudRunV2ServiceBinaryAuthorizationEl {
    pub fn build(self) -> CloudRunV2ServiceBinaryAuthorizationEl {
        CloudRunV2ServiceBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceBinaryAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceBinaryAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceBinaryAuthorizationElRef {
        CloudRunV2ServiceBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceBinaryAuthorizationElRef {
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
pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc =
        "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc =
        "The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: self.secret,
            version: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
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
struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElDynamic {
    secret_key_ref: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key_ref: std
    ::option
    ::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElDynamic,
}

impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_key_ref", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElEnvElDynamic {
    value_source: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElEnvEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_source: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElEnvElDynamic,
}

impl CloudRunV2ServiceTemplateElContainersElEnvEl {
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
                    ::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElEnvEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElEnvEl {
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER, and mnay not exceed 32768 characters."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElContainersElEnvEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElEnvEl {
        CloudRunV2ServiceTemplateElContainersElEnvEl {
            name: self.name,
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElEnvElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElEnvElRef {
        CloudRunV2ServiceTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElEnvElRef {
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
    pub fn value_source(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[doc =
        "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `value`.\nThe header field value"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "The header field name"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
            name: self.name,
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElDynamic {
    http_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std
    ::option
    ::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElDynamic,
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
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
                            CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    http_get: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElLivenessProbeElDynamic,
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[doc =
        "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
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
                    ::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
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
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTemplateElContainersElPortsEl {
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElPortsEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElPortsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElPortsEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElPortsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElPortsEl {
        CloudRunV2ServiceTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElPortsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElPortsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElPortsElRef {
        CloudRunV2ServiceTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElPortsElRef {
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
pub struct CloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_idle: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_cpu_boost: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[doc = "Set the field `cpu_idle`.\nDetermines whether CPU should be throttled or not outside of requests."]
    pub fn set_cpu_idle(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cpu_idle = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `limits`.\nOnly memory and CPU are supported. Use key 'cpu' for CPU limit and 'memory' for memory limit. Note: The only supported values for CPU are '1', '2', '4', and '8'. Setting 4 CPU requires at least 2Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `startup_cpu_boost`.\nDetermines whether CPU should be boosted on startup of a new container instance above the requested CPU threshold, this can help reduce cold-start latency."]
    pub fn set_startup_cpu_boost(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.startup_cpu_boost = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElResourcesEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElResourcesEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElResourcesEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElResourcesEl {
        CloudRunV2ServiceTemplateElContainersElResourcesEl {
            cpu_idle: core::default::Default::default(),
            limits: core::default::Default::default(),
            startup_cpu_boost: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElResourcesElRef {
        CloudRunV2ServiceTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_idle` after provisioning.\nDetermines whether CPU should be throttled or not outside of requests."]
    pub fn cpu_idle(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_idle", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limits` after provisioning.\nOnly memory and CPU are supported. Use key 'cpu' for CPU limit and 'memory' for memory limit. Note: The only supported values for CPU are '1', '2', '4', and '8'. Setting 4 CPU requires at least 2Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startup_cpu_boost` after provisioning.\nDetermines whether CPU should be boosted on startup of a new container instance above the requested CPU threshold, this can help reduce cold-start latency."]
    pub fn startup_cpu_boost(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_cpu_boost", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[doc =
        "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `value`.\nThe header field value"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "The header field name"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            name: self.name,
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElDynamic {
    http_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std
    ::option
    ::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElDynamic,
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc =
        "Set the field `port`.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl { port: core::default::Default::default() }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElStartupProbeElDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    http_get: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    tcp_socket: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_socket: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElStartupProbeElDynamic,
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[doc =
        "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
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
                    ::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
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
                        terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeEl {}

impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    mount_path: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl CloudRunV2ServiceTemplateElContainersElVolumeMountsEl { }

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    #[doc =
        "Path within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub mount_path: terrars::PrimField<std::string::String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
        CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
            mount_path: self.mount_path,
            name: self.name,
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
        CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
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
struct CloudRunV2ServiceTemplateElContainersElDynamic {
    env: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElEnvEl>>,
    liveness_probe: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    ports: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElPortsEl>>,
    resources: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    startup_probe: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    volume_mounts: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    depends_on: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_probe: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_probe: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElDynamic,
}

impl CloudRunV2ServiceTemplateElContainersEl {
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

    #[doc =
        "Set the field `depends_on`.\nContainers which should be started before this container. If specified the container will wait to start until all containers with the listed names are healthy."]
    pub fn set_depends_on(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.depends_on = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvEl>>,
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

    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElPortsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElResourcesEl>>,
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

    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
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

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElContainersEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElContainersEl {
    #[doc =
        "URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElContainersEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersEl {
        CloudRunV2ServiceTemplateElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            image: self.image,
            name: core::default::Default::default(),
            working_dir: core::default::Default::default(),
            env: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            ports: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElContainersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElContainersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTemplateElContainersElRef {
        CloudRunV2ServiceTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElContainersElRef {
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
        "Get a reference to the value of field `depends_on` after provisioning.\nContainers which should be started before this container. If specified the container will wait to start until all containers with the listed names are healthy."]
    pub fn depends_on(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
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
    pub fn env(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElEnvElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.startup_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElScalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instance_count: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudRunV2ServiceTemplateElScalingEl {
    #[doc = "Set the field `max_instance_count`.\nMaximum number of serving instances that this resource should have."]
    pub fn set_max_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_instance_count`.\nMinimum number of serving instances that this resource should have."]
    pub fn set_min_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instance_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElScalingEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElScalingEl {}

impl BuildCloudRunV2ServiceTemplateElScalingEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElScalingEl {
        CloudRunV2ServiceTemplateElScalingEl {
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElScalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTemplateElScalingElRef {
        CloudRunV2ServiceTemplateElScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElScalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_instance_count` after provisioning.\nMaximum number of serving instances that this resource should have."]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instance_count` after provisioning.\nMinimum number of serving instances that this resource should have."]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[doc =
        "Set the field `instances`.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn set_instances(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instances = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {}

impl BuildCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
        CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl { instances: core::default::Default::default() }
    }
}

pub struct CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
        CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instances` after provisioning.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn instances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[doc =
        "Set the field `mode`.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[doc = "The relative path of the secret in the container."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
        CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: self.path,
            version: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
        CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
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
struct CloudRunV2ServiceTemplateElVolumesElSecretElDynamic {
    items: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_mode: std::option::Option<terrars::PrimField<f64>>,
    secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
    dynamic: CloudRunV2ServiceTemplateElVolumesElSecretElDynamic,
}

impl CloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[doc =
        "Set the field `default_mode`.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn set_default_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElVolumesElSecretEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[doc =
        "The name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElVolumesElSecretEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElSecretEl {
        CloudRunV2ServiceTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            secret: self.secret,
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElVolumesElSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElVolumesElSecretElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElVolumesElSecretElRef {
        CloudRunV2ServiceTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElVolumesElSecretElRef {
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
    pub fn items(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElVolumesElDynamic {
    cloud_sql_instance: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    secret: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElVolumesElSecretEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElVolumesElSecretEl>>,
    dynamic: CloudRunV2ServiceTemplateElVolumesElDynamic,
}

impl CloudRunV2ServiceTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretEl>>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElVolumesEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElVolumesEl {
    #[doc = "Volume's name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2ServiceTemplateElVolumesEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesEl {
        CloudRunV2ServiceTemplateElVolumesEl {
            name: self.name,
            cloud_sql_instance: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElVolumesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTemplateElVolumesElRef {
        CloudRunV2ServiceTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElVolumesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nVolume's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElVolumesElSecretElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
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

    #[doc = "Set the field `tags`.\nNetwork tags applied to this Cloud Run service."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {}

impl BuildCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
        CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
        CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
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
        "Get a reference to the value of field `tags` after provisioning.\nNetwork tags applied to this Cloud Run service."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElVpcAccessElDynamic {
    network_interfaces: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
    dynamic: CloudRunV2ServiceTemplateElVpcAccessElDynamic,
}

impl CloudRunV2ServiceTemplateElVpcAccessEl {
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
                    ::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateElVpcAccessEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateElVpcAccessEl {}

impl BuildCloudRunV2ServiceTemplateElVpcAccessEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVpcAccessEl {
        CloudRunV2ServiceTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElVpcAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElVpcAccessElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTemplateElVpcAccessElRef {
        CloudRunV2ServiceTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElVpcAccessElRef {
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
    pub fn network_interfaces(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElDynamic {
    containers: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElContainersEl>>,
    scaling: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElScalingEl>>,
    volumes: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElVolumesEl>>,
    vpc_access: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateElVpcAccessEl>>,
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
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
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElScalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_access: std::option::Option<std::vec::Vec<CloudRunV2ServiceTemplateElVpcAccessEl>>,
    dynamic: CloudRunV2ServiceTemplateElDynamic,
}

impl CloudRunV2ServiceTemplateEl {
    #[doc =
        "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 RevisionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encryption_key`.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn set_encryption_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `execution_environment`.\nThe sandbox environment to host this Revision. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn set_execution_environment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_environment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.\nFor more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 RevisionTemplate."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_instance_request_concurrency`.\nSets the maximum number of requests that each serving instance can receive."]
    pub fn set_max_instance_request_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `revision`.\nThe unique name for the revision. If this field is omitted, it will be automatically generated based on the Service name."]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nEmail address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_affinity`.\nEnables session affinity. For more information, go to https://cloud.google.com/run/docs/configuring/session-affinity"]
    pub fn set_session_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.session_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout`.\nMax allowed time for an instance to respond to a request.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElContainersEl>>,
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

    #[doc = "Set the field `scaling`.\n"]
    pub fn set_scaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElScalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElVolumesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessEl>>,
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

impl terrars::ToListMappable for CloudRunV2ServiceTemplateEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTemplateEl {}

impl BuildCloudRunV2ServiceTemplateEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateEl {
        CloudRunV2ServiceTemplateEl {
            annotations: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            labels: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            revision: core::default::Default::default(),
            service_account: core::default::Default::default(),
            session_affinity: core::default::Default::default(),
            timeout: core::default::Default::default(),
            containers: core::default::Default::default(),
            scaling: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTemplateElRef {
        CloudRunV2ServiceTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 RevisionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key` after provisioning.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_environment` after provisioning.\nThe sandbox environment to host this Revision. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn execution_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.\nFor more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 RevisionTemplate."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\nSets the maximum number of requests that each serving instance can receive."]
    pub fn max_instance_request_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_request_concurrency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nThe unique name for the revision. If this field is omitted, it will be automatically generated based on the Service name."]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nEnables session affinity. For more information, go to https://cloud.google.com/run/docs/configuring/session-affinity"]
    pub fn session_affinity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nMax allowed time for an instance to respond to a request.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElContainersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> terrars::ListRef<CloudRunV2ServiceTemplateElVpcAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2ServiceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTimeoutsEl {
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

impl terrars::ToListMappable for CloudRunV2ServiceTimeoutsEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTimeoutsEl {}

impl BuildCloudRunV2ServiceTimeoutsEl {
    pub fn build(self) -> CloudRunV2ServiceTimeoutsEl {
        CloudRunV2ServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTimeoutsElRef {
        CloudRunV2ServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTimeoutsElRef {
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

#[derive(Serialize)]
pub struct CloudRunV2ServiceTrafficEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2ServiceTrafficEl {
    #[doc =
        "Set the field `percent`.\nSpecifies percent of the traffic to this Revision. This defaults to zero if unspecified."]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `revision`.\nRevision to which to send this portion of traffic, if traffic allocation is by revision."]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nIndicates a string to be part of the URI to exclusively reference this target."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe allocation type for this traffic target. Possible values: [\"TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST\", \"TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2ServiceTrafficEl {
    type O = terrars::BlockAssignable<CloudRunV2ServiceTrafficEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2ServiceTrafficEl {}

impl BuildCloudRunV2ServiceTrafficEl {
    pub fn build(self) -> CloudRunV2ServiceTrafficEl {
        CloudRunV2ServiceTrafficEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct CloudRunV2ServiceTrafficElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2ServiceTrafficElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2ServiceTrafficElRef {
        CloudRunV2ServiceTrafficElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2ServiceTrafficElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nSpecifies percent of the traffic to this Revision. This defaults to zero if unspecified."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nRevision to which to send this portion of traffic, if traffic allocation is by revision."]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nIndicates a string to be part of the URI to exclusively reference this target."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe allocation type for this traffic target. Possible values: [\"TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST\", \"TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2ServiceDynamic {
    binary_authorization: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceBinaryAuthorizationEl>>,
    template: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTemplateEl>>,
    traffic: std::option::Option<terrars::DynamicBlock<CloudRunV2ServiceTrafficEl>>,
}
