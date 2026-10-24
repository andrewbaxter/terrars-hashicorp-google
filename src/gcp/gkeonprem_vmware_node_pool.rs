use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeonpremVmwareNodePoolData {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    vmware_cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<GkeonpremVmwareNodePoolConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_autoscaling: std::option::Option<std::vec::Vec<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeonpremVmwareNodePoolTimeoutsEl>,
    dynamic: GkeonpremVmwareNodePoolDynamic,
}

struct GkeonpremVmwareNodePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeonpremVmwareNodePoolData>,
}

#[derive(Clone)]
pub struct GkeonpremVmwareNodePool(std::rc::Rc<GkeonpremVmwareNodePool_>);

impl GkeonpremVmwareNodePool {
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
        "Set the field `annotations`.\nAnnotations on the node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name for the node pool."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareNodePoolConfigEl>>,
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

    #[doc = "Set the field `node_pool_autoscaling`.\n"]
    pub fn set_node_pool_autoscaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool_autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool_autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeonpremVmwareNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time the cluster was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the node pool."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe vmware node pool name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_prem_version` after provisioning.\nAnthos version for the node pool. Defaults to the user cluster version."]
    pub fn on_prem_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_prem_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the node pool."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the node pool."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vmware_cluster` after provisioning.\nThe cluster this node pool belongs to."]
    pub fn vmware_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_autoscaling` after provisioning.\n"]
    pub fn node_pool_autoscaling(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolNodePoolAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareNodePoolTimeoutsElRef {
        GkeonpremVmwareNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeonpremVmwareNodePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeonpremVmwareNodePool { }

impl terrars::ToListMappable for GkeonpremVmwareNodePool {
    type O = terrars::ListRef<GkeonpremVmwareNodePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeonpremVmwareNodePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gkeonprem_vmware_node_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeonpremVmwareNodePool {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The vmware node pool name."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The cluster this node pool belongs to."]
    pub vmware_cluster: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareNodePool {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeonpremVmwareNodePool {
        let out = GkeonpremVmwareNodePool(std::rc::Rc::new(GkeonpremVmwareNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeonpremVmwareNodePoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                vmware_cluster: self.vmware_cluster,
                config: core::default::Default::default(),
                node_pool_autoscaling: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeonpremVmwareNodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeonpremVmwareNodePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the node Pool.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time the cluster was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the node pool."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe vmware node pool name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_prem_version` after provisioning.\nAnthos version for the node pool. Defaults to the user cluster version."]
    pub fn on_prem_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_prem_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the node pool."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the node pool."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vmware_cluster` after provisioning.\nThe cluster this node pool belongs to."]
    pub fn vmware_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_autoscaling` after provisioning.\n"]
    pub fn node_pool_autoscaling(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolNodePoolAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareNodePoolTimeoutsElRef {
        GkeonpremVmwareNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolStatusElConditionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transition_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareNodePoolStatusElConditionsEl {
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

impl terrars::ToListMappable for GkeonpremVmwareNodePoolStatusElConditionsEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolStatusElConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolStatusElConditionsEl {}

impl BuildGkeonpremVmwareNodePoolStatusElConditionsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolStatusElConditionsEl {
        GkeonpremVmwareNodePoolStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremVmwareNodePoolStatusElConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolStatusElConditionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareNodePoolStatusElConditionsElRef {
        GkeonpremVmwareNodePoolStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolStatusElConditionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct GkeonpremVmwareNodePoolStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<GkeonpremVmwareNodePoolStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareNodePoolStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremVmwareNodePoolStatusElConditionsEl>>,
    ) -> Self {
        self.conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareNodePoolStatusEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolStatusEl {}

impl BuildGkeonpremVmwareNodePoolStatusEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolStatusEl {
        GkeonpremVmwareNodePoolStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremVmwareNodePoolStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareNodePoolStatusElRef {
        GkeonpremVmwareNodePoolStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolStatusElConditionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {}

impl BuildGkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
        GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl {
            category: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
        GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl>>,
}

impl GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    #[doc = "Set the field `datastore`.\n"]
    pub fn set_datastore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datastore = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsEl>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolConfigElVsphereConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolConfigElVsphereConfigEl {}

impl BuildGkeonpremVmwareNodePoolConfigElVsphereConfigEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
        GkeonpremVmwareNodePoolConfigElVsphereConfigEl {
            datastore: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
        GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolConfigElVsphereConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datastore` after provisioning.\n"]
    pub fn datastore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolConfigElVsphereConfigElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolConfigElTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareNodePoolConfigElTaintsEl {
    #[doc =
        "Set the field `effect`.\nAvailable taint effects. Possible values: [\"EFFECT_UNSPECIFIED\", \"NO_SCHEDULE\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareNodePoolConfigElTaintsEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolConfigElTaintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolConfigElTaintsEl {
    #[doc = "Key associated with the effect."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Value associated with the effect."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareNodePoolConfigElTaintsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigElTaintsEl {
        GkeonpremVmwareNodePoolConfigElTaintsEl {
            effect: core::default::Default::default(),
            key: self.key,
            value: self.value,
        }
    }
}

pub struct GkeonpremVmwareNodePoolConfigElTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolConfigElTaintsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareNodePoolConfigElTaintsElRef {
        GkeonpremVmwareNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolConfigElTaintsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effect` after provisioning.\nAvailable taint effects. Possible values: [\"EFFECT_UNSPECIFIED\", \"NO_SCHEDULE\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey associated with the effect."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue associated with the effect."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareNodePoolConfigElDynamic {
    taints: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareNodePoolConfigElTaintsEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_load_balancer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    image_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std::option::Option<std::vec::Vec<GkeonpremVmwareNodePoolConfigElTaintsEl>>,
    dynamic: GkeonpremVmwareNodePoolConfigElDynamic,
}

impl GkeonpremVmwareNodePoolConfigEl {
    #[doc = "Set the field `boot_disk_size_gb`.\nVMware disk size to be used during creation."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpus`.\nThe number of CPUs for each node in the node pool."]
    pub fn set_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_load_balancer`.\nAllow node pool traffic to be load balanced. Only works for clusters with\nMetalLB load balancers."]
    pub fn set_enable_load_balancer(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_load_balancer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\nThe OS image name in vCenter, only valid when using Windows."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node.\nThese will added in addition to any default label(s) that\nKubernetes may apply to the node.\nIn case of conflict in label keys, the applied set may differ depending on\nthe Kubernetes version -- it's best to assume the behavior is undefined\nand conflicts should be avoided."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_mb`.\nThe megabytes of memory for each node in the node pool."]
    pub fn set_memory_mb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicas`.\nThe number of nodes in the node pool."]
    pub fn set_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareNodePoolConfigElTaintsEl>>,
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

impl terrars::ToListMappable for GkeonpremVmwareNodePoolConfigEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolConfigEl {
    #[doc =
        "The OS image to be used for each node in a node pool.\nCurrently 'cos', 'ubuntu', 'ubuntu_containerd' and 'windows' are supported."]
    pub image_type: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareNodePoolConfigEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolConfigEl {
        GkeonpremVmwareNodePoolConfigEl {
            boot_disk_size_gb: core::default::Default::default(),
            cpus: core::default::Default::default(),
            enable_load_balancer: core::default::Default::default(),
            image: core::default::Default::default(),
            image_type: self.image_type,
            labels: core::default::Default::default(),
            memory_mb: core::default::Default::default(),
            replicas: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremVmwareNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareNodePoolConfigElRef {
        GkeonpremVmwareNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nVMware disk size to be used during creation."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpus` after provisioning.\nThe number of CPUs for each node in the node pool."]
    pub fn cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_load_balancer` after provisioning.\nAllow node pool traffic to be load balanced. Only works for clusters with\nMetalLB load balancers."]
    pub fn enable_load_balancer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_load_balancer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe OS image name in vCenter, only valid when using Windows."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_type` after provisioning.\nThe OS image to be used for each node in a node pool.\nCurrently 'cos', 'ubuntu', 'ubuntu_containerd' and 'windows' are supported."]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node.\nThese will added in addition to any default label(s) that\nKubernetes may apply to the node.\nIn case of conflict in label keys, the applied set may differ depending on\nthe Kubernetes version -- it's best to assume the behavior is undefined\nand conflicts should be avoided."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_mb` after provisioning.\nThe megabytes of memory for each node in the node pool."]
    pub fn memory_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_mb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replicas` after provisioning.\nThe number of nodes in the node pool."]
    pub fn replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vsphere_config` after provisioning.\nSpecifies the vSphere config for node pool."]
    pub fn vsphere_config(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolConfigElVsphereConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vsphere_config", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> terrars::ListRef<GkeonpremVmwareNodePoolConfigElTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    max_replicas: terrars::PrimField<f64>,
    min_replicas: terrars::PrimField<f64>,
}

impl GkeonpremVmwareNodePoolNodePoolAutoscalingEl { }

impl terrars::ToListMappable for GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    #[doc = "Maximum number of replicas in the NodePool."]
    pub max_replicas: terrars::PrimField<f64>,
    #[doc = "Minimum number of replicas in the NodePool."]
    pub min_replicas: terrars::PrimField<f64>,
}

impl BuildGkeonpremVmwareNodePoolNodePoolAutoscalingEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
        GkeonpremVmwareNodePoolNodePoolAutoscalingEl {
            max_replicas: self.max_replicas,
            min_replicas: self.min_replicas,
        }
    }
}

pub struct GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
        GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolNodePoolAutoscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_replicas` after provisioning.\nMaximum number of replicas in the NodePool."]
    pub fn max_replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replicas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replicas` after provisioning.\nMinimum number of replicas in the NodePool."]
    pub fn min_replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareNodePoolTimeoutsEl {
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

impl terrars::ToListMappable for GkeonpremVmwareNodePoolTimeoutsEl {
    type O = terrars::BlockAssignable<GkeonpremVmwareNodePoolTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareNodePoolTimeoutsEl {}

impl BuildGkeonpremVmwareNodePoolTimeoutsEl {
    pub fn build(self) -> GkeonpremVmwareNodePoolTimeoutsEl {
        GkeonpremVmwareNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremVmwareNodePoolTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareNodePoolTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareNodePoolTimeoutsElRef {
        GkeonpremVmwareNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareNodePoolTimeoutsElRef {
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
struct GkeonpremVmwareNodePoolDynamic {
    config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareNodePoolConfigEl>>,
    node_pool_autoscaling: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareNodePoolNodePoolAutoscalingEl>>,
}
