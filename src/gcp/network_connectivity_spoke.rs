#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkConnectivitySpokeData_ {
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
    group: std::option::Option<terrars::PrimField<std::string::String>>,
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
    gateway: std::option::Option<std::vec::Vec<NetworkConnectivitySpoke_Gateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_interconnect_attachments: std
    ::option
    ::Option<std::vec::Vec<NetworkConnectivitySpoke_LinkedInterconnectAttachments>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_producer_vpc_network: std::option::Option<std::vec::Vec<NetworkConnectivitySpoke_LinkedProducerVpcNetwork>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_router_appliance_instances: std
    ::option
    ::Option<std::vec::Vec<NetworkConnectivitySpoke_LinkedRouterApplianceInstances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_vpc_network: std::option::Option<std::vec::Vec<NetworkConnectivitySpoke_LinkedVpcNetwork>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_vpn_tunnels: std::option::Option<std::vec::Vec<NetworkConnectivitySpoke_LinkedVpnTunnels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkConnectivitySpoke_Timeouts>,
    dynamic: NetworkConnectivitySpokeDynamic,
}

struct NetworkConnectivitySpoke_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkConnectivitySpokeData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of the spoke."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\nThe name of the group that this spoke is associated with."]
    pub fn set_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway`.\n"]
    pub fn set_gateway(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpoke_Gateway>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gateway = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gateway = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linked_interconnect_attachments`.\n"]
    pub fn set_linked_interconnect_attachments(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedInterconnectAttachments>>,
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

    #[doc = "Set the field `linked_producer_vpc_network`.\n"]
    pub fn set_linked_producer_vpc_network(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedProducerVpcNetwork>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_producer_vpc_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_producer_vpc_network = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedRouterApplianceInstances>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedVpcNetwork>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedVpnTunnels>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkConnectivitySpoke_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time the spoke was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `group` after provisioning.\nThe name of the group that this spoke is associated with."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reasons` after provisioning.\nThe reasons for the current state in the lifecycle"]
    pub fn reasons(&self) -> terrars::ListRef<NetworkConnectivitySpoke_ReasonsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reasons", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current lifecycle state of this spoke."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
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

    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> terrars::ListRef<NetworkConnectivitySpoke_GatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_interconnect_attachments` after provisioning.\n"]
    pub fn linked_interconnect_attachments(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedInterconnectAttachmentsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_interconnect_attachments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_producer_vpc_network` after provisioning.\n"]
    pub fn linked_producer_vpc_network(&self) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedProducerVpcNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_producer_vpc_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_router_appliance_instances` after provisioning.\n"]
    pub fn linked_router_appliance_instances(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedRouterApplianceInstancesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_router_appliance_instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_vpc_network` after provisioning.\n"]
    pub fn linked_vpc_network(&self) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedVpcNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpc_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_vpn_tunnels` after provisioning.\n"]
    pub fn linked_vpn_tunnels(&self) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedVpnTunnelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpn_tunnels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivitySpoke_TimeoutsRef {
        NetworkConnectivitySpoke_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(NetworkConnectivitySpokeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                group: core::default::Default::default(),
                hub: self.hub,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                gateway: core::default::Default::default(),
                linked_interconnect_attachments: core::default::Default::default(),
                linked_producer_vpc_network: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `group` after provisioning.\nThe name of the group that this spoke is associated with."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reasons` after provisioning.\nThe reasons for the current state in the lifecycle"]
    pub fn reasons(&self) -> terrars::ListRef<NetworkConnectivitySpoke_ReasonsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reasons", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current lifecycle state of this spoke."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
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

    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> terrars::ListRef<NetworkConnectivitySpoke_GatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_interconnect_attachments` after provisioning.\n"]
    pub fn linked_interconnect_attachments(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedInterconnectAttachmentsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_interconnect_attachments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_producer_vpc_network` after provisioning.\n"]
    pub fn linked_producer_vpc_network(&self) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedProducerVpcNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_producer_vpc_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_router_appliance_instances` after provisioning.\n"]
    pub fn linked_router_appliance_instances(
        &self,
    ) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedRouterApplianceInstancesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.linked_router_appliance_instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `linked_vpc_network` after provisioning.\n"]
    pub fn linked_vpc_network(&self) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedVpcNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpc_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `linked_vpn_tunnels` after provisioning.\n"]
    pub fn linked_vpn_tunnels(&self) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedVpnTunnelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_vpn_tunnels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivitySpoke_TimeoutsRef {
        NetworkConnectivitySpoke_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkConnectivitySpokeDynamic {
    gateway: std::option::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_Gateway>>,
    linked_interconnect_attachments: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_LinkedInterconnectAttachments>>,
    linked_producer_vpc_network: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_LinkedProducerVpcNetwork>>,
    linked_router_appliance_instances: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_LinkedRouterApplianceInstances>>,
    linked_vpc_network: std::option::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_LinkedVpcNetwork>>,
    linked_vpn_tunnels: std::option::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_LinkedVpnTunnels>>,
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_Reasons {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_details: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpoke_Reasons {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_details`.\n"]
    pub fn set_user_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_details = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpoke_Reasons {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_Reasons>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_Reasons {}

impl BuildNetworkConnectivitySpoke_Reasons {
    pub fn build(self) -> NetworkConnectivitySpoke_Reasons {
        NetworkConnectivitySpoke_Reasons {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
            user_details: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_ReasonsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_ReasonsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpoke_ReasonsRef {
        NetworkConnectivitySpoke_ReasonsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_ReasonsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `user_details` after provisioning.\n"]
    pub fn user_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_details", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_Gateway_IpRangeReservations {
    ip_range: terrars::PrimField<std::string::String>,
}

impl NetworkConnectivitySpoke_Gateway_IpRangeReservations { }

impl terrars::ToListMappable for NetworkConnectivitySpoke_Gateway_IpRangeReservations {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_Gateway_IpRangeReservations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_Gateway_IpRangeReservations {
    #[doc =
        "A block of IP address ranges used to allocate supporting infrastructure for this gateway—for example, 10.1.2.0/23. The IP address block must be a /23 range. This IP address block must not overlap with subnets in any spoke or peer network that the gateway can communicate with."]
    pub ip_range: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivitySpoke_Gateway_IpRangeReservations {
    pub fn build(self) -> NetworkConnectivitySpoke_Gateway_IpRangeReservations {
        NetworkConnectivitySpoke_Gateway_IpRangeReservations { ip_range: self.ip_range }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_Gateway_IpRangeReservationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_Gateway_IpRangeReservationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpoke_Gateway_IpRangeReservationsRef {
        NetworkConnectivitySpoke_Gateway_IpRangeReservationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_Gateway_IpRangeReservationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_range` after provisioning.\nA block of IP address ranges used to allocate supporting infrastructure for this gateway—for example, 10.1.2.0/23. The IP address block must be a /23 range. This IP address block must not overlap with subnets in any spoke or peer network that the gateway can communicate with."]
    pub fn ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkConnectivitySpoke_GatewayDynamic {
    ip_range_reservations: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_Gateway_IpRangeReservations>>,
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_Gateway {
    capacity: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_range_reservations: std::option::Option<std::vec::Vec<NetworkConnectivitySpoke_Gateway_IpRangeReservations>>,
    dynamic: NetworkConnectivitySpoke_GatewayDynamic,
}

impl NetworkConnectivitySpoke_Gateway {
    #[doc = "Set the field `ip_range_reservations`.\n"]
    pub fn set_ip_range_reservations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivitySpoke_Gateway_IpRangeReservations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_range_reservations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_range_reservations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpoke_Gateway {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_Gateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_Gateway {
    #[doc =
        "the capacity of the gateway spoke, in Gbps. Possible values: [\"CAPACITY_1_GBPS\", \"CAPACITY_10_GBPS\", \"CAPACITY_100_GBPS\"]"]
    pub capacity: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivitySpoke_Gateway {
    pub fn build(self) -> NetworkConnectivitySpoke_Gateway {
        NetworkConnectivitySpoke_Gateway {
            capacity: self.capacity,
            ip_range_reservations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_GatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_GatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpoke_GatewayRef {
        NetworkConnectivitySpoke_GatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_GatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `capacity` after provisioning.\nthe capacity of the gateway spoke, in Gbps. Possible values: [\"CAPACITY_1_GBPS\", \"CAPACITY_10_GBPS\", \"CAPACITY_100_GBPS\"]"]
    pub fn capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `routers` after provisioning.\nSet of Cloud Routers that are attached to this NCC-GW"]
    pub fn routers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routers", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_range_reservations` after provisioning.\n"]
    pub fn ip_range_reservations(&self) -> terrars::ListRef<NetworkConnectivitySpoke_Gateway_IpRangeReservationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_range_reservations", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_LinkedInterconnectAttachments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_import_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_import_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    site_to_site_data_transfer: terrars::PrimField<bool>,
    uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpoke_LinkedInterconnectAttachments {
    #[doc =
        "Set the field `exclude_export_ranges`.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn set_exclude_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude_import_ranges`.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn set_exclude_import_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_import_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_export_ranges`.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn set_include_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_import_ranges`.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn set_include_import_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_import_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpoke_LinkedInterconnectAttachments {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedInterconnectAttachments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_LinkedInterconnectAttachments {
    #[doc =
        "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: terrars::PrimField<bool>,
    #[doc = "The URIs of linked interconnect attachment resources"]
    pub uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkConnectivitySpoke_LinkedInterconnectAttachments {
    pub fn build(self) -> NetworkConnectivitySpoke_LinkedInterconnectAttachments {
        NetworkConnectivitySpoke_LinkedInterconnectAttachments {
            exclude_export_ranges: core::default::Default::default(),
            exclude_import_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            include_import_ranges: core::default::Default::default(),
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            uris: self.uris,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_LinkedInterconnectAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_LinkedInterconnectAttachmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpoke_LinkedInterconnectAttachmentsRef {
        NetworkConnectivitySpoke_LinkedInterconnectAttachmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_LinkedInterconnectAttachmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn exclude_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_import_ranges` after provisioning.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn exclude_import_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_import_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_export_ranges` after provisioning.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn include_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_import_ranges` after provisioning.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn include_import_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_import_ranges", self.base))
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
pub struct NetworkConnectivitySpoke_LinkedProducerVpcNetwork {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    network: terrars::PrimField<std::string::String>,
    peering: terrars::PrimField<std::string::String>,
}

impl NetworkConnectivitySpoke_LinkedProducerVpcNetwork {
    #[doc = "Set the field `exclude_export_ranges`.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn set_exclude_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_export_ranges`.\nIP ranges allowed to be included from peering."]
    pub fn set_include_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_export_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpoke_LinkedProducerVpcNetwork {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedProducerVpcNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_LinkedProducerVpcNetwork {
    #[doc = "The URI of the Service Consumer VPC that the Producer VPC is peered with."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the VPC peering between the Service Consumer VPC and the Producer VPC (defined in the Tenant project) which is added to the NCC hub. This peering must be in ACTIVE state."]
    pub peering: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivitySpoke_LinkedProducerVpcNetwork {
    pub fn build(self) -> NetworkConnectivitySpoke_LinkedProducerVpcNetwork {
        NetworkConnectivitySpoke_LinkedProducerVpcNetwork {
            exclude_export_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            network: self.network,
            peering: self.peering,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_LinkedProducerVpcNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_LinkedProducerVpcNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpoke_LinkedProducerVpcNetworkRef {
        NetworkConnectivitySpoke_LinkedProducerVpcNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_LinkedProducerVpcNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn exclude_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_export_ranges` after provisioning.\nIP ranges allowed to be included from peering."]
    pub fn include_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe URI of the Service Consumer VPC that the Producer VPC is peered with."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peering` after provisioning.\nThe name of the VPC peering between the Service Consumer VPC and the Producer VPC (defined in the Tenant project) which is added to the NCC hub. This peering must be in ACTIVE state."]
    pub fn peering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `producer_network` after provisioning.\nThe URI of the Producer VPC."]
    pub fn producer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.producer_network", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances {
    ip_address: terrars::PrimField<std::string::String>,
    virtual_machine: terrars::PrimField<std::string::String>,
}

impl NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances { }

impl terrars::ToListMappable for NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances {
    #[doc = "The IP address on the VM to use for peering."]
    pub ip_address: terrars::PrimField<std::string::String>,
    #[doc = "The URI of the virtual machine resource"]
    pub virtual_machine: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances {
    pub fn build(self) -> NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances {
        NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances {
            ip_address: self.ip_address,
            virtual_machine: self.virtual_machine,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_LinkedRouterApplianceInstances_InstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_LinkedRouterApplianceInstances_InstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpoke_LinkedRouterApplianceInstances_InstancesRef {
        NetworkConnectivitySpoke_LinkedRouterApplianceInstances_InstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_LinkedRouterApplianceInstances_InstancesRef {
    #[allow(dead_code)]
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
struct NetworkConnectivitySpoke_LinkedRouterApplianceInstancesDynamic {
    instances: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances>>,
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_LinkedRouterApplianceInstances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_import_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_import_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    site_to_site_data_transfer: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<std::vec::Vec<NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances>>,
    dynamic: NetworkConnectivitySpoke_LinkedRouterApplianceInstancesDynamic,
}

impl NetworkConnectivitySpoke_LinkedRouterApplianceInstances {
    #[doc =
        "Set the field `exclude_export_ranges`.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn set_exclude_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude_import_ranges`.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn set_exclude_import_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_import_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_export_ranges`.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn set_include_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_import_ranges`.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn set_include_import_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_import_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedRouterApplianceInstances_Instances>,
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

impl terrars::ToListMappable for NetworkConnectivitySpoke_LinkedRouterApplianceInstances {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedRouterApplianceInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_LinkedRouterApplianceInstances {
    #[doc =
        "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: terrars::PrimField<bool>,
}

impl BuildNetworkConnectivitySpoke_LinkedRouterApplianceInstances {
    pub fn build(self) -> NetworkConnectivitySpoke_LinkedRouterApplianceInstances {
        NetworkConnectivitySpoke_LinkedRouterApplianceInstances {
            exclude_export_ranges: core::default::Default::default(),
            exclude_import_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            include_import_ranges: core::default::Default::default(),
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            instances: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_LinkedRouterApplianceInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_LinkedRouterApplianceInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivitySpoke_LinkedRouterApplianceInstancesRef {
        NetworkConnectivitySpoke_LinkedRouterApplianceInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_LinkedRouterApplianceInstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn exclude_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_import_ranges` after provisioning.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn exclude_import_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_import_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_export_ranges` after provisioning.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn include_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_import_ranges` after provisioning.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn include_import_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_import_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `site_to_site_data_transfer` after provisioning.\nA value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub fn site_to_site_data_transfer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_to_site_data_transfer", self.base))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<NetworkConnectivitySpoke_LinkedRouterApplianceInstances_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_LinkedVpcNetwork {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl NetworkConnectivitySpoke_LinkedVpcNetwork {
    #[doc = "Set the field `exclude_export_ranges`.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn set_exclude_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_export_ranges`.\nIP ranges allowed to be included from peering."]
    pub fn set_include_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_export_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpoke_LinkedVpcNetwork {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedVpcNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_LinkedVpcNetwork {
    #[doc = "The URI of the VPC network resource."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivitySpoke_LinkedVpcNetwork {
    pub fn build(self) -> NetworkConnectivitySpoke_LinkedVpcNetwork {
        NetworkConnectivitySpoke_LinkedVpcNetwork {
            exclude_export_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_LinkedVpcNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_LinkedVpcNetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpoke_LinkedVpcNetworkRef {
        NetworkConnectivitySpoke_LinkedVpcNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_LinkedVpcNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn exclude_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_export_ranges` after provisioning.\nIP ranges allowed to be included from peering."]
    pub fn include_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_export_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the VPC network resource."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivitySpoke_LinkedVpnTunnels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_import_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_export_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_import_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    site_to_site_data_transfer: terrars::PrimField<bool>,
    uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpoke_LinkedVpnTunnels {
    #[doc =
        "Set the field `exclude_export_ranges`.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn set_exclude_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude_import_ranges`.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn set_exclude_import_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_import_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_export_ranges`.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn set_include_export_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_export_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_import_ranges`.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn set_include_import_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_import_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivitySpoke_LinkedVpnTunnels {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_LinkedVpnTunnels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_LinkedVpnTunnels {
    #[doc =
        "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: terrars::PrimField<bool>,
    #[doc = "The URIs of linked VPN tunnel resources."]
    pub uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkConnectivitySpoke_LinkedVpnTunnels {
    pub fn build(self) -> NetworkConnectivitySpoke_LinkedVpnTunnels {
        NetworkConnectivitySpoke_LinkedVpnTunnels {
            exclude_export_ranges: core::default::Default::default(),
            exclude_import_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            include_import_ranges: core::default::Default::default(),
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            uris: self.uris,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_LinkedVpnTunnelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_LinkedVpnTunnelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpoke_LinkedVpnTunnelsRef {
        NetworkConnectivitySpoke_LinkedVpnTunnelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_LinkedVpnTunnelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn exclude_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_import_ranges` after provisioning.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn exclude_import_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_import_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_export_ranges` after provisioning.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn include_export_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_export_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_import_ranges` after provisioning.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn include_import_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_import_ranges", self.base))
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
pub struct NetworkConnectivitySpoke_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivitySpoke_Timeouts {
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

impl terrars::ToListMappable for NetworkConnectivitySpoke_Timeouts {
    type O = terrars::BlockAssignable<NetworkConnectivitySpoke_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivitySpoke_Timeouts {}

impl BuildNetworkConnectivitySpoke_Timeouts {
    pub fn build(self) -> NetworkConnectivitySpoke_Timeouts {
        NetworkConnectivitySpoke_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkConnectivitySpoke_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivitySpoke_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkConnectivitySpoke_TimeoutsRef {
        NetworkConnectivitySpoke_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivitySpoke_TimeoutsRef {
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
