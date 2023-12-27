use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkManagementConnectivityTestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_projects: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<NetworkManagementConnectivityTestDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<NetworkManagementConnectivityTestSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkManagementConnectivityTestTimeoutsEl>,
    dynamic: NetworkManagementConnectivityTestDynamic,
}

struct NetworkManagementConnectivityTest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkManagementConnectivityTestData>,
}

#[derive(Clone)]
pub struct NetworkManagementConnectivityTest(Rc<NetworkManagementConnectivityTest_>);

impl NetworkManagementConnectivityTest {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `related_projects`.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn set_related_projects(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().related_projects = Some(v.into());
        self
    }

    #[doc= "Set the field `destination`.\n"]
    pub fn set_destination(self, v: impl Into<BlockAssignable<NetworkManagementConnectivityTestDestinationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<NetworkManagementConnectivityTestSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkManagementConnectivityTestTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `related_projects` after provisioning.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn related_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.related_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<NetworkManagementConnectivityTestDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<NetworkManagementConnectivityTestSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementConnectivityTestTimeoutsElRef {
        NetworkManagementConnectivityTestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for NetworkManagementConnectivityTest {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkManagementConnectivityTest { }

impl ToListMappable for NetworkManagementConnectivityTest {
    type O = ListRef<NetworkManagementConnectivityTestRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkManagementConnectivityTest_ {
    fn extract_resource_type(&self) -> String {
        "google_network_management_connectivity_test".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkManagementConnectivityTest {
    pub tf_id: String,
    #[doc= "Unique name for the connectivity test."]
    pub name: PrimField<String>,
}

impl BuildNetworkManagementConnectivityTest {
    pub fn build(self, stack: &mut Stack) -> NetworkManagementConnectivityTest {
        let out = NetworkManagementConnectivityTest(Rc::new(NetworkManagementConnectivityTest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkManagementConnectivityTestData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                related_projects: core::default::Default::default(),
                destination: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkManagementConnectivityTestRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkManagementConnectivityTestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkManagementConnectivityTestRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `related_projects` after provisioning.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn related_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.related_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<NetworkManagementConnectivityTestDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<NetworkManagementConnectivityTestSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementConnectivityTestTimeoutsElRef {
        NetworkManagementConnectivityTestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}

impl NetworkManagementConnectivityTestDestinationEl {
    #[doc= "Set the field `instance`.\nA Compute Engine instance URI."]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_address`.\nThe IP address of the endpoint, which can be an external or\ninternal IP. An IPv6 address is only allowed when the test's\ndestination is a global load balancer VIP."]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\nA Compute Engine network URI."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\nThe IP protocol port of the endpoint. Only applicable when\nprotocol is TCP or UDP."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\nProject ID where the endpoint is located. The Project ID can be\nderived from the URI if you provide a VM instance or network URI.\nThe following are two cases where you must provide the project ID:\n1. Only the IP address is specified, and the IP address is within\na GCP project. 2. When you are using Shared VPC and the IP address\nthat you provide is from the service project. In this case, the\nnetwork that the IP address resides in is defined in the host\nproject."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkManagementConnectivityTestDestinationEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTestDestinationEl {}

impl BuildNetworkManagementConnectivityTestDestinationEl {
    pub fn build(self) -> NetworkManagementConnectivityTestDestinationEl {
        NetworkManagementConnectivityTestDestinationEl {
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

pub struct NetworkManagementConnectivityTestDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkManagementConnectivityTestDestinationElRef {
    fn new(shared: StackShared, base: String) -> NetworkManagementConnectivityTestDestinationElRef {
        NetworkManagementConnectivityTestDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTestDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance` after provisioning.\nA Compute Engine instance URI."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the endpoint, which can be an external or\ninternal IP. An IPv6 address is only allowed when the test's\ndestination is a global load balancer VIP."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nA Compute Engine network URI."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nThe IP protocol port of the endpoint. Only applicable when\nprotocol is TCP or UDP."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nProject ID where the endpoint is located. The Project ID can be\nderived from the URI if you provide a VM instance or network URI.\nThe following are two cases where you must provide the project ID:\n1. Only the IP address is specified, and the IP address is within\na GCP project. 2. When you are using Shared VPC and the IP address\nthat you provide is from the service project. In this case, the\nnetwork that the IP address resides in is defined in the host\nproject."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}

impl NetworkManagementConnectivityTestSourceEl {
    #[doc= "Set the field `instance`.\nA Compute Engine instance URI."]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_address`.\nThe IP address of the endpoint, which can be an external or\ninternal IP. An IPv6 address is only allowed when the test's\ndestination is a global load balancer VIP."]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\nA Compute Engine network URI."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }

    #[doc= "Set the field `network_type`.\nType of the network where the endpoint is located. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\"]"]
    pub fn set_network_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_type = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\nThe IP protocol port of the endpoint. Only applicable when\nprotocol is TCP or UDP."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\nProject ID where the endpoint is located. The Project ID can be\nderived from the URI if you provide a VM instance or network URI.\nThe following are two cases where you must provide the project ID:\n\n1. Only the IP address is specified, and the IP address is\n   within a GCP project.\n2. When you are using Shared VPC and the IP address\n   that you provide is from the service project. In this case,\n   the network that the IP address resides in is defined in the\n   host project."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkManagementConnectivityTestSourceEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTestSourceEl {}

impl BuildNetworkManagementConnectivityTestSourceEl {
    pub fn build(self) -> NetworkManagementConnectivityTestSourceEl {
        NetworkManagementConnectivityTestSourceEl {
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_type: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

pub struct NetworkManagementConnectivityTestSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkManagementConnectivityTestSourceElRef {
    fn new(shared: StackShared, base: String) -> NetworkManagementConnectivityTestSourceElRef {
        NetworkManagementConnectivityTestSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTestSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance` after provisioning.\nA Compute Engine instance URI."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the endpoint, which can be an external or\ninternal IP. An IPv6 address is only allowed when the test's\ndestination is a global load balancer VIP."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nA Compute Engine network URI."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `network_type` after provisioning.\nType of the network where the endpoint is located. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\"]"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nThe IP protocol port of the endpoint. Only applicable when\nprotocol is TCP or UDP."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nProject ID where the endpoint is located. The Project ID can be\nderived from the URI if you provide a VM instance or network URI.\nThe following are two cases where you must provide the project ID:\n\n1. Only the IP address is specified, and the IP address is\n   within a GCP project.\n2. When you are using Shared VPC and the IP address\n   that you provide is from the service project. In this case,\n   the network that the IP address resides in is defined in the\n   host project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NetworkManagementConnectivityTestTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkManagementConnectivityTestTimeoutsEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTestTimeoutsEl {}

impl BuildNetworkManagementConnectivityTestTimeoutsEl {
    pub fn build(self) -> NetworkManagementConnectivityTestTimeoutsEl {
        NetworkManagementConnectivityTestTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkManagementConnectivityTestTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkManagementConnectivityTestTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkManagementConnectivityTestTimeoutsElRef {
        NetworkManagementConnectivityTestTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTestTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkManagementConnectivityTestDynamic {
    destination: Option<DynamicBlock<NetworkManagementConnectivityTestDestinationEl>>,
    source: Option<DynamicBlock<NetworkManagementConnectivityTestSourceEl>>,
}
