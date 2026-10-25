#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerAwsNodePoolData_ {
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
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_id: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std::option::Option<std::vec::Vec<ContainerAwsNodePool_KubeletConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_constraint: std::option::Option<std::vec::Vec<ContainerAwsNodePool_MaxPodsConstraint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerAwsNodePool_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_settings: std::option::Option<std::vec::Vec<ContainerAwsNodePool_UpdateSettings>>,
    dynamic: ContainerAwsNodePoolDynamic,
}

struct ContainerAwsNodePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerAwsNodePoolData_>,
}

#[derive(Clone)]
pub struct ContainerAwsNodePool(std::rc::Rc<ContainerAwsNodePool_>);

impl ContainerAwsNodePool {
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
        "Set the field `annotations`.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Autoscaling>>,
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
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Config>>) -> Self {
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

    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_KubeletConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kubelet_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kubelet_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Management>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_MaxPodsConstraint>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerAwsNodePool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_settings`.\n"]
    pub fn set_update_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_UpdateSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe awsCluster for the resource"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this node pool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the node pool."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The lifecycle state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_id` after provisioning.\nThe subnet where the node pool node run."]
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
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerAwsNodePool_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ContainerAwsNodePool_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<ContainerAwsNodePool_KubeletConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerAwsNodePool_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> terrars::ListRef<ContainerAwsNodePool_MaxPodsConstraintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_pods_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAwsNodePool_TimeoutsRef {
        ContainerAwsNodePool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_settings` after provisioning.\n"]
    pub fn update_settings(&self) -> terrars::ListRef<ContainerAwsNodePool_UpdateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_settings", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerAwsNodePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerAwsNodePool { }

impl terrars::ToListMappable for ContainerAwsNodePool {
    type O = terrars::ListRef<ContainerAwsNodePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerAwsNodePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_aws_node_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerAwsNodePool {
    pub tf_id: std::string::String,
    #[doc = "The awsCluster for the resource"]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of this resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The subnet where the node pool node run."]
    pub subnet_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsNodePool {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerAwsNodePool {
        let out = ContainerAwsNodePool(std::rc::Rc::new(ContainerAwsNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerAwsNodePoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                cluster: self.cluster,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                subnet_id: self.subnet_id,
                version: self.version,
                autoscaling: core::default::Default::default(),
                config: core::default::Default::default(),
                kubelet_config: core::default::Default::default(),
                management: core::default::Default::default(),
                max_pods_constraint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                update_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerAwsNodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerAwsNodePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe awsCluster for the resource"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this node pool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the node pool."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The lifecycle state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_id` after provisioning.\nThe subnet where the node pool node run."]
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
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerAwsNodePool_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ContainerAwsNodePool_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<ContainerAwsNodePool_KubeletConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerAwsNodePool_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_pods_constraint` after provisioning.\n"]
    pub fn max_pods_constraint(&self) -> terrars::ListRef<ContainerAwsNodePool_MaxPodsConstraintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_pods_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAwsNodePool_TimeoutsRef {
        ContainerAwsNodePool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_settings` after provisioning.\n"]
    pub fn update_settings(&self) -> terrars::ListRef<ContainerAwsNodePool_UpdateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ContainerAwsNodePoolDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Autoscaling>>,
    config: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Config>>,
    kubelet_config: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_KubeletConfig>>,
    management: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Management>>,
    max_pods_constraint: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_MaxPodsConstraint>>,
    update_settings: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_UpdateSettings>>,
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_Autoscaling {
    max_node_count: terrars::PrimField<f64>,
    min_node_count: terrars::PrimField<f64>,
}

impl ContainerAwsNodePool_Autoscaling { }

impl terrars::ToListMappable for ContainerAwsNodePool_Autoscaling {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Autoscaling {
    #[doc = "Maximum number of nodes in the NodePool. Must be >= min_node_count."]
    pub max_node_count: terrars::PrimField<f64>,
    #[doc = "Minimum number of nodes in the NodePool. Must be >= 1 and <= max_node_count."]
    pub min_node_count: terrars::PrimField<f64>,
}

impl BuildContainerAwsNodePool_Autoscaling {
    pub fn build(self) -> ContainerAwsNodePool_Autoscaling {
        ContainerAwsNodePool_Autoscaling {
            max_node_count: self.max_node_count,
            min_node_count: self.min_node_count,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_AutoscalingRef {
        ContainerAwsNodePool_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_AutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes in the NodePool. Must be >= min_node_count."]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes in the NodePool. Must be >= 1 and <= max_node_count."]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_Config_AutoscalingMetricsCollection {
    granularity: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ContainerAwsNodePool_Config_AutoscalingMetricsCollection {
    #[doc =
        "Set the field `metrics`.\nThe metrics to enable. For a list of valid metrics, see https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_EnableMetricsCollection.html. If you specify granularity and don't specify any metrics, all metrics are enabled."]
    pub fn set_metrics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metrics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsNodePool_Config_AutoscalingMetricsCollection {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Config_AutoscalingMetricsCollection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Config_AutoscalingMetricsCollection {
    #[doc =
        "The frequency at which EC2 Auto Scaling sends aggregated data to AWS CloudWatch. The only valid value is \"1Minute\"."]
    pub granularity: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsNodePool_Config_AutoscalingMetricsCollection {
    pub fn build(self) -> ContainerAwsNodePool_Config_AutoscalingMetricsCollection {
        ContainerAwsNodePool_Config_AutoscalingMetricsCollection {
            granularity: self.granularity,
            metrics: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_Config_AutoscalingMetricsCollectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_Config_AutoscalingMetricsCollectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsNodePool_Config_AutoscalingMetricsCollectionRef {
        ContainerAwsNodePool_Config_AutoscalingMetricsCollectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_Config_AutoscalingMetricsCollectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `granularity` after provisioning.\nThe frequency at which EC2 Auto Scaling sends aggregated data to AWS CloudWatch. The only valid value is \"1Minute\"."]
    pub fn granularity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.granularity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metrics` after provisioning.\nThe metrics to enable. For a list of valid metrics, see https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_EnableMetricsCollection.html. If you specify granularity and don't specify any metrics, all metrics are enabled."]
    pub fn metrics(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_Config_ConfigEncryption {
    kms_key_arn: terrars::PrimField<std::string::String>,
}

impl ContainerAwsNodePool_Config_ConfigEncryption { }

impl terrars::ToListMappable for ContainerAwsNodePool_Config_ConfigEncryption {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Config_ConfigEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Config_ConfigEncryption {
    #[doc = "The ARN of the AWS KMS key used to encrypt node pool configuration."]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsNodePool_Config_ConfigEncryption {
    pub fn build(self) -> ContainerAwsNodePool_Config_ConfigEncryption {
        ContainerAwsNodePool_Config_ConfigEncryption { kms_key_arn: self.kms_key_arn }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_Config_ConfigEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_Config_ConfigEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsNodePool_Config_ConfigEncryptionRef {
        ContainerAwsNodePool_Config_ConfigEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_Config_ConfigEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_arn` after provisioning.\nThe ARN of the AWS KMS key used to encrypt node pool configuration."]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_Config_ProxyConfig {
    secret_arn: terrars::PrimField<std::string::String>,
    secret_version: terrars::PrimField<std::string::String>,
}

impl ContainerAwsNodePool_Config_ProxyConfig { }

impl terrars::ToListMappable for ContainerAwsNodePool_Config_ProxyConfig {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Config_ProxyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Config_ProxyConfig {
    #[doc = "The ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub secret_arn: terrars::PrimField<std::string::String>,
    #[doc = "The version string of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsNodePool_Config_ProxyConfig {
    pub fn build(self) -> ContainerAwsNodePool_Config_ProxyConfig {
        ContainerAwsNodePool_Config_ProxyConfig {
            secret_arn: self.secret_arn,
            secret_version: self.secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_Config_ProxyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_Config_ProxyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_Config_ProxyConfigRef {
        ContainerAwsNodePool_Config_ProxyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_Config_ProxyConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerAwsNodePool_Config_RootVolume {
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

impl ContainerAwsNodePool_Config_RootVolume {
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

impl terrars::ToListMappable for ContainerAwsNodePool_Config_RootVolume {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Config_RootVolume>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Config_RootVolume {}

impl BuildContainerAwsNodePool_Config_RootVolume {
    pub fn build(self) -> ContainerAwsNodePool_Config_RootVolume {
        ContainerAwsNodePool_Config_RootVolume {
            iops: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            size_gib: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_Config_RootVolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_Config_RootVolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_Config_RootVolumeRef {
        ContainerAwsNodePool_Config_RootVolumeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_Config_RootVolumeRef {
    #[allow(dead_code)]
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
pub struct ContainerAwsNodePool_Config_SshConfig {
    ec2_key_pair: terrars::PrimField<std::string::String>,
}

impl ContainerAwsNodePool_Config_SshConfig { }

impl terrars::ToListMappable for ContainerAwsNodePool_Config_SshConfig {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Config_SshConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Config_SshConfig {
    #[doc = "The name of the EC2 key pair used to login into cluster machines."]
    pub ec2_key_pair: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsNodePool_Config_SshConfig {
    pub fn build(self) -> ContainerAwsNodePool_Config_SshConfig {
        ContainerAwsNodePool_Config_SshConfig { ec2_key_pair: self.ec2_key_pair }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_Config_SshConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_Config_SshConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_Config_SshConfigRef {
        ContainerAwsNodePool_Config_SshConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_Config_SshConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ec2_key_pair` after provisioning.\nThe name of the EC2 key pair used to login into cluster machines."]
    pub fn ec2_key_pair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_key_pair", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_Config_Taints {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerAwsNodePool_Config_Taints { }

impl terrars::ToListMappable for ContainerAwsNodePool_Config_Taints {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Config_Taints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Config_Taints {
    #[doc = "The taint effect. Possible values: EFFECT_UNSPECIFIED, NO_SCHEDULE, PREFER_NO_SCHEDULE, NO_EXECUTE"]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = "Key for the taint."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Value for the taint."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsNodePool_Config_Taints {
    pub fn build(self) -> ContainerAwsNodePool_Config_Taints {
        ContainerAwsNodePool_Config_Taints {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_Config_TaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_Config_TaintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_Config_TaintsRef {
        ContainerAwsNodePool_Config_TaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_Config_TaintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effect` after provisioning.\nThe taint effect. Possible values: EFFECT_UNSPECIFIED, NO_SCHEDULE, PREFER_NO_SCHEDULE, NO_EXECUTE"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for the taint."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue for the taint."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAwsNodePool_ConfigDynamic {
    autoscaling_metrics_collection: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerAwsNodePool_Config_AutoscalingMetricsCollection>>,
    config_encryption: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Config_ConfigEncryption>>,
    proxy_config: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Config_ProxyConfig>>,
    root_volume: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Config_RootVolume>>,
    ssh_config: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Config_SshConfig>>,
    taints: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_Config_Taints>>,
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_Config {
    iam_instance_profile: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_metrics_collection: std
    ::option
    ::Option<std::vec::Vec<ContainerAwsNodePool_Config_AutoscalingMetricsCollection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_encryption: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Config_ConfigEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_config: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Config_ProxyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_volume: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Config_RootVolume>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_config: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Config_SshConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std::option::Option<std::vec::Vec<ContainerAwsNodePool_Config_Taints>>,
    dynamic: ContainerAwsNodePool_ConfigDynamic,
}

impl ContainerAwsNodePool_Config {
    #[doc =
        "Set the field `instance_type`.\nOptional. The AWS instance type. When unspecified, it defaults to `m5.large`."]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

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
        "Set the field `security_group_ids`.\nOptional. The IDs of additional security groups to add to nodes in this pool. The manager will automatically create security groups with minimum rules needed for a functioning cluster."]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nOptional. Key/value metadata to assign to each underlying AWS resource. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_metrics_collection`.\n"]
    pub fn set_autoscaling_metrics_collection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerAwsNodePool_Config_AutoscalingMetricsCollection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_metrics_collection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_metrics_collection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config_encryption`.\n"]
    pub fn set_config_encryption(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Config_ConfigEncryption>>,
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

    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Config_ProxyConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Config_RootVolume>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Config_SshConfig>>,
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

    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_Config_Taints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.taints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.taints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAwsNodePool_Config {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Config {
    #[doc = "The name of the AWS IAM role assigned to nodes in the pool."]
    pub iam_instance_profile: terrars::PrimField<std::string::String>,
}

impl BuildContainerAwsNodePool_Config {
    pub fn build(self) -> ContainerAwsNodePool_Config {
        ContainerAwsNodePool_Config {
            iam_instance_profile: self.iam_instance_profile,
            instance_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            tags: core::default::Default::default(),
            autoscaling_metrics_collection: core::default::Default::default(),
            config_encryption: core::default::Default::default(),
            proxy_config: core::default::Default::default(),
            root_volume: core::default::Default::default(),
            ssh_config: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_ConfigRef {
        ContainerAwsNodePool_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `iam_instance_profile` after provisioning.\nThe name of the AWS IAM role assigned to nodes in the pool."]
    pub fn iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nOptional. The AWS instance type. When unspecified, it defaults to `m5.large`."]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The initial labels assigned to nodes of this node pool. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_group_ids` after provisioning.\nOptional. The IDs of additional security groups to add to nodes in this pool. The manager will automatically create security groups with minimum rules needed for a functioning cluster."]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional. Key/value metadata to assign to each underlying AWS resource. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_metrics_collection` after provisioning.\n"]
    pub fn autoscaling_metrics_collection(
        &self,
    ) -> terrars::ListRef<ContainerAwsNodePool_Config_AutoscalingMetricsCollectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_metrics_collection", self.base))
    }

    #[doc = "Get a reference to the value of field `config_encryption` after provisioning.\n"]
    pub fn config_encryption(&self) -> terrars::ListRef<ContainerAwsNodePool_Config_ConfigEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<ContainerAwsNodePool_Config_ProxyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.base))
    }

    #[doc = "Get a reference to the value of field `root_volume` after provisioning.\n"]
    pub fn root_volume(&self) -> terrars::ListRef<ContainerAwsNodePool_Config_RootVolumeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_config` after provisioning.\n"]
    pub fn ssh_config(&self) -> terrars::ListRef<ContainerAwsNodePool_Config_SshConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_config", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> terrars::ListRef<ContainerAwsNodePool_Config_TaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_KubeletConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_manager_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_pids_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerAwsNodePool_KubeletConfig {
    #[doc = "Set the field `cpu_cfs_quota`.\nWhether or not to enable CPU CFS quota. Defaults to true."]
    pub fn set_cpu_cfs_quota(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cpu_cfs_quota_period`.\nOptional. The CPU CFS quota period to use for the node. Defaults to \"100ms\"."]
    pub fn set_cpu_cfs_quota_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cpu_cfs_quota_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manager_policy`.\nThe CpuManagerPolicy to use for the node. Defaults to \"none\"."]
    pub fn set_cpu_manager_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cpu_manager_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pod_pids_limit`.\nOptional. The maximum number of PIDs in each pod running on the node. The limit scales automatically based on underlying machine size if left unset."]
    pub fn set_pod_pids_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pod_pids_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsNodePool_KubeletConfig {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_KubeletConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_KubeletConfig {}

impl BuildContainerAwsNodePool_KubeletConfig {
    pub fn build(self) -> ContainerAwsNodePool_KubeletConfig {
        ContainerAwsNodePool_KubeletConfig {
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_KubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_KubeletConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_KubeletConfigRef {
        ContainerAwsNodePool_KubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_KubeletConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\nWhether or not to enable CPU CFS quota. Defaults to true."]
    pub fn cpu_cfs_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\nOptional. The CPU CFS quota period to use for the node. Defaults to \"100ms\"."]
    pub fn cpu_cfs_quota_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_manager_policy` after provisioning.\nThe CpuManagerPolicy to use for the node. Defaults to \"none\"."]
    pub fn cpu_manager_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_manager_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_pids_limit` after provisioning.\nOptional. The maximum number of PIDs in each pod running on the node. The limit scales automatically based on underlying machine size if left unset."]
    pub fn pod_pids_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_pids_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerAwsNodePool_Management {
    #[doc = "Set the field `auto_repair`.\nOptional. Whether or not the nodes will be automatically repaired."]
    pub fn set_auto_repair(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_repair = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsNodePool_Management {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Management {}

impl BuildContainerAwsNodePool_Management {
    pub fn build(self) -> ContainerAwsNodePool_Management {
        ContainerAwsNodePool_Management { auto_repair: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_ManagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_ManagementRef {
        ContainerAwsNodePool_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_ManagementRef {
    #[allow(dead_code)]
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
pub struct ContainerAwsNodePool_MaxPodsConstraint {
    max_pods_per_node: terrars::PrimField<f64>,
}

impl ContainerAwsNodePool_MaxPodsConstraint { }

impl terrars::ToListMappable for ContainerAwsNodePool_MaxPodsConstraint {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_MaxPodsConstraint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_MaxPodsConstraint {
    #[doc = "The maximum number of pods to schedule on a single node."]
    pub max_pods_per_node: terrars::PrimField<f64>,
}

impl BuildContainerAwsNodePool_MaxPodsConstraint {
    pub fn build(self) -> ContainerAwsNodePool_MaxPodsConstraint {
        ContainerAwsNodePool_MaxPodsConstraint { max_pods_per_node: self.max_pods_per_node }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_MaxPodsConstraintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_MaxPodsConstraintRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_MaxPodsConstraintRef {
        ContainerAwsNodePool_MaxPodsConstraintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_MaxPodsConstraintRef {
    #[allow(dead_code)]
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
pub struct ContainerAwsNodePool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAwsNodePool_Timeouts {
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

impl terrars::ToListMappable for ContainerAwsNodePool_Timeouts {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_Timeouts {}

impl BuildContainerAwsNodePool_Timeouts {
    pub fn build(self) -> ContainerAwsNodePool_Timeouts {
        ContainerAwsNodePool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_TimeoutsRef {
        ContainerAwsNodePool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_TimeoutsRef {
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
pub struct ContainerAwsNodePool_UpdateSettings_SurgeSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerAwsNodePool_UpdateSettings_SurgeSettings {
    #[doc =
        "Set the field `max_surge`.\nOptional. The maximum number of nodes that can be created beyond the current size of the node pool during the update process."]
    pub fn set_max_surge(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable`.\nOptional. The maximum number of nodes that can be simultaneously unavailable during the update process. A node is considered unavailable if its status is not Ready."]
    pub fn set_max_unavailable(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAwsNodePool_UpdateSettings_SurgeSettings {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_UpdateSettings_SurgeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_UpdateSettings_SurgeSettings {}

impl BuildContainerAwsNodePool_UpdateSettings_SurgeSettings {
    pub fn build(self) -> ContainerAwsNodePool_UpdateSettings_SurgeSettings {
        ContainerAwsNodePool_UpdateSettings_SurgeSettings {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_UpdateSettings_SurgeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_UpdateSettings_SurgeSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAwsNodePool_UpdateSettings_SurgeSettingsRef {
        ContainerAwsNodePool_UpdateSettings_SurgeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_UpdateSettings_SurgeSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_surge` after provisioning.\nOptional. The maximum number of nodes that can be created beyond the current size of the node pool during the update process."]
    pub fn max_surge(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable` after provisioning.\nOptional. The maximum number of nodes that can be simultaneously unavailable during the update process. A node is considered unavailable if its status is not Ready."]
    pub fn max_unavailable(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAwsNodePool_UpdateSettingsDynamic {
    surge_settings: std::option::Option<terrars::DynamicBlock<ContainerAwsNodePool_UpdateSettings_SurgeSettings>>,
}

#[derive(Serialize)]
pub struct ContainerAwsNodePool_UpdateSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surge_settings: std::option::Option<std::vec::Vec<ContainerAwsNodePool_UpdateSettings_SurgeSettings>>,
    dynamic: ContainerAwsNodePool_UpdateSettingsDynamic,
}

impl ContainerAwsNodePool_UpdateSettings {
    #[doc = "Set the field `surge_settings`.\n"]
    pub fn set_surge_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAwsNodePool_UpdateSettings_SurgeSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.surge_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.surge_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAwsNodePool_UpdateSettings {
    type O = terrars::BlockAssignable<ContainerAwsNodePool_UpdateSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAwsNodePool_UpdateSettings {}

impl BuildContainerAwsNodePool_UpdateSettings {
    pub fn build(self) -> ContainerAwsNodePool_UpdateSettings {
        ContainerAwsNodePool_UpdateSettings {
            surge_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAwsNodePool_UpdateSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAwsNodePool_UpdateSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAwsNodePool_UpdateSettingsRef {
        ContainerAwsNodePool_UpdateSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAwsNodePool_UpdateSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `surge_settings` after provisioning.\n"]
    pub fn surge_settings(&self) -> terrars::ListRef<ContainerAwsNodePool_UpdateSettings_SurgeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.surge_settings", self.base))
    }
}
