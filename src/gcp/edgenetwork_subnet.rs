use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EdgenetworkSubnetData {
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
    ipv4_cidr: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    subnet_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vlan_id: Option<PrimField<f64>>,
    zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EdgenetworkSubnetTimeoutsEl>,
}

struct EdgenetworkSubnet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EdgenetworkSubnetData>,
}

#[derive(Clone)]
pub struct EdgenetworkSubnet(Rc<EdgenetworkSubnet_>);

impl EdgenetworkSubnet {
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

    #[doc= "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv4_cidr`.\nThe ranges of ipv4 addresses that are owned by this subnetwork, in CIDR format."]
    pub fn set_ipv4_cidr(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ipv4_cidr = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_cidr`.\nThe ranges of ipv6 addresses that are owned by this subnetwork, in CIDR format."]
    pub fn set_ipv6_cidr(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ipv6_cidr = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nLabels associated with this resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `vlan_id`.\nVLAN ID for this subnetwork. If not specified, one is assigned automatically."]
    pub fn set_vlan_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vlan_id = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EdgenetworkSubnetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nThe time when the subnet was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_cidr` after provisioning.\nThe ranges of ipv4 addresses that are owned by this subnetwork, in CIDR format."]
    pub fn ipv4_cidr(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv4_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr` after provisioning.\nThe ranges of ipv6 addresses that are owned by this subnetwork, in CIDR format."]
    pub fn ipv6_cidr(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud region to which the target Distributed Cloud Edge zone belongs."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe canonical name of this resource, with format\n'projects/{{project}}/locations/{{location}}/zones/{{zone}}/subnets/{{subnet_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe ID of the network to which this router belongs.\nMust be of the form: 'projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}'"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nCurrent stage of the resource to the device by config push."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\nA unique ID that identifies this subnet."]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nThe time when the subnet was last updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vlan_id` after provisioning.\nVLAN ID for this subnetwork. If not specified, one is assigned automatically."]
    pub fn vlan_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe name of the target Distributed Cloud Edge zone."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgenetworkSubnetTimeoutsElRef {
        EdgenetworkSubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for EdgenetworkSubnet {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EdgenetworkSubnet { }

impl ToListMappable for EdgenetworkSubnet {
    type O = ListRef<EdgenetworkSubnetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EdgenetworkSubnet_ {
    fn extract_resource_type(&self) -> String {
        "google_edgenetwork_subnet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEdgenetworkSubnet {
    pub tf_id: String,
    #[doc= "The Google Cloud region to which the target Distributed Cloud Edge zone belongs."]
    pub location: PrimField<String>,
    #[doc= "The ID of the network to which this router belongs.\nMust be of the form: 'projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}'"]
    pub network: PrimField<String>,
    #[doc= "A unique ID that identifies this subnet."]
    pub subnet_id: PrimField<String>,
    #[doc= "The name of the target Distributed Cloud Edge zone."]
    pub zone: PrimField<String>,
}

impl BuildEdgenetworkSubnet {
    pub fn build(self, stack: &mut Stack) -> EdgenetworkSubnet {
        let out = EdgenetworkSubnet(Rc::new(EdgenetworkSubnet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EdgenetworkSubnetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ipv4_cidr: core::default::Default::default(),
                ipv6_cidr: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                network: self.network,
                project: core::default::Default::default(),
                subnet_id: self.subnet_id,
                vlan_id: core::default::Default::default(),
                zone: self.zone,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EdgenetworkSubnetRef {
    shared: StackShared,
    base: String,
}

impl Ref for EdgenetworkSubnetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EdgenetworkSubnetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nThe time when the subnet was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_cidr` after provisioning.\nThe ranges of ipv4 addresses that are owned by this subnetwork, in CIDR format."]
    pub fn ipv4_cidr(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv4_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr` after provisioning.\nThe ranges of ipv6 addresses that are owned by this subnetwork, in CIDR format."]
    pub fn ipv6_cidr(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud region to which the target Distributed Cloud Edge zone belongs."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe canonical name of this resource, with format\n'projects/{{project}}/locations/{{location}}/zones/{{zone}}/subnets/{{subnet_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe ID of the network to which this router belongs.\nMust be of the form: 'projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}'"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nCurrent stage of the resource to the device by config push."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\nA unique ID that identifies this subnet."]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nThe time when the subnet was last updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vlan_id` after provisioning.\nVLAN ID for this subnetwork. If not specified, one is assigned automatically."]
    pub fn vlan_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe name of the target Distributed Cloud Edge zone."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgenetworkSubnetTimeoutsElRef {
        EdgenetworkSubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EdgenetworkSubnetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl EdgenetworkSubnetTimeoutsEl {
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
}

impl ToListMappable for EdgenetworkSubnetTimeoutsEl {
    type O = BlockAssignable<EdgenetworkSubnetTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgenetworkSubnetTimeoutsEl {}

impl BuildEdgenetworkSubnetTimeoutsEl {
    pub fn build(self) -> EdgenetworkSubnetTimeoutsEl {
        EdgenetworkSubnetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct EdgenetworkSubnetTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EdgenetworkSubnetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EdgenetworkSubnetTimeoutsElRef {
        EdgenetworkSubnetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgenetworkSubnetTimeoutsElRef {
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
}
