use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeReservationData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation_required: Option<PrimField<bool>>,
    zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_settings: Option<Vec<ComputeReservationShareSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation: Option<Vec<ComputeReservationSpecificReservationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeReservationTimeoutsEl>,
    dynamic: ComputeReservationDynamic,
}

struct ComputeReservation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeReservationData>,
}

#[derive(Clone)]
pub struct ComputeReservation(Rc<ComputeReservation_>);

impl ComputeReservation {
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

    #[doc= "Set the field `description`.\nAn optional description of this resource."]
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

    #[doc= "Set the field `specific_reservation_required`.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn set_specific_reservation_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().specific_reservation_required = Some(v.into());
        self
    }

    #[doc= "Set the field `share_settings`.\n"]
    pub fn set_share_settings(self, v: impl Into<BlockAssignable<ComputeReservationShareSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().share_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.share_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        self,
        v: impl Into<BlockAssignable<ComputeReservationSpecificReservationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().specific_reservation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.specific_reservation = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeReservationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `commitment` after provisioning.\nFull or partial URL to a parent commitment. This field displays for\nreservations that are tied to a commitment."]
    pub fn commitment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commitment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `specific_reservation_required` after provisioning.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn specific_reservation_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.specific_reservation_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the reservation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation is made."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> ListRef<ComputeReservationShareSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> ListRef<ComputeReservationSpecificReservationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeReservation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeReservation { }

impl ToListMappable for ComputeReservation {
    type O = ListRef<ComputeReservationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeReservation_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_reservation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeReservation {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "The zone where the reservation is made."]
    pub zone: PrimField<String>,
}

impl BuildComputeReservation {
    pub fn build(self, stack: &mut Stack) -> ComputeReservation {
        let out = ComputeReservation(Rc::new(ComputeReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeReservationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                specific_reservation_required: core::default::Default::default(),
                zone: self.zone,
                share_settings: core::default::Default::default(),
                specific_reservation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeReservationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeReservationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `commitment` after provisioning.\nFull or partial URL to a parent commitment. This field displays for\nreservations that are tied to a commitment."]
    pub fn commitment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commitment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `specific_reservation_required` after provisioning.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn specific_reservation_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.specific_reservation_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the reservation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation is made."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> ListRef<ComputeReservationShareSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> ListRef<ComputeReservationSpecificReservationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationShareSettingsElProjectMapEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}

impl ComputeReservationShareSettingsElProjectMapEl {
    #[doc= "Set the field `project_id`.\nThe project id/number, should be same as the key of this project config in the project map."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeReservationShareSettingsElProjectMapEl {
    type O = BlockAssignable<ComputeReservationShareSettingsElProjectMapEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationShareSettingsElProjectMapEl {
    #[doc= ""]
    pub id: PrimField<String>,
}

impl BuildComputeReservationShareSettingsElProjectMapEl {
    pub fn build(self) -> ComputeReservationShareSettingsElProjectMapEl {
        ComputeReservationShareSettingsElProjectMapEl {
            id: self.id,
            project_id: core::default::Default::default(),
        }
    }
}

pub struct ComputeReservationShareSettingsElProjectMapElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationShareSettingsElProjectMapElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationShareSettingsElProjectMapElRef {
        ComputeReservationShareSettingsElProjectMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationShareSettingsElProjectMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe project id/number, should be same as the key of this project config in the project map."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservationShareSettingsElDynamic {
    project_map: Option<DynamicBlock<ComputeReservationShareSettingsElProjectMapEl>>,
}

#[derive(Serialize)]
pub struct ComputeReservationShareSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    share_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_map: Option<Vec<ComputeReservationShareSettingsElProjectMapEl>>,
    dynamic: ComputeReservationShareSettingsElDynamic,
}

impl ComputeReservationShareSettingsEl {
    #[doc= "Set the field `share_type`.\nType of sharing for this shared-reservation Possible values: [\"LOCAL\", \"SPECIFIC_PROJECTS\"]"]
    pub fn set_share_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_type = Some(v.into());
        self
    }

    #[doc= "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl Into<BlockAssignable<ComputeReservationShareSettingsElProjectMapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.project_map = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.project_map = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeReservationShareSettingsEl {
    type O = BlockAssignable<ComputeReservationShareSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationShareSettingsEl {}

impl BuildComputeReservationShareSettingsEl {
    pub fn build(self) -> ComputeReservationShareSettingsEl {
        ComputeReservationShareSettingsEl {
            share_type: core::default::Default::default(),
            project_map: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeReservationShareSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationShareSettingsElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationShareSettingsElRef {
        ComputeReservationShareSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationShareSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `share_type` after provisioning.\nType of sharing for this shared-reservation Possible values: [\"LOCAL\", \"SPECIFIC_PROJECTS\"]"]
    pub fn share_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    accelerator_count: PrimField<f64>,
    accelerator_type: PrimField<String>,
}

impl ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl { }

impl ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    type O = BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    #[doc= "The number of the guest accelerator cards exposed to\nthis instance."]
    pub accelerator_count: PrimField<f64>,
    #[doc= "The full or partial URL of the accelerator type to\nattach to this instance. For example:\n'projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100'\n\nIf you are creating an instance template, specify only the accelerator name."]
    pub accelerator_type: PrimField<String>,
}

impl BuildComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    pub fn build(self) -> ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
        ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
            accelerator_count: self.accelerator_count,
            accelerator_type: self.accelerator_type,
        }
    }
}

pub struct ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the guest accelerator cards exposed to\nthis instance."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc= "Get a reference to the value of field `accelerator_type` after provisioning.\nThe full or partial URL of the accelerator type to\nattach to this instance. For example:\n'projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100'\n\nIf you are creating an instance template, specify only the accelerator name."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    disk_size_gb: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<PrimField<String>>,
}

impl ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[doc= "Set the field `interface`.\nThe disk interface to use for attaching this disk. Default value: \"SCSI\" Possible values: [\"SCSI\", \"NVME\"]"]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    type O = BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[doc= "The size of the disk in base-2 GB."]
    pub disk_size_gb: PrimField<f64>,
}

impl BuildComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    pub fn build(self) -> ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
        ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
            disk_size_gb: self.disk_size_gb,
            interface: core::default::Default::default(),
        }
    }
}

