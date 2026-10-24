use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MemcacheInstanceData {
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
    authorized_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memcache_version: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    node_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<MemcacheInstanceMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memcache_parameters: std::option::Option<std::vec::Vec<MemcacheInstanceMemcacheParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<MemcacheInstanceNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MemcacheInstanceTimeoutsEl>,
    dynamic: MemcacheInstanceDynamic,
}

struct MemcacheInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MemcacheInstanceData>,
}

#[derive(Clone)]
pub struct MemcacheInstance(std::rc::Rc<MemcacheInstance_>);

impl MemcacheInstance {
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
        "Set the field `authorized_network`.\nThe full name of the GCE network to connect the instance to.  If not provided,\n'default' will be used."]
    pub fn set_authorized_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorized_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nA user-visible name for the instance."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memcache_version`.\nThe major version of Memcached software. If not provided, latest supported version will be used.\nCurrently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically\ndetermined by our system based on the latest supported minor version. Default value: \"MEMCACHE_1_5\" Possible values: [\"MEMCACHE_1_5\", \"MEMCACHE_1_6_15\"]"]
    pub fn set_memcache_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().memcache_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe region of the Memcache instance. If it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zones`.\nZones where memcache nodes should be provisioned.  If not\nprovided, all zones will be used."]
    pub fn set_zones(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemcacheInstanceMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memcache_parameters`.\n"]
    pub fn set_memcache_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemcacheInstanceMemcacheParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().memcache_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.memcache_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemcacheInstanceNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MemcacheInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the GCE network to connect the instance to.  If not provided,\n'default' will be used."]
    pub fn authorized_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoint` after provisioning.\nEndpoint for Discovery API"]
    pub fn discovery_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-visible name for the instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nOutput only. Published maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<MemcacheInstanceMaintenanceScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memcache_full_version` after provisioning.\nThe full version of memcached server running on this instance."]
    pub fn memcache_full_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memcache_full_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memcache_nodes` after provisioning.\nAdditional information about the instance state, if available."]
    pub fn memcache_nodes(&self) -> terrars::ListRef<MemcacheInstanceMemcacheNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memcache_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memcache_version` after provisioning.\nThe major version of Memcached software. If not provided, latest supported version will be used.\nCurrently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically\ndetermined by our system based on the latest supported minor version. Default value: \"MEMCACHE_1_5\" Possible values: [\"MEMCACHE_1_5\", \"MEMCACHE_1_6_15\"]"]
    pub fn memcache_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memcache_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nNumber of nodes in the memcache instance."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the Memcache instance. If it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zones` after provisioning.\nZones where memcache nodes should be provisioned.  If not\nprovided, all zones will be used."]
    pub fn zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<MemcacheInstanceMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memcache_parameters` after provisioning.\n"]
    pub fn memcache_parameters(&self) -> terrars::ListRef<MemcacheInstanceMemcacheParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memcache_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<MemcacheInstanceNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemcacheInstanceTimeoutsElRef {
        MemcacheInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MemcacheInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MemcacheInstance { }

impl terrars::ToListMappable for MemcacheInstance {
    type O = terrars::ListRef<MemcacheInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MemcacheInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_memcache_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMemcacheInstance {
    pub tf_id: std::string::String,
    #[doc = "The resource name of the instance."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Number of nodes in the memcache instance."]
    pub node_count: terrars::PrimField<f64>,
}

impl BuildMemcacheInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> MemcacheInstance {
        let out = MemcacheInstance(std::rc::Rc::new(MemcacheInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MemcacheInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorized_network: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                memcache_version: core::default::Default::default(),
                name: self.name,
                node_count: self.node_count,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                zones: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                memcache_parameters: core::default::Default::default(),
                node_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MemcacheInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MemcacheInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the GCE network to connect the instance to.  If not provided,\n'default' will be used."]
    pub fn authorized_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoint` after provisioning.\nEndpoint for Discovery API"]
    pub fn discovery_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-visible name for the instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nOutput only. Published maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<MemcacheInstanceMaintenanceScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memcache_full_version` after provisioning.\nThe full version of memcached server running on this instance."]
    pub fn memcache_full_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memcache_full_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memcache_nodes` after provisioning.\nAdditional information about the instance state, if available."]
    pub fn memcache_nodes(&self) -> terrars::ListRef<MemcacheInstanceMemcacheNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memcache_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memcache_version` after provisioning.\nThe major version of Memcached software. If not provided, latest supported version will be used.\nCurrently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically\ndetermined by our system based on the latest supported minor version. Default value: \"MEMCACHE_1_5\" Possible values: [\"MEMCACHE_1_5\", \"MEMCACHE_1_6_15\"]"]
    pub fn memcache_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memcache_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nNumber of nodes in the memcache instance."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the Memcache instance. If it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zones` after provisioning.\nZones where memcache nodes should be provisioned.  If not\nprovided, all zones will be used."]
    pub fn zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<MemcacheInstanceMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memcache_parameters` after provisioning.\n"]
    pub fn memcache_parameters(&self) -> terrars::ListRef<MemcacheInstanceMemcacheParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memcache_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<MemcacheInstanceNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemcacheInstanceTimeoutsElRef {
        MemcacheInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MemcacheInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_deadline_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemcacheInstanceMaintenanceScheduleEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_deadline_time`.\n"]
    pub fn set_schedule_deadline_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_deadline_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemcacheInstanceMaintenanceScheduleEl {
    type O = terrars::BlockAssignable<MemcacheInstanceMaintenanceScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceMaintenanceScheduleEl {}

impl BuildMemcacheInstanceMaintenanceScheduleEl {
    pub fn build(self) -> MemcacheInstanceMaintenanceScheduleEl {
        MemcacheInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct MemcacheInstanceMaintenanceScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceMaintenanceScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemcacheInstanceMaintenanceScheduleElRef {
        MemcacheInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceMaintenanceScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_deadline_time` after provisioning.\n"]
    pub fn schedule_deadline_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_deadline_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct MemcacheInstanceMemcacheNodesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemcacheInstanceMemcacheNodesEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_id`.\n"]
    pub fn set_node_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemcacheInstanceMemcacheNodesEl {
    type O = terrars::BlockAssignable<MemcacheInstanceMemcacheNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceMemcacheNodesEl {}

impl BuildMemcacheInstanceMemcacheNodesEl {
    pub fn build(self) -> MemcacheInstanceMemcacheNodesEl {
        MemcacheInstanceMemcacheNodesEl {
            host: core::default::Default::default(),
            node_id: core::default::Default::default(),
            port: core::default::Default::default(),
            state: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct MemcacheInstanceMemcacheNodesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceMemcacheNodesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemcacheInstanceMemcacheNodesElRef {
        MemcacheInstanceMemcacheNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceMemcacheNodesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[doc =
        "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O = terrars::BlockAssignable<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}

impl BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic {
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
}

#[derive(Serialize)]
pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    day: terrars::PrimField<std::string::String>,
    duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<std::vec::Vec<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
    dynamic: MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic,
}

impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = terrars::BlockAssignable<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc =
        "Required. The day of week that maintenance updates occur.\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The length of the maintenance window, ranging from 3 hours to 8 hours.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub duration: terrars::PrimField<std::string::String>,
}

impl BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: self.day,
            duration: self.duration,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nRequired. The day of week that maintenance updates occur.\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nRequired. The length of the maintenance window, ranging from 3 hours to 8 hours.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemcacheInstanceMaintenancePolicyElDynamic {
    weekly_maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}

#[derive(Serialize)]
pub struct MemcacheInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window: std
    ::option
    ::Option<std::vec::Vec<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    dynamic: MemcacheInstanceMaintenancePolicyElDynamic,
}

impl MemcacheInstanceMaintenancePolicyEl {
    #[doc =
        "Set the field `description`.\nOptional. Description of what this policy is for.\nCreate/Update methods return INVALID_ARGUMENT if the\nlength is greater than 512."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly_maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MemcacheInstanceMaintenancePolicyEl {
    type O = terrars::BlockAssignable<MemcacheInstanceMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceMaintenancePolicyEl {}

impl BuildMemcacheInstanceMaintenancePolicyEl {
    pub fn build(self) -> MemcacheInstanceMaintenancePolicyEl {
        MemcacheInstanceMaintenancePolicyEl {
            description: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MemcacheInstanceMaintenancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceMaintenancePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemcacheInstanceMaintenancePolicyElRef {
        MemcacheInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the policy was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of what this policy is for.\nCreate/Update methods return INVALID_ARGUMENT if the\nlength is greater than 512."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the policy was updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> terrars::ListRef<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct MemcacheInstanceMemcacheParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl MemcacheInstanceMemcacheParametersEl {
    #[doc = "Set the field `params`.\nUser-defined set of parameters to use in the memcache process."]
    pub fn set_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.params = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemcacheInstanceMemcacheParametersEl {
    type O = terrars::BlockAssignable<MemcacheInstanceMemcacheParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceMemcacheParametersEl {}

impl BuildMemcacheInstanceMemcacheParametersEl {
    pub fn build(self) -> MemcacheInstanceMemcacheParametersEl {
        MemcacheInstanceMemcacheParametersEl { params: core::default::Default::default() }
    }
}

pub struct MemcacheInstanceMemcacheParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceMemcacheParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemcacheInstanceMemcacheParametersElRef {
        MemcacheInstanceMemcacheParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceMemcacheParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThis is a unique ID associated with this set of parameters."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nUser-defined set of parameters to use in the memcache process."]
    pub fn params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
}

#[derive(Serialize)]
pub struct MemcacheInstanceNodeConfigEl {
    cpu_count: terrars::PrimField<f64>,
    memory_size_mb: terrars::PrimField<f64>,
}

impl MemcacheInstanceNodeConfigEl { }

impl terrars::ToListMappable for MemcacheInstanceNodeConfigEl {
    type O = terrars::BlockAssignable<MemcacheInstanceNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceNodeConfigEl {
    #[doc = "Number of CPUs per node."]
    pub cpu_count: terrars::PrimField<f64>,
    #[doc = "Memory size in Mebibytes for each memcache node."]
    pub memory_size_mb: terrars::PrimField<f64>,
}

impl BuildMemcacheInstanceNodeConfigEl {
    pub fn build(self) -> MemcacheInstanceNodeConfigEl {
        MemcacheInstanceNodeConfigEl {
            cpu_count: self.cpu_count,
            memory_size_mb: self.memory_size_mb,
        }
    }
}

pub struct MemcacheInstanceNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemcacheInstanceNodeConfigElRef {
        MemcacheInstanceNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceNodeConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\nNumber of CPUs per node."]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_mb` after provisioning.\nMemory size in Mebibytes for each memcache node."]
    pub fn memory_size_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_mb", self.base))
    }
}

#[derive(Serialize)]
pub struct MemcacheInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemcacheInstanceTimeoutsEl {
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

impl terrars::ToListMappable for MemcacheInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<MemcacheInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemcacheInstanceTimeoutsEl {}

impl BuildMemcacheInstanceTimeoutsEl {
    pub fn build(self) -> MemcacheInstanceTimeoutsEl {
        MemcacheInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MemcacheInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemcacheInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemcacheInstanceTimeoutsElRef {
        MemcacheInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemcacheInstanceTimeoutsElRef {
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
struct MemcacheInstanceDynamic {
    maintenance_policy: std::option::Option<terrars::DynamicBlock<MemcacheInstanceMaintenancePolicyEl>>,
    memcache_parameters: std::option::Option<terrars::DynamicBlock<MemcacheInstanceMemcacheParametersEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<MemcacheInstanceNodeConfigEl>>,
}
