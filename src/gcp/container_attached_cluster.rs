#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerAttachedClusterData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    distribution: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    platform_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<ContainerAttachedCluster_Authorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<ContainerAttachedCluster_BinaryAuthorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<ContainerAttachedCluster_Fleet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<ContainerAttachedCluster_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_config: std::option::Option<std::vec::Vec<ContainerAttachedCluster_MonitoringConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_config: std::option::Option<std::vec::Vec<ContainerAttachedCluster_OidcConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_config: std::option::Option<std::vec::Vec<ContainerAttachedCluster_ProxyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_posture_config: std::option::Option<std::vec::Vec<ContainerAttachedCluster_SecurityPostureConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerAttachedCluster_Timeouts>,
    dynamic: ContainerAttachedClusterDynamic,
}

struct ContainerAttachedCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerAttachedClusterData_>,
}

#[derive(Clone)]
pub struct ContainerAttachedCluster(std::rc::Rc<ContainerAttachedCluster_>);

impl ContainerAttachedCluster {
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
        "Set the field `annotations`.\nOptional. Annotations on the cluster. This field has the same\nrestrictions as Kubernetes annotations. The total size of all keys and\nvalues combined is limited to 256k. Key can have 2 segments: prefix (optional)\nand name (required), separated by a slash (/). Prefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/container_attached_cluster.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nA human readable description of this attached cluster. Cannot be longer\nthan 255 UTF-8 encoded bytes."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_Authorization>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_BinaryAuthorization>>,
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

    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_Fleet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_config`.\n"]
    pub fn set_monitoring_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_MonitoringConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_config`.\n"]
    pub fn set_oidc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_OidcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_ProxyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_posture_config`.\n"]
    pub fn set_security_posture_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_SecurityPostureConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_posture_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_posture_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerAttachedCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same\nrestrictions as Kubernetes annotations. The total size of all keys and\nvalues combined is limited to 256k. Key can have 2 segments: prefix (optional)\nand name (required), separated by a slash (/). Prefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_region` after provisioning.\nOutput only. The region where this cluster runs.\n\nFor EKS clusters, this is an AWS region. For AKS clusters,\nthis is an Azure region."]
    pub fn cluster_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/container_attached_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this attached cluster. Cannot be longer\nthan 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `distribution` after provisioning.\nThe Kubernetes distribution of the underlying attached cluster. Supported values:\n\"eks\", \"aks\", \"generic\". The generic distribution provides the ability to register\nor migrate any CNCF conformant cluster."]
    pub fn distribution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `errors` after provisioning.\nA set of errors found in the cluster."]
    pub fn errors(&self) -> terrars::ListRef<ContainerAttachedCluster_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kubernetes_version` after provisioning.\nThe Kubernetes version of the cluster."]
    pub fn kubernetes_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform_version` after provisioning.\nThe platform version for the cluster (e.g. '1.23.0-gke.1')."]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values:\nSTATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR,\nDEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nA globally unique identifier for the cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_config` after provisioning.\nWorkload Identity settings."]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerAttachedCluster_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<ContainerAttachedCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerAttachedCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerAttachedCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<ContainerAttachedCluster_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<ContainerAttachedCluster_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> terrars::ListRef<ContainerAttachedCluster_OidcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<ContainerAttachedCluster_ProxyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(&self) -> terrars::ListRef<ContainerAttachedCluster_SecurityPostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAttachedCluster_TimeoutsRef {
        ContainerAttachedCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerAttachedCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerAttachedCluster { }

impl terrars::ToListMappable for ContainerAttachedCluster {
    type O = terrars::ListRef<ContainerAttachedClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerAttachedCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_attached_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerAttachedCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The Kubernetes distribution of the underlying attached cluster. Supported values:\n\"eks\", \"aks\", \"generic\". The generic distribution provides the ability to register\nor migrate any CNCF conformant cluster."]
    pub distribution: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of this resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The platform version for the cluster (e.g. '1.23.0-gke.1')."]
    pub platform_version: terrars::PrimField<std::string::String>,
}

