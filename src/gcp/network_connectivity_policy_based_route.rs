use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkConnectivityPolicyBasedRouteData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_ilb_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_other_routes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<NetworkConnectivityPolicyBasedRouteFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnect_attachment: std
    ::option
    ::Option<std::vec::Vec<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkConnectivityPolicyBasedRouteTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_machine: std::option::Option<std::vec::Vec<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>>,
    dynamic: NetworkConnectivityPolicyBasedRouteDynamic,
}

struct NetworkConnectivityPolicyBasedRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkConnectivityPolicyBasedRouteData>,
}

#[derive(Clone)]
pub struct NetworkConnectivityPolicyBasedRoute(std::rc::Rc<NetworkConnectivityPolicyBasedRoute_>);

impl NetworkConnectivityPolicyBasedRoute {
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

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
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
        "Set the field `labels`.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `next_hop_ilb_ip`.\nThe IP address of a global-access-enabled L4 ILB that is the next hop for matching packets."]
    pub fn set_next_hop_ilb_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().next_hop_ilb_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `next_hop_other_routes`.\nOther routes that will be referenced to determine the next hop of the packet. Possible values: [\"DEFAULT_ROUTING\"]"]
    pub fn set_next_hop_other_routes(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().next_hop_other_routes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `priority`.\nThe priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive."]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interconnect_attachment`.\n"]
    pub fn set_interconnect_attachment(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interconnect_attachment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interconnect_attachment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkConnectivityPolicyBasedRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_machine`.\n"]
    pub fn set_virtual_machine(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_machine = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_machine = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the policy-based route was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of this resource."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy based route."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nFully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_hop_ilb_ip` after provisioning.\nThe IP address of a global-access-enabled L4 ILB that is the next hop for matching packets."]
    pub fn next_hop_ilb_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_hop_other_routes` after provisioning.\nOther routes that will be referenced to determine the next hop of the packet. Possible values: [\"DEFAULT_ROUTING\"]"]
    pub fn next_hop_other_routes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_other_routes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime when the policy-based route was created."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `warnings` after provisioning.\nIf potential misconfigurations are detected for this route, this field will be populated with warning messages."]
    pub fn warnings(&self) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteWarningsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warnings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interconnect_attachment` after provisioning.\n"]
    pub fn interconnect_attachment(
        &self,
    ) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interconnect_attachment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
        NetworkConnectivityPolicyBasedRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_machine` after provisioning.\n"]
    pub fn virtual_machine(&self) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteVirtualMachineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_machine", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkConnectivityPolicyBasedRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkConnectivityPolicyBasedRoute { }

impl terrars::ToListMappable for NetworkConnectivityPolicyBasedRoute {
    type O = terrars::ListRef<NetworkConnectivityPolicyBasedRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkConnectivityPolicyBasedRoute_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_connectivity_policy_based_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkConnectivityPolicyBasedRoute {
    pub tf_id: std::string::String,
    #[doc = "The name of the policy based route."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivityPolicyBasedRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkConnectivityPolicyBasedRoute {
        let out = NetworkConnectivityPolicyBasedRoute(std::rc::Rc::new(NetworkConnectivityPolicyBasedRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkConnectivityPolicyBasedRouteData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                network: self.network,
                next_hop_ilb_ip: core::default::Default::default(),
                next_hop_other_routes: core::default::Default::default(),
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                filter: core::default::Default::default(),
                interconnect_attachment: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_machine: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkConnectivityPolicyBasedRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityPolicyBasedRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkConnectivityPolicyBasedRouteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the policy-based route was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of this resource."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy based route."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nFully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_hop_ilb_ip` after provisioning.\nThe IP address of a global-access-enabled L4 ILB that is the next hop for matching packets."]
    pub fn next_hop_ilb_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_hop_other_routes` after provisioning.\nOther routes that will be referenced to determine the next hop of the packet. Possible values: [\"DEFAULT_ROUTING\"]"]
    pub fn next_hop_other_routes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_other_routes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime when the policy-based route was created."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `warnings` after provisioning.\nIf potential misconfigurations are detected for this route, this field will be populated with warning messages."]
    pub fn warnings(&self) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteWarningsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warnings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interconnect_attachment` after provisioning.\n"]
    pub fn interconnect_attachment(
        &self,
    ) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interconnect_attachment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
        NetworkConnectivityPolicyBasedRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_machine` after provisioning.\n"]
    pub fn virtual_machine(&self) -> terrars::ListRef<NetworkConnectivityPolicyBasedRouteVirtualMachineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_machine", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteWarningsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warning_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityPolicyBasedRouteWarningsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data`.\n"]
    pub fn set_data(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warning_message`.\n"]
    pub fn set_warning_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.warning_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivityPolicyBasedRouteWarningsEl {
    type O = terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteWarningsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityPolicyBasedRouteWarningsEl {}

impl BuildNetworkConnectivityPolicyBasedRouteWarningsEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteWarningsEl {
        NetworkConnectivityPolicyBasedRouteWarningsEl {
            code: core::default::Default::default(),
            data: core::default::Default::default(),
            warning_message: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivityPolicyBasedRouteWarningsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityPolicyBasedRouteWarningsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityPolicyBasedRouteWarningsElRef {
        NetworkConnectivityPolicyBasedRouteWarningsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityPolicyBasedRouteWarningsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_range: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityPolicyBasedRouteFilterEl {
    #[doc =
        "Set the field `dest_range`.\nThe destination IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\" if protocol version is IPv4."]
    pub fn set_dest_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dest_range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_protocol`.\nThe IP protocol that this policy-based route applies to. Valid values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'."]
    pub fn set_ip_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_range`.\nThe source IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\" if protocol version is IPv4."]
    pub fn set_src_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.src_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivityPolicyBasedRouteFilterEl {
    type O = terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityPolicyBasedRouteFilterEl {
    #[doc = "Internet protocol versions this policy-based route applies to. Possible values: [\"IPV4\"]"]
    pub protocol_version: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivityPolicyBasedRouteFilterEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteFilterEl {
        NetworkConnectivityPolicyBasedRouteFilterEl {
            dest_range: core::default::Default::default(),
            ip_protocol: core::default::Default::default(),
            protocol_version: self.protocol_version,
            src_range: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivityPolicyBasedRouteFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityPolicyBasedRouteFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityPolicyBasedRouteFilterElRef {
        NetworkConnectivityPolicyBasedRouteFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityPolicyBasedRouteFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dest_range` after provisioning.\nThe destination IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\" if protocol version is IPv4."]
    pub fn dest_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol that this policy-based route applies to. Valid values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'."]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol_version` after provisioning.\nInternet protocol versions this policy-based route applies to. Possible values: [\"IPV4\"]"]
    pub fn protocol_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_range` after provisioning.\nThe source IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\" if protocol version is IPv4."]
    pub fn src_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.src_range", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    region: terrars::PrimField<std::string::String>,
}

impl NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl { }

impl terrars::ToListMappable for NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    type O = terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    #[doc =
        "Cloud region to install this policy-based route on for Interconnect attachments. Use 'all' to install it on all Interconnect attachments."]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
        NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl { region: self.region }
    }
}

pub struct NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
        NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nCloud region to install this policy-based route on for Interconnect attachments. Use 'all' to install it on all Interconnect attachments."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityPolicyBasedRouteTimeoutsEl {
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
}

impl terrars::ToListMappable for NetworkConnectivityPolicyBasedRouteTimeoutsEl {
    type O = terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityPolicyBasedRouteTimeoutsEl {}

impl BuildNetworkConnectivityPolicyBasedRouteTimeoutsEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteTimeoutsEl {
        NetworkConnectivityPolicyBasedRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
        NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
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
}

#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    tags: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityPolicyBasedRouteVirtualMachineEl { }

impl terrars::ToListMappable for NetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    type O = terrars::BlockAssignable<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    #[doc =
        "A list of VM instance tags that this policy-based route applies to. VM instances that have ANY of tags specified here will install this PBR."]
    pub tags: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteVirtualMachineEl {
        NetworkConnectivityPolicyBasedRouteVirtualMachineEl { tags: self.tags }
    }
}

pub struct NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
        NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA list of VM instance tags that this policy-based route applies to. VM instances that have ANY of tags specified here will install this PBR."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkConnectivityPolicyBasedRouteDynamic {
    filter: std::option::Option<terrars::DynamicBlock<NetworkConnectivityPolicyBasedRouteFilterEl>>,
    interconnect_attachment: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>>,
    virtual_machine: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>>,
}
