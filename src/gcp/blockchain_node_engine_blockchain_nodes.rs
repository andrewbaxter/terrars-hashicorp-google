#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BlockchainNodeEngineBlockchainNodesData_ {
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
    blockchain_node_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blockchain_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ethereum_details: std::option::Option<std::vec::Vec<BlockchainNodeEngineBlockchainNodes_EthereumDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BlockchainNodeEngineBlockchainNodes_Timeouts>,
    dynamic: BlockchainNodeEngineBlockchainNodesDynamic,
}

struct BlockchainNodeEngineBlockchainNodes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BlockchainNodeEngineBlockchainNodesData_>,
}

#[derive(Clone)]
pub struct BlockchainNodeEngineBlockchainNodes(std::rc::Rc<BlockchainNodeEngineBlockchainNodes_>);

impl BlockchainNodeEngineBlockchainNodes {
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

    #[doc = "Set the field `blockchain_type`.\nUser-provided key-value pairs Possible values: [\"ETHEREUM\"]"]
    pub fn set_blockchain_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().blockchain_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-provided key-value pairs\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `ethereum_details`.\n"]
    pub fn set_ethereum_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_EthereumDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ethereum_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ethereum_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BlockchainNodeEngineBlockchainNodes_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `blockchain_node_id` after provisioning.\nID of the requesting object."]
    pub fn blockchain_node_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blockchain_node_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blockchain_type` after provisioning.\nUser-provided key-value pairs Possible values: [\"ETHEREUM\"]"]
    pub fn blockchain_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blockchain_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_info` after provisioning.\nThe connection information through which to interact with a blockchain node."]
    pub fn connection_info(&self) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_ConnectionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which the blockchain node was first created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-provided key-value pairs\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of Blockchain Node being created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe fully qualified name of the blockchain node. e.g. projects/my-project/locations/us-central1/blockchainNodes/my-node."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which the blockchain node was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ethereum_details` after provisioning.\n"]
    pub fn ethereum_details(&self) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_EthereumDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ethereum_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BlockchainNodeEngineBlockchainNodes_TimeoutsRef {
        BlockchainNodeEngineBlockchainNodes_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BlockchainNodeEngineBlockchainNodes {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BlockchainNodeEngineBlockchainNodes { }

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes {
    type O = terrars::ListRef<BlockchainNodeEngineBlockchainNodesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BlockchainNodeEngineBlockchainNodes_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_blockchain_node_engine_blockchain_nodes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes {
    pub tf_id: std::string::String,
    #[doc = "ID of the requesting object."]
    pub blockchain_node_id: terrars::PrimField<std::string::String>,
    #[doc = "Location of Blockchain Node being created."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBlockchainNodeEngineBlockchainNodes {
    pub fn build(self, stack: &mut terrars::Stack) -> BlockchainNodeEngineBlockchainNodes {
        let out = BlockchainNodeEngineBlockchainNodes(std::rc::Rc::new(BlockchainNodeEngineBlockchainNodes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BlockchainNodeEngineBlockchainNodesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                blockchain_node_id: self.blockchain_node_id,
                blockchain_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                ethereum_details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BlockchainNodeEngineBlockchainNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BlockchainNodeEngineBlockchainNodesRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `blockchain_node_id` after provisioning.\nID of the requesting object."]
    pub fn blockchain_node_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blockchain_node_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blockchain_type` after provisioning.\nUser-provided key-value pairs Possible values: [\"ETHEREUM\"]"]
    pub fn blockchain_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blockchain_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_info` after provisioning.\nThe connection information through which to interact with a blockchain node."]
    pub fn connection_info(&self) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_ConnectionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which the blockchain node was first created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-provided key-value pairs\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of Blockchain Node being created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe fully qualified name of the blockchain node. e.g. projects/my-project/locations/us-central1/blockchainNodes/my-node."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which the blockchain node was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ethereum_details` after provisioning.\n"]
    pub fn ethereum_details(&self) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_EthereumDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ethereum_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BlockchainNodeEngineBlockchainNodes_TimeoutsRef {
        BlockchainNodeEngineBlockchainNodes_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BlockchainNodeEngineBlockchainNodesDynamic {
    ethereum_details: std
    ::option
    ::Option<terrars::DynamicBlock<BlockchainNodeEngineBlockchainNodes_EthereumDetails>>,
}

#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_rpc_api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    websockets_api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo {
    #[doc = "Set the field `json_rpc_api_endpoint`.\n"]
    pub fn set_json_rpc_api_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.json_rpc_api_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `websockets_api_endpoint`.\n"]
    pub fn set_websockets_api_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.websockets_api_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo {
    type O = terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo {}

impl BuildBlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo {
        BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo {
            json_rpc_api_endpoint: core::default::Default::default(),
            websockets_api_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfoRef {
        BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_rpc_api_endpoint` after provisioning.\n"]
    pub fn json_rpc_api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_rpc_api_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `websockets_api_endpoint` after provisioning.\n"]
    pub fn websockets_api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.websockets_api_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodes_ConnectionInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_info: std
    ::option
    ::Option<terrars::ListField<BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BlockchainNodeEngineBlockchainNodes_ConnectionInfo {
    #[doc = "Set the field `endpoint_info`.\n"]
    pub fn set_endpoint_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfo>>,
    ) -> Self {
        self.endpoint_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes_ConnectionInfo {
    type O = terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_ConnectionInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes_ConnectionInfo {}

impl BuildBlockchainNodeEngineBlockchainNodes_ConnectionInfo {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodes_ConnectionInfo {
        BlockchainNodeEngineBlockchainNodes_ConnectionInfo {
            endpoint_info: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BlockchainNodeEngineBlockchainNodes_ConnectionInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodes_ConnectionInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BlockchainNodeEngineBlockchainNodes_ConnectionInfoRef {
        BlockchainNodeEngineBlockchainNodes_ConnectionInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BlockchainNodeEngineBlockchainNodes_ConnectionInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_info` after provisioning.\n"]
    pub fn endpoint_info(&self) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_ConnectionInfo_EndpointInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_info", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    beacon_api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    beacon_prometheus_metrics_api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_client_prometheus_metrics_api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints {
    #[doc = "Set the field `beacon_api_endpoint`.\n"]
    pub fn set_beacon_api_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.beacon_api_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `beacon_prometheus_metrics_api_endpoint`.\n"]
    pub fn set_beacon_prometheus_metrics_api_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.beacon_prometheus_metrics_api_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_client_prometheus_metrics_api_endpoint`.\n"]
    pub fn set_execution_client_prometheus_metrics_api_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_client_prometheus_metrics_api_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints {
    type O = terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints {}

impl BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints {
        BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpoints {
            beacon_api_endpoint: core::default::Default::default(),
            beacon_prometheus_metrics_api_endpoint: core::default::Default::default(),
            execution_client_prometheus_metrics_api_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpointsRef {
        BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `beacon_api_endpoint` after provisioning.\n"]
    pub fn beacon_api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.beacon_api_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `beacon_prometheus_metrics_api_endpoint` after provisioning.\n"]
    pub fn beacon_prometheus_metrics_api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.beacon_prometheus_metrics_api_endpoint", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `execution_client_prometheus_metrics_api_endpoint` after provisioning.\n"]
    pub fn execution_client_prometheus_metrics_api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_client_prometheus_metrics_api_endpoint", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    garbage_collection_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails {
    #[doc =
        "Set the field `garbage_collection_mode`.\nBlockchain garbage collection modes. Only applicable when NodeType is FULL or ARCHIVE. Possible values: [\"FULL\", \"ARCHIVE\"]"]
    pub fn set_garbage_collection_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.garbage_collection_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails {
    type O = terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails {}

impl BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails {
        BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails {
            garbage_collection_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetailsRef {
        BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `garbage_collection_mode` after provisioning.\nBlockchain garbage collection modes. Only applicable when NodeType is FULL or ARCHIVE. Possible values: [\"FULL\", \"ARCHIVE\"]"]
    pub fn garbage_collection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.garbage_collection_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    beacon_fee_recipient: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mev_relay_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig {
    #[doc =
        "Set the field `beacon_fee_recipient`.\nAn Ethereum address which the beacon client will send fee rewards to if no recipient is configured in the validator client. See https://lighthouse-book.sigmaprime.io/suggested-fee-recipient.html or https://docs.prylabs.network/docs/execution-node/fee-recipient for examples of how this is used. Note that while this is often described as \"suggested\", as we run the execution node we can trust the execution node, and therefore this is considered enforced."]
    pub fn set_beacon_fee_recipient(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.beacon_fee_recipient = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mev_relay_urls`.\nURLs for MEV-relay services to use for block building. When set, a managed MEV-boost service is configured on the beacon client."]
    pub fn set_mev_relay_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.mev_relay_urls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig {
    type O = terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig {}

impl BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig {
        BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig {
            beacon_fee_recipient: core::default::Default::default(),
            mev_relay_urls: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfigRef {
        BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `beacon_fee_recipient` after provisioning.\nAn Ethereum address which the beacon client will send fee rewards to if no recipient is configured in the validator client. See https://lighthouse-book.sigmaprime.io/suggested-fee-recipient.html or https://docs.prylabs.network/docs/execution-node/fee-recipient for examples of how this is used. Note that while this is often described as \"suggested\", as we run the execution node we can trust the execution node, and therefore this is considered enforced."]
    pub fn beacon_fee_recipient(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.beacon_fee_recipient", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mev_relay_urls` after provisioning.\nURLs for MEV-relay services to use for block building. When set, a managed MEV-boost service is configured on the beacon client."]
    pub fn mev_relay_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mev_relay_urls", self.base))
    }
}

#[derive(Serialize, Default)]
struct BlockchainNodeEngineBlockchainNodes_EthereumDetailsDynamic {
    geth_details: std
    ::option
    ::Option<terrars::DynamicBlock<BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails>>,
    validator_config: std
    ::option
    ::Option<terrars::DynamicBlock<BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig>>,
}

#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_enable_admin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_enable_debug: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consensus_client: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_client: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geth_details: std::option::Option<std::vec::Vec<BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validator_config: std
    ::option
    ::Option<std::vec::Vec<BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig>>,
    dynamic: BlockchainNodeEngineBlockchainNodes_EthereumDetailsDynamic,
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetails {
    #[doc =
        "Set the field `api_enable_admin`.\nEnables JSON-RPC access to functions in the admin namespace. Defaults to false."]
    pub fn set_api_enable_admin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.api_enable_admin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `api_enable_debug`.\nEnables JSON-RPC access to functions in the debug namespace. Defaults to false."]
    pub fn set_api_enable_debug(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.api_enable_debug = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consensus_client`.\nThe consensus client Possible values: [\"CONSENSUS_CLIENT_UNSPECIFIED\", \"LIGHTHOUSE\"]"]
    pub fn set_consensus_client(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consensus_client = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `execution_client`.\nThe execution client Possible values: [\"EXECUTION_CLIENT_UNSPECIFIED\", \"GETH\", \"ERIGON\"]"]
    pub fn set_execution_client(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_client = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe Ethereum environment being accessed. Possible values: [\"MAINNET\", \"TESTNET_GOERLI_PRATER\", \"TESTNET_SEPOLIA\"]"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_type`.\nThe type of Ethereum node. Possible values: [\"LIGHT\", \"FULL\", \"ARCHIVE\"]"]
    pub fn set_node_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geth_details`.\n"]
    pub fn set_geth_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geth_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geth_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validator_config`.\n"]
    pub fn set_validator_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validator_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validator_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes_EthereumDetails {
    type O = terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_EthereumDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails {}

impl BuildBlockchainNodeEngineBlockchainNodes_EthereumDetails {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodes_EthereumDetails {
        BlockchainNodeEngineBlockchainNodes_EthereumDetails {
            api_enable_admin: core::default::Default::default(),
            api_enable_debug: core::default::Default::default(),
            consensus_client: core::default::Default::default(),
            execution_client: core::default::Default::default(),
            network: core::default::Default::default(),
            node_type: core::default::Default::default(),
            geth_details: core::default::Default::default(),
            validator_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BlockchainNodeEngineBlockchainNodes_EthereumDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodes_EthereumDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BlockchainNodeEngineBlockchainNodes_EthereumDetailsRef {
        BlockchainNodeEngineBlockchainNodes_EthereumDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BlockchainNodeEngineBlockchainNodes_EthereumDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_endpoints` after provisioning.\nUser-provided key-value pairs"]
    pub fn additional_endpoints(
        &self,
    ) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_EthereumDetails_AdditionalEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_endpoints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `api_enable_admin` after provisioning.\nEnables JSON-RPC access to functions in the admin namespace. Defaults to false."]
    pub fn api_enable_admin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_enable_admin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `api_enable_debug` after provisioning.\nEnables JSON-RPC access to functions in the debug namespace. Defaults to false."]
    pub fn api_enable_debug(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_enable_debug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consensus_client` after provisioning.\nThe consensus client Possible values: [\"CONSENSUS_CLIENT_UNSPECIFIED\", \"LIGHTHOUSE\"]"]
    pub fn consensus_client(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consensus_client", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_client` after provisioning.\nThe execution client Possible values: [\"EXECUTION_CLIENT_UNSPECIFIED\", \"GETH\", \"ERIGON\"]"]
    pub fn execution_client(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_client", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe Ethereum environment being accessed. Possible values: [\"MAINNET\", \"TESTNET_GOERLI_PRATER\", \"TESTNET_SEPOLIA\"]"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nThe type of Ethereum node. Possible values: [\"LIGHT\", \"FULL\", \"ARCHIVE\"]"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }

    #[doc = "Get a reference to the value of field `geth_details` after provisioning.\n"]
    pub fn geth_details(&self) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_EthereumDetails_GethDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geth_details", self.base))
    }

    #[doc = "Get a reference to the value of field `validator_config` after provisioning.\n"]
    pub fn validator_config(
        &self,
    ) -> terrars::ListRef<BlockchainNodeEngineBlockchainNodes_EthereumDetails_ValidatorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validator_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodes_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BlockchainNodeEngineBlockchainNodes_Timeouts {
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

impl terrars::ToListMappable for BlockchainNodeEngineBlockchainNodes_Timeouts {
    type O = terrars::BlockAssignable<BlockchainNodeEngineBlockchainNodes_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBlockchainNodeEngineBlockchainNodes_Timeouts {}

impl BuildBlockchainNodeEngineBlockchainNodes_Timeouts {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodes_Timeouts {
        BlockchainNodeEngineBlockchainNodes_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BlockchainNodeEngineBlockchainNodes_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BlockchainNodeEngineBlockchainNodes_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BlockchainNodeEngineBlockchainNodes_TimeoutsRef {
        BlockchainNodeEngineBlockchainNodes_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BlockchainNodeEngineBlockchainNodes_TimeoutsRef {
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
