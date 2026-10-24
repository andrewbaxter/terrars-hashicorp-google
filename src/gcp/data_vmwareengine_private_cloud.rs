use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareenginePrivateCloudData {
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
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataVmwareenginePrivateCloud_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVmwareenginePrivateCloudData>,
}

#[derive(Clone)]
pub struct DataVmwareenginePrivateCloud(std::rc::Rc<DataVmwareenginePrivateCloud_>);

impl DataVmwareenginePrivateCloud {
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudHcxElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hcx", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the PrivateCloud should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management_cluster` after provisioning.\nThe management cluster for this private cloud. This used for creating and managing the default cluster."]
    pub fn management_cluster(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudManagementClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration in the consumer project with which the peering has to be done."]
    pub fn network_config(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudNsxElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nsx", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of the resource. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nInitial type of the private cloud. Default value: \"STANDARD\" Possible values: [\"STANDARD\", \"TIME_LIMITED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vcenter` after provisioning.\nDetails about a vCenter Server management appliance."]
    pub fn vcenter(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudVcenterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

impl terrars::Referable for DataVmwareenginePrivateCloud {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVmwareenginePrivateCloud { }

impl terrars::ToListMappable for DataVmwareenginePrivateCloud {
    type O = terrars::ListRef<DataVmwareenginePrivateCloudRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVmwareenginePrivateCloud_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vmwareengine_private_cloud".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareenginePrivateCloud {
    pub tf_id: std::string::String,
    #[doc = "The location where the PrivateCloud should reside."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the PrivateCloud."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataVmwareenginePrivateCloud {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVmwareenginePrivateCloud {
        let out = DataVmwareenginePrivateCloud(std::rc::Rc::new(DataVmwareenginePrivateCloud_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVmwareenginePrivateCloudData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareenginePrivateCloudRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudHcxElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hcx", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the PrivateCloud should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management_cluster` after provisioning.\nThe management cluster for this private cloud. This used for creating and managing the default cluster."]
    pub fn management_cluster(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudManagementClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration in the consumer project with which the peering has to be done."]
    pub fn network_config(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudNsxElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nsx", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of the resource. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nInitial type of the private cloud. Default value: \"STANDARD\" Possible values: [\"STANDARD\", \"TIME_LIMITED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vcenter` after provisioning.\nDetails about a vCenter Server management appliance."]
    pub fn vcenter(&self) -> terrars::ListRef<DataVmwareenginePrivateCloudVcenterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudHcxEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloudHcxEl {
    #[doc = "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fqdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.internal_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloudHcxEl {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloudHcxEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudHcxElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareenginePrivateCloudHcxElRef {
        DataVmwareenginePrivateCloudHcxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudHcxElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_ip` after provisioning.\n"]
    pub fn internal_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    #[doc = "Set the field `custom_core_count`.\n"]
    pub fn set_custom_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
        DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_core_count` after provisioning.\n"]
    pub fn custom_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudManagementClusterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_configs: std
    ::option
    ::Option<terrars::SetField<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>>,
}

impl DataVmwareenginePrivateCloudManagementClusterEl {
    #[doc = "Set the field `cluster_id`.\n"]
    pub fn set_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>>,
    ) -> Self {
        self.node_type_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloudManagementClusterEl {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloudManagementClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudManagementClusterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloudManagementClusterElRef {
        DataVmwareenginePrivateCloudManagementClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudManagementClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_configs` after provisioning.\n"]
    pub fn node_type_configs(
        &self,
    ) -> terrars::SetRef<DataVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_server_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management_ip_address_layout_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_network_canonical: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloudNetworkConfigEl {
    #[doc = "Set the field `dns_server_ip`.\n"]
    pub fn set_dns_server_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_server_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management_cidr`.\n"]
    pub fn set_management_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.management_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management_ip_address_layout_version`.\n"]
    pub fn set_management_ip_address_layout_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.management_ip_address_layout_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vmware_engine_network`.\n"]
    pub fn set_vmware_engine_network(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vmware_engine_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vmware_engine_network_canonical`.\n"]
    pub fn set_vmware_engine_network_canonical(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vmware_engine_network_canonical = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloudNetworkConfigEl {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloudNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudNetworkConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloudNetworkConfigElRef {
        DataVmwareenginePrivateCloudNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_server_ip` after provisioning.\n"]
    pub fn dns_server_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_server_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `management_cidr` after provisioning.\n"]
    pub fn management_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `management_ip_address_layout_version` after provisioning.\n"]
    pub fn management_ip_address_layout_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_ip_address_layout_version", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_engine_network` after provisioning.\n"]
    pub fn vmware_engine_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\n"]
    pub fn vmware_engine_network_canonical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network_canonical", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudNsxEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloudNsxEl {
    #[doc = "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fqdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.internal_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloudNsxEl {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloudNsxEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudNsxElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareenginePrivateCloudNsxElRef {
        DataVmwareenginePrivateCloudNsxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudNsxElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_ip` after provisioning.\n"]
    pub fn internal_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloudVcenterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloudVcenterEl {
    #[doc = "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fqdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.internal_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloudVcenterEl {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloudVcenterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudVcenterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareenginePrivateCloudVcenterElRef {
        DataVmwareenginePrivateCloudVcenterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloudVcenterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_ip` after provisioning.\n"]
    pub fn internal_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
