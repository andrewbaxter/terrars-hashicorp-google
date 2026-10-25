#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeServiceAttachmentsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeServiceAttachments_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeServiceAttachmentsData_>,
}

#[derive(Clone)]
pub struct DataComputeServiceAttachments(std::rc::Rc<DataComputeServiceAttachments_>);

impl DataComputeServiceAttachments {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_attachments` after provisioning.\n"]
    pub fn service_attachments(&self) -> terrars::ListRef<DataComputeServiceAttachments_ServiceAttachmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_attachments", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeServiceAttachments {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeServiceAttachments { }

impl terrars::ToListMappable for DataComputeServiceAttachments {
    type O = terrars::ListRef<DataComputeServiceAttachmentsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeServiceAttachments_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_service_attachments".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeServiceAttachments {
    pub tf_id: std::string::String,
}

impl BuildDataComputeServiceAttachments {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeServiceAttachments {
        let out = DataComputeServiceAttachments(std::rc::Rc::new(DataComputeServiceAttachments_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeServiceAttachmentsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeServiceAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeServiceAttachmentsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_attachments` after provisioning.\n"]
    pub fn service_attachments(&self) -> terrars::ListRef<DataComputeServiceAttachments_ServiceAttachmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_attachments", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagated_connection_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints {
    #[doc = "Set the field `consumer_network`.\n"]
    pub fn set_consumer_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_ips`.\n"]
    pub fn set_nat_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.nat_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagated_connection_count`.\n"]
    pub fn set_propagated_connection_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.propagated_connection_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.psc_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints {
    type O = terrars::BlockAssignable<DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints {}

impl BuildDataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints {
    pub fn build(self) -> DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints {
        DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints {
            consumer_network: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            nat_ips: core::default::Default::default(),
            propagated_connection_count: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpointsRef {
        DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\n"]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_ips` after provisioning.\n"]
    pub fn nat_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nat_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `propagated_connection_count` after provisioning.\n"]
    pub fn propagated_connection_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagated_connection_count", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id_or_num: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists {
    #[doc = "Set the field `connection_limit`.\n"]
    pub fn set_connection_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_url`.\n"]
    pub fn set_endpoint_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_url`.\n"]
    pub fn set_network_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id_or_num`.\n"]
    pub fn set_project_id_or_num(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id_or_num = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists {
    type O = terrars::BlockAssignable<DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists {}

impl BuildDataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists {
    pub fn build(self) -> DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists {
        DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists {
            connection_limit: core::default::Default::default(),
            endpoint_url: core::default::Default::default(),
            network_url: core::default::Default::default(),
            project_id_or_num: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptListsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptListsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptListsRef {
        DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptListsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptListsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_limit` after provisioning.\n"]
    pub fn connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_url", self.base))
    }

    #[doc = "Get a reference to the value of field `network_url` after provisioning.\n"]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id_or_num` after provisioning.\n"]
    pub fn project_id_or_num(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id_or_num", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    low: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId {
    #[doc = "Set the field `high`.\n"]
    pub fn set_high(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.high = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `low`.\n"]
    pub fn set_low(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.low = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId {
    type O = terrars::BlockAssignable<DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId {}

impl BuildDataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId {
    pub fn build(self) -> DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId {
        DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId {
            high: core::default::Default::default(),
            low: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentIdRef {
        DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `high` after provisioning.\n"]
    pub fn high(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.high", self.base))
    }

    #[doc = "Get a reference to the value of field `low` after provisioning.\n"]
    pub fn low(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.low", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeServiceAttachments_ServiceAttachments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connected_endpoints: std
    ::option
    ::Option<terrars::ListField<DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_accept_lists: std
    ::option
    ::Option<terrars::SetField<DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_reject_lists: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_proxy_protocol: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ips_per_endpoint: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagated_connection_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_service_attachment_id: std
    ::option
    ::Option<terrars::ListField<DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reconcile_connections: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_propagated_connection_limit_if_zero: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_nat_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeServiceAttachments_ServiceAttachments {
    #[doc = "Set the field `connected_endpoints`.\n"]
    pub fn set_connected_endpoints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpoints>>,
    ) -> Self {
        self.connected_endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_preference`.\n"]
    pub fn set_connection_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connection_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_accept_lists`.\n"]
    pub fn set_consumer_accept_lists(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptLists>>,
    ) -> Self {
        self.consumer_accept_lists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_reject_lists`.\n"]
    pub fn set_consumer_reject_lists(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.consumer_reject_lists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_names`.\n"]
    pub fn set_domain_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domain_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_proxy_protocol`.\n"]
    pub fn set_enable_proxy_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_proxy_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fingerprint`.\n"]
    pub fn set_fingerprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_ips_per_endpoint`.\n"]
    pub fn set_nat_ips_per_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nat_ips_per_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_subnets`.\n"]
    pub fn set_nat_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.nat_subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagated_connection_limit`.\n"]
    pub fn set_propagated_connection_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.propagated_connection_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_service_attachment_id`.\n"]
    pub fn set_psc_service_attachment_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentId>,
                    >,
    ) -> Self {
        self.psc_service_attachment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reconcile_connections`.\n"]
    pub fn set_reconcile_connections(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reconcile_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `send_propagated_connection_limit_if_zero`.\n"]
    pub fn set_send_propagated_connection_limit_if_zero(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.send_propagated_connection_limit_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_nat_ips`.\n"]
    pub fn set_show_nat_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show_nat_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_service`.\n"]
    pub fn set_target_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeServiceAttachments_ServiceAttachments {
    type O = terrars::BlockAssignable<DataComputeServiceAttachments_ServiceAttachments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeServiceAttachments_ServiceAttachments {}

impl BuildDataComputeServiceAttachments_ServiceAttachments {
    pub fn build(self) -> DataComputeServiceAttachments_ServiceAttachments {
        DataComputeServiceAttachments_ServiceAttachments {
            connected_endpoints: core::default::Default::default(),
            connection_preference: core::default::Default::default(),
            consumer_accept_lists: core::default::Default::default(),
            consumer_reject_lists: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            domain_names: core::default::Default::default(),
            enable_proxy_protocol: core::default::Default::default(),
            fingerprint: core::default::Default::default(),
            name: core::default::Default::default(),
            nat_ips_per_endpoint: core::default::Default::default(),
            nat_subnets: core::default::Default::default(),
            project: core::default::Default::default(),
            propagated_connection_limit: core::default::Default::default(),
            psc_service_attachment_id: core::default::Default::default(),
            reconcile_connections: core::default::Default::default(),
            region: core::default::Default::default(),
            self_link: core::default::Default::default(),
            send_propagated_connection_limit_if_zero: core::default::Default::default(),
            show_nat_ips: core::default::Default::default(),
            target_service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeServiceAttachments_ServiceAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachments_ServiceAttachmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeServiceAttachments_ServiceAttachmentsRef {
        DataComputeServiceAttachments_ServiceAttachmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeServiceAttachments_ServiceAttachmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connected_endpoints` after provisioning.\n"]
    pub fn connected_endpoints(
        &self,
    ) -> terrars::ListRef<DataComputeServiceAttachments_ServiceAttachments_ConnectedEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connected_endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_preference` after provisioning.\n"]
    pub fn connection_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_accept_lists` after provisioning.\n"]
    pub fn consumer_accept_lists(
        &self,
    ) -> terrars::SetRef<DataComputeServiceAttachments_ServiceAttachments_ConsumerAcceptListsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_accept_lists", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_reject_lists` after provisioning.\n"]
    pub fn consumer_reject_lists(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_reject_lists", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_names` after provisioning.\n"]
    pub fn domain_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_names", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\n"]
    pub fn enable_proxy_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\n"]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_ips_per_endpoint` after provisioning.\n"]
    pub fn nat_ips_per_endpoint(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ips_per_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_subnets` after provisioning.\n"]
    pub fn nat_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.nat_subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `propagated_connection_limit` after provisioning.\n"]
    pub fn propagated_connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagated_connection_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_service_attachment_id` after provisioning.\n"]
    pub fn psc_service_attachment_id(
        &self,
    ) -> terrars::ListRef<DataComputeServiceAttachments_ServiceAttachments_PscServiceAttachmentIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_service_attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `reconcile_connections` after provisioning.\n"]
    pub fn reconcile_connections(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconcile_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `send_propagated_connection_limit_if_zero` after provisioning.\n"]
    pub fn send_propagated_connection_limit_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_propagated_connection_limit_if_zero", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `show_nat_ips` after provisioning.\n"]
    pub fn show_nat_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_nat_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `target_service` after provisioning.\n"]
    pub fn target_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_service", self.base))
    }
}
