use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVpcAccessConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataVpcAccessConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcAccessConnectorData>,
}

#[derive(Clone)]
pub struct DataVpcAccessConnector(Rc<DataVpcAccessConnector_>);

impl DataVpcAccessConnector {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nRegion where the VPC Access connector resides. If it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `connected_projects` after provisioning.\nList of projects using the connector."]
    pub fn connected_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.connected_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that follows RFC 4632 notation. Example: '10.132.0.0/28'."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nMachine type of VM Instance underlying connector. Default is e2-micro"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_instances` after provisioning.\nMaximum value of instances in autoscaling group underlying the connector."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_throughput` after provisioning.\nMaximum throughput of the connector in Mbps, must be greater than 'min_throughput'. Default is 300."]
    pub fn max_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_instances` after provisioning.\nMinimum value of instances in autoscaling group underlying the connector."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_throughput` after provisioning.\nMinimum throughput of the connector in Mbps. Default and min is 200."]
    pub fn min_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the resource (Max 25 characters)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nName or self_link of the VPC network. Required if 'ip_cidr_range' is set."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion where the VPC Access connector resides. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this VPC connector"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the VPC access connector."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet` after provisioning.\nThe subnet in which to house the connector"]
    pub fn subnet(&self) -> ListRef<DataVpcAccessConnectorSubnetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet", self.extract_ref()))
    }
}

impl Referable for DataVpcAccessConnector {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataVpcAccessConnector { }

impl ToListMappable for DataVpcAccessConnector {
    type O = ListRef<DataVpcAccessConnectorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVpcAccessConnector_ {
    fn extract_datasource_type(&self) -> String {
        "google_vpc_access_connector".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcAccessConnector {
    pub tf_id: String,
    #[doc= "The name of the resource (Max 25 characters)."]
    pub name: PrimField<String>,
}

impl BuildDataVpcAccessConnector {
    pub fn build(self, stack: &mut Stack) -> DataVpcAccessConnector {
        let out = DataVpcAccessConnector(Rc::new(DataVpcAccessConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcAccessConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcAccessConnectorRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcAccessConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcAccessConnectorRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `connected_projects` after provisioning.\nList of projects using the connector."]
    pub fn connected_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.connected_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that follows RFC 4632 notation. Example: '10.132.0.0/28'."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nMachine type of VM Instance underlying connector. Default is e2-micro"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_instances` after provisioning.\nMaximum value of instances in autoscaling group underlying the connector."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_throughput` after provisioning.\nMaximum throughput of the connector in Mbps, must be greater than 'min_throughput'. Default is 300."]
    pub fn max_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_instances` after provisioning.\nMinimum value of instances in autoscaling group underlying the connector."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_throughput` after provisioning.\nMinimum throughput of the connector in Mbps. Default and min is 200."]
    pub fn min_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the resource (Max 25 characters)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nName or self_link of the VPC network. Required if 'ip_cidr_range' is set."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion where the VPC Access connector resides. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this VPC connector"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the VPC access connector."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet` after provisioning.\nThe subnet in which to house the connector"]
    pub fn subnet(&self) -> ListRef<DataVpcAccessConnectorSubnetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcAccessConnectorSubnetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}

impl DataVpcAccessConnectorSubnetEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcAccessConnectorSubnetEl {
    type O = BlockAssignable<DataVpcAccessConnectorSubnetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcAccessConnectorSubnetEl {}

impl BuildDataVpcAccessConnectorSubnetEl {
    pub fn build(self) -> DataVpcAccessConnectorSubnetEl {
        DataVpcAccessConnectorSubnetEl {
            name: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

pub struct DataVpcAccessConnectorSubnetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcAccessConnectorSubnetElRef {
    fn new(shared: StackShared, base: String) -> DataVpcAccessConnectorSubnetElRef {
        DataVpcAccessConnectorSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcAccessConnectorSubnetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