impl BuildContainerAttachedCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerAttachedCluster {
        let out = ContainerAttachedCluster(std::rc::Rc::new(ContainerAttachedCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerAttachedClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                distribution: self.distribution,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                platform_version: self.platform_version,
                project: core::default::Default::default(),
                authorization: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                fleet: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                monitoring_config: core::default::Default::default(),
                oidc_config: core::default::Default::default(),
                proxy_config: core::default::Default::default(),
                security_posture_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerAttachedClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerAttachedClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same\nrestrictions as Kubernetes annotations. The total size of all keys and\nvalues combined is limited to 256k. Key can have 2 segments: prefix (optional)\nand name (required), separated by a slash (/). Prefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_region` after provisioning.\nOutput only. The region where this cluster runs.\n\nFor EKS clusters, this is an AWS region. For AKS clusters,\nthis is an Azure region."]
    pub fn cluster_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/container_attached_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this attached cluster. Cannot be longer\nthan 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `distribution` after provisioning.\nThe Kubernetes distribution of the underlying attached cluster. Supported values:\n\"eks\", \"aks\", \"generic\". The generic distribution provides the ability to register\nor migrate any CNCF conformant cluster."]
    pub fn distribution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `errors` after provisioning.\nA set of errors found in the cluster."]
    pub fn errors(&self) -> terrars::ListRef<ContainerAttachedCluster_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kubernetes_version` after provisioning.\nThe Kubernetes version of the cluster."]
    pub fn kubernetes_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform_version` after provisioning.\nThe platform version for the cluster (e.g. '1.23.0-gke.1')."]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values:\nSTATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR,\nDEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nA globally unique identifier for the cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_config` after provisioning.\nWorkload Identity settings."]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerAttachedCluster_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<ContainerAttachedCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerAttachedCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerAttachedCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<ContainerAttachedCluster_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<ContainerAttachedCluster_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> terrars::ListRef<ContainerAttachedCluster_OidcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<ContainerAttachedCluster_ProxyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(&self) -> terrars::ListRef<ContainerAttachedCluster_SecurityPostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAttachedCluster_TimeoutsRef {
        ContainerAttachedCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ContainerAttachedClusterDynamic {
    authorization: std::option::Option<terrars::DynamicBlock<ContainerAttachedCluster_Authorization>>,
    binary_authorization: std::option::Option<terrars::DynamicBlock<ContainerAttachedCluster_BinaryAuthorization>>,
    fleet: std::option::Option<terrars::DynamicBlock<ContainerAttachedCluster_Fleet>>,
    logging_config: std::option::Option<terrars::DynamicBlock<ContainerAttachedCluster_LoggingConfig>>,
    monitoring_config: std::option::Option<terrars::DynamicBlock<ContainerAttachedCluster_MonitoringConfig>>,
    oidc_config: std::option::Option<terrars::DynamicBlock<ContainerAttachedCluster_OidcConfig>>,
    proxy_config: std::option::Option<terrars::DynamicBlock<ContainerAttachedCluster_ProxyConfig>>,
    security_posture_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAttachedCluster_SecurityPostureConfig>>,
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAttachedCluster_Errors {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_Errors {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_Errors {}

impl BuildContainerAttachedCluster_Errors {
    pub fn build(self) -> ContainerAttachedCluster_Errors {
        ContainerAttachedCluster_Errors { message: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_ErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_ErrorsRef {
        ContainerAttachedCluster_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_ErrorsRef {
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
pub struct ContainerAttachedCluster_WorkloadIdentityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAttachedCluster_WorkloadIdentityConfig {
    #[doc = "Set the field `identity_provider`.\n"]
    pub fn set_identity_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer_uri`.\n"]
    pub fn set_issuer_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workload_pool`.\n"]
    pub fn set_workload_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_WorkloadIdentityConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_WorkloadIdentityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_WorkloadIdentityConfig {}

impl BuildContainerAttachedCluster_WorkloadIdentityConfig {
    pub fn build(self) -> ContainerAttachedCluster_WorkloadIdentityConfig {
        ContainerAttachedCluster_WorkloadIdentityConfig {
            identity_provider: core::default::Default::default(),
            issuer_uri: core::default::Default::default(),
            workload_pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_WorkloadIdentityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_WorkloadIdentityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAttachedCluster_WorkloadIdentityConfigRef {
        ContainerAttachedCluster_WorkloadIdentityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_WorkloadIdentityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_uri` after provisioning.\n"]
    pub fn issuer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `workload_pool` after provisioning.\n"]
    pub fn workload_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_Authorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ContainerAttachedCluster_Authorization {
    #[doc =
        "Set the field `admin_groups`.\nGroups that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the groups. Up to ten admin groups can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn set_admin_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.admin_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `admin_users`.\nUsers that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the users. Up to ten admin users can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn set_admin_users(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.admin_users = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_Authorization {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_Authorization {}

impl BuildContainerAttachedCluster_Authorization {
    pub fn build(self) -> ContainerAttachedCluster_Authorization {
        ContainerAttachedCluster_Authorization {
            admin_groups: core::default::Default::default(),
            admin_users: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_AuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_AuthorizationRef {
        ContainerAttachedCluster_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_groups` after provisioning.\nGroups that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the groups. Up to ten admin groups can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn admin_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `admin_users` after provisioning.\nUsers that can perform operations as a cluster admin. A managed\nClusterRoleBinding will be created to grant the 'cluster-admin' ClusterRole\nto the users. Up to ten admin users can be provided.\n\nFor more info on RBAC, see\nhttps://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles"]
    pub fn admin_users(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_BinaryAuthorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAttachedCluster_BinaryAuthorization {
    #[doc =
        "Set the field `evaluation_mode`.\nConfigure Binary Authorization evaluation mode. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn set_evaluation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_BinaryAuthorization {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_BinaryAuthorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_BinaryAuthorization {}

impl BuildContainerAttachedCluster_BinaryAuthorization {
    pub fn build(self) -> ContainerAttachedCluster_BinaryAuthorization {
        ContainerAttachedCluster_BinaryAuthorization { evaluation_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_BinaryAuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_BinaryAuthorizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAttachedCluster_BinaryAuthorizationRef {
        ContainerAttachedCluster_BinaryAuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_BinaryAuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluation_mode` after provisioning.\nConfigure Binary Authorization evaluation mode. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_Fleet {
    project: terrars::PrimField<std::string::String>,
}

impl ContainerAttachedCluster_Fleet { }

impl terrars::ToListMappable for ContainerAttachedCluster_Fleet {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_Fleet {
    #[doc = "The number of the Fleet host project where this cluster will be registered."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildContainerAttachedCluster_Fleet {
    pub fn build(self) -> ContainerAttachedCluster_Fleet {
        ContainerAttachedCluster_Fleet { project: self.project }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_FleetRef {
        ContainerAttachedCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_FleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nThe name of the managed Hub Membership resource associated to this\ncluster. Membership names are formatted as\nprojects/<project-number>/locations/global/membership/<cluster-id>."]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe number of the Fleet host project where this cluster will be registered."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_LoggingConfig_ComponentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_components: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ContainerAttachedCluster_LoggingConfig_ComponentConfig {
    #[doc =
        "Set the field `enable_components`.\nThe components to be enabled. Possible values: [\"SYSTEM_COMPONENTS\", \"WORKLOADS\"]"]
    pub fn set_enable_components(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enable_components = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_LoggingConfig_ComponentConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_LoggingConfig_ComponentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_LoggingConfig_ComponentConfig {}

impl BuildContainerAttachedCluster_LoggingConfig_ComponentConfig {
    pub fn build(self) -> ContainerAttachedCluster_LoggingConfig_ComponentConfig {
        ContainerAttachedCluster_LoggingConfig_ComponentConfig {
            enable_components: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_LoggingConfig_ComponentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_LoggingConfig_ComponentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAttachedCluster_LoggingConfig_ComponentConfigRef {
        ContainerAttachedCluster_LoggingConfig_ComponentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_LoggingConfig_ComponentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_components` after provisioning.\nThe components to be enabled. Possible values: [\"SYSTEM_COMPONENTS\", \"WORKLOADS\"]"]
    pub fn enable_components(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAttachedCluster_LoggingConfigDynamic {
    component_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAttachedCluster_LoggingConfig_ComponentConfig>>,
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_config: std::option::Option<std::vec::Vec<ContainerAttachedCluster_LoggingConfig_ComponentConfig>>,
    dynamic: ContainerAttachedCluster_LoggingConfigDynamic,
}

impl ContainerAttachedCluster_LoggingConfig {
    #[doc = "Set the field `component_config`.\n"]
    pub fn set_component_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_LoggingConfig_ComponentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.component_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.component_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_LoggingConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_LoggingConfig {}

impl BuildContainerAttachedCluster_LoggingConfig {
    pub fn build(self) -> ContainerAttachedCluster_LoggingConfig {
        ContainerAttachedCluster_LoggingConfig {
            component_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_LoggingConfigRef {
        ContainerAttachedCluster_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_config` after provisioning.\n"]
    pub fn component_config(&self) -> terrars::ListRef<ContainerAttachedCluster_LoggingConfig_ComponentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig {
    #[doc = "Set the field `enabled`.\nEnable Managed Collection."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig {}

impl BuildContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig {
    pub fn build(self) -> ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig {
        ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfigRef {
        ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnable Managed Collection."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAttachedCluster_MonitoringConfigDynamic {
    managed_prometheus_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig>>,
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_MonitoringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_prometheus_config: std
    ::option
    ::Option<std::vec::Vec<ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig>>,
    dynamic: ContainerAttachedCluster_MonitoringConfigDynamic,
}

impl ContainerAttachedCluster_MonitoringConfig {
    #[doc = "Set the field `managed_prometheus_config`.\n"]
    pub fn set_managed_prometheus_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_prometheus_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_prometheus_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_MonitoringConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_MonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_MonitoringConfig {}

impl BuildContainerAttachedCluster_MonitoringConfig {
    pub fn build(self) -> ContainerAttachedCluster_MonitoringConfig {
        ContainerAttachedCluster_MonitoringConfig {
            managed_prometheus_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_MonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_MonitoringConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_MonitoringConfigRef {
        ContainerAttachedCluster_MonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_MonitoringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_prometheus_config` after provisioning.\n"]
    pub fn managed_prometheus_config(
        &self,
    ) -> terrars::ListRef<ContainerAttachedCluster_MonitoringConfig_ManagedPrometheusConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_prometheus_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_OidcConfig {
    issuer_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwks: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAttachedCluster_OidcConfig {
    #[doc = "Set the field `jwks`.\nOIDC verification keys in JWKS format (RFC 7517)."]
    pub fn set_jwks(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_OidcConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_OidcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_OidcConfig {
    #[doc = "A JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://'"]
    pub issuer_url: terrars::PrimField<std::string::String>,
}

impl BuildContainerAttachedCluster_OidcConfig {
    pub fn build(self) -> ContainerAttachedCluster_OidcConfig {
        ContainerAttachedCluster_OidcConfig {
            issuer_url: self.issuer_url,
            jwks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_OidcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_OidcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_OidcConfigRef {
        ContainerAttachedCluster_OidcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_OidcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `issuer_url` after provisioning.\nA JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://'"]
    pub fn issuer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jwks` after provisioning.\nOIDC verification keys in JWKS format (RFC 7517)."]
    pub fn jwks(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_ProxyConfig_KubernetesSecret {
    name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
}

impl ContainerAttachedCluster_ProxyConfig_KubernetesSecret { }

impl terrars::ToListMappable for ContainerAttachedCluster_ProxyConfig_KubernetesSecret {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_ProxyConfig_KubernetesSecret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_ProxyConfig_KubernetesSecret {
    #[doc = "Name of the kubernetes secret containing the proxy config."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Namespace of the kubernetes secret containing the proxy config."]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildContainerAttachedCluster_ProxyConfig_KubernetesSecret {
    pub fn build(self) -> ContainerAttachedCluster_ProxyConfig_KubernetesSecret {
        ContainerAttachedCluster_ProxyConfig_KubernetesSecret {
            name: self.name,
            namespace: self.namespace,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_ProxyConfig_KubernetesSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_ProxyConfig_KubernetesSecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAttachedCluster_ProxyConfig_KubernetesSecretRef {
        ContainerAttachedCluster_ProxyConfig_KubernetesSecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_ProxyConfig_KubernetesSecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the kubernetes secret containing the proxy config."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nNamespace of the kubernetes secret containing the proxy config."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAttachedCluster_ProxyConfigDynamic {
    kubernetes_secret: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAttachedCluster_ProxyConfig_KubernetesSecret>>,
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_ProxyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_secret: std::option::Option<std::vec::Vec<ContainerAttachedCluster_ProxyConfig_KubernetesSecret>>,
    dynamic: ContainerAttachedCluster_ProxyConfigDynamic,
}

impl ContainerAttachedCluster_ProxyConfig {
    #[doc = "Set the field `kubernetes_secret`.\n"]
    pub fn set_kubernetes_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAttachedCluster_ProxyConfig_KubernetesSecret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kubernetes_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes_secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAttachedCluster_ProxyConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_ProxyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_ProxyConfig {}

impl BuildContainerAttachedCluster_ProxyConfig {
    pub fn build(self) -> ContainerAttachedCluster_ProxyConfig {
        ContainerAttachedCluster_ProxyConfig {
            kubernetes_secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_ProxyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_ProxyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_ProxyConfigRef {
        ContainerAttachedCluster_ProxyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_ProxyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kubernetes_secret` after provisioning.\n"]
    pub fn kubernetes_secret(&self) -> terrars::ListRef<ContainerAttachedCluster_ProxyConfig_KubernetesSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_SecurityPostureConfig {
    vulnerability_mode: terrars::PrimField<std::string::String>,
}

impl ContainerAttachedCluster_SecurityPostureConfig { }

impl terrars::ToListMappable for ContainerAttachedCluster_SecurityPostureConfig {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_SecurityPostureConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_SecurityPostureConfig {
    #[doc =
        "Sets the mode of the Kubernetes security posture API's workload vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub vulnerability_mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerAttachedCluster_SecurityPostureConfig {
    pub fn build(self) -> ContainerAttachedCluster_SecurityPostureConfig {
        ContainerAttachedCluster_SecurityPostureConfig { vulnerability_mode: self.vulnerability_mode }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_SecurityPostureConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_SecurityPostureConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAttachedCluster_SecurityPostureConfigRef {
        ContainerAttachedCluster_SecurityPostureConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_SecurityPostureConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `vulnerability_mode` after provisioning.\nSets the mode of the Kubernetes security posture API's workload vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub fn vulnerability_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vulnerability_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAttachedCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAttachedCluster_Timeouts {
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

impl terrars::ToListMappable for ContainerAttachedCluster_Timeouts {
    type O = terrars::BlockAssignable<ContainerAttachedCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAttachedCluster_Timeouts {}

impl BuildContainerAttachedCluster_Timeouts {
    pub fn build(self) -> ContainerAttachedCluster_Timeouts {
        ContainerAttachedCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAttachedCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAttachedCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAttachedCluster_TimeoutsRef {
        ContainerAttachedCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAttachedCluster_TimeoutsRef {
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
