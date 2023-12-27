use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VmwareenginePrivateCloudData {
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
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management_cluster: Option<Vec<VmwareenginePrivateCloudManagementClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<VmwareenginePrivateCloudNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareenginePrivateCloudTimeoutsEl>,
    dynamic: VmwareenginePrivateCloudDynamic,
}

struct VmwareenginePrivateCloud_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareenginePrivateCloudData>,
}

#[derive(Clone)]
pub struct VmwareenginePrivateCloud(Rc<VmwareenginePrivateCloud_>);

impl VmwareenginePrivateCloud {
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

    #[doc= "Set the field `description`.\nUser-provided description for this private cloud."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc= "Set the field `type_`.\nInitial type of the private cloud. Default value: \"STANDARD\" Possible values: [\"STANDARD\", \"TIME_LIMITED\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `management_cluster`.\n"]
    pub fn set_management_cluster(
        self,
        v: impl Into<BlockAssignable<VmwareenginePrivateCloudManagementClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management_cluster = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management_cluster = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `network_config`.\n"]
    pub fn set_network_config(self, v: impl Into<BlockAssignable<VmwareenginePrivateCloudNetworkConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareenginePrivateCloudTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> ListRef<VmwareenginePrivateCloudHcxElRef> {
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> ListRef<VmwareenginePrivateCloudNsxElRef> {
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
    pub fn vcenter(&self) -> ListRef<VmwareenginePrivateCloudVcenterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management_cluster` after provisioning.\n"]
    pub fn management_cluster(&self) -> ListRef<VmwareenginePrivateCloudManagementClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<VmwareenginePrivateCloudNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareenginePrivateCloudTimeoutsElRef {
        VmwareenginePrivateCloudTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VmwareenginePrivateCloud {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VmwareenginePrivateCloud { }

impl ToListMappable for VmwareenginePrivateCloud {
    type O = ListRef<VmwareenginePrivateCloudRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VmwareenginePrivateCloud_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_private_cloud".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVmwareenginePrivateCloud {
    pub tf_id: String,
    #[doc= "The location where the PrivateCloud should reside."]
    pub location: PrimField<String>,
    #[doc= "The ID of the PrivateCloud."]
    pub name: PrimField<String>,
}

impl BuildVmwareenginePrivateCloud {
    pub fn build(self, stack: &mut Stack) -> VmwareenginePrivateCloud {
        let out = VmwareenginePrivateCloud(Rc::new(VmwareenginePrivateCloud_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareenginePrivateCloudData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                management_cluster: core::default::Default::default(),
                network_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VmwareenginePrivateCloudRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VmwareenginePrivateCloudRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> ListRef<VmwareenginePrivateCloudHcxElRef> {
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> ListRef<VmwareenginePrivateCloudNsxElRef> {
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
    pub fn vcenter(&self) -> ListRef<VmwareenginePrivateCloudVcenterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management_cluster` after provisioning.\n"]
    pub fn management_cluster(&self) -> ListRef<VmwareenginePrivateCloudManagementClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<VmwareenginePrivateCloudNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareenginePrivateCloudTimeoutsElRef {
        VmwareenginePrivateCloudTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloudHcxEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl VmwareenginePrivateCloudHcxEl {
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

impl ToListMappable for VmwareenginePrivateCloudHcxEl {
    type O = BlockAssignable<VmwareenginePrivateCloudHcxEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloudHcxEl {}

impl BuildVmwareenginePrivateCloudHcxEl {
    pub fn build(self) -> VmwareenginePrivateCloudHcxEl {
        VmwareenginePrivateCloudHcxEl {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct VmwareenginePrivateCloudHcxElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudHcxElRef {
    fn new(shared: StackShared, base: String) -> VmwareenginePrivateCloudHcxElRef {
        VmwareenginePrivateCloudHcxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloudHcxElRef {
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
pub struct VmwareenginePrivateCloudNsxEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl VmwareenginePrivateCloudNsxEl {
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

impl ToListMappable for VmwareenginePrivateCloudNsxEl {
    type O = BlockAssignable<VmwareenginePrivateCloudNsxEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloudNsxEl {}

impl BuildVmwareenginePrivateCloudNsxEl {
    pub fn build(self) -> VmwareenginePrivateCloudNsxEl {
        VmwareenginePrivateCloudNsxEl {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct VmwareenginePrivateCloudNsxElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudNsxElRef {
    fn new(shared: StackShared, base: String) -> VmwareenginePrivateCloudNsxElRef {
        VmwareenginePrivateCloudNsxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloudNsxElRef {
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
pub struct VmwareenginePrivateCloudVcenterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl VmwareenginePrivateCloudVcenterEl {
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

impl ToListMappable for VmwareenginePrivateCloudVcenterEl {
    type O = BlockAssignable<VmwareenginePrivateCloudVcenterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloudVcenterEl {}

impl BuildVmwareenginePrivateCloudVcenterEl {
    pub fn build(self) -> VmwareenginePrivateCloudVcenterEl {
        VmwareenginePrivateCloudVcenterEl {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct VmwareenginePrivateCloudVcenterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudVcenterElRef {
    fn new(shared: StackShared, base: String) -> VmwareenginePrivateCloudVcenterElRef {
        VmwareenginePrivateCloudVcenterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloudVcenterElRef {
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
pub struct VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_core_count: Option<PrimField<f64>>,
    node_count: PrimField<f64>,
    node_type_id: PrimField<String>,
}

impl VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    #[doc= "Set the field `custom_core_count`.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nThis cannot be changed once the PrivateCloud is created."]
    pub fn set_custom_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_core_count = Some(v.into());
        self
    }
}

impl ToListMappable for VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    type O = BlockAssignable<VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    #[doc= "The number of nodes of this type in the cluster."]
    pub node_count: PrimField<f64>,
    #[doc= ""]
    pub node_type_id: PrimField<String>,
}

impl BuildVmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
    pub fn build(self) -> VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
        VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl {
            custom_core_count: core::default::Default::default(),
            node_count: self.node_count,
            node_type_id: self.node_type_id,
        }
    }
}

pub struct VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
        VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_core_count` after provisioning.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nThis cannot be changed once the PrivateCloud is created."]
    pub fn custom_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes of this type in the cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareenginePrivateCloudManagementClusterElDynamic {
    node_type_configs: Option<DynamicBlock<VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>>,
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloudManagementClusterEl {
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_configs: Option<Vec<VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>>,
    dynamic: VmwareenginePrivateCloudManagementClusterElDynamic,
}

impl VmwareenginePrivateCloudManagementClusterEl {
    #[doc= "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(
        mut self,
        v: impl Into<BlockAssignable<VmwareenginePrivateCloudManagementClusterElNodeTypeConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_type_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_type_configs = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VmwareenginePrivateCloudManagementClusterEl {
    type O = BlockAssignable<VmwareenginePrivateCloudManagementClusterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloudManagementClusterEl {
    #[doc= "The user-provided identifier of the new Cluster. The identifier must meet the following requirements:\n  * Only contains 1-63 alphanumeric characters and hyphens\n  * Begins with an alphabetical character\n  * Ends with a non-hyphen character\n  * Not formatted as a UUID\n  * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)"]
    pub cluster_id: PrimField<String>,
}

impl BuildVmwareenginePrivateCloudManagementClusterEl {
    pub fn build(self) -> VmwareenginePrivateCloudManagementClusterEl {
        VmwareenginePrivateCloudManagementClusterEl {
            cluster_id: self.cluster_id,
            node_type_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VmwareenginePrivateCloudManagementClusterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudManagementClusterElRef {
    fn new(shared: StackShared, base: String) -> VmwareenginePrivateCloudManagementClusterElRef {
        VmwareenginePrivateCloudManagementClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloudManagementClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\nThe user-provided identifier of the new Cluster. The identifier must meet the following requirements:\n  * Only contains 1-63 alphanumeric characters and hyphens\n  * Begins with an alphabetical character\n  * Ends with a non-hyphen character\n  * Not formatted as a UUID\n  * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloudNetworkConfigEl {
    management_cidr: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vmware_engine_network: Option<PrimField<String>>,
}

impl VmwareenginePrivateCloudNetworkConfigEl {
    #[doc= "Set the field `vmware_engine_network`.\nThe relative resource name of the VMware Engine network attached to the private cloud.\nSpecify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}\nwhere {project} can either be a project number or a project ID."]
    pub fn set_vmware_engine_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vmware_engine_network = Some(v.into());
        self
    }
}

impl ToListMappable for VmwareenginePrivateCloudNetworkConfigEl {
    type O = BlockAssignable<VmwareenginePrivateCloudNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloudNetworkConfigEl {
    #[doc= "Management CIDR used by VMware management appliances."]
    pub management_cidr: PrimField<String>,
}

impl BuildVmwareenginePrivateCloudNetworkConfigEl {
    pub fn build(self) -> VmwareenginePrivateCloudNetworkConfigEl {
        VmwareenginePrivateCloudNetworkConfigEl {
            management_cidr: self.management_cidr,
            vmware_engine_network: core::default::Default::default(),
        }
    }
}

pub struct VmwareenginePrivateCloudNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> VmwareenginePrivateCloudNetworkConfigElRef {
        VmwareenginePrivateCloudNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloudNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dns_server_ip` after provisioning.\nDNS Server IP of the Private Cloud."]
    pub fn dns_server_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_server_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `management_cidr` after provisioning.\nManagement CIDR used by VMware management appliances."]
    pub fn management_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management_cidr", self.base))
    }

    #[doc= "Get a reference to the value of field `management_ip_address_layout_version` after provisioning.\nThe IP address layout version of the management IP address range.\nPossible versions include:\n* managementIpAddressLayoutVersion=1: Indicates the legacy IP address layout used by some existing private clouds. This is no longer supported for new private clouds\nas it does not support all features.\n* managementIpAddressLayoutVersion=2: Indicates the latest IP address layout\nused by all newly created private clouds. This version supports all current features."]
    pub fn management_ip_address_layout_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.management_ip_address_layout_version", self.base))
    }

    #[doc= "Get a reference to the value of field `vmware_engine_network` after provisioning.\nThe relative resource name of the VMware Engine network attached to the private cloud.\nSpecify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}\nwhere {project} can either be a project number or a project ID."]
    pub fn vmware_engine_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network", self.base))
    }

    #[doc= "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\nThe canonical name of the VMware Engine network in\nthe form: projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}"]
    pub fn vmware_engine_network_canonical(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network_canonical", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloudTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VmwareenginePrivateCloudTimeoutsEl {
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

impl ToListMappable for VmwareenginePrivateCloudTimeoutsEl {
    type O = BlockAssignable<VmwareenginePrivateCloudTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloudTimeoutsEl {}

impl BuildVmwareenginePrivateCloudTimeoutsEl {
    pub fn build(self) -> VmwareenginePrivateCloudTimeoutsEl {
        VmwareenginePrivateCloudTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VmwareenginePrivateCloudTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareenginePrivateCloudTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareenginePrivateCloudTimeoutsElRef {
        VmwareenginePrivateCloudTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloudTimeoutsElRef {
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
struct VmwareenginePrivateCloudDynamic {
    management_cluster: Option<DynamicBlock<VmwareenginePrivateCloudManagementClusterEl>>,
    network_config: Option<DynamicBlock<VmwareenginePrivateCloudNetworkConfigEl>>,
}
