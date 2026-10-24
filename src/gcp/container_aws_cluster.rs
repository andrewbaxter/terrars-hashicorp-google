use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerAwsClusterData {
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
    aws_region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<ContainerAwsClusterAuthorizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<ContainerAwsClusterBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<ContainerAwsClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<ContainerAwsClusterFleetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networking: std::option::Option<std::vec::Vec<ContainerAwsClusterNetworkingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerAwsClusterTimeoutsEl>,
    dynamic: ContainerAwsClusterDynamic,
}

struct ContainerAwsCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerAwsClusterData>,
}

#[derive(Clone)]
pub struct ContainerAwsCluster(std::rc::Rc<ContainerAwsCluster_>);

impl ContainerAwsCluster {
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
        "Set the field `annotations`.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterAuthorizationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterBinaryAuthorizationEl>>,
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

    #[doc = "Set the field `control_plane`.\n"]
    pub fn set_control_plane(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneEl>>,
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
    pub fn set_fleet(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterFleetEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterNetworkingEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerAwsClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `aws_region` after provisioning.\nThe AWS region where the cluster runs. Each Google Cloud region supports a subset of nearby AWS regions. You can call to list all supported AWS regions within a given Google Cloud region."]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerAwsClusterWorkloadIdentityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<ContainerAwsClusterAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerAwsClusterBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerAwsClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<ContainerAwsClusterNetworkingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAwsClusterTimeoutsElRef {
        ContainerAwsClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerAwsCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerAwsCluster { }

impl terrars::ToListMappable for ContainerAwsCluster {
    type O = terrars::ListRef<ContainerAwsClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerAwsCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_aws_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerAwsCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The AWS region where the cluster runs. Each Google Cloud region supports a subset of nearby AWS regions. You can call to list all supported AWS regions within a given Google Cloud region."]
    pub aws_region: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of this resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerAwsCluster {
        let out = ContainerAwsCluster(std::rc::Rc::new(ContainerAwsCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerAwsClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                aws_region: self.aws_region,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                authorization: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
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

pub struct ContainerAwsClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerAwsClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `aws_region` after provisioning.\nThe AWS region where the cluster runs. Each Google Cloud region supports a subset of nearby AWS regions. You can call to list all supported AWS regions within a given Google Cloud region."]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerAwsClusterWorkloadIdentityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<ContainerAwsClusterAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerAwsClusterBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerAwsClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<ContainerAwsClusterNetworkingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAwsClusterTimeoutsElRef {
        ContainerAwsClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterWorkloadIdentityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsClusterWorkloadIdentityConfigEl {
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

impl terrars::ToListMappable for ContainerAwsClusterWorkloadIdentityConfigEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterWorkloadIdentityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterWorkloadIdentityConfigEl {}

impl BuildContainerAwsClusterWorkloadIdentityConfigEl {
    pub fn build(self) -> ContainerAwsClusterWorkloadIdentityConfigEl {
        ContainerAwsClusterWorkloadIdentityConfigEl {
            identity_provider: core::default::Default::default(),
            issuer_uri: core::default::Default::default(),
            workload_pool: core::default::Default::default(),
        }
    }
}

pub struct ContainerAwsClusterWorkloadIdentityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterWorkloadIdentityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterWorkloadIdentityConfigElRef {
        ContainerAwsClusterWorkloadIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterWorkloadIdentityConfigElRef {
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
pub struct ContainerAwsClusterAuthorizationElAdminGroupsEl {
    group: terrars::PrimField<std::string::String>,
}

impl ContainerAwsClusterAuthorizationElAdminGroupsEl { }

impl terrars::ToListMappable for ContainerAwsClusterAuthorizationElAdminGroupsEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterAuthorizationElAdminGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterAuthorizationElAdminGroupsEl {
    #[doc = "The name of the group, e.g. `my-group@domain.com`."]
    pub group: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterAuthorizationElAdminGroupsEl {
    pub fn build(self) -> ContainerAwsClusterAuthorizationElAdminGroupsEl {
        ContainerAwsClusterAuthorizationElAdminGroupsEl { group: self.group }
    }
}

pub struct ContainerAwsClusterAuthorizationElAdminGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterAuthorizationElAdminGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterAuthorizationElAdminGroupsElRef {
        ContainerAwsClusterAuthorizationElAdminGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterAuthorizationElAdminGroupsElRef {
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
pub struct ContainerAwsClusterAuthorizationElAdminUsersEl {
    username: terrars::PrimField<std::string::String>,
}

impl ContainerAwsClusterAuthorizationElAdminUsersEl { }

impl terrars::ToListMappable for ContainerAwsClusterAuthorizationElAdminUsersEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterAuthorizationElAdminUsersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterAuthorizationElAdminUsersEl {
    #[doc = "The name of the user, e.g. `my-gcp-id@gmail.com`."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterAuthorizationElAdminUsersEl {
    pub fn build(self) -> ContainerAwsClusterAuthorizationElAdminUsersEl {
        ContainerAwsClusterAuthorizationElAdminUsersEl { username: self.username }
    }
}

pub struct ContainerAwsClusterAuthorizationElAdminUsersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterAuthorizationElAdminUsersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterAuthorizationElAdminUsersElRef {
        ContainerAwsClusterAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterAuthorizationElAdminUsersElRef {
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
struct ContainerAwsClusterAuthorizationElDynamic {
    admin_groups: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterAuthorizationElAdminGroupsEl>>,
    admin_users: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterAuthorizationElAdminUsersEl>>,
}

#[derive(Serialize)]
pub struct ContainerAwsClusterAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_groups: std::option::Option<std::vec::Vec<ContainerAwsClusterAuthorizationElAdminGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<std::vec::Vec<ContainerAwsClusterAuthorizationElAdminUsersEl>>,
    dynamic: ContainerAwsClusterAuthorizationElDynamic,
}

impl ContainerAwsClusterAuthorizationEl {
    #[doc = "Set the field `admin_groups`.\n"]
    pub fn set_admin_groups(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterAuthorizationElAdminGroupsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterAuthorizationElAdminUsersEl>>,
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

impl terrars::ToListMappable for ContainerAwsClusterAuthorizationEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterAuthorizationEl {}

impl BuildContainerAwsClusterAuthorizationEl {
    pub fn build(self) -> ContainerAwsClusterAuthorizationEl {
        ContainerAwsClusterAuthorizationEl {
            admin_groups: core::default::Default::default(),
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerAwsClusterAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsClusterAuthorizationElRef {
        ContainerAwsClusterAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_groups` after provisioning.\n"]
    pub fn admin_groups(&self) -> terrars::ListRef<ContainerAwsClusterAuthorizationElAdminGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> terrars::ListRef<ContainerAwsClusterAuthorizationElAdminUsersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsClusterBinaryAuthorizationEl {
    #[doc =
        "Set the field `evaluation_mode`.\nMode of operation for Binary Authorization policy evaluation. Possible values: DISABLED, PROJECT_SINGLETON_POLICY_ENFORCE"]
    pub fn set_evaluation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsClusterBinaryAuthorizationEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterBinaryAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterBinaryAuthorizationEl {}

impl BuildContainerAwsClusterBinaryAuthorizationEl {
    pub fn build(self) -> ContainerAwsClusterBinaryAuthorizationEl {
        ContainerAwsClusterBinaryAuthorizationEl { evaluation_mode: core::default::Default::default() }
    }
}

pub struct ContainerAwsClusterBinaryAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterBinaryAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsClusterBinaryAuthorizationElRef {
        ContainerAwsClusterBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterBinaryAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluation_mode` after provisioning.\nMode of operation for Binary Authorization policy evaluation. Possible values: DISABLED, PROJECT_SINGLETON_POLICY_ENFORCE"]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl {
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_session_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl {
    #[doc =
        "Set the field `role_session_name`.\nOptional. An identifier for the assumed role session. When unspecified, it defaults to `multicloud-service-agent`."]
    pub fn set_role_session_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_session_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl {
    #[doc =
        "The Amazon Resource Name (ARN) of the role that the Anthos Multi-Cloud API will assume when managing AWS resources on your account."]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl {
        ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl {
            role_arn: self.role_arn,
            role_session_name: core::default::Default::default(),
        }
    }
}

pub struct ContainerAwsClusterControlPlaneElAwsServicesAuthenticationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElAwsServicesAuthenticationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterControlPlaneElAwsServicesAuthenticationElRef {
        ContainerAwsClusterControlPlaneElAwsServicesAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElAwsServicesAuthenticationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the role that the Anthos Multi-Cloud API will assume when managing AWS resources on your account."]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `role_session_name` after provisioning.\nOptional. An identifier for the assumed role session. When unspecified, it defaults to `multicloud-service-agent`."]
    pub fn role_session_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_session_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneElConfigEncryptionEl {
    kms_key_arn: terrars::PrimField<std::string::String>,
}

impl ContainerAwsClusterControlPlaneElConfigEncryptionEl { }

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneElConfigEncryptionEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneElConfigEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneElConfigEncryptionEl {
    #[doc = "The ARN of the AWS KMS key used to encrypt cluster configuration."]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterControlPlaneElConfigEncryptionEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneElConfigEncryptionEl {
        ContainerAwsClusterControlPlaneElConfigEncryptionEl { kms_key_arn: self.kms_key_arn }
    }
}

pub struct ContainerAwsClusterControlPlaneElConfigEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElConfigEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterControlPlaneElConfigEncryptionElRef {
        ContainerAwsClusterControlPlaneElConfigEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElConfigEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_arn` after provisioning.\nThe ARN of the AWS KMS key used to encrypt cluster configuration."]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneElDatabaseEncryptionEl {
    kms_key_arn: terrars::PrimField<std::string::String>,
}

impl ContainerAwsClusterControlPlaneElDatabaseEncryptionEl { }

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneElDatabaseEncryptionEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneElDatabaseEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneElDatabaseEncryptionEl {
    #[doc = "The ARN of the AWS KMS key used to encrypt cluster secrets."]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterControlPlaneElDatabaseEncryptionEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneElDatabaseEncryptionEl {
        ContainerAwsClusterControlPlaneElDatabaseEncryptionEl { kms_key_arn: self.kms_key_arn }
    }
}

pub struct ContainerAwsClusterControlPlaneElDatabaseEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElDatabaseEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterControlPlaneElDatabaseEncryptionElRef {
        ContainerAwsClusterControlPlaneElDatabaseEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElDatabaseEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_arn` after provisioning.\nThe ARN of the AWS KMS key used to encrypt cluster secrets."]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneElMainVolumeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsClusterControlPlaneElMainVolumeEl {
    #[doc =
        "Set the field `iops`.\nOptional. The number of I/O operations per second (IOPS) to provision for GP3 volume."]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_arn`.\nOptional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used."]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `size_gib`.\nOptional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gib = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `throughput`.\nOptional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125."]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_type`.\nOptional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneElMainVolumeEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneElMainVolumeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneElMainVolumeEl {}

impl BuildContainerAwsClusterControlPlaneElMainVolumeEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneElMainVolumeEl {
        ContainerAwsClusterControlPlaneElMainVolumeEl {
            iops: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            size_gib: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct ContainerAwsClusterControlPlaneElMainVolumeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElMainVolumeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterControlPlaneElMainVolumeElRef {
        ContainerAwsClusterControlPlaneElMainVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElMainVolumeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `iops` after provisioning.\nOptional. The number of I/O operations per second (IOPS) to provision for GP3 volume."]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_arn` after provisioning.\nOptional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used."]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }

    #[doc =
        "Get a reference to the value of field `throughput` after provisioning.\nOptional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125."]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_type` after provisioning.\nOptional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneElProxyConfigEl {
    secret_arn: terrars::PrimField<std::string::String>,
    secret_version: terrars::PrimField<std::string::String>,
}

impl ContainerAwsClusterControlPlaneElProxyConfigEl { }

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneElProxyConfigEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneElProxyConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneElProxyConfigEl {
    #[doc = "The ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub secret_arn: terrars::PrimField<std::string::String>,
    #[doc = "The version string of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterControlPlaneElProxyConfigEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneElProxyConfigEl {
        ContainerAwsClusterControlPlaneElProxyConfigEl {
            secret_arn: self.secret_arn,
            secret_version: self.secret_version,
        }
    }
}

pub struct ContainerAwsClusterControlPlaneElProxyConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElProxyConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterControlPlaneElProxyConfigElRef {
        ContainerAwsClusterControlPlaneElProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElProxyConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_arn` after provisioning.\nThe ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe version string of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneElRootVolumeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsClusterControlPlaneElRootVolumeEl {
    #[doc =
        "Set the field `iops`.\nOptional. The number of I/O operations per second (IOPS) to provision for GP3 volume."]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_arn`.\nOptional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used."]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `size_gib`.\nOptional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gib = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `throughput`.\nOptional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125."]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_type`.\nOptional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneElRootVolumeEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneElRootVolumeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneElRootVolumeEl {}

impl BuildContainerAwsClusterControlPlaneElRootVolumeEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneElRootVolumeEl {
        ContainerAwsClusterControlPlaneElRootVolumeEl {
            iops: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            size_gib: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct ContainerAwsClusterControlPlaneElRootVolumeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElRootVolumeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterControlPlaneElRootVolumeElRef {
        ContainerAwsClusterControlPlaneElRootVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElRootVolumeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `iops` after provisioning.\nOptional. The number of I/O operations per second (IOPS) to provision for GP3 volume."]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_arn` after provisioning.\nOptional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used."]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }

    #[doc =
        "Get a reference to the value of field `throughput` after provisioning.\nOptional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125."]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_type` after provisioning.\nOptional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneElSshConfigEl {
    ec2_key_pair: terrars::PrimField<std::string::String>,
}

impl ContainerAwsClusterControlPlaneElSshConfigEl { }

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneElSshConfigEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneElSshConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneElSshConfigEl {
    #[doc = "The name of the EC2 key pair used to login into cluster machines."]
    pub ec2_key_pair: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterControlPlaneElSshConfigEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneElSshConfigEl {
        ContainerAwsClusterControlPlaneElSshConfigEl { ec2_key_pair: self.ec2_key_pair }
    }
}

pub struct ContainerAwsClusterControlPlaneElSshConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElSshConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsClusterControlPlaneElSshConfigElRef {
        ContainerAwsClusterControlPlaneElSshConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElSshConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ec2_key_pair` after provisioning.\nThe name of the EC2 key pair used to login into cluster machines."]
    pub fn ec2_key_pair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_key_pair", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAwsClusterControlPlaneElDynamic {
    aws_services_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl>>,
    config_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneElConfigEncryptionEl>>,
    database_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneElDatabaseEncryptionEl>>,
    main_volume: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneElMainVolumeEl>>,
    proxy_config: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneElProxyConfigEl>>,
    root_volume: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneElRootVolumeEl>>,
    ssh_config: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneElSshConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerAwsClusterControlPlaneEl {
    iam_instance_profile: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_services_authentication: std
    ::option
    ::Option<std::vec::Vec<ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_encryption: std::option::Option<std::vec::Vec<ContainerAwsClusterControlPlaneElConfigEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_encryption: std::option::Option<std::vec::Vec<ContainerAwsClusterControlPlaneElDatabaseEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_volume: std::option::Option<std::vec::Vec<ContainerAwsClusterControlPlaneElMainVolumeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_config: std::option::Option<std::vec::Vec<ContainerAwsClusterControlPlaneElProxyConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_volume: std::option::Option<std::vec::Vec<ContainerAwsClusterControlPlaneElRootVolumeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_config: std::option::Option<std::vec::Vec<ContainerAwsClusterControlPlaneElSshConfigEl>>,
    dynamic: ContainerAwsClusterControlPlaneElDynamic,
}

impl ContainerAwsClusterControlPlaneEl {
    #[doc =
        "Set the field `instance_type`.\nOptional. The AWS instance type. When unspecified, it defaults to `m5.large`."]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_group_ids`.\nOptional. The IDs of additional security groups to add to control plane replicas. The Anthos Multi-Cloud API will automatically create and manage security groups with the minimum rules needed for a functioning cluster."]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nOptional. A set of AWS resource tags to propagate to all underlying managed AWS resources. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_services_authentication`.\n"]
    pub fn set_aws_services_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneElAwsServicesAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_services_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_services_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config_encryption`.\n"]
    pub fn set_config_encryption(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneElConfigEncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database_encryption`.\n"]
    pub fn set_database_encryption(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneElDatabaseEncryptionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneElMainVolumeEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneElProxyConfigEl>>,
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

    #[doc = "Set the field `root_volume`.\n"]
    pub fn set_root_volume(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneElRootVolumeEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsClusterControlPlaneElSshConfigEl>>,
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

impl terrars::ToListMappable for ContainerAwsClusterControlPlaneEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterControlPlaneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterControlPlaneEl {
    #[doc = "The name of the AWS IAM instance pofile to assign to each control plane replica."]
    pub iam_instance_profile: terrars::PrimField<std::string::String>,
    #[doc =
        "The list of subnets where control plane replicas will run. A replica will be provisioned on each subnet and up to three values can be provided. Each subnet must be in a different AWS Availability Zone (AZ)."]
    pub subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling ."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterControlPlaneEl {
    pub fn build(self) -> ContainerAwsClusterControlPlaneEl {
        ContainerAwsClusterControlPlaneEl {
            iam_instance_profile: self.iam_instance_profile,
            instance_type: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            tags: core::default::Default::default(),
            version: self.version,
            aws_services_authentication: core::default::Default::default(),
            config_encryption: core::default::Default::default(),
            database_encryption: core::default::Default::default(),
            main_volume: core::default::Default::default(),
            proxy_config: core::default::Default::default(),
            root_volume: core::default::Default::default(),
            ssh_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerAwsClusterControlPlaneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterControlPlaneElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsClusterControlPlaneElRef {
        ContainerAwsClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterControlPlaneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `iam_instance_profile` after provisioning.\nThe name of the AWS IAM instance pofile to assign to each control plane replica."]
    pub fn iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nOptional. The AWS instance type. When unspecified, it defaults to `m5.large`."]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_group_ids` after provisioning.\nOptional. The IDs of additional security groups to add to control plane replicas. The Anthos Multi-Cloud API will automatically create and manage security groups with the minimum rules needed for a functioning cluster."]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnet_ids` after provisioning.\nThe list of subnets where control plane replicas will run. A replica will be provisioned on each subnet and up to three values can be provided. Each subnet must be in a different AWS Availability Zone (AZ)."]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional. A set of AWS resource tags to propagate to all underlying managed AWS resources. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling ."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_services_authentication` after provisioning.\n"]
    pub fn aws_services_authentication(
        &self,
    ) -> terrars::ListRef<ContainerAwsClusterControlPlaneElAwsServicesAuthenticationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_services_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `config_encryption` after provisioning.\n"]
    pub fn config_encryption(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElConfigEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\n"]
    pub fn database_encryption(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElDatabaseEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `main_volume` after provisioning.\n"]
    pub fn main_volume(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElMainVolumeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.main_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElProxyConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.base))
    }

    #[doc = "Get a reference to the value of field `root_volume` after provisioning.\n"]
    pub fn root_volume(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElRootVolumeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_config` after provisioning.\n"]
    pub fn ssh_config(&self) -> terrars::ListRef<ContainerAwsClusterControlPlaneElSshConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterFleetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsClusterFleetEl {
    #[doc = "Set the field `project`.\nThe number of the Fleet host project where this cluster will be registered."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsClusterFleetEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterFleetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterFleetEl {}

impl BuildContainerAwsClusterFleetEl {
    pub fn build(self) -> ContainerAwsClusterFleetEl {
        ContainerAwsClusterFleetEl { project: core::default::Default::default() }
    }
}

pub struct ContainerAwsClusterFleetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterFleetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsClusterFleetElRef {
        ContainerAwsClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterFleetElRef {
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
pub struct ContainerAwsClusterNetworkingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_node_pool_sg_rules_disabled: std::option::Option<terrars::PrimField<bool>>,
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl ContainerAwsClusterNetworkingEl {
    #[doc =
        "Set the field `per_node_pool_sg_rules_disabled`.\nDisable the per node pool subnet security group rules on the control plane security group. When set to true, you must also provide one or more security groups that ensure node pools are able to send requests to the control plane on TCP/443 and TCP/8132. Failure to do so may result in unavailable node pools."]
    pub fn set_per_node_pool_sg_rules_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.per_node_pool_sg_rules_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsClusterNetworkingEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterNetworkingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterNetworkingEl {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The VPC associated with the cluster. All component clusters (i.e. control plane and node pools) run on a single VPC. This field cannot be changed after creation."]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsClusterNetworkingEl {
    pub fn build(self) -> ContainerAwsClusterNetworkingEl {
        ContainerAwsClusterNetworkingEl {
            per_node_pool_sg_rules_disabled: core::default::Default::default(),
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct ContainerAwsClusterNetworkingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterNetworkingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsClusterNetworkingElRef {
        ContainerAwsClusterNetworkingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterNetworkingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `per_node_pool_sg_rules_disabled` after provisioning.\nDisable the per node pool subnet security group rules on the control plane security group. When set to true, you must also provide one or more security groups that ensure node pools are able to send requests to the control plane on TCP/443 and TCP/8132. Failure to do so may result in unavailable node pools."]
    pub fn per_node_pool_sg_rules_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_node_pool_sg_rules_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_address_cidr_blocks` after provisioning.\nAll pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub fn pod_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_address_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_address_cidr_blocks` after provisioning.\nAll services in the cluster are assigned an RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub fn service_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_address_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc_id` after provisioning.\nThe VPC associated with the cluster. All component clusters (i.e. control plane and node pools) run on a single VPC. This field cannot be changed after creation."]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsClusterTimeoutsEl {
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

impl terrars::ToListMappable for ContainerAwsClusterTimeoutsEl {
    type O = terrars::BlockAssignable<ContainerAwsClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsClusterTimeoutsEl {}

impl BuildContainerAwsClusterTimeoutsEl {
    pub fn build(self) -> ContainerAwsClusterTimeoutsEl {
        ContainerAwsClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ContainerAwsClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsClusterTimeoutsElRef {
        ContainerAwsClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsClusterTimeoutsElRef {
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
struct ContainerAwsClusterDynamic {
    authorization: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterAuthorizationEl>>,
    binary_authorization: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterBinaryAuthorizationEl>>,
    control_plane: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterControlPlaneEl>>,
    fleet: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterFleetEl>>,
    networking: std::option::Option<terrars::DynamicBlock<ContainerAwsClusterNetworkingEl>>,
}