pub struct ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe size of the disk in base-2 GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc= "Get a reference to the value of field `interface` after provisioning.\nThe disk interface to use for attaching this disk. Default value: \"SCSI\" Possible values: [\"SCSI\", \"NVME\"]"]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservationSpecificReservationElInstancePropertiesElDynamic {
    guest_accelerators: Option<
        DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl>,
    >,
    local_ssds: Option<DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>>,
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesEl {
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerators: Option<Vec<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssds: Option<Vec<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>>,
    dynamic: ComputeReservationSpecificReservationElInstancePropertiesElDynamic,
}

impl ComputeReservationSpecificReservationElInstancePropertiesEl {
    #[doc= "Set the field `min_cpu_platform`.\nThe minimum CPU platform for the reservation. For example,\n'\"Intel Skylake\"'. See\nthe CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)\nfor information on available CPU platforms."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }

    #[doc= "Set the field `guest_accelerators`.\n"]
    pub fn set_guest_accelerators(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guest_accelerators = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_accelerators = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `local_ssds`.\n"]
    pub fn set_local_ssds(
        mut self,
        v: impl Into<BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_ssds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesEl {
    type O = BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationElInstancePropertiesEl {
    #[doc= "The name of the machine type to reserve."]
    pub machine_type: PrimField<String>,
}

impl BuildComputeReservationSpecificReservationElInstancePropertiesEl {
    pub fn build(self) -> ComputeReservationSpecificReservationElInstancePropertiesEl {
        ComputeReservationSpecificReservationElInstancePropertiesEl {
            machine_type: self.machine_type,
            min_cpu_platform: core::default::Default::default(),
            guest_accelerators: core::default::Default::default(),
            local_ssds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeReservationSpecificReservationElInstancePropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationSpecificReservationElInstancePropertiesElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nThe name of the machine type to reserve."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe minimum CPU platform for the reservation. For example,\n'\"Intel Skylake\"'. See\nthe CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)\nfor information on available CPU platforms."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc= "Get a reference to the value of field `guest_accelerators` after provisioning.\n"]
    pub fn guest_accelerators(
        &self,
    ) -> ListRef<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerators", self.base))
    }

    #[doc= "Get a reference to the value of field `local_ssds` after provisioning.\n"]
    pub fn local_ssds(&self) -> ListRef<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_ssds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservationSpecificReservationElDynamic {
    instance_properties: Option<DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationEl {
    count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_properties: Option<Vec<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
    dynamic: ComputeReservationSpecificReservationElDynamic,
}

impl ComputeReservationSpecificReservationEl {
    #[doc= "Set the field `instance_properties`.\n"]
    pub fn set_instance_properties(
        mut self,
        v: impl Into<BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_properties = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_properties = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeReservationSpecificReservationEl {
    type O = BlockAssignable<ComputeReservationSpecificReservationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationEl {
    #[doc= "The number of resources that are allocated."]
    pub count: PrimField<f64>,
}

impl BuildComputeReservationSpecificReservationEl {
    pub fn build(self) -> ComputeReservationSpecificReservationEl {
        ComputeReservationSpecificReservationEl {
            count: self.count,
            instance_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeReservationSpecificReservationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationSpecificReservationElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationSpecificReservationElRef {
        ComputeReservationSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\nThe number of resources that are allocated."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `in_use_count` after provisioning.\nHow many instances are in use."]
    pub fn in_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.in_use_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_properties` after provisioning.\n"]
    pub fn instance_properties(&self) -> ListRef<ComputeReservationSpecificReservationElInstancePropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.instance_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeReservationTimeoutsEl {
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

impl ToListMappable for ComputeReservationTimeoutsEl {
    type O = BlockAssignable<ComputeReservationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationTimeoutsEl {}

impl BuildComputeReservationTimeoutsEl {
    pub fn build(self) -> ComputeReservationTimeoutsEl {
        ComputeReservationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeReservationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeReservationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationTimeoutsElRef {
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
struct ComputeReservationDynamic {
    share_settings: Option<DynamicBlock<ComputeReservationShareSettingsEl>>,
    specific_reservation: Option<DynamicBlock<ComputeReservationSpecificReservationEl>>,
}
