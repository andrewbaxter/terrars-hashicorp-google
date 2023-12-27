use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareenginePrivateCloudData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

struct DataVmwareenginePrivateCloud_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareenginePrivateCloudData>,
}

#[derive(Clone)]
pub struct DataVmwareenginePrivateCloud(Rc<DataVmwareenginePrivateCloud_>);

impl DataVmwareenginePrivateCloud {
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

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> ListRef<DataVmwareenginePrivateCloudHcxElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hcx", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location where the PrivateCloud should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management_cluster` after provisioning.\nThe management cluster for this private cloud. This used for creating and managing the default cluster."]
    pub fn management_cluster(&self) -> ListRef<DataVmwareenginePrivateCloudManagementClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration in the consumer project with which the peering has to be done."]
    pub fn network_config(&self) -> ListRef<DataVmwareenginePrivateCloudNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> ListRef<DataVmwareenginePrivateCloudNsxElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nsx", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the resource. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nInitial type of the private cloud. Default value: \"STANDARD\" Possible values: [\"STANDARD\", \"TIME_LIMITED\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vcenter` after provisioning.\nDetails about a vCenter Server management appliance."]
    pub fn vcenter(&self) -> ListRef<DataVmwareenginePrivateCloudVcenterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

impl Referable for DataVmwareenginePrivateCloud {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataVmwareenginePrivateCloud { }

impl ToListMappable for DataVmwareenginePrivateCloud {
    type O = ListRef<DataVmwareenginePrivateCloudRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVmwareenginePrivateCloud_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_private_cloud".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareenginePrivateCloud {
    pub tf_id: String,
    #[doc= "The location where the PrivateCloud should reside."]
    pub location: PrimField<String>,
    #[doc= "The ID of the PrivateCloud."]
    pub name: PrimField<String>,
}

impl BuildDataVmwareenginePrivateCloud {
    pub fn build(self, stack: &mut Stack) -> DataVmwareenginePrivateCloud {
        let out = DataVmwareenginePrivateCloud(Rc::new(DataVmwareenginePrivateCloud_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVmwareenginePrivateCloudData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVmwareenginePrivateCloudRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareenginePrivateCloudRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareenginePrivateCloudRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> ListRef<DataVmwareenginePrivateCloudHcxElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hcx", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location where the PrivateCloud should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management_cluster` after provisioning.\nThe management cluster for this private cloud. This used for creating and managing the default cluster."]
    pub fn management_cluster(&self) -> ListRef<DataVmwareenginePrivateCloudManagementClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration in the consumer project with which the peering has to be done."]
    pub fn network_config(&self) -> ListRef<DataVmwareenginePrivateCloudNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> ListRef<DataVmwareenginePrivateCloudNsxElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nsx", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the resource. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nInitial type of the private cloud. Default value: \"STANDARD\" Possible values: [\"STANDARD\", \"TIME_LIMITED\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vcenter` after provisioning.\nDetails about a vCenter Server management appliance."]
    pub fn vcenter(&self) -> ListRef<DataVmwareenginePrivateCloudVcenterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudHcxEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataVmwareenginePrivateCloudHcxEl {
    #[doc= "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fqdn = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.internal_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataVmwareenginePrivateCloudHcxEl {
    type O = BlockAssignable<DataVmwareenginePrivateCloudHcxEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloudHcxEl {}

impl BuildDataVmwareenginePrivateCloudHcxEl {
    pub fn build(self) -> DataVmwareenginePrivateCloudHcxEl {
        DataVmwareenginePrivateCloudHcxEl {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataVmwareenginePrivateCloudHcxElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareenginePrivateCloudHcxElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareenginePrivateCloudHcxElRef {
        DataVmwareenginePrivateCloudHcxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudHcxElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc= "Get a reference to the value of field `internal_ip` after provisioning.\n"]
    pub fn internal_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_id: Option<PrimField<String>>,
}

impl DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    #[doc= "Set the field `custom_core_count`.\n"]
    pub fn set_custom_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_core_count = Some(v.into());
        self
    }

    #[doc= "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    type O = BlockAssignable<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {}

impl BuildDataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    pub fn build(self) -> DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
        DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
            custom_core_count: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
        }
    }
}

pub struct DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
        DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_core_count` after provisioning.\n"]
    pub fn custom_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudManagementClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_configs: Option<SetField<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>>,
}

impl DataVmwareenginePrivateCloudManagementClusterEl {
    #[doc= "Set the field `cluster_id`.\n"]
    pub fn set_cluster_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_id = Some(v.into());
        self
    }

    #[doc= "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(
        mut self,
        v: impl Into<SetField<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>>,
    ) -> Self {
        self.node_type_configs = Some(v.into());
        self
    }
}

impl ToListMappable for DataVmwareenginePrivateCloudManagementClusterEl {
    type O = BlockAssignable<DataVmwareenginePrivateCloudManagementClusterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloudManagementClusterEl {}

impl BuildDataVmwareenginePrivateCloudManagementClusterEl {
    pub fn build(self) -> DataVmwareenginePrivateCloudManagementClusterEl {
        DataVmwareenginePrivateCloudManagementClusterEl {
            cluster_id: core::default::Default::default(),
            node_type_configs: core::default::Default::default(),
        }
    }
}

pub struct DataVmwareenginePrivateCloudManagementClusterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareenginePrivateCloudManagementClusterElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareenginePrivateCloudManagementClusterElRef {
        DataVmwareenginePrivateCloudManagementClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudManagementClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc= "Get a reference to the value of field `node_type_configs` after provisioning.\n"]
    pub fn node_type_configs(&self) -> SetRef<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_server_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management_ip_address_layout_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vmware_engine_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vmware_engine_network_canonical: Option<PrimField<String>>,
}

impl DataVmwareenginePrivateCloudNetworkConfigEl {
    #[doc= "Set the field `dns_server_ip`.\n"]
    pub fn set_dns_server_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_server_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `management_cidr`.\n"]
    pub fn set_management_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.management_cidr = Some(v.into());
        self
    }

    #[doc= "Set the field `management_ip_address_layout_version`.\n"]
    pub fn set_management_ip_address_layout_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.management_ip_address_layout_version = Some(v.into());
        self
    }

    #[doc= "Set the field `vmware_engine_network`.\n"]
    pub fn set_vmware_engine_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vmware_engine_network = Some(v.into());
        self
    }

    #[doc= "Set the field `vmware_engine_network_canonical`.\n"]
    pub fn set_vmware_engine_network_canonical(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vmware_engine_network_canonical = Some(v.into());
        self
    }
}

impl ToListMappable for DataVmwareenginePrivateCloudNetworkConfigEl {
    type O = BlockAssignable<DataVmwareenginePrivateCloudNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloudNetworkConfigEl {}

impl BuildDataVmwareenginePrivateCloudNetworkConfigEl {
    pub fn build(self) -> DataVmwareenginePrivateCloudNetworkConfigEl {
        DataVmwareenginePrivateCloudNetworkConfigEl {
            dns_server_ip: core::default::Default::default(),
            management_cidr: core::default::Default::default(),
            management_ip_address_layout_version: core::default::Default::default(),
            vmware_engine_network: core::default::Default::default(),
            vmware_engine_network_canonical: core::default::Default::default(),
        }
    }
}

pub struct DataVmwareenginePrivateCloudNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareenginePrivateCloudNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareenginePrivateCloudNetworkConfigElRef {
        DataVmwareenginePrivateCloudNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dns_server_ip` after provisioning.\n"]
    pub fn dns_server_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_server_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `management_cidr` after provisioning.\n"]
    pub fn management_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management_cidr", self.base))
    }

