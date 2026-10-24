use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkConnectivitySpokeData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hub: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_interconnect_attachments: std
    ::option
    ::Option<std::vec::Vec<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_router_appliance_instances: std
    ::option
    ::Option<std::vec::Vec<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_vpc_network: std::option::Option<std::vec::Vec<NetworkConnectivitySpokeLinkedVpcNetworkEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_vpn_tunnels: std::option::Option<std::vec::Vec<NetworkConnectivitySpokeLinkedVpnTunnelsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkConnectivitySpokeTimeoutsEl>,
    dynamic: NetworkConnectivitySpokeDynamic,
}

struct NetworkConnectivitySpoke_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkConnectivitySpokeData>,
}

#[derive(Clone)]
pub struct NetworkConnectivitySpoke(std::rc::Rc<NetworkConnectivitySpoke_>);

impl NetworkConnectivitySpoke {
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

    #[doc = "Set the field `description`.\nAn optional description of the spoke."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_interconnect_attachments`.\n"]
    pub fn set_linked_interconnect_attachments(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_interconnect_attachments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_interconnect_attachments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linked_router_appliance_instances`.\n"]
    pub fn set_linked_router_appliance_instances(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_router_appliance_instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_router_appliance_instances = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linked_vpc_network`.\n"]
    pub fn set_linked_vpc_network(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpokeLinkedVpcNetworkEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_vpc_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_vpc_network = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linked_vpn_tunnels`.\n"]
    pub fn set_linked_vpn_tunnels(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpokeLinkedVpnTunnelsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_vpn_tunnels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_vpn_tunnels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkConnectivitySpokeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time the spoke was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the spoke."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hub` after provisioning.\nImmutable. The URI of the hub that this spoke is attached to."]
    pub fn hub(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hub", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The name of the spoke. Spoke names must be unique."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current lifecycle state of this spoke. Possible values: STATE_UNSPECIFIED, CREATING, ACTIVE, DELETING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unique_id` after provisioning.\nOutput only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different unique_id."]
    pub fn unique_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time the spoke was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_interconnect_attachments` after provisioning.\n"]
    pub fn linked_interconnect_attachments(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_interconnect_attachments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_router_appliance_instances` after provisioning.\n"]
    pub fn linked_router_appliance_instances(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_router_appliance_instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_vpc_network` after provisioning.\n"]
    pub fn linked_vpc_network(&self) -> terrars::ListRef<NetworkConnectivitySpokeLinkedVpcNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpc_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_vpn_tunnels` after provisioning.\n"]
    pub fn linked_vpn_tunnels(&self) -> terrars::ListRef<NetworkConnectivitySpokeLinkedVpnTunnelsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpn_tunnels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivitySpokeTimeoutsElRef {
        NetworkConnectivitySpokeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkConnectivitySpoke {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkConnectivitySpoke { }

impl terrars::ToListMappable for NetworkConnectivitySpoke {
    type O = terrars::ListRef<NetworkConnectivitySpokeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkConnectivitySpoke_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_connectivity_spoke".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkConnectivitySpoke {
    pub tf_id: std::string::String,
    #[doc = "Immutable. The URI of the hub that this spoke is attached to."]
    pub hub: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Immutable. The name of the spoke. Spoke names must be unique."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivitySpoke {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkConnectivitySpoke {
        let out = NetworkConnectivitySpoke(std::rc::Rc::new(NetworkConnectivitySpoke_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkConnectivitySpokeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                hub: self.hub,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                linked_interconnect_attachments: core::default::Default::default(),
                linked_router_appliance_instances: core::default::Default::default(),
                linked_vpc_network: core::default::Default::default(),
                linked_vpn_tunnels: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkConnectivitySpokeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpokeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkConnectivitySpokeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time the spoke was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the spoke."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hub` after provisioning.\nImmutable. The URI of the hub that this spoke is attached to."]
    pub fn hub(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hub", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The name of the spoke. Spoke names must be unique."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current lifecycle state of this spoke. Possible values: STATE_UNSPECIFIED, CREATING, ACTIVE, DELETING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unique_id` after provisioning.\nOutput only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different unique_id."]
    pub fn unique_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time the spoke was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_interconnect_attachments` after provisioning.\n"]
    pub fn linked_interconnect_attachments(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_interconnect_attachments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_router_appliance_instances` after provisioning.\n"]
    pub fn linked_router_appliance_instances(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_router_appliance_instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_vpc_network` after provisioning.\n"]
    pub fn linked_vpc_network(&self) -> terrars::ListRef<NetworkConnectivitySpokeLinkedVpcNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpc_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_vpn_tunnels` after provisioning.\n"]
    pub fn linked_vpn_tunnels(&self) -> terrars::ListRef<NetworkConnectivitySpokeLinkedVpnTunnelsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpn_tunnels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivitySpokeTimeoutsElRef {
        NetworkConnectivitySpokeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    site_to_site_data_transfer: terrars::PrimField<bool>,
    uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl { }

impl terrars::ToListMappable for NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    type O = terrars::BlockAssignable<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    #[doc =
        "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: terrars::PrimField<bool>,
    #[doc = "The URIs of linked interconnect attachment resources"]
    pub uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
        NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            uris: self.uris,
        }
    }
}

pub struct NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
        NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `site_to_site_data_transfer` after provisioning.\nA value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub fn site_to_site_data_transfer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_to_site_data_transfer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uris` after provisioning.\nThe URIs of linked interconnect attachment resources"]
    pub fn uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_machine: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    #[doc = "Set the field `ip_address`.\nThe IP address on the VM to use for peering."]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_machine`.\nThe URI of the virtual machine resource"]
    pub fn set_virtual_machine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_machine = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    type O = terrars::BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {}

impl BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
            ip_address: core::default::Default::default(),
            virtual_machine: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address on the VM to use for peering."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_machine` after provisioning.\nThe URI of the virtual machine resource"]
    pub fn virtual_machine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_machine", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElDynamic {
    instances: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>>,
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    site_to_site_data_transfer: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<std::vec::Vec<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>>,
    dynamic: NetworkConnectivitySpokeLinkedRouterApplianceInstancesElDynamic,
}

impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instances = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    type O = terrars::BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    #[doc =
        "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: terrars::PrimField<bool>,
}

impl BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            instances: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `site_to_site_data_transfer` after provisioning.\nA value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub fn site_to_site_data_transfer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_to_site_data_transfer", self.base))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedVpcNetworkEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl NetworkConnectivitySpokeLinkedVpcNetworkEl {
    #[doc = "Set the field `exclude_export_ranges`.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn set_exclude_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_export_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpokeLinkedVpcNetworkEl {
    type O = terrars::BlockAssignable<NetworkConnectivitySpokeLinkedVpcNetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpokeLinkedVpcNetworkEl {
    #[doc = "The URI of the VPC network resource."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivitySpokeLinkedVpcNetworkEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedVpcNetworkEl {
        NetworkConnectivitySpokeLinkedVpcNetworkEl {
            exclude_export_ranges: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct NetworkConnectivitySpokeLinkedVpcNetworkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpokeLinkedVpcNetworkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpokeLinkedVpcNetworkElRef {
        NetworkConnectivitySpokeLinkedVpcNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpokeLinkedVpcNetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn exclude_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_export_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the VPC network resource."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedVpnTunnelsEl {
    site_to_site_data_transfer: terrars::PrimField<bool>,
    uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpokeLinkedVpnTunnelsEl { }

impl terrars::ToListMappable for NetworkConnectivitySpokeLinkedVpnTunnelsEl {
    type O = terrars::BlockAssignable<NetworkConnectivitySpokeLinkedVpnTunnelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpokeLinkedVpnTunnelsEl {
    #[doc =
        "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: terrars::PrimField<bool>,
    #[doc = "The URIs of linked VPN tunnel resources."]
    pub uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkConnectivitySpokeLinkedVpnTunnelsEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedVpnTunnelsEl {
        NetworkConnectivitySpokeLinkedVpnTunnelsEl {
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            uris: self.uris,
        }
    }
}

pub struct NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
        NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `site_to_site_data_transfer` after provisioning.\nA value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub fn site_to_site_data_transfer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_to_site_data_transfer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uris` after provisioning.\nThe URIs of linked VPN tunnel resources."]
    pub fn uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpokeTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpokeTimeoutsEl {
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

impl terrars::ToListMappable for NetworkConnectivitySpokeTimeoutsEl {
    type O = terrars::BlockAssignable<NetworkConnectivitySpokeTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpokeTimeoutsEl {}

impl BuildNetworkConnectivitySpokeTimeoutsEl {
    pub fn build(self) -> NetworkConnectivitySpokeTimeoutsEl {
        NetworkConnectivitySpokeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivitySpokeTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpokeTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpokeTimeoutsElRef {
        NetworkConnectivitySpokeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpokeTimeoutsElRef {
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
struct NetworkConnectivitySpokeDynamic {
    linked_interconnect_attachments: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>>,
    linked_router_appliance_instances: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>>,
    linked_vpc_network: std::option::Option<terrars::DynamicBlock<NetworkConnectivitySpokeLinkedVpcNetworkEl>>,
    linked_vpn_tunnels: std::option::Option<terrars::DynamicBlock<NetworkConnectivitySpokeLinkedVpnTunnelsEl>>,
}
