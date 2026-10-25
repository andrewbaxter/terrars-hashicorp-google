#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataNetworkManagementConnectivityTestRunData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataNetworkManagementConnectivityTestRun_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkManagementConnectivityTestRunData_>,
}

#[derive(Clone)]
pub struct DataNetworkManagementConnectivityTestRun(std::rc::Rc<DataNetworkManagementConnectivityTestRun_>);

impl DataNetworkManagementConnectivityTestRun {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reachability_details` after provisioning.\nConnectivity test reachability details."]
    pub fn reachability_details(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTestRun_ReachabilityDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reachability_details", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkManagementConnectivityTestRun {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkManagementConnectivityTestRun { }

impl terrars::ToListMappable for DataNetworkManagementConnectivityTestRun {
    type O = terrars::ListRef<DataNetworkManagementConnectivityTestRunRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkManagementConnectivityTestRun_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_network_management_connectivity_test_run".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkManagementConnectivityTestRun {
    pub tf_id: std::string::String,
    #[doc = "Unique name for the connectivity test."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkManagementConnectivityTestRun {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkManagementConnectivityTestRun {
        let out =
            DataNetworkManagementConnectivityTestRun(std::rc::Rc::new(DataNetworkManagementConnectivityTestRun_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataNetworkManagementConnectivityTestRunData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    id: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkManagementConnectivityTestRunRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTestRunRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkManagementConnectivityTestRunRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reachability_details` after provisioning.\nConnectivity test reachability details."]
    pub fn reachability_details(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTestRun_ReachabilityDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reachability_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_network_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_agent_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_network_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo {
    #[doc = "Set the field `destination_ip`.\n"]
    pub fn set_destination_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_network_uri`.\n"]
    pub fn set_destination_network_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_network_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port`.\n"]
    pub fn set_destination_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.destination_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_agent_uri`.\n"]
    pub fn set_source_agent_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_agent_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_network_uri`.\n"]
    pub fn set_source_network_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_network_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_port`.\n"]
    pub fn set_source_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.source_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo {
    type O =
        terrars::BlockAssignable<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo {}

impl BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo {
    pub fn build(self) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo {
            destination_ip: core::default::Default::default(),
            destination_network_uri: core::default::Default::default(),
            destination_port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_agent_uri: core::default::Default::default(),
            source_ip: core::default::Default::default(),
            source_network_uri: core::default::Default::default(),
            source_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfoRef {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_network_uri` after provisioning.\n"]
    pub fn destination_network_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_network_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_agent_uri` after provisioning.\n"]
    pub fn source_agent_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_agent_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `source_network_uri` after provisioning.\n"]
    pub fn source_network_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_network_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port` after provisioning.\n"]
    pub fn source_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    causes_drop: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps {
    #[doc = "Set the field `causes_drop`.\n"]
    pub fn set_causes_drop(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.causes_drop = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps {
    type O = terrars::BlockAssignable<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps {}

impl BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps {
    pub fn build(self) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps {
            causes_drop: core::default::Default::default(),
            description: core::default::Default::default(),
            project_id: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_StepsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_StepsRef {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_StepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `causes_drop` after provisioning.\n"]
    pub fn causes_drop(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.causes_drop", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_info: std
    ::option
    ::Option<terrars::ListField<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_trace_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std
    ::option
    ::Option<terrars::ListField<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps>>,
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces {
    #[doc = "Set the field `endpoint_info`.\n"]
    pub fn set_endpoint_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfo>,
                    >,
    ) -> Self {
        self.endpoint_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forward_trace_id`.\n"]
    pub fn set_forward_trace_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.forward_trace_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_Steps>,
                    >,
    ) -> Self {
        self.steps = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces {
    type O = terrars::BlockAssignable<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces {}

impl BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces {
    pub fn build(self) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces {
            endpoint_info: core::default::Default::default(),
            forward_trace_id: core::default::Default::default(),
            steps: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetails_TracesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTestRun_ReachabilityDetails_TracesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetails_TracesRef {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetails_TracesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetails_TracesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_info` after provisioning.\n"]
    pub fn endpoint_info(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_EndpointInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_info", self.base))
    }

    #[doc = "Get a reference to the value of field `forward_trace_id` after provisioning.\n"]
    pub fn forward_trace_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward_trace_id", self.base))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traces: std::option::Option<terrars::ListField<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetails {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traces`.\n"]
    pub fn set_traces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_Traces>>,
    ) -> Self {
        self.traces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verify_time`.\n"]
    pub fn set_verify_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.verify_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTestRun_ReachabilityDetails {
    type O = terrars::BlockAssignable<DataNetworkManagementConnectivityTestRun_ReachabilityDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails {}

impl BuildDataNetworkManagementConnectivityTestRun_ReachabilityDetails {
    pub fn build(self) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetails {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetails {
            result: core::default::Default::default(),
            traces: core::default::Default::default(),
            verify_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTestRun_ReachabilityDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTestRun_ReachabilityDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTestRun_ReachabilityDetailsRef {
        DataNetworkManagementConnectivityTestRun_ReachabilityDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTestRun_ReachabilityDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `traces` after provisioning.\n"]
    pub fn traces(&self) -> terrars::ListRef<DataNetworkManagementConnectivityTestRun_ReachabilityDetails_TracesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traces", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_time` after provisioning.\n"]
    pub fn verify_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_time", self.base))
    }
}