    #[doc= "Get a reference to the value of field `management_ip_address_layout_version` after provisioning.\n"]
    pub fn management_ip_address_layout_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.management_ip_address_layout_version", self.base))
    }

    #[doc= "Get a reference to the value of field `vmware_engine_network` after provisioning.\n"]
    pub fn vmware_engine_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network", self.base))
    }

    #[doc= "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\n"]
    pub fn vmware_engine_network_canonical(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network_canonical", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudNsxEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataVmwareenginePrivateCloudNsxEl {
    #[doc= "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fqdn = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.internal_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataVmwareenginePrivateCloudNsxEl {
    type O = BlockAssignable<DataVmwareenginePrivateCloudNsxEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloudNsxEl {}

impl BuildDataVmwareenginePrivateCloudNsxEl {
    pub fn build(self) -> DataVmwareenginePrivateCloudNsxEl {
        DataVmwareenginePrivateCloudNsxEl {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataVmwareenginePrivateCloudNsxElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareenginePrivateCloudNsxElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareenginePrivateCloudNsxElRef {
        DataVmwareenginePrivateCloudNsxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudNsxElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc= "Get a reference to the value of field `internal_ip` after provisioning.\n"]
    pub fn internal_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudVcenterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataVmwareenginePrivateCloudVcenterEl {
    #[doc= "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fqdn = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.internal_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataVmwareenginePrivateCloudVcenterEl {
    type O = BlockAssignable<DataVmwareenginePrivateCloudVcenterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloudVcenterEl {}

impl BuildDataVmwareenginePrivateCloudVcenterEl {
    pub fn build(self) -> DataVmwareenginePrivateCloudVcenterEl {
        DataVmwareenginePrivateCloudVcenterEl {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataVmwareenginePrivateCloudVcenterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareenginePrivateCloudVcenterElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareenginePrivateCloudVcenterElRef {
        DataVmwareenginePrivateCloudVcenterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudVcenterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc= "Get a reference to the value of field `internal_ip` after provisioning.\n"]
    pub fn internal_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
