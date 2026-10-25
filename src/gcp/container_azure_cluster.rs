#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerAzureClusterData_ {
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
    azure_region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_group_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<ContainerAzureCluster_Authorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_services_authentication: std::option::Option<std::vec::Vec<ContainerAzureCluster_AzureServicesAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<ContainerAzureCluster_ControlPlane>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<ContainerAzureCluster_Fleet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networking: std::option::Option<std::vec::Vec<ContainerAzureCluster_Networking>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerAzureCluster_Timeouts>,
    dynamic: ContainerAzureClusterDynamic,
}

struct ContainerAzureCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerAzureClusterData_>,
}

#[derive(Clone)]
pub struct ContainerAzureCluster(std::rc::Rc<ContainerAzureCluster_>);

impl ContainerAzureCluster {
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
        "Set the field `annotations`.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client`.\nName of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the `AzureCluster`. `AzureClient` names are formatted as `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names."]
    pub fn set_client(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nOptional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_Authorization>>,
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

    #[doc = "Set the field `azure_services_authentication`.\n"]
    pub fn set_azure_services_authentication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_AzureServicesAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().azure_services_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.azure_services_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane`.\n"]
    pub fn set_control_plane(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_ControlPlane>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_Fleet>>) -> Self {
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

    #[doc = "Set the field `networking`.\n"]
    pub fn set_networking(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_Networking>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networking = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networking = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerAzureCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `azure_region` after provisioning.\nThe Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region."]
    pub fn azure_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client` after provisioning.\nName of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the `AzureCluster`. `AzureClient` names are formatted as `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names."]
    pub fn client(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nOutput only. The endpoint of the cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nAllows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_group_id` after provisioning.\nThe ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`"]
    pub fn resource_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the cluster. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_config` after provisioning.\nOutput only. Workload Identity settings."]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerAzureCluster_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<ContainerAzureCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `azure_services_authentication` after provisioning.\n"]
    pub fn azure_services_authentication(
        &self,
    ) -> terrars::ListRef<ContainerAzureCluster_AzureServicesAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_services_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerAzureCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<ContainerAzureCluster_NetworkingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureCluster_TimeoutsRef {
        ContainerAzureCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerAzureCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerAzureCluster { }

impl terrars::ToListMappable for ContainerAzureCluster {
    type O = terrars::ListRef<ContainerAzureClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerAzureCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_azure_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerAzureCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region."]
    pub azure_region: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of this resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`"]
    pub resource_group_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerAzureCluster {
        let out = ContainerAzureCluster(std::rc::Rc::new(ContainerAzureCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerAzureClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                azure_region: self.azure_region,
                client: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                resource_group_id: self.resource_group_id,
                authorization: core::default::Default::default(),
                azure_services_authentication: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                fleet: core::default::Default::default(),
                networking: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerAzureClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerAzureClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `azure_region` after provisioning.\nThe Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region."]
    pub fn azure_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client` after provisioning.\nName of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the `AzureCluster`. `AzureClient` names are formatted as `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names."]
    pub fn client(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nOutput only. The endpoint of the cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nAllows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_group_id` after provisioning.\nThe ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`"]
    pub fn resource_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the cluster. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_config` after provisioning.\nOutput only. Workload Identity settings."]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerAzureCluster_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<ContainerAzureCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `azure_services_authentication` after provisioning.\n"]
    pub fn azure_services_authentication(
        &self,
    ) -> terrars::ListRef<ContainerAzureCluster_AzureServicesAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_services_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerAzureCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<ContainerAzureCluster_NetworkingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureCluster_TimeoutsRef {
        ContainerAzureCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ContainerAzureClusterDynamic {
    authorization: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_Authorization>>,
    azure_services_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAzureCluster_AzureServicesAuthentication>>,
    control_plane: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_ControlPlane>>,
    fleet: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_Fleet>>,
    networking: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_Networking>>,
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_WorkloadIdentityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAzureCluster_WorkloadIdentityConfig {
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

impl terrars::ToListMappable for ContainerAzureCluster_WorkloadIdentityConfig {
    type O = terrars::BlockAssignable<ContainerAzureCluster_WorkloadIdentityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_WorkloadIdentityConfig {}

impl BuildContainerAzureCluster_WorkloadIdentityConfig {
    pub fn build(self) -> ContainerAzureCluster_WorkloadIdentityConfig {
        ContainerAzureCluster_WorkloadIdentityConfig {
            identity_provider: core::default::Default::default(),
            issuer_uri: core::default::Default::default(),
            workload_pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_WorkloadIdentityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_WorkloadIdentityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_WorkloadIdentityConfigRef {
        ContainerAzureCluster_WorkloadIdentityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_WorkloadIdentityConfigRef {
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
pub struct ContainerAzureCluster_Authorization_AdminGroups {
    group: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_Authorization_AdminGroups { }

impl terrars::ToListMappable for ContainerAzureCluster_Authorization_AdminGroups {
    type O = terrars::BlockAssignable<ContainerAzureCluster_Authorization_AdminGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_Authorization_AdminGroups {
    #[doc = "The name of the group, e.g. `my-group@domain.com`."]
    pub group: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_Authorization_AdminGroups {
    pub fn build(self) -> ContainerAzureCluster_Authorization_AdminGroups {
        ContainerAzureCluster_Authorization_AdminGroups { group: self.group }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_Authorization_AdminGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_Authorization_AdminGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_Authorization_AdminGroupsRef {
        ContainerAzureCluster_Authorization_AdminGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_Authorization_AdminGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group` after provisioning.\nThe name of the group, e.g. `my-group@domain.com`."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_Authorization_AdminUsers {
    username: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_Authorization_AdminUsers { }

impl terrars::ToListMappable for ContainerAzureCluster_Authorization_AdminUsers {
    type O = terrars::BlockAssignable<ContainerAzureCluster_Authorization_AdminUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_Authorization_AdminUsers {
    #[doc = "The name of the user, e.g. `my-gcp-id@gmail.com`."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_Authorization_AdminUsers {
    pub fn build(self) -> ContainerAzureCluster_Authorization_AdminUsers {
        ContainerAzureCluster_Authorization_AdminUsers { username: self.username }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_Authorization_AdminUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_Authorization_AdminUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_Authorization_AdminUsersRef {
        ContainerAzureCluster_Authorization_AdminUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_Authorization_AdminUsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe name of the user, e.g. `my-gcp-id@gmail.com`."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAzureCluster_AuthorizationDynamic {
    admin_groups: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_Authorization_AdminGroups>>,
    admin_users: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_Authorization_AdminUsers>>,
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_Authorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_groups: std::option::Option<std::vec::Vec<ContainerAzureCluster_Authorization_AdminGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<std::vec::Vec<ContainerAzureCluster_Authorization_AdminUsers>>,
    dynamic: ContainerAzureCluster_AuthorizationDynamic,
}

impl ContainerAzureCluster_Authorization {
    #[doc = "Set the field `admin_groups`.\n"]
    pub fn set_admin_groups(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_Authorization_AdminGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.admin_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_Authorization_AdminUsers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.admin_users = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_users = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAzureCluster_Authorization {
    type O = terrars::BlockAssignable<ContainerAzureCluster_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_Authorization {}

impl BuildContainerAzureCluster_Authorization {
    pub fn build(self) -> ContainerAzureCluster_Authorization {
        ContainerAzureCluster_Authorization {
            admin_groups: core::default::Default::default(),
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_AuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureCluster_AuthorizationRef {
        ContainerAzureCluster_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_groups` after provisioning.\n"]
    pub fn admin_groups(&self) -> terrars::ListRef<ContainerAzureCluster_Authorization_AdminGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> terrars::ListRef<ContainerAzureCluster_Authorization_AdminUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_AzureServicesAuthentication {
    application_id: terrars::PrimField<std::string::String>,
    tenant_id: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_AzureServicesAuthentication { }

impl terrars::ToListMappable for ContainerAzureCluster_AzureServicesAuthentication {
    type O = terrars::BlockAssignable<ContainerAzureCluster_AzureServicesAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_AzureServicesAuthentication {
    #[doc = "The Azure Active Directory Application ID for Authentication configuration."]
    pub application_id: terrars::PrimField<std::string::String>,
    #[doc = "The Azure Active Directory Tenant ID for Authentication configuration."]
    pub tenant_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_AzureServicesAuthentication {
    pub fn build(self) -> ContainerAzureCluster_AzureServicesAuthentication {
        ContainerAzureCluster_AzureServicesAuthentication {
            application_id: self.application_id,
            tenant_id: self.tenant_id,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_AzureServicesAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_AzureServicesAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_AzureServicesAuthenticationRef {
        ContainerAzureCluster_AzureServicesAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_AzureServicesAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nThe Azure Active Directory Application ID for Authentication configuration."]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nThe Azure Active Directory Tenant ID for Authentication configuration."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_ControlPlane_DatabaseEncryption {
    key_id: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_ControlPlane_DatabaseEncryption { }

impl terrars::ToListMappable for ContainerAzureCluster_ControlPlane_DatabaseEncryption {
    type O = terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_DatabaseEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_ControlPlane_DatabaseEncryption {
    #[doc =
        "The ARM ID of the Azure Key Vault key to encrypt / decrypt data. For example: `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.KeyVault/vaults/<key-vault-id>/keys/<key-name>` Encryption will always take the latest version of the key and hence specific version is not supported."]
    pub key_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_ControlPlane_DatabaseEncryption {
    pub fn build(self) -> ContainerAzureCluster_ControlPlane_DatabaseEncryption {
        ContainerAzureCluster_ControlPlane_DatabaseEncryption { key_id: self.key_id }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_ControlPlane_DatabaseEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_ControlPlane_DatabaseEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_ControlPlane_DatabaseEncryptionRef {
        ContainerAzureCluster_ControlPlane_DatabaseEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_ControlPlane_DatabaseEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_id` after provisioning.\nThe ARM ID of the Azure Key Vault key to encrypt / decrypt data. For example: `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.KeyVault/vaults/<key-vault-id>/keys/<key-name>` Encryption will always take the latest version of the key and hence specific version is not supported."]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_ControlPlane_MainVolume {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gib: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerAzureCluster_ControlPlane_MainVolume {
    #[doc =
        "Set the field `size_gib`.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gib = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAzureCluster_ControlPlane_MainVolume {
    type O = terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_MainVolume>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_ControlPlane_MainVolume {}

impl BuildContainerAzureCluster_ControlPlane_MainVolume {
    pub fn build(self) -> ContainerAzureCluster_ControlPlane_MainVolume {
        ContainerAzureCluster_ControlPlane_MainVolume { size_gib: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_ControlPlane_MainVolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_ControlPlane_MainVolumeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_ControlPlane_MainVolumeRef {
        ContainerAzureCluster_ControlPlane_MainVolumeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_ControlPlane_MainVolumeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_ControlPlane_ProxyConfig {
    resource_group_id: terrars::PrimField<std::string::String>,
    secret_id: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_ControlPlane_ProxyConfig { }

impl terrars::ToListMappable for ContainerAzureCluster_ControlPlane_ProxyConfig {
    type O = terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_ProxyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_ControlPlane_ProxyConfig {
    #[doc =
        "The ARM ID the of the resource group containing proxy keyvault. Resource group ids are formatted as `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`"]
    pub resource_group_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The URL the of the proxy setting secret with its version. Secret ids are formatted as `https:<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`."]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_ControlPlane_ProxyConfig {
    pub fn build(self) -> ContainerAzureCluster_ControlPlane_ProxyConfig {
        ContainerAzureCluster_ControlPlane_ProxyConfig {
            resource_group_id: self.resource_group_id,
            secret_id: self.secret_id,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_ControlPlane_ProxyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_ControlPlane_ProxyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_ControlPlane_ProxyConfigRef {
        ContainerAzureCluster_ControlPlane_ProxyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_ControlPlane_ProxyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_group_id` after provisioning.\nThe ARM ID the of the resource group containing proxy keyvault. Resource group ids are formatted as `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`"]
    pub fn resource_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_id` after provisioning.\nThe URL the of the proxy setting secret with its version. Secret ids are formatted as `https:<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`."]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_ControlPlane_ReplicaPlacements {
    azure_availability_zone: terrars::PrimField<std::string::String>,
    subnet_id: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_ControlPlane_ReplicaPlacements { }

impl terrars::ToListMappable for ContainerAzureCluster_ControlPlane_ReplicaPlacements {
    type O = terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_ReplicaPlacements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_ControlPlane_ReplicaPlacements {
    #[doc =
        "For a given replica, the Azure availability zone where to provision the control plane VM and the ETCD disk."]
    pub azure_availability_zone: terrars::PrimField<std::string::String>,
    #[doc =
        "For a given replica, the ARM ID of the subnet where the control plane VM is deployed. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub subnet_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_ControlPlane_ReplicaPlacements {
    pub fn build(self) -> ContainerAzureCluster_ControlPlane_ReplicaPlacements {
        ContainerAzureCluster_ControlPlane_ReplicaPlacements {
            azure_availability_zone: self.azure_availability_zone,
            subnet_id: self.subnet_id,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_ControlPlane_ReplicaPlacementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_ControlPlane_ReplicaPlacementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_ControlPlane_ReplicaPlacementsRef {
        ContainerAzureCluster_ControlPlane_ReplicaPlacementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_ControlPlane_ReplicaPlacementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `azure_availability_zone` after provisioning.\nFor a given replica, the Azure availability zone where to provision the control plane VM and the ETCD disk."]
    pub fn azure_availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_availability_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnet_id` after provisioning.\nFor a given replica, the ARM ID of the subnet where the control plane VM is deployed. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_ControlPlane_RootVolume {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gib: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerAzureCluster_ControlPlane_RootVolume {
    #[doc =
        "Set the field `size_gib`.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gib = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAzureCluster_ControlPlane_RootVolume {
    type O = terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_RootVolume>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_ControlPlane_RootVolume {}

impl BuildContainerAzureCluster_ControlPlane_RootVolume {
    pub fn build(self) -> ContainerAzureCluster_ControlPlane_RootVolume {
        ContainerAzureCluster_ControlPlane_RootVolume { size_gib: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_ControlPlane_RootVolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_ControlPlane_RootVolumeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_ControlPlane_RootVolumeRef {
        ContainerAzureCluster_ControlPlane_RootVolumeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_ControlPlane_RootVolumeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_ControlPlane_SshConfig {
    authorized_key: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_ControlPlane_SshConfig { }

impl terrars::ToListMappable for ContainerAzureCluster_ControlPlane_SshConfig {
    type O = terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_SshConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_ControlPlane_SshConfig {
    #[doc =
        "The SSH public key data for VMs managed by Anthos. This accepts the authorized_keys file format used in OpenSSH according to the sshd(8) manual page."]
    pub authorized_key: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_ControlPlane_SshConfig {
    pub fn build(self) -> ContainerAzureCluster_ControlPlane_SshConfig {
        ContainerAzureCluster_ControlPlane_SshConfig { authorized_key: self.authorized_key }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_ControlPlane_SshConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_ControlPlane_SshConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureCluster_ControlPlane_SshConfigRef {
        ContainerAzureCluster_ControlPlane_SshConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_ControlPlane_SshConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authorized_key` after provisioning.\nThe SSH public key data for VMs managed by Anthos. This accepts the authorized_keys file format used in OpenSSH according to the sshd(8) manual page."]
    pub fn authorized_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAzureCluster_ControlPlaneDynamic {
    database_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAzureCluster_ControlPlane_DatabaseEncryption>>,
    main_volume: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_ControlPlane_MainVolume>>,
    proxy_config: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_ControlPlane_ProxyConfig>>,
    replica_placements: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAzureCluster_ControlPlane_ReplicaPlacements>>,
    root_volume: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_ControlPlane_RootVolume>>,
    ssh_config: std::option::Option<terrars::DynamicBlock<ContainerAzureCluster_ControlPlane_SshConfig>>,
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_ControlPlane {
    subnet_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_encryption: std::option::Option<std::vec::Vec<ContainerAzureCluster_ControlPlane_DatabaseEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_volume: std::option::Option<std::vec::Vec<ContainerAzureCluster_ControlPlane_MainVolume>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_config: std::option::Option<std::vec::Vec<ContainerAzureCluster_ControlPlane_ProxyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_placements: std::option::Option<std::vec::Vec<ContainerAzureCluster_ControlPlane_ReplicaPlacements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_volume: std::option::Option<std::vec::Vec<ContainerAzureCluster_ControlPlane_RootVolume>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_config: std::option::Option<std::vec::Vec<ContainerAzureCluster_ControlPlane_SshConfig>>,
    dynamic: ContainerAzureCluster_ControlPlaneDynamic,
}

impl ContainerAzureCluster_ControlPlane {
    #[doc = "Set the field `tags`.\nOptional. A set of tags to apply to all underlying control plane Azure resources."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_size`.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. For available VM sizes, see https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn set_vm_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vm_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_encryption`.\n"]
    pub fn set_database_encryption(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_DatabaseEncryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `main_volume`.\n"]
    pub fn set_main_volume(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_MainVolume>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.main_volume = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.main_volume = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_ProxyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proxy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replica_placements`.\n"]
    pub fn set_replica_placements(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_ReplicaPlacements>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replica_placements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replica_placements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_volume`.\n"]
    pub fn set_root_volume(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_RootVolume>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.root_volume = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.root_volume = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssh_config`.\n"]
    pub fn set_ssh_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureCluster_ControlPlane_SshConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssh_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssh_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAzureCluster_ControlPlane {
    type O = terrars::BlockAssignable<ContainerAzureCluster_ControlPlane>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_ControlPlane {
    #[doc =
        "The ARM ID of the subnet where the control plane VMs are deployed. Example: `/subscriptions//resourceGroups//providers/Microsoft.Network/virtualNetworks//subnets/default`."]
    pub subnet_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAzureServerConfig."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_ControlPlane {
    pub fn build(self) -> ContainerAzureCluster_ControlPlane {
        ContainerAzureCluster_ControlPlane {
            subnet_id: self.subnet_id,
            tags: core::default::Default::default(),
            version: self.version,
            vm_size: core::default::Default::default(),
            database_encryption: core::default::Default::default(),
            main_volume: core::default::Default::default(),
            proxy_config: core::default::Default::default(),
            replica_placements: core::default::Default::default(),
            root_volume: core::default::Default::default(),
            ssh_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_ControlPlaneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_ControlPlaneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureCluster_ControlPlaneRef {
        ContainerAzureCluster_ControlPlaneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_ControlPlaneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `subnet_id` after provisioning.\nThe ARM ID of the subnet where the control plane VMs are deployed. Example: `/subscriptions//resourceGroups//providers/Microsoft.Network/virtualNetworks//subnets/default`."]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional. A set of tags to apply to all underlying control plane Azure resources."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAzureServerConfig."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_size` after provisioning.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. For available VM sizes, see https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn vm_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_size", self.base))
    }

    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\n"]
    pub fn database_encryption(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlane_DatabaseEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `main_volume` after provisioning.\n"]
    pub fn main_volume(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlane_MainVolumeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.main_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlane_ProxyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_placements` after provisioning.\n"]
    pub fn replica_placements(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlane_ReplicaPlacementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_placements", self.base))
    }

    #[doc = "Get a reference to the value of field `root_volume` after provisioning.\n"]
    pub fn root_volume(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlane_RootVolumeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_config` after provisioning.\n"]
    pub fn ssh_config(&self) -> terrars::ListRef<ContainerAzureCluster_ControlPlane_SshConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAzureCluster_Fleet {
    #[doc = "Set the field `project`.\nThe number of the Fleet host project where this cluster will be registered."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAzureCluster_Fleet {
    type O = terrars::BlockAssignable<ContainerAzureCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_Fleet {}

impl BuildContainerAzureCluster_Fleet {
    pub fn build(self) -> ContainerAzureCluster_Fleet {
        ContainerAzureCluster_Fleet { project: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureCluster_FleetRef {
        ContainerAzureCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_FleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nThe name of the managed Hub Membership resource associated to this cluster. Membership names are formatted as projects/<project-number>/locations/global/membership/<cluster-id>."]
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
pub struct ContainerAzureCluster_Networking {
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    virtual_network_id: terrars::PrimField<std::string::String>,
}

impl ContainerAzureCluster_Networking { }

impl terrars::ToListMappable for ContainerAzureCluster_Networking {
    type O = terrars::BlockAssignable<ContainerAzureCluster_Networking>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_Networking {
    #[doc =
        "The IP address range of the pods in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All pods in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The IP address range for services in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All services in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creating a cluster."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The Azure Resource Manager (ARM) ID of the VNet associated with your cluster. All components in the cluster (i.e. control plane and node pools) run on a single VNet. Example: `/subscriptions/*/resourceGroups/*/providers/Microsoft.Network/virtualNetworks/*` This field cannot be changed after creation."]
    pub virtual_network_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureCluster_Networking {
    pub fn build(self) -> ContainerAzureCluster_Networking {
        ContainerAzureCluster_Networking {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
            virtual_network_id: self.virtual_network_id,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_NetworkingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_NetworkingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureCluster_NetworkingRef {
        ContainerAzureCluster_NetworkingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_NetworkingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pod_address_cidr_blocks` after provisioning.\nThe IP address range of the pods in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All pods in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub fn pod_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_address_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_address_cidr_blocks` after provisioning.\nThe IP address range for services in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All services in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creating a cluster."]
    pub fn service_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_address_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nThe Azure Resource Manager (ARM) ID of the VNet associated with your cluster. All components in the cluster (i.e. control plane and node pools) run on a single VNet. Example: `/subscriptions/*/resourceGroups/*/providers/Microsoft.Network/virtualNetworks/*` This field cannot be changed after creation."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAzureCluster_Timeouts {
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

impl terrars::ToListMappable for ContainerAzureCluster_Timeouts {
    type O = terrars::BlockAssignable<ContainerAzureCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureCluster_Timeouts {}

impl BuildContainerAzureCluster_Timeouts {
    pub fn build(self) -> ContainerAzureCluster_Timeouts {
        ContainerAzureCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAzureCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureCluster_TimeoutsRef {
        ContainerAzureCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureCluster_TimeoutsRef {
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
