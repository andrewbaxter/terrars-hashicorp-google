#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudRunV2InstanceData_ {
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
    default_uri_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_revocation_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_shutdown_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoker_iam_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_stage: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudRunV2Instance_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<CloudRunV2Instance_Volumes>>,
    dynamic: CloudRunV2InstanceDynamic,
}

struct CloudRunV2Instance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudRunV2InstanceData_>,
}

#[derive(Clone)]
pub struct CloudRunV2Instance(std::rc::Rc<CloudRunV2Instance_>);

impl CloudRunV2Instance {
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
        "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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

    #[doc = "Set the field `default_uri_disabled`.\nDisables public resolution of the default URI of this Instance."]
    pub fn set_default_uri_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_uri_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encryption_key`.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn set_encryption_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encryption_key_revocation_action`.\nThe action to take if the encryption key is revoked. Possible values: [\"PREVENT_NEW\", \"SHUTDOWN\"]"]
    pub fn set_encryption_key_revocation_action(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_key_revocation_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encryption_key_shutdown_duration`.\nIf encryptionKeyRevocationAction is SHUTDOWN, the duration before shutting down all instances. The minimum increment is 1 hour.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_encryption_key_shutdown_duration(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_key_shutdown_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ingress`.\nProvides the ingress settings for this Instance. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn set_ingress(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invoker_iam_disabled`.\nDisables IAM permission check for run.routes.invoke for callers of this Instance. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn set_invoker_iam_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().invoker_iam_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `launch_stage`.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn set_launch_stage(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().launch_stage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restart_policy`.\nRestart policy for the Instance. If not provided, the default is ON_FAILURE. Possible values: [\"ALWAYS\", \"ON_FAILURE\", \"NEVER\"]"]
    pub fn set_restart_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().restart_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nEmail address of the IAM service account associated with the Instance. The service account represents the identity of the running Instance, and determines what permissions the instance has."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().containers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.containers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudRunV2Instance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Volumes>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.volumes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Instance does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<CloudRunV2Instance_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `container_statuses` after provisioning.\nStatus information for each of the specified containers. The status includes the resolved digest for specified images."]
    pub fn container_statuses(&self) -> terrars::ListRef<CloudRunV2Instance_ContainerStatusesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_statuses", self.extract_ref()))
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
        "Get a reference to the value of field `default_uri_disabled` after provisioning.\nDisables public resolution of the default URI of this Instance."]
    pub fn default_uri_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_uri_disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
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
        "Get a reference to the value of field `encryption_key` after provisioning.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_revocation_action` after provisioning.\nThe action to take if the encryption key is revoked. Possible values: [\"PREVENT_NEW\", \"SHUTDOWN\"]"]
    pub fn encryption_key_revocation_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_revocation_action", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_shutdown_duration` after provisioning.\nIf encryptionKeyRevocationAction is SHUTDOWN, the duration before shutting down all instances. The minimum increment is 1 hour.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn encryption_key_shutdown_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_shutdown_duration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
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
        "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Instance. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoker_iam_disabled` after provisioning.\nDisables IAM permission check for run.routes.invoke for callers of this Instance. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn invoker_iam_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoker_iam_disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run instance"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_uri` after provisioning.\nThe Google Console URI to obtain logs for the Instance."]
    pub fn log_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Instance currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observed_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Instance is currently being acted upon by the system to bring it into the desired state.\nWhen a new Instance is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Instance to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration will have a transient value that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Instance, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restart_policy` after provisioning.\nRestart policy for the Instance. If not provided, the default is ON_FAILURE. Possible values: [\"ALWAYS\", \"ON_FAILURE\", \"NEVER\"]"]
    pub fn restart_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use. True if this service satisfies zone separation."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the IAM service account associated with the Instance. The service account represents the identity of the running Instance, and determines what permissions the instance has."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Instance, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> terrars::ListRef<CloudRunV2Instance_TerminalConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the instance. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nAll URLs serving traffic for this Instance."]
    pub fn urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<CloudRunV2Instance_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2Instance_TimeoutsRef {
        CloudRunV2Instance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<CloudRunV2Instance_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.extract_ref()))
    }
}

