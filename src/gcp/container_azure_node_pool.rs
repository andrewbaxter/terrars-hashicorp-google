use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerAzureNodePoolData {
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
    azure_availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_id: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<ContainerAzureNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ContainerAzureNodePoolConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<std::vec::Vec<ContainerAzureNodePoolManagementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_constraint: std::option::Option<std::vec::Vec<ContainerAzureNodePoolMaxPodsConstraintEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerAzureNodePoolTimeoutsEl>,
    dynamic: ContainerAzureNodePoolDynamic,
}

struct ContainerAzureNodePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerAzureNodePoolData>,
}

#[derive(Clone)]
pub struct ContainerAzureNodePool(std::rc::Rc<ContainerAzureNodePool_>);

impl ContainerAzureNodePool {
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
        "Set the field `annotations`.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `azure_availability_zone`.\nOptional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`."]
    pub fn set_azure_availability_zone(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().azure_availability_zone = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureNodePoolAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureNodePoolConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureNodePoolManagementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `max_pods_constraint`.\n"]
    pub fn set_max_pods_constraint(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureNodePoolMaxPodsConstraintEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_pods_constraint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_pods_constraint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerAzureNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `azure_availability_zone` after provisioning.\nOptional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`."]
    pub fn azure_availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe azureCluster for the resource"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this node pool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
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
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently pending changes to the node pool."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_id` after provisioning.\nThe ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the node pool."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this node pool was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerAzureNodePoolAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ContainerAzureNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerAzureNodePoolManagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> terrars::ListRef<ContainerAzureNodePoolMaxPodsConstraintElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_pods_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureNodePoolTimeoutsElRef {
        ContainerAzureNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerAzureNodePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerAzureNodePool { }

impl terrars::ToListMappable for ContainerAzureNodePool {
    type O = terrars::ListRef<ContainerAzureNodePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerAzureNodePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_azure_node_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerAzureNodePool {
    pub tf_id: std::string::String,
    #[doc = "The azureCluster for the resource"]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of this resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub subnet_id: terrars::PrimField<std::string::String>,
    #[doc = "The Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureNodePool {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerAzureNodePool {
        let out = ContainerAzureNodePool(std::rc::Rc::new(ContainerAzureNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerAzureNodePoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                azure_availability_zone: core::default::Default::default(),
                cluster: self.cluster,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                subnet_id: self.subnet_id,
                version: self.version,
                autoscaling: core::default::Default::default(),
                config: core::default::Default::default(),
                management: core::default::Default::default(),
                max_pods_constraint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerAzureNodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerAzureNodePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `azure_availability_zone` after provisioning.\nOptional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`."]
    pub fn azure_availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe azureCluster for the resource"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this node pool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
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
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently pending changes to the node pool."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_id` after provisioning.\nThe ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the node pool."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this node pool was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerAzureNodePoolAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ContainerAzureNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerAzureNodePoolManagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> terrars::ListRef<ContainerAzureNodePoolMaxPodsConstraintElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_pods_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureNodePoolTimeoutsElRef {
        ContainerAzureNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureNodePoolAutoscalingEl {
    max_node_count: terrars::PrimField<f64>,
    min_node_count: terrars::PrimField<f64>,
}

impl ContainerAzureNodePoolAutoscalingEl { }

impl terrars::ToListMappable for ContainerAzureNodePoolAutoscalingEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolAutoscalingEl {
    #[doc = "Maximum number of nodes in the node pool. Must be >= min_node_count."]
    pub max_node_count: terrars::PrimField<f64>,
    #[doc = "Minimum number of nodes in the node pool. Must be >= 1 and <= max_node_count."]
    pub min_node_count: terrars::PrimField<f64>,
}

impl BuildContainerAzureNodePoolAutoscalingEl {
    pub fn build(self) -> ContainerAzureNodePoolAutoscalingEl {
        ContainerAzureNodePoolAutoscalingEl {
            max_node_count: self.max_node_count,
            min_node_count: self.min_node_count,
        }
    }
}

pub struct ContainerAzureNodePoolAutoscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolAutoscalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureNodePoolAutoscalingElRef {
        ContainerAzureNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolAutoscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes in the node pool. Must be >= min_node_count."]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes in the node pool. Must be >= 1 and <= max_node_count."]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureNodePoolConfigElProxyConfigEl {
    resource_group_id: terrars::PrimField<std::string::String>,
    secret_id: terrars::PrimField<std::string::String>,
}

impl ContainerAzureNodePoolConfigElProxyConfigEl { }

impl terrars::ToListMappable for ContainerAzureNodePoolConfigElProxyConfigEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolConfigElProxyConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolConfigElProxyConfigEl {
    #[doc =
        "The ARM ID the of the resource group containing proxy keyvault. Resource group ids are formatted as `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`"]
    pub resource_group_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The URL the of the proxy setting secret with its version. Secret ids are formatted as `https:<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`."]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureNodePoolConfigElProxyConfigEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigElProxyConfigEl {
        ContainerAzureNodePoolConfigElProxyConfigEl {
            resource_group_id: self.resource_group_id,
            secret_id: self.secret_id,
        }
    }
}

pub struct ContainerAzureNodePoolConfigElProxyConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolConfigElProxyConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAzureNodePoolConfigElProxyConfigElRef {
        ContainerAzureNodePoolConfigElProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolConfigElProxyConfigElRef {
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
pub struct ContainerAzureNodePoolConfigElRootVolumeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gib: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerAzureNodePoolConfigElRootVolumeEl {
    #[doc =
        "Set the field `size_gib`.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gib = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAzureNodePoolConfigElRootVolumeEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolConfigElRootVolumeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolConfigElRootVolumeEl {}

impl BuildContainerAzureNodePoolConfigElRootVolumeEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigElRootVolumeEl {
        ContainerAzureNodePoolConfigElRootVolumeEl { size_gib: core::default::Default::default() }
    }
}

pub struct ContainerAzureNodePoolConfigElRootVolumeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolConfigElRootVolumeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureNodePoolConfigElRootVolumeElRef {
        ContainerAzureNodePoolConfigElRootVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolConfigElRootVolumeElRef {
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
pub struct ContainerAzureNodePoolConfigElSshConfigEl {
    authorized_key: terrars::PrimField<std::string::String>,
}

impl ContainerAzureNodePoolConfigElSshConfigEl { }

impl terrars::ToListMappable for ContainerAzureNodePoolConfigElSshConfigEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolConfigElSshConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolConfigElSshConfigEl {
    #[doc =
        "The SSH public key data for VMs managed by Anthos. This accepts the authorized_keys file format used in OpenSSH according to the sshd(8) manual page."]
    pub authorized_key: terrars::PrimField<std::string::String>,
}

impl BuildContainerAzureNodePoolConfigElSshConfigEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigElSshConfigEl {
        ContainerAzureNodePoolConfigElSshConfigEl { authorized_key: self.authorized_key }
    }
}

pub struct ContainerAzureNodePoolConfigElSshConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolConfigElSshConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureNodePoolConfigElSshConfigElRef {
        ContainerAzureNodePoolConfigElSshConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolConfigElSshConfigElRef {
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
struct ContainerAzureNodePoolConfigElDynamic {
    proxy_config: std::option::Option<terrars::DynamicBlock<ContainerAzureNodePoolConfigElProxyConfigEl>>,
    root_volume: std::option::Option<terrars::DynamicBlock<ContainerAzureNodePoolConfigElRootVolumeEl>>,
    ssh_config: std::option::Option<terrars::DynamicBlock<ContainerAzureNodePoolConfigElSshConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerAzureNodePoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_config: std::option::Option<std::vec::Vec<ContainerAzureNodePoolConfigElProxyConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_volume: std::option::Option<std::vec::Vec<ContainerAzureNodePoolConfigElRootVolumeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_config: std::option::Option<std::vec::Vec<ContainerAzureNodePoolConfigElSshConfigEl>>,
    dynamic: ContainerAzureNodePoolConfigElDynamic,
}

impl ContainerAzureNodePoolConfigEl {
    #[doc =
        "Set the field `labels`.\nOptional. The initial labels assigned to nodes of this node pool. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nOptional. A set of tags to apply to all underlying Azure resources for this node pool. This currently only includes Virtual Machine Scale Sets. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_size`.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. See (/anthos/clusters/docs/azure/reference/supported-vms) for options. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn set_vm_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vm_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureNodePoolConfigElProxyConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureNodePoolConfigElRootVolumeEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAzureNodePoolConfigElSshConfigEl>>,
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

impl terrars::ToListMappable for ContainerAzureNodePoolConfigEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolConfigEl {}

impl BuildContainerAzureNodePoolConfigEl {
    pub fn build(self) -> ContainerAzureNodePoolConfigEl {
        ContainerAzureNodePoolConfigEl {
            labels: core::default::Default::default(),
            tags: core::default::Default::default(),
            vm_size: core::default::Default::default(),
            proxy_config: core::default::Default::default(),
            root_volume: core::default::Default::default(),
            ssh_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerAzureNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureNodePoolConfigElRef {
        ContainerAzureNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The initial labels assigned to nodes of this node pool. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional. A set of tags to apply to all underlying Azure resources for this node pool. This currently only includes Virtual Machine Scale Sets. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_size` after provisioning.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. See (/anthos/clusters/docs/azure/reference/supported-vms) for options. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn vm_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_size", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<ContainerAzureNodePoolConfigElProxyConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.base))
    }

    #[doc = "Get a reference to the value of field `root_volume` after provisioning.\n"]
    pub fn root_volume(&self) -> terrars::ListRef<ContainerAzureNodePoolConfigElRootVolumeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_config` after provisioning.\n"]
    pub fn ssh_config(&self) -> terrars::ListRef<ContainerAzureNodePoolConfigElSshConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureNodePoolManagementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerAzureNodePoolManagementEl {
    #[doc = "Set the field `auto_repair`.\nOptional. Whether or not the nodes will be automatically repaired."]
    pub fn set_auto_repair(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_repair = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAzureNodePoolManagementEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolManagementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolManagementEl {}

impl BuildContainerAzureNodePoolManagementEl {
    pub fn build(self) -> ContainerAzureNodePoolManagementEl {
        ContainerAzureNodePoolManagementEl { auto_repair: core::default::Default::default() }
    }
}

pub struct ContainerAzureNodePoolManagementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolManagementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureNodePoolManagementElRef {
        ContainerAzureNodePoolManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolManagementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_repair` after provisioning.\nOptional. Whether or not the nodes will be automatically repaired."]
    pub fn auto_repair(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureNodePoolMaxPodsConstraintEl {
    max_pods_per_node: terrars::PrimField<f64>,
}

impl ContainerAzureNodePoolMaxPodsConstraintEl { }

impl terrars::ToListMappable for ContainerAzureNodePoolMaxPodsConstraintEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolMaxPodsConstraintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolMaxPodsConstraintEl {
    #[doc = "The maximum number of pods to schedule on a single node."]
    pub max_pods_per_node: terrars::PrimField<f64>,
}

impl BuildContainerAzureNodePoolMaxPodsConstraintEl {
    pub fn build(self) -> ContainerAzureNodePoolMaxPodsConstraintEl {
        ContainerAzureNodePoolMaxPodsConstraintEl { max_pods_per_node: self.max_pods_per_node }
    }
}

pub struct ContainerAzureNodePoolMaxPodsConstraintElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolMaxPodsConstraintElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureNodePoolMaxPodsConstraintElRef {
        ContainerAzureNodePoolMaxPodsConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolMaxPodsConstraintElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods to schedule on a single node."]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAzureNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAzureNodePoolTimeoutsEl {
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

impl terrars::ToListMappable for ContainerAzureNodePoolTimeoutsEl {
    type O = terrars::BlockAssignable<ContainerAzureNodePoolTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAzureNodePoolTimeoutsEl {}

impl BuildContainerAzureNodePoolTimeoutsEl {
    pub fn build(self) -> ContainerAzureNodePoolTimeoutsEl {
        ContainerAzureNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ContainerAzureNodePoolTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAzureNodePoolTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAzureNodePoolTimeoutsElRef {
        ContainerAzureNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAzureNodePoolTimeoutsElRef {
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
struct ContainerAzureNodePoolDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<ContainerAzureNodePoolAutoscalingEl>>,
    config: std::option::Option<terrars::DynamicBlock<ContainerAzureNodePoolConfigEl>>,
    management: std::option::Option<terrars::DynamicBlock<ContainerAzureNodePoolManagementEl>>,
    max_pods_constraint: std::option::Option<terrars::DynamicBlock<ContainerAzureNodePoolMaxPodsConstraintEl>>,
}
