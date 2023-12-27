use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct TpuNodeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    accelerator_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    tensorflow_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_service_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_config: Option<Vec<TpuNodeSchedulingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<TpuNodeTimeoutsEl>,
    dynamic: TpuNodeDynamic,
}

struct TpuNode_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TpuNodeData>,
}

#[derive(Clone)]
pub struct TpuNode(Rc<TpuNode_>);

impl TpuNode {
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

    #[doc= "Set the field `cidr_block`.\nThe CIDR block that the TPU node will use when selecting an IP\naddress. This CIDR block must be a /29 block; the Compute Engine\nnetworks API forbids a smaller block, and using a larger block would\nbe wasteful (a node can only consume one IP address).\n\nErrors will occur if the CIDR block has already been used for a\ncurrently existing TPU node, the CIDR block conflicts with any\nsubnetworks in the user's provided network, or the provided network\nis peered with another network that is using that CIDR block."]
    pub fn set_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nThe user-supplied description of the TPU. Maximum of 512 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\nThe name of a network to peer the TPU node to. It must be a\npreexisting Compute Engine network inside of the project on which\nthis API has been activated. If none is provided, \"default\" will be\nused."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `use_service_networking`.\nWhether the VPC peering for the node is set up through Service Networking API.\nThe VPC Peering should be set up before provisioning the node. If this field is set,\ncidr_block field should not be specified. If the network that you want to peer the\nTPU Node to is a Shared VPC network, the node must be created with this this field enabled."]
    pub fn set_use_service_networking(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_service_networking = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\nThe GCP location for the TPU. If it is not provided, the provider zone is used."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }

    #[doc= "Set the field `scheduling_config`.\n"]
    pub fn set_scheduling_config(self, v: impl Into<BlockAssignable<TpuNodeSchedulingConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduling_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<TpuNodeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of hardware accelerators associated with this node."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cidr_block` after provisioning.\nThe CIDR block that the TPU node will use when selecting an IP\naddress. This CIDR block must be a /29 block; the Compute Engine\nnetworks API forbids a smaller block, and using a larger block would\nbe wasteful (a node can only consume one IP address).\n\nErrors will occur if the CIDR block has already been used for a\ncurrently existing TPU node, the CIDR block conflicts with any\nsubnetworks in the user's provided network, or the provided network\nis peered with another network that is using that CIDR block."]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the TPU. Maximum of 512 characters."]
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

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe immutable name of the TPU."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name of a network to peer the TPU node to. It must be a\npreexisting Compute Engine network inside of the project on which\nthis API has been activated. If none is provided, \"default\" will be\nused."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_endpoints` after provisioning.\nThe network endpoints where TPU workers can be accessed and sent work.\nIt is recommended that Tensorflow clients of the node first reach out\nto the first (index 0) entry."]
    pub fn network_endpoints(&self) -> ListRef<TpuNodeNetworkEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_endpoints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\nThe service account used to run the tensor flow services within the\nnode. To share resources, including Google Cloud Storage data, with\nthe Tensorflow job running in the Node, this account must have\npermissions to that data."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tensorflow_version` after provisioning.\nThe version of Tensorflow running in the Node."]
    pub fn tensorflow_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tensorflow_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_service_networking` after provisioning.\nWhether the VPC peering for the node is set up through Service Networking API.\nThe VPC Peering should be set up before provisioning the node. If this field is set,\ncidr_block field should not be specified. If the network that you want to peer the\nTPU Node to is a Shared VPC network, the node must be created with this this field enabled."]
    pub fn use_service_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_service_networking", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe GCP location for the TPU. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling_config` after provisioning.\n"]
    pub fn scheduling_config(&self) -> ListRef<TpuNodeSchedulingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduling_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TpuNodeTimeoutsElRef {
        TpuNodeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for TpuNode {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TpuNode { }

impl ToListMappable for TpuNode {
    type O = ListRef<TpuNodeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TpuNode_ {
    fn extract_resource_type(&self) -> String {
        "google_tpu_node".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTpuNode {
    pub tf_id: String,
    #[doc= "The type of hardware accelerators associated with this node."]
    pub accelerator_type: PrimField<String>,
    #[doc= "The immutable name of the TPU."]
    pub name: PrimField<String>,
    #[doc= "The version of Tensorflow running in the Node."]
    pub tensorflow_version: PrimField<String>,
}

impl BuildTpuNode {
    pub fn build(self, stack: &mut Stack) -> TpuNode {
        let out = TpuNode(Rc::new(TpuNode_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TpuNodeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                accelerator_type: self.accelerator_type,
                cidr_block: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                tensorflow_version: self.tensorflow_version,
                use_service_networking: core::default::Default::default(),
                zone: core::default::Default::default(),
                scheduling_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TpuNodeRef {
    shared: StackShared,
    base: String,
}

impl Ref for TpuNodeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TpuNodeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of hardware accelerators associated with this node."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cidr_block` after provisioning.\nThe CIDR block that the TPU node will use when selecting an IP\naddress. This CIDR block must be a /29 block; the Compute Engine\nnetworks API forbids a smaller block, and using a larger block would\nbe wasteful (a node can only consume one IP address).\n\nErrors will occur if the CIDR block has already been used for a\ncurrently existing TPU node, the CIDR block conflicts with any\nsubnetworks in the user's provided network, or the provided network\nis peered with another network that is using that CIDR block."]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the TPU. Maximum of 512 characters."]
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

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe immutable name of the TPU."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name of a network to peer the TPU node to. It must be a\npreexisting Compute Engine network inside of the project on which\nthis API has been activated. If none is provided, \"default\" will be\nused."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_endpoints` after provisioning.\nThe network endpoints where TPU workers can be accessed and sent work.\nIt is recommended that Tensorflow clients of the node first reach out\nto the first (index 0) entry."]
    pub fn network_endpoints(&self) -> ListRef<TpuNodeNetworkEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_endpoints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\nThe service account used to run the tensor flow services within the\nnode. To share resources, including Google Cloud Storage data, with\nthe Tensorflow job running in the Node, this account must have\npermissions to that data."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tensorflow_version` after provisioning.\nThe version of Tensorflow running in the Node."]
    pub fn tensorflow_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tensorflow_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_service_networking` after provisioning.\nWhether the VPC peering for the node is set up through Service Networking API.\nThe VPC Peering should be set up before provisioning the node. If this field is set,\ncidr_block field should not be specified. If the network that you want to peer the\nTPU Node to is a Shared VPC network, the node must be created with this this field enabled."]
    pub fn use_service_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_service_networking", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe GCP location for the TPU. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling_config` after provisioning.\n"]
    pub fn scheduling_config(&self) -> ListRef<TpuNodeSchedulingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduling_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TpuNodeTimeoutsElRef {
        TpuNodeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TpuNodeNetworkEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl TpuNodeNetworkEndpointsEl {
    #[doc= "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for TpuNodeNetworkEndpointsEl {
    type O = BlockAssignable<TpuNodeNetworkEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTpuNodeNetworkEndpointsEl {}

impl BuildTpuNodeNetworkEndpointsEl {
    pub fn build(self) -> TpuNodeNetworkEndpointsEl {
        TpuNodeNetworkEndpointsEl {
            ip_address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct TpuNodeNetworkEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TpuNodeNetworkEndpointsElRef {
    fn new(shared: StackShared, base: String) -> TpuNodeNetworkEndpointsElRef {
        TpuNodeNetworkEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TpuNodeNetworkEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct TpuNodeSchedulingConfigEl {
    preemptible: PrimField<bool>,
}

impl TpuNodeSchedulingConfigEl { }

impl ToListMappable for TpuNodeSchedulingConfigEl {
    type O = BlockAssignable<TpuNodeSchedulingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTpuNodeSchedulingConfigEl {
    #[doc= "Defines whether the TPU instance is preemptible."]
    pub preemptible: PrimField<bool>,
}

impl BuildTpuNodeSchedulingConfigEl {
    pub fn build(self) -> TpuNodeSchedulingConfigEl {
        TpuNodeSchedulingConfigEl { preemptible: self.preemptible }
    }
}

pub struct TpuNodeSchedulingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TpuNodeSchedulingConfigElRef {
    fn new(shared: StackShared, base: String) -> TpuNodeSchedulingConfigElRef {
        TpuNodeSchedulingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TpuNodeSchedulingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `preemptible` after provisioning.\nDefines whether the TPU instance is preemptible."]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }
}

#[derive(Serialize)]
pub struct TpuNodeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl TpuNodeTimeoutsEl {
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

impl ToListMappable for TpuNodeTimeoutsEl {
    type O = BlockAssignable<TpuNodeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTpuNodeTimeoutsEl {}

impl BuildTpuNodeTimeoutsEl {
    pub fn build(self) -> TpuNodeTimeoutsEl {
        TpuNodeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct TpuNodeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TpuNodeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> TpuNodeTimeoutsElRef {
        TpuNodeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TpuNodeTimeoutsElRef {
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
struct TpuNodeDynamic {
    scheduling_config: Option<DynamicBlock<TpuNodeSchedulingConfigEl>>,
}
