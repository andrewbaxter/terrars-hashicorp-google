#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesHttpRouteData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateways: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    hostnames: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    meshes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesHttpRoute_Timeouts>,
    dynamic: NetworkServicesHttpRouteDynamic,
}

struct NetworkServicesHttpRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesHttpRouteData_>,
}

#[derive(Clone)]
pub struct NetworkServicesHttpRoute(std::rc::Rc<NetworkServicesHttpRoute_>);

impl NetworkServicesHttpRoute {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gateways`.\nGateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn set_gateways(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().gateways = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nSet of label tags associated with the HttpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `meshes`.\nMeshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>.\nThe attached Mesh should be of a type SIDECAR."]
    pub fn set_meshes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().meshes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesHttpRoute_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the HttpRoute was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn gateways(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateways", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nSet of hosts that should match against the HTTP host header to select a HttpRoute to process the request."]
    pub fn hostnames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the HttpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>.\nThe attached Mesh should be of a type SIDECAR."]
    pub fn meshes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meshes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the HttpRoute resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the HttpRoute was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<NetworkServicesHttpRoute_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesHttpRoute_TimeoutsRef {
        NetworkServicesHttpRoute_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkServicesHttpRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesHttpRoute { }

impl terrars::ToListMappable for NetworkServicesHttpRoute {
    type O = terrars::ListRef<NetworkServicesHttpRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesHttpRoute_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_http_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesHttpRoute {
    pub tf_id: std::string::String,
    #[doc =
        "Set of hosts that should match against the HTTP host header to select a HttpRoute to process the request."]
    pub hostnames: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Name of the HttpRoute resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesHttpRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesHttpRoute {
        let out = NetworkServicesHttpRoute(std::rc::Rc::new(NetworkServicesHttpRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesHttpRouteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                gateways: core::default::Default::default(),
                hostnames: self.hostnames,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                meshes: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesHttpRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesHttpRouteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the HttpRoute was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn gateways(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateways", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nSet of hosts that should match against the HTTP host header to select a HttpRoute to process the request."]
    pub fn hostnames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the HttpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>.\nThe attached Mesh should be of a type SIDECAR."]
    pub fn meshes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meshes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the HttpRoute resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the HttpRoute was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<NetworkServicesHttpRoute_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesHttpRoute_TimeoutsRef {
        NetworkServicesHttpRoute_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesHttpRouteDynamic {
    rules: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules>>,
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_CorsPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origin_regexes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesHttpRoute_Rules_Action_CorsPolicy {
    #[doc =
        "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\nSpecifies the content for Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\nSpecifies the content for Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins."]
    pub fn set_allow_origin_regexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origin_regexes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests."]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nIf true, the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\nSpecifies the content for Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\nSpecifies how long result of a preflight request can be cached in seconds."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_CorsPolicy {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_CorsPolicy {}

impl BuildNetworkServicesHttpRoute_Rules_Action_CorsPolicy {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_CorsPolicy {
        NetworkServicesHttpRoute_Rules_Action_CorsPolicy {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origin_regexes: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_CorsPolicyRef {
        NetworkServicesHttpRoute_Rules_Action_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_CorsPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins."]
    pub fn allow_origin_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests."]
    pub fn allow_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf true, the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long result of a preflight request can be cached in seconds."]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesHttpRoute_Rules_Action_Destinations {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_Destinations {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_Destinations {}

impl BuildNetworkServicesHttpRoute_Rules_Action_Destinations {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_Destinations {
        NetworkServicesHttpRoute_Rules_Action_Destinations {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_DestinationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_DestinationsRef {
        NetworkServicesHttpRoute_Rules_Action_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe URL of a BackendService to route traffic to."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort {
    #[doc = "Set the field `http_status`.\nThe HTTP status code used to abort the request."]
    pub fn set_http_status(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percentage`.\nThe percentage of traffic which will be aborted."]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort {}

impl BuildNetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort {
        NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
        NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request."]
    pub fn http_status(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic which will be aborted."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay {
    #[doc = "Set the field `fixed_delay`.\nSpecify a fixed delay before forwarding the request."]
    pub fn set_fixed_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percentage`.\nThe percentage of traffic on which delay will be injected."]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay {}

impl BuildNetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay {
        NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay {
            fixed_delay: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
        NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed_delay` after provisioning.\nSpecify a fixed delay before forwarding the request."]
    pub fn fixed_delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic on which delay will be injected."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyDynamic {
    abort: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort>>,
    delay: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay>>,
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay>>,
    dynamic: NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyDynamic,
}

impl NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Abort>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.abort = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_Delay>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy {}

impl BuildNetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy {
        NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyRef {
        NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_Redirect {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_redirect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_redirect: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_query: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesHttpRoute_Rules_Action_Redirect {
    #[doc =
        "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn set_host_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https."]
    pub fn set_https_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.https_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was supplied in the request. pathRedirect can not be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn set_path_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_redirect`.\nThe port that will be used in the redirected request instead of the one that was supplied in the request."]
    pub fn set_port_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_rewrite`.\nIndicates that during redirection, the matched prefix (or path) should be swapped with this value."]
    pub fn set_prefix_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_code`.\nThe HTTP Status code to use for the redirect."]
    pub fn set_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request."]
    pub fn set_strip_query(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_Redirect {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_Redirect {}

impl BuildNetworkServicesHttpRoute_Rules_Action_Redirect {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_Redirect {
        NetworkServicesHttpRoute_Rules_Action_Redirect {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            port_redirect: core::default::Default::default(),
            prefix_rewrite: core::default::Default::default(),
            response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_RedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_RedirectRef {
        NetworkServicesHttpRoute_Rules_Action_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_RedirectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn host_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https."]
    pub fn https_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was supplied in the request. pathRedirect can not be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn path_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_redirect` after provisioning.\nThe port that will be used in the redirected request instead of the one that was supplied in the request."]
    pub fn port_redirect(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_rewrite` after provisioning.\nIndicates that during redirection, the matched prefix (or path) should be swapped with this value."]
    pub fn prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_code` after provisioning.\nThe HTTP Status code to use for the redirect."]
    pub fn response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier {
    #[doc =
        "Set the field `add`.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_add(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.add = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remove`.\nRemove headers (matching by header names) specified in the list."]
    pub fn set_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `set`.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_set(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.set = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier {}

impl BuildNetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier {
        NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier {
            add: core::default::Default::default(),
            remove: core::default::Default::default(),
            set: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifierRef {
        NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `add` after provisioning.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn add(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.add", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remove` after provisioning.\nRemove headers (matching by header names) specified in the list."]
    pub fn remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `set` after provisioning.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.set", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination {}

impl BuildNetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination {
        NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_DestinationRef {
        NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe URL of a BackendService to route traffic to."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination>>,
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination>>,
    dynamic: NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyDynamic,
}

impl NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_Destination>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy {}

impl BuildNetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy {
        NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy {
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyRef {
        NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier {
    #[doc =
        "Set the field `add`.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_add(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.add = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remove`.\nRemove headers (matching by header names) specified in the list."]
    pub fn set_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `set`.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_set(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.set = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier {}

impl BuildNetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier {
        NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier {
            add: core::default::Default::default(),
            remove: core::default::Default::default(),
            set: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifierRef {
        NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `add` after provisioning.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn add(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.add", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remove` after provisioning.\nRemove headers (matching by header names) specified in the list."]
    pub fn remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `set` after provisioning.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.set", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesHttpRoute_Rules_Action_RetryPolicy {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number of retries."]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `per_try_timeout`.\nSpecifies a non-zero timeout per retry attempt. A duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_per_try_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.per_try_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry policy applies."]
    pub fn set_retry_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.retry_conditions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_RetryPolicy {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_RetryPolicy {}

impl BuildNetworkServicesHttpRoute_Rules_Action_RetryPolicy {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_RetryPolicy {
        NetworkServicesHttpRoute_Rules_Action_RetryPolicy {
            num_retries: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_RetryPolicyRef {
        NetworkServicesHttpRoute_Rules_Action_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number of retries."]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `per_try_timeout` after provisioning.\nSpecifies a non-zero timeout per retry attempt. A duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn per_try_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry policy applies."]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesHttpRoute_Rules_Action_UrlRewrite {
    #[doc =
        "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected destination, the requests host header is replaced by this value."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected destination, the matching portion of the requests path is replaced by this value."]
    pub fn set_path_prefix_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_prefix_rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action_UrlRewrite {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action_UrlRewrite {}

impl BuildNetworkServicesHttpRoute_Rules_Action_UrlRewrite {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action_UrlRewrite {
        NetworkServicesHttpRoute_Rules_Action_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Action_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Action_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Action_UrlRewriteRef {
        NetworkServicesHttpRoute_Rules_Action_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Action_UrlRewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected destination, the requests host header is replaced by this value."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected destination, the matching portion of the requests path is replaced by this value."]
    pub fn path_prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesHttpRoute_Rules_ActionDynamic {
    cors_policy: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_CorsPolicy>>,
    destinations: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_Destinations>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy>>,
    redirect: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_Redirect>>,
    request_header_modifier: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy>>,
    response_header_modifier: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_RetryPolicy>>,
    url_rewrite: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action_UrlRewrite>>,
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_Destinations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_Redirect>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header_modifier: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header_modifier: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action_UrlRewrite>>,
    dynamic: NetworkServicesHttpRoute_Rules_ActionDynamic,
}

impl NetworkServicesHttpRoute_Rules_Action {
    #[doc = "Set the field `timeout`.\nSpecifies the timeout for selected route."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_CorsPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fault_injection_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_Redirect>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_header_modifier`.\n"]
    pub fn set_request_header_modifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifier>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_header_modifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header_modifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_mirror_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_header_modifier`.\n"]
    pub fn set_response_header_modifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifier>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_header_modifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header_modifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_RetryPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action_UrlRewrite>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Action {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Action {}

impl BuildNetworkServicesHttpRoute_Rules_Action {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Action {
        NetworkServicesHttpRoute_Rules_Action {
            timeout: core::default::Default::default(),
            cors_policy: core::default::Default::default(),
            destinations: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            redirect: core::default::Default::default(),
            request_header_modifier: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            response_header_modifier: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesHttpRoute_Rules_ActionRef {
        NetworkServicesHttpRoute_Rules_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for selected route."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header_modifier` after provisioning.\n"]
    pub fn request_header_modifier(
        &self,
    ) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_RequestHeaderModifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header_modifier", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header_modifier` after provisioning.\n"]
    pub fn response_header_modifier(
        &self,
    ) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_ResponseHeaderModifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header_modifier", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Action_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch {
    end: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch { }

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch {
    #[doc = "End of the range (exclusive)."]
    pub end: terrars::PrimField<f64>,
    #[doc = "Start of the range (inclusive)."]
    pub start: terrars::PrimField<f64>,
}

impl BuildNetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch {
        NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch {
            end: self.end,
            start: self.start,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatchRef {
        NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\nEnd of the range (exclusive)."]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\nStart of the range (inclusive)."]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesHttpRoute_Rules_Matches_HeadersDynamic {
    range_match: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch>>,
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Matches_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_match: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch>>,
    dynamic: NetworkServicesHttpRoute_Rules_Matches_HeadersDynamic,
}

impl NetworkServicesHttpRoute_Rules_Matches_Headers {
    #[doc = "Set the field `exact_match`.\nThe value of the header should match exactly the content of exactMatch."]
    pub fn set_exact_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\nThe name of the HTTP header to match against."]
    pub fn set_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_match`.\nIf specified, the match result will be inverted before checking. Default value is set to false."]
    pub fn set_invert_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_match`.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn set_prefix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `present_match`.\nA header with headerName must exist. The match takes place whether or not the header has a value."]
    pub fn set_present_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.present_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `regex_match`.\nThe value of the header must match the regular expression specified in regexMatch."]
    pub fn set_regex_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix_match`.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn set_suffix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_match`.\n"]
    pub fn set_range_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Matches_Headers {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Matches_Headers {}

impl BuildNetworkServicesHttpRoute_Rules_Matches_Headers {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Matches_Headers {
        NetworkServicesHttpRoute_Rules_Matches_Headers {
            exact_match: core::default::Default::default(),
            header: core::default::Default::default(),
            invert_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            suffix_match: core::default::Default::default(),
            range_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Matches_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Matches_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Matches_HeadersRef {
        NetworkServicesHttpRoute_Rules_Matches_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Matches_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nThe value of the header should match exactly the content of exactMatch."]
    pub fn exact_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header` after provisioning.\nThe name of the HTTP header to match against."]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_match` after provisioning.\nIf specified, the match result will be inverted before checking. Default value is set to false."]
    pub fn invert_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present_match` after provisioning.\nA header with headerName must exist. The match takes place whether or not the header has a value."]
    pub fn present_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `regex_match` after provisioning.\nThe value of the header must match the regular expression specified in regexMatch."]
    pub fn regex_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix_match` after provisioning.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn suffix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix_match", self.base))
    }

    #[doc = "Get a reference to the value of field `range_match` after provisioning.\n"]
    pub fn range_match(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Matches_Headers_RangeMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_match", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Matches_QueryParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesHttpRoute_Rules_Matches_QueryParameters {
    #[doc =
        "Set the field `exact_match`.\nThe value of the query parameter must exactly match the contents of exactMatch."]
    pub fn set_exact_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `present_match`.\nSpecifies that the QueryParameterMatcher matches if request contains query parameter, irrespective of whether the parameter has a value or not."]
    pub fn set_present_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.present_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_parameter`.\nThe name of the query parameter to match."]
    pub fn set_query_parameter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `regex_match`.\nThe value of the query parameter must match the regular expression specified by regexMatch.For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn set_regex_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_match = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Matches_QueryParameters {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches_QueryParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Matches_QueryParameters {}

impl BuildNetworkServicesHttpRoute_Rules_Matches_QueryParameters {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Matches_QueryParameters {
        NetworkServicesHttpRoute_Rules_Matches_QueryParameters {
            exact_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            regex_match: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_Matches_QueryParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_Matches_QueryParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesHttpRoute_Rules_Matches_QueryParametersRef {
        NetworkServicesHttpRoute_Rules_Matches_QueryParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_Matches_QueryParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nThe value of the query parameter must exactly match the contents of exactMatch."]
    pub fn exact_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present_match` after provisioning.\nSpecifies that the QueryParameterMatcher matches if request contains query parameter, irrespective of whether the parameter has a value or not."]
    pub fn present_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_parameter` after provisioning.\nThe name of the query parameter to match."]
    pub fn query_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_parameter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `regex_match` after provisioning.\nThe value of the query parameter must match the regular expression specified by regexMatch.For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn regex_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesHttpRoute_Rules_MatchesDynamic {
    headers: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Matches_Headers>>,
    query_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Matches_QueryParameters>>,
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules_Matches {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_path_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Matches_Headers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameters: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Matches_QueryParameters>>,
    dynamic: NetworkServicesHttpRoute_Rules_MatchesDynamic,
}

impl NetworkServicesHttpRoute_Rules_Matches {
    #[doc = "Set the field `full_path_match`.\nThe HTTP request path value should exactly match this value."]
    pub fn set_full_path_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.full_path_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nSpecifies if prefixMatch and fullPathMatch matches are case sensitive. The default value is false."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_match`.\nThe HTTP request path value must begin with specified prefixMatch. prefixMatch must begin with a /."]
    pub fn set_prefix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `regex_match`.\nThe HTTP request path value must satisfy the regular expression specified by regexMatch after removing any query parameters and anchor supplied with the original URL. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn set_regex_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches_Headers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_parameters`.\n"]
    pub fn set_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches_QueryParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules_Matches {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules_Matches {}

impl BuildNetworkServicesHttpRoute_Rules_Matches {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules_Matches {
        NetworkServicesHttpRoute_Rules_Matches {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            headers: core::default::Default::default(),
            query_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_Rules_MatchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_Rules_MatchesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesHttpRoute_Rules_MatchesRef {
        NetworkServicesHttpRoute_Rules_MatchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_Rules_MatchesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `full_path_match` after provisioning.\nThe HTTP request path value should exactly match this value."]
    pub fn full_path_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_path_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies if prefixMatch and fullPathMatch matches are case sensitive. The default value is false."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nThe HTTP request path value must begin with specified prefixMatch. prefixMatch must begin with a /."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `regex_match` after provisioning.\nThe HTTP request path value must satisfy the regular expression specified by regexMatch after removing any query parameters and anchor supplied with the original URL. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn regex_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Matches_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameters` after provisioning.\n"]
    pub fn query_parameters(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_Matches_QueryParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesHttpRoute_RulesDynamic {
    action: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Action>>,
    matches: std::option::Option<terrars::DynamicBlock<NetworkServicesHttpRoute_Rules_Matches>>,
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matches: std::option::Option<std::vec::Vec<NetworkServicesHttpRoute_Rules_Matches>>,
    dynamic: NetworkServicesHttpRoute_RulesDynamic,
}

impl NetworkServicesHttpRoute_Rules {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `matches`.\n"]
    pub fn set_matches(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesHttpRoute_Rules_Matches>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.matches = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.matches = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesHttpRoute_Rules {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Rules {}

impl BuildNetworkServicesHttpRoute_Rules {
    pub fn build(self) -> NetworkServicesHttpRoute_Rules {
        NetworkServicesHttpRoute_Rules {
            action: core::default::Default::default(),
            matches: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesHttpRoute_RulesRef {
        NetworkServicesHttpRoute_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `matches` after provisioning.\n"]
    pub fn matches(&self) -> terrars::ListRef<NetworkServicesHttpRoute_Rules_MatchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesHttpRoute_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesHttpRoute_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesHttpRoute_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesHttpRoute_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesHttpRoute_Timeouts {}

impl BuildNetworkServicesHttpRoute_Timeouts {
    pub fn build(self) -> NetworkServicesHttpRoute_Timeouts {
        NetworkServicesHttpRoute_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesHttpRoute_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesHttpRoute_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesHttpRoute_TimeoutsRef {
        NetworkServicesHttpRoute_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesHttpRoute_TimeoutsRef {
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