impl terrars::Referable for CloudRunV2Instance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudRunV2Instance { }

impl terrars::ToListMappable for CloudRunV2Instance {
    type O = terrars::ListRef<CloudRunV2InstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudRunV2Instance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_run_v2_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudRunV2Instance {
    pub tf_id: std::string::String,
    #[doc = "The location of the cloud run instance"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Instance."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudRunV2Instance {
        let out = CloudRunV2Instance(std::rc::Rc::new(CloudRunV2Instance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudRunV2InstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                client: core::default::Default::default(),
                client_version: core::default::Default::default(),
                default_uri_disabled: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                encryption_key: core::default::Default::default(),
                encryption_key_revocation_action: core::default::Default::default(),
                encryption_key_shutdown_duration: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress: core::default::Default::default(),
                invoker_iam_disabled: core::default::Default::default(),
                labels: core::default::Default::default(),
                launch_stage: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                restart_policy: core::default::Default::default(),
                service_account: core::default::Default::default(),
                containers: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                volumes: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudRunV2InstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2InstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudRunV2InstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
        "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Instance does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> terrars::ListRef<CloudRunV2Instance_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `container_statuses` after provisioning.\nStatus information for each of the specified containers. The status includes the resolved digest for specified images."]
    pub fn container_statuses(&self) -> terrars::ListRef<CloudRunV2Instance_ContainerStatusesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_statuses", self.extract_ref()))
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
        "Get a reference to the value of field `default_uri_disabled` after provisioning.\nDisables public resolution of the default URI of this Instance."]
    pub fn default_uri_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_uri_disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
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
        "Get a reference to the value of field `encryption_key` after provisioning.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_revocation_action` after provisioning.\nThe action to take if the encryption key is revoked. Possible values: [\"PREVENT_NEW\", \"SHUTDOWN\"]"]
    pub fn encryption_key_revocation_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_revocation_action", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_shutdown_duration` after provisioning.\nIf encryptionKeyRevocationAction is SHUTDOWN, the duration before shutting down all instances. The minimum increment is 1 hour.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn encryption_key_shutdown_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key_shutdown_duration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
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
        "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Instance. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoker_iam_disabled` after provisioning.\nDisables IAM permission check for run.routes.invoke for callers of this Instance. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn invoker_iam_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoker_iam_disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run instance"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_uri` after provisioning.\nThe Google Console URI to obtain logs for the Instance."]
    pub fn log_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Instance currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observed_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Instance is currently being acted upon by the system to bring it into the desired state.\nWhen a new Instance is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Instance to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration will have a transient value that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Instance, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restart_policy` after provisioning.\nRestart policy for the Instance. If not provided, the default is ON_FAILURE. Possible values: [\"ALWAYS\", \"ON_FAILURE\", \"NEVER\"]"]
    pub fn restart_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use. True if this service satisfies zone separation."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the IAM service account associated with the Instance. The service account represents the identity of the running Instance, and determines what permissions the instance has."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Instance, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> terrars::ListRef<CloudRunV2Instance_TerminalConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminal_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the instance. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nAll URLs serving traffic for this Instance."]
    pub fn urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<CloudRunV2Instance_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2Instance_TimeoutsRef {
        CloudRunV2Instance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<CloudRunV2Instance_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2InstanceDynamic {
    containers: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers>>,
    volumes: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Volumes>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transition_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Conditions {
    #[doc = "Set the field `instance_reason`.\n"]
    pub fn set_instance_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_reason = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for CloudRunV2Instance_Conditions {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Conditions {}

impl BuildCloudRunV2Instance_Conditions {
    pub fn build(self) -> CloudRunV2Instance_Conditions {
        CloudRunV2Instance_Conditions {
            instance_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_ConditionsRef {
        CloudRunV2Instance_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_reason` after provisioning.\n"]
    pub fn instance_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_reason", self.base))
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
pub struct CloudRunV2Instance_ContainerStatuses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_ContainerStatuses {
    #[doc = "Set the field `image_digest`.\n"]
    pub fn set_image_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_ContainerStatuses {
    type O = terrars::BlockAssignable<CloudRunV2Instance_ContainerStatuses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_ContainerStatuses {}

impl BuildCloudRunV2Instance_ContainerStatuses {
    pub fn build(self) -> CloudRunV2Instance_ContainerStatuses {
        CloudRunV2Instance_ContainerStatuses {
            image_digest: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_ContainerStatusesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_ContainerStatusesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_ContainerStatusesRef {
        CloudRunV2Instance_ContainerStatusesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_ContainerStatusesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\n"]
    pub fn image_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_TerminalCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transition_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_TerminalCondition {
    #[doc = "Set the field `instance_reason`.\n"]
    pub fn set_instance_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_reason = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for CloudRunV2Instance_TerminalCondition {
    type O = terrars::BlockAssignable<CloudRunV2Instance_TerminalCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_TerminalCondition {}

impl BuildCloudRunV2Instance_TerminalCondition {
    pub fn build(self) -> CloudRunV2Instance_TerminalCondition {
        CloudRunV2Instance_TerminalCondition {
            instance_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_TerminalConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_TerminalConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_TerminalConditionRef {
        CloudRunV2Instance_TerminalConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_TerminalConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_reason` after provisioning.\n"]
    pub fn instance_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_reason", self.base))
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
pub struct CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef {
    secret: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef { }

impl terrars::ToListMappable for CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef {
    #[doc =
        "The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub secret: terrars::PrimField<std::string::String>,
    #[doc =
        "The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef {
    pub fn build(self) -> CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef {
        CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef {
            secret: self.secret,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRefRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRefRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRefRef {
        CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRefRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRefRef {
    #[allow(dead_code)]
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
struct CloudRunV2Instance_Containers_Env_ValueSourceDynamic {
    secret_key_ref: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_Env_ValueSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key_ref: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef>>,
    dynamic: CloudRunV2Instance_Containers_Env_ValueSourceDynamic,
}

impl CloudRunV2Instance_Containers_Env_ValueSource {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRef>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Containers_Env_ValueSource {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_Env_ValueSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_Env_ValueSource {}

impl BuildCloudRunV2Instance_Containers_Env_ValueSource {
    pub fn build(self) -> CloudRunV2Instance_Containers_Env_ValueSource {
        CloudRunV2Instance_Containers_Env_ValueSource {
            secret_key_ref: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_Env_ValueSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_Env_ValueSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Containers_Env_ValueSourceRef {
        CloudRunV2Instance_Containers_Env_ValueSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_Env_ValueSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_Env_ValueSource_SecretKeyRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_key_ref", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2Instance_Containers_EnvDynamic {
    value_source: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_Env_ValueSource>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_Env {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_source: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_Env_ValueSource>>,
    dynamic: CloudRunV2Instance_Containers_EnvDynamic,
}

impl CloudRunV2Instance_Containers_Env {
    #[doc =
        "Set the field `value`.\nLiteral value of the environment variable. Defaults to \"\" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_source`.\n"]
    pub fn set_value_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_Env_ValueSource>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Containers_Env {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_Env {
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER, and may not exceed 32768 characters."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Containers_Env {
    pub fn build(self) -> CloudRunV2Instance_Containers_Env {
        CloudRunV2Instance_Containers_Env {
            name: self.name,
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Containers_EnvRef {
        CloudRunV2Instance_Containers_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_EnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a C_IDENTIFIER, and may not exceed 32768 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nLiteral value of the environment variable. Defaults to \"\" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_Env_ValueSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_StartupProbe_Grpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Containers_StartupProbe_Grpc {
    #[doc =
        "Set the field `port`.\nOptional. Port number of the gRPC service. Number must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nOptional. Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md ). If this is not specified, the default behavior is defined by gRPC"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Containers_StartupProbe_Grpc {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_Grpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_StartupProbe_Grpc {}

impl BuildCloudRunV2Instance_Containers_StartupProbe_Grpc {
    pub fn build(self) -> CloudRunV2Instance_Containers_StartupProbe_Grpc {
        CloudRunV2Instance_Containers_StartupProbe_Grpc {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_StartupProbe_GrpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_StartupProbe_GrpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Containers_StartupProbe_GrpcRef {
        CloudRunV2Instance_Containers_StartupProbe_GrpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_StartupProbe_GrpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nOptional. Port number of the gRPC service. Number must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nOptional. Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md ). If this is not specified, the default behavior is defined by gRPC"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders {
    #[doc = "Set the field `name`.\nRequired. The header field name"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nOptional. The header field value"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders {}

impl BuildCloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders {
    pub fn build(self) -> CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders {
        CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeadersRef {
        CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The header field name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nOptional. The header field value"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2Instance_Containers_StartupProbe_HttpGetDynamic {
    http_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_StartupProbe_HttpGet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders>>,
    dynamic: CloudRunV2Instance_Containers_StartupProbe_HttpGetDynamic,
}

impl CloudRunV2Instance_Containers_StartupProbe_HttpGet {
    #[doc = "Set the field `path`.\nOptional. Path to access on the HTTP server. Defaults to '/'."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
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
                    ::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeaders>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Containers_StartupProbe_HttpGet {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_HttpGet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_StartupProbe_HttpGet {}

impl BuildCloudRunV2Instance_Containers_StartupProbe_HttpGet {
    pub fn build(self) -> CloudRunV2Instance_Containers_StartupProbe_HttpGet {
        CloudRunV2Instance_Containers_StartupProbe_HttpGet {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_StartupProbe_HttpGetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_StartupProbe_HttpGetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Containers_StartupProbe_HttpGetRef {
        CloudRunV2Instance_Containers_StartupProbe_HttpGetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_StartupProbe_HttpGetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nOptional. Path to access on the HTTP server. Defaults to '/'."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_StartupProbe_HttpGet_HttpHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2Instance_Containers_LivenessProbeDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_StartupProbe_Grpc>>,
    http_get: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_StartupProbe_HttpGet>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_LivenessProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_StartupProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_StartupProbe_HttpGet>>,
    dynamic: CloudRunV2Instance_Containers_LivenessProbeDynamic,
}

impl CloudRunV2Instance_Containers_LivenessProbe {
    #[doc =
        "Set the field `failure_threshold`.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_Grpc>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_HttpGet>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Containers_LivenessProbe {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_LivenessProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_LivenessProbe {}

impl BuildCloudRunV2Instance_Containers_LivenessProbe {
    pub fn build(self) -> CloudRunV2Instance_Containers_LivenessProbe {
        CloudRunV2Instance_Containers_LivenessProbe {
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

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_LivenessProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_LivenessProbeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Containers_LivenessProbeRef {
        CloudRunV2Instance_Containers_LivenessProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_LivenessProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_StartupProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_StartupProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_Ports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Containers_Ports {
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

impl terrars::ToListMappable for CloudRunV2Instance_Containers_Ports {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_Ports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_Ports {}

impl BuildCloudRunV2Instance_Containers_Ports {
    pub fn build(self) -> CloudRunV2Instance_Containers_Ports {
        CloudRunV2Instance_Containers_Ports {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_PortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_PortsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Containers_PortsRef {
        CloudRunV2Instance_Containers_PortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_PortsRef {
    #[allow(dead_code)]
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
pub struct CloudRunV2Instance_Containers_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunV2Instance_Containers_Resources {
    #[doc =
        "Set the field `limits`.\nOnly memory and CPU are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit. Note: The only supported values for CPU are '1', '2', '4', '6', and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Containers_Resources {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_Resources {}

impl BuildCloudRunV2Instance_Containers_Resources {
    pub fn build(self) -> CloudRunV2Instance_Containers_Resources {
        CloudRunV2Instance_Containers_Resources { limits: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Containers_ResourcesRef {
        CloudRunV2Instance_Containers_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `limits` after provisioning.\nOnly memory and CPU are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit. Note: The only supported values for CPU are '1', '2', '4', '6', and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_StartupProbe_TcpSocket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudRunV2Instance_Containers_StartupProbe_TcpSocket {
    #[doc =
        "Set the field `port`.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Containers_StartupProbe_TcpSocket {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_TcpSocket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_StartupProbe_TcpSocket {}

impl BuildCloudRunV2Instance_Containers_StartupProbe_TcpSocket {
    pub fn build(self) -> CloudRunV2Instance_Containers_StartupProbe_TcpSocket {
        CloudRunV2Instance_Containers_StartupProbe_TcpSocket { port: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_StartupProbe_TcpSocketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_StartupProbe_TcpSocketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Containers_StartupProbe_TcpSocketRef {
        CloudRunV2Instance_Containers_StartupProbe_TcpSocketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_StartupProbe_TcpSocketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nOptional. Port number to access on the container. Must be in the range 1 to 65535. If not specified, defaults to the exposed port of the container, which is the value of container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2Instance_Containers_StartupProbeDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_StartupProbe_Grpc>>,
    http_get: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_StartupProbe_HttpGet>>,
    tcp_socket: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_StartupProbe_TcpSocket>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_StartupProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_StartupProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_StartupProbe_HttpGet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_socket: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_StartupProbe_TcpSocket>>,
    dynamic: CloudRunV2Instance_Containers_StartupProbeDynamic,
}

impl CloudRunV2Instance_Containers_StartupProbe {
    #[doc =
        "Set the field `failure_threshold`.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_Grpc>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_HttpGet>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe_TcpSocket>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Containers_StartupProbe {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_StartupProbe {}

impl BuildCloudRunV2Instance_Containers_StartupProbe {
    pub fn build(self) -> CloudRunV2Instance_Containers_StartupProbe {
        CloudRunV2Instance_Containers_StartupProbe {
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

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_StartupProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_StartupProbeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Containers_StartupProbeRef {
        CloudRunV2Instance_Containers_StartupProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_StartupProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nOptional. Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nOptional. Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nOptional. How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeout_seconds."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nOptional. Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than period_seconds."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_StartupProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_StartupProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_StartupProbe_TcpSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers_VolumeMounts {
    mount_path: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Containers_VolumeMounts {
    #[doc = "Set the field `sub_path`.\nPath within the volume from which the container's volume should be mounted."]
    pub fn set_sub_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sub_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Containers_VolumeMounts {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers_VolumeMounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers_VolumeMounts {
    #[doc =
        "Path within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub mount_path: terrars::PrimField<std::string::String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Containers_VolumeMounts {
    pub fn build(self) -> CloudRunV2Instance_Containers_VolumeMounts {
        CloudRunV2Instance_Containers_VolumeMounts {
            mount_path: self.mount_path,
            name: self.name,
            sub_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Containers_VolumeMountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Containers_VolumeMountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Containers_VolumeMountsRef {
        CloudRunV2Instance_Containers_VolumeMountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Containers_VolumeMountsRef {
    #[allow(dead_code)]
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

    #[doc =
        "Get a reference to the value of field `sub_path` after provisioning.\nPath within the volume from which the container's volume should be mounted."]
    pub fn sub_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2Instance_ContainersDynamic {
    env: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_Env>>,
    liveness_probe: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_LivenessProbe>>,
    ports: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_Ports>>,
    resources: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_Resources>>,
    startup_probe: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_StartupProbe>>,
    volume_mounts: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Containers_VolumeMounts>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Containers {
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
    env: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_probe: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_LivenessProbe>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_Ports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_probe: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_StartupProbe>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std::option::Option<std::vec::Vec<CloudRunV2Instance_Containers_VolumeMounts>>,
    dynamic: CloudRunV2Instance_ContainersDynamic,
}

impl CloudRunV2Instance_Containers {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_Env>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_LivenessProbe>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_Ports>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_Resources>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_StartupProbe>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Containers_VolumeMounts>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Containers {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Containers {
    #[doc =
        "URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Containers {
    pub fn build(self) -> CloudRunV2Instance_Containers {
        CloudRunV2Instance_Containers {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct CloudRunV2Instance_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_ContainersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_ContainersRef {
        CloudRunV2Instance_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_ContainersRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_LivenessProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_StartupProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.startup_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(&self) -> terrars::ListRef<CloudRunV2Instance_Containers_VolumeMountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Timeouts {
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

impl terrars::ToListMappable for CloudRunV2Instance_Timeouts {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Timeouts {}

impl BuildCloudRunV2Instance_Timeouts {
    pub fn build(self) -> CloudRunV2Instance_Timeouts {
        CloudRunV2Instance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_TimeoutsRef {
        CloudRunV2Instance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_TimeoutsRef {
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

#[derive(Serialize)]
pub struct CloudRunV2Instance_Volumes_CloudSqlInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunV2Instance_Volumes_CloudSqlInstance {
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

impl terrars::ToListMappable for CloudRunV2Instance_Volumes_CloudSqlInstance {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Volumes_CloudSqlInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Volumes_CloudSqlInstance {}

impl BuildCloudRunV2Instance_Volumes_CloudSqlInstance {
    pub fn build(self) -> CloudRunV2Instance_Volumes_CloudSqlInstance {
        CloudRunV2Instance_Volumes_CloudSqlInstance { instances: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Volumes_CloudSqlInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Volumes_CloudSqlInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunV2Instance_Volumes_CloudSqlInstanceRef {
        CloudRunV2Instance_Volumes_CloudSqlInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Volumes_CloudSqlInstanceRef {
    #[allow(dead_code)]
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
pub struct CloudRunV2Instance_Volumes_EmptyDir {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_limit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Volumes_EmptyDir {
    #[doc =
        "Set the field `medium`.\nThe different types of medium supported for EmptyDir. Default value: \"MEMORY\" Possible values: [\"MEMORY\", \"DISK\"]"]
    pub fn set_medium(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.medium = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `size_limit`.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn set_size_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.size_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Volumes_EmptyDir {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Volumes_EmptyDir>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Volumes_EmptyDir {}

impl BuildCloudRunV2Instance_Volumes_EmptyDir {
    pub fn build(self) -> CloudRunV2Instance_Volumes_EmptyDir {
        CloudRunV2Instance_Volumes_EmptyDir {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Volumes_EmptyDirRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Volumes_EmptyDirRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Volumes_EmptyDirRef {
        CloudRunV2Instance_Volumes_EmptyDirRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Volumes_EmptyDirRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `medium` after provisioning.\nThe different types of medium supported for EmptyDir. Default value: \"MEMORY\" Possible values: [\"MEMORY\", \"DISK\"]"]
    pub fn medium(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_limit` after provisioning.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn size_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Volumes_Gcs {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudRunV2Instance_Volumes_Gcs {
    #[doc =
        "Set the field `mount_options`.\nA list of flags to pass to the gcsfuse command for configuring this volume.\nFlags should be passed without leading dashes."]
    pub fn set_mount_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.mount_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\nIf true, mount the GCS bucket as read-only"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Volumes_Gcs {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Volumes_Gcs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Volumes_Gcs {
    #[doc = "GCS Bucket name"]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Volumes_Gcs {
    pub fn build(self) -> CloudRunV2Instance_Volumes_Gcs {
        CloudRunV2Instance_Volumes_Gcs {
            bucket: self.bucket,
            mount_options: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Volumes_GcsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Volumes_GcsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Volumes_GcsRef {
        CloudRunV2Instance_Volumes_GcsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Volumes_GcsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nGCS Bucket name"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mount_options` after provisioning.\nA list of flags to pass to the gcsfuse command for configuring this volume.\nFlags should be passed without leading dashes."]
    pub fn mount_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mount_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount the GCS bucket as read-only"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Volumes_Nfs {
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    server: terrars::PrimField<std::string::String>,
}

impl CloudRunV2Instance_Volumes_Nfs {
    #[doc = "Set the field `read_only`.\nIf true, mount the NFS volume as read only"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunV2Instance_Volumes_Nfs {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Volumes_Nfs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Volumes_Nfs {
    #[doc = "Path that is exported by the NFS server."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "Hostname or IP address of the NFS server"]
    pub server: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Volumes_Nfs {
    pub fn build(self) -> CloudRunV2Instance_Volumes_Nfs {
        CloudRunV2Instance_Volumes_Nfs {
            path: self.path,
            read_only: core::default::Default::default(),
            server: self.server,
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Volumes_NfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Volumes_NfsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Volumes_NfsRef {
        CloudRunV2Instance_Volumes_NfsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Volumes_NfsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath that is exported by the NFS server."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount the NFS volume as read only"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server` after provisioning.\nHostname or IP address of the NFS server"]
    pub fn server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Volumes_Secret_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunV2Instance_Volumes_Secret_Items {
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

impl terrars::ToListMappable for CloudRunV2Instance_Volumes_Secret_Items {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Volumes_Secret_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Volumes_Secret_Items {
    #[doc = "The relative path of the secret in the container."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Volumes_Secret_Items {
    pub fn build(self) -> CloudRunV2Instance_Volumes_Secret_Items {
        CloudRunV2Instance_Volumes_Secret_Items {
            mode: core::default::Default::default(),
            path: self.path,
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Volumes_Secret_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Volumes_Secret_ItemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Volumes_Secret_ItemsRef {
        CloudRunV2Instance_Volumes_Secret_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Volumes_Secret_ItemsRef {
    #[allow(dead_code)]
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
struct CloudRunV2Instance_Volumes_SecretDynamic {
    items: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Volumes_Secret_Items>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Volumes_Secret {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_mode: std::option::Option<terrars::PrimField<f64>>,
    secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudRunV2Instance_Volumes_Secret_Items>>,
    dynamic: CloudRunV2Instance_Volumes_SecretDynamic,
}

impl CloudRunV2Instance_Volumes_Secret {
    #[doc =
        "Set the field `default_mode`.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn set_default_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Volumes_Secret_Items>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Volumes_Secret {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Volumes_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Volumes_Secret {
    #[doc =
        "The name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Volumes_Secret {
    pub fn build(self) -> CloudRunV2Instance_Volumes_Secret {
        CloudRunV2Instance_Volumes_Secret {
            default_mode: core::default::Default::default(),
            secret: self.secret,
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_Volumes_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_Volumes_SecretRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_Volumes_SecretRef {
        CloudRunV2Instance_Volumes_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_Volumes_SecretRef {
    #[allow(dead_code)]
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
    pub fn items(&self) -> terrars::ListRef<CloudRunV2Instance_Volumes_Secret_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunV2Instance_VolumesDynamic {
    cloud_sql_instance: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Volumes_CloudSqlInstance>>,
    empty_dir: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Volumes_EmptyDir>>,
    gcs: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Volumes_Gcs>>,
    nfs: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Volumes_Nfs>>,
    secret: std::option::Option<terrars::DynamicBlock<CloudRunV2Instance_Volumes_Secret>>,
}

#[derive(Serialize)]
pub struct CloudRunV2Instance_Volumes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std::option::Option<std::vec::Vec<CloudRunV2Instance_Volumes_CloudSqlInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    empty_dir: std::option::Option<std::vec::Vec<CloudRunV2Instance_Volumes_EmptyDir>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std::option::Option<std::vec::Vec<CloudRunV2Instance_Volumes_Gcs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs: std::option::Option<std::vec::Vec<CloudRunV2Instance_Volumes_Nfs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<CloudRunV2Instance_Volumes_Secret>>,
    dynamic: CloudRunV2Instance_VolumesDynamic,
}

impl CloudRunV2Instance_Volumes {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Volumes_CloudSqlInstance>>,
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

    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Volumes_EmptyDir>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.empty_dir = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.empty_dir = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Volumes_Gcs>>,
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

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Volumes_Nfs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nfs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunV2Instance_Volumes_Secret>>,
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

impl terrars::ToListMappable for CloudRunV2Instance_Volumes {
    type O = terrars::BlockAssignable<CloudRunV2Instance_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunV2Instance_Volumes {
    #[doc = "Volume's name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunV2Instance_Volumes {
    pub fn build(self) -> CloudRunV2Instance_Volumes {
        CloudRunV2Instance_Volumes {
            name: self.name,
            cloud_sql_instance: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            gcs: core::default::Default::default(),
            nfs: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunV2Instance_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunV2Instance_VolumesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunV2Instance_VolumesRef {
        CloudRunV2Instance_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunV2Instance_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nVolume's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(&self) -> terrars::ListRef<CloudRunV2Instance_Volumes_CloudSqlInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> terrars::ListRef<CloudRunV2Instance_Volumes_EmptyDirRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(&self) -> terrars::ListRef<CloudRunV2Instance_Volumes_GcsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> terrars::ListRef<CloudRunV2Instance_Volumes_NfsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<CloudRunV2Instance_Volumes_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
