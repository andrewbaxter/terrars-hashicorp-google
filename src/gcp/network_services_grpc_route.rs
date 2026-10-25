#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesGrpcRouteData_ {
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
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    meshes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesGrpcRoute_Timeouts>,
    dynamic: NetworkServicesGrpcRouteDynamic,
}

struct NetworkServicesGrpcRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesGrpcRouteData_>,
}

#[derive(Clone)]
pub struct NetworkServicesGrpcRoute(std::rc::Rc<NetworkServicesGrpcRoute_>);

impl NetworkServicesGrpcRoute {
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
        "Set the field `gateways`.\nList of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway."]
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
        "Set the field `labels`.\nSet of label tags associated with the GrpcRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nLocation (region) of the GRPCRoute resource to be created. Only the value 'global' is currently allowed; defaults to 'global' if omitted."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `meshes`.\nList of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesGrpcRoute_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the GrpcRoute was created in UTC."]
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
        "Get a reference to the value of field `gateways` after provisioning.\nList of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway."]
    pub fn gateways(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateways", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nRequired. Service hostnames with an optional port for which this route describes traffic."]
    pub fn hostnames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the GrpcRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the GRPCRoute resource to be created. Only the value 'global' is currently allowed; defaults to 'global' if omitted."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meshes` after provisioning.\nList of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh."]
    pub fn meshes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meshes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the GrpcRoute resource."]
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
        "Get a reference to the value of field `update_time` after provisioning.\nTime the GrpcRoute was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGrpcRoute_TimeoutsRef {
        NetworkServicesGrpcRoute_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkServicesGrpcRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesGrpcRoute { }

impl terrars::ToListMappable for NetworkServicesGrpcRoute {
    type O = terrars::ListRef<NetworkServicesGrpcRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesGrpcRoute_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_grpc_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesGrpcRoute {
    pub tf_id: std::string::String,
    #[doc = "Required. Service hostnames with an optional port for which this route describes traffic."]
    pub hostnames: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Name of the GrpcRoute resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesGrpcRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesGrpcRoute {
        let out = NetworkServicesGrpcRoute(std::rc::Rc::new(NetworkServicesGrpcRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesGrpcRouteData_ {
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
                location: core::default::Default::default(),
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

pub struct NetworkServicesGrpcRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesGrpcRouteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the GrpcRoute was created in UTC."]
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
        "Get a reference to the value of field `gateways` after provisioning.\nList of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway."]
    pub fn gateways(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateways", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nRequired. Service hostnames with an optional port for which this route describes traffic."]
    pub fn hostnames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the GrpcRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the GRPCRoute resource to be created. Only the value 'global' is currently allowed; defaults to 'global' if omitted."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meshes` after provisioning.\nList of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh."]
    pub fn meshes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meshes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the GrpcRoute resource."]
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
        "Get a reference to the value of field `update_time` after provisioning.\nTime the GrpcRoute was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGrpcRoute_TimeoutsRef {
        NetworkServicesGrpcRoute_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesGrpcRouteDynamic {
    rules: std::option::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules>>,
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Action_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesGrpcRoute_Rules_Action_Destinations {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Action_Destinations {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Action_Destinations {}

impl BuildNetworkServicesGrpcRoute_Rules_Action_Destinations {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Action_Destinations {
        NetworkServicesGrpcRoute_Rules_Action_Destinations {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_Action_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_Action_DestinationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesGrpcRoute_Rules_Action_DestinationsRef {
        NetworkServicesGrpcRoute_Rules_Action_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_Action_DestinationsRef {
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
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort {
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

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort {}

impl BuildNetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort {
        NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
        NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_AbortRef {
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
pub struct NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay {
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

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay {}

impl BuildNetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay {
        NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay {
            fixed_delay: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
        NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_DelayRef {
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
struct NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyDynamic {
    abort: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort>>,
    delay: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay>>,
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay>>,
    dynamic: NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyDynamic,
}

impl NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Abort>,
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
                        terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_Delay>,
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

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy {}

impl BuildNetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy {
        NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyRef {
        NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Action_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesGrpcRoute_Rules_Action_RetryPolicy {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number of retries."]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry policy applies. Possible values: [\"connect-failure\", \"refused-stream\", \"cancelled\", \"deadline-exceeded\", \"resource-exhausted\", \"unavailable\"]"]
    pub fn set_retry_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.retry_conditions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Action_RetryPolicy {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Action_RetryPolicy {}

impl BuildNetworkServicesGrpcRoute_Rules_Action_RetryPolicy {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Action_RetryPolicy {
        NetworkServicesGrpcRoute_Rules_Action_RetryPolicy {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_Action_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_Action_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesGrpcRoute_Rules_Action_RetryPolicyRef {
        NetworkServicesGrpcRoute_Rules_Action_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_Action_RetryPolicyRef {
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
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry policy applies. Possible values: [\"connect-failure\", \"refused-stream\", \"cancelled\", \"deadline-exceeded\", \"resource-exhausted\", \"unavailable\"]"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesGrpcRoute_Rules_ActionDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Action_Destinations>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Action_RetryPolicy>>,
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Action_Destinations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Action_RetryPolicy>>,
    dynamic: NetworkServicesGrpcRoute_Rules_ActionDynamic,
}

impl NetworkServicesGrpcRoute_Rules_Action {
    #[doc = "Set the field `timeout`.\nSpecifies the timeout for selected route."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_Destinations>>,
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
                    ::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicy>>,
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

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action_RetryPolicy>>,
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
}

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Action {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Action {}

impl BuildNetworkServicesGrpcRoute_Rules_Action {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Action {
        NetworkServicesGrpcRoute_Rules_Action {
            timeout: core::default::Default::default(),
            destinations: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesGrpcRoute_Rules_ActionRef {
        NetworkServicesGrpcRoute_Rules_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for selected route."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_Action_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_Action_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_Action_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Matches_Headers {
    key: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl NetworkServicesGrpcRoute_Rules_Matches_Headers {
    #[doc =
        "Set the field `type_`.\nThe type of match. Default value: \"EXACT\" Possible values: [\"TYPE_UNSPECIFIED\", \"EXACT\", \"REGULAR_EXPRESSION\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Matches_Headers {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Matches_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Matches_Headers {
    #[doc = "Required. The key of the header."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Required. The value of the header."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesGrpcRoute_Rules_Matches_Headers {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Matches_Headers {
        NetworkServicesGrpcRoute_Rules_Matches_Headers {
            key: self.key,
            type_: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_Matches_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_Matches_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesGrpcRoute_Rules_Matches_HeadersRef {
        NetworkServicesGrpcRoute_Rules_Matches_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_Matches_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nRequired. The key of the header."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of match. Default value: \"EXACT\" Possible values: [\"TYPE_UNSPECIFIED\", \"EXACT\", \"REGULAR_EXPRESSION\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nRequired. The value of the header."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Matches_Method {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_sensitive: std::option::Option<terrars::PrimField<bool>>,
    grpc_method: terrars::PrimField<std::string::String>,
    grpc_service: terrars::PrimField<std::string::String>,
}

impl NetworkServicesGrpcRoute_Rules_Matches_Method {
    #[doc = "Set the field `case_sensitive`.\nSpecifies that matches are case sensitive. The default value is true."]
    pub fn set_case_sensitive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.case_sensitive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Matches_Method {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Matches_Method>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Matches_Method {
    #[doc = "Required. Name of the method to match against."]
    pub grpc_method: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the service to match against."]
    pub grpc_service: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesGrpcRoute_Rules_Matches_Method {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Matches_Method {
        NetworkServicesGrpcRoute_Rules_Matches_Method {
            case_sensitive: core::default::Default::default(),
            grpc_method: self.grpc_method,
            grpc_service: self.grpc_service,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_Matches_MethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_Matches_MethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesGrpcRoute_Rules_Matches_MethodRef {
        NetworkServicesGrpcRoute_Rules_Matches_MethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_Matches_MethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `case_sensitive` after provisioning.\nSpecifies that matches are case sensitive. The default value is true."]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }

    #[doc =
        "Get a reference to the value of field `grpc_method` after provisioning.\nRequired. Name of the method to match against."]
    pub fn grpc_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `grpc_service` after provisioning.\nRequired. Name of the service to match against."]
    pub fn grpc_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesGrpcRoute_Rules_MatchesDynamic {
    headers: std::option::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Matches_Headers>>,
    method: std::option::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Matches_Method>>,
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules_Matches {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Matches_Headers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Matches_Method>>,
    dynamic: NetworkServicesGrpcRoute_Rules_MatchesDynamic,
}

impl NetworkServicesGrpcRoute_Rules_Matches {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Matches_Headers>>,
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

    #[doc = "Set the field `method`.\n"]
    pub fn set_method(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Matches_Method>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules_Matches {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Matches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules_Matches {}

impl BuildNetworkServicesGrpcRoute_Rules_Matches {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules_Matches {
        NetworkServicesGrpcRoute_Rules_Matches {
            headers: core::default::Default::default(),
            method: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_Rules_MatchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_Rules_MatchesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesGrpcRoute_Rules_MatchesRef {
        NetworkServicesGrpcRoute_Rules_MatchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_Rules_MatchesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_Matches_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_Matches_MethodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesGrpcRoute_RulesDynamic {
    action: std::option::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Action>>,
    matches: std::option::Option<terrars::DynamicBlock<NetworkServicesGrpcRoute_Rules_Matches>>,
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matches: std::option::Option<std::vec::Vec<NetworkServicesGrpcRoute_Rules_Matches>>,
    dynamic: NetworkServicesGrpcRoute_RulesDynamic,
}

impl NetworkServicesGrpcRoute_Rules {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Action>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules_Matches>>,
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

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Rules {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Rules {}

impl BuildNetworkServicesGrpcRoute_Rules {
    pub fn build(self) -> NetworkServicesGrpcRoute_Rules {
        NetworkServicesGrpcRoute_Rules {
            action: core::default::Default::default(),
            matches: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesGrpcRoute_RulesRef {
        NetworkServicesGrpcRoute_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `matches` after provisioning.\n"]
    pub fn matches(&self) -> terrars::ListRef<NetworkServicesGrpcRoute_Rules_MatchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesGrpcRoute_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesGrpcRoute_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesGrpcRoute_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesGrpcRoute_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesGrpcRoute_Timeouts {}

impl BuildNetworkServicesGrpcRoute_Timeouts {
    pub fn build(self) -> NetworkServicesGrpcRoute_Timeouts {
        NetworkServicesGrpcRoute_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesGrpcRoute_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesGrpcRoute_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesGrpcRoute_TimeoutsRef {
        NetworkServicesGrpcRoute_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesGrpcRoute_TimeoutsRef {
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
