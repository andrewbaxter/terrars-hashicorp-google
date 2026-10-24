use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeReservationData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_reservation_required: std::option::Option<terrars::PrimField<bool>>,
    zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_settings: std::option::Option<std::vec::Vec<ComputeReservationShareSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_reservation: std::option::Option<std::vec::Vec<ComputeReservationSpecificReservationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeReservationTimeoutsEl>,
    dynamic: ComputeReservationDynamic,
}

struct ComputeReservation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeReservationData>,
}

#[derive(Clone)]
pub struct ComputeReservation(std::rc::Rc<ComputeReservation_>);

impl ComputeReservation {
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

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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
        "Set the field `specific_reservation_required`.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn set_specific_reservation_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().specific_reservation_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_settings`.\n"]
    pub fn set_share_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservationShareSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().share_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.share_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservationSpecificReservationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().specific_reservation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.specific_reservation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeReservationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `commitment` after provisioning.\nFull or partial URL to a parent commitment. This field displays for\nreservations that are tied to a commitment."]
    pub fn commitment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `specific_reservation_required` after provisioning.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn specific_reservation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.specific_reservation_required", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the reservation."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation is made."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> terrars::ListRef<ComputeReservationShareSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> terrars::ListRef<ComputeReservationSpecificReservationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeReservation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeReservation { }

impl terrars::ToListMappable for ComputeReservation {
    type O = terrars::ListRef<ComputeReservationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeReservation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_reservation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeReservation {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The zone where the reservation is made."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildComputeReservation {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeReservation {
        let out = ComputeReservation(std::rc::Rc::new(ComputeReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeReservationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                specific_reservation_required: core::default::Default::default(),
                zone: self.zone,
                share_settings: core::default::Default::default(),
                specific_reservation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeReservationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeReservationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `commitment` after provisioning.\nFull or partial URL to a parent commitment. This field displays for\nreservations that are tied to a commitment."]
    pub fn commitment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `specific_reservation_required` after provisioning.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn specific_reservation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.specific_reservation_required", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the reservation."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation is made."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> terrars::ListRef<ComputeReservationShareSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> terrars::ListRef<ComputeReservationSpecificReservationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationShareSettingsElProjectMapEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservationShareSettingsElProjectMapEl {
    #[doc =
        "Set the field `project_id`.\nThe project id/number, should be same as the key of this project config in the project map."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservationShareSettingsElProjectMapEl {
    type O = terrars::BlockAssignable<ComputeReservationShareSettingsElProjectMapEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationShareSettingsElProjectMapEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationShareSettingsElProjectMapElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservationShareSettingsElProjectMapElRef {
        ComputeReservationShareSettingsElProjectMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationShareSettingsElProjectMapElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project id/number, should be same as the key of this project config in the project map."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservationShareSettingsElDynamic {
    project_map: std::option::Option<terrars::DynamicBlock<ComputeReservationShareSettingsElProjectMapEl>>,
}

#[derive(Serialize)]
pub struct ComputeReservationShareSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_map: std::option::Option<std::vec::Vec<ComputeReservationShareSettingsElProjectMapEl>>,
    dynamic: ComputeReservationShareSettingsElDynamic,
}

impl ComputeReservationShareSettingsEl {
    #[doc =
        "Set the field `share_type`.\nType of sharing for this shared-reservation Possible values: [\"LOCAL\", \"SPECIFIC_PROJECTS\"]"]
    pub fn set_share_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservationShareSettingsElProjectMapEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.project_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.project_map = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeReservationShareSettingsEl {
    type O = terrars::BlockAssignable<ComputeReservationShareSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeReservationShareSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationShareSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservationShareSettingsElRef {
        ComputeReservationShareSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationShareSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `share_type` after provisioning.\nType of sharing for this shared-reservation Possible values: [\"LOCAL\", \"SPECIFIC_PROJECTS\"]"]
    pub fn share_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    accelerator_count: terrars::PrimField<f64>,
    accelerator_type: terrars::PrimField<std::string::String>,
}

impl ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl { }

impl terrars::ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    type O =
        terrars::BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    #[doc = "The number of the guest accelerator cards exposed to\nthis instance."]
    pub accelerator_count: terrars::PrimField<f64>,
    #[doc =
        "The full or partial URL of the accelerator type to\nattach to this instance. For example:\n'projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100'\n\nIf you are creating an instance template, specify only the accelerator name."]
    pub accelerator_type: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the guest accelerator cards exposed to\nthis instance."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nThe full or partial URL of the accelerator type to\nattach to this instance. For example:\n'projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100'\n\nIf you are creating an instance template, specify only the accelerator name."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    disk_size_gb: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[doc =
        "Set the field `interface`.\nThe disk interface to use for attaching this disk. Default value: \"SCSI\" Possible values: [\"SCSI\", \"NVME\"]"]
    pub fn set_interface(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    type O = terrars::BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[doc = "The size of the disk in base-2 GB."]
    pub disk_size_gb: terrars::PrimField<f64>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe size of the disk in base-2 GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interface` after provisioning.\nThe disk interface to use for attaching this disk. Default value: \"SCSI\" Possible values: [\"SCSI\", \"NVME\"]"]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservationSpecificReservationElInstancePropertiesElDynamic {
    guest_accelerators: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl>>,
    local_ssds: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>>,
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesEl {
    machine_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerators: std
    ::option
    ::Option<std::vec::Vec<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssds: std
    ::option
    ::Option<std::vec::Vec<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>>,
    dynamic: ComputeReservationSpecificReservationElInstancePropertiesElDynamic,
}

impl ComputeReservationSpecificReservationElInstancePropertiesEl {
    #[doc =
        "Set the field `min_cpu_platform`.\nThe minimum CPU platform for the reservation. For example,\n'\"Intel Skylake\"'. See\nthe CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)\nfor information on available CPU platforms."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_accelerators`.\n"]
    pub fn set_guest_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guest_accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `local_ssds`.\n"]
    pub fn set_local_ssds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_ssds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesEl {
    type O = terrars::BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationElInstancePropertiesEl {
    #[doc = "The name of the machine type to reserve."]
    pub machine_type: terrars::PrimField<std::string::String>,
}

impl BuildComputeReservationSpecificReservationElInstancePropertiesEl {
    pub fn build(self) -> ComputeReservationSpecificReservationElInstancePropertiesEl {
        ComputeReservationSpecificReservationElInstancePropertiesEl {
            machine_type: self.machine_type,
            min_cpu_platform: core::default::Default::default(),
            guest_accelerators: core::default::Default::default(),
            local_ssds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeReservationSpecificReservationElInstancePropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe name of the machine type to reserve."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe minimum CPU platform for the reservation. For example,\n'\"Intel Skylake\"'. See\nthe CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)\nfor information on available CPU platforms."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_accelerators` after provisioning.\n"]
    pub fn guest_accelerators(
        &self,
    ) -> terrars::ListRef<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssds` after provisioning.\n"]
    pub fn local_ssds(
        &self,
    ) -> terrars::ListRef<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_ssds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservationSpecificReservationElDynamic {
    instance_properties: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
}

#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationEl {
    count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_properties: std::option::Option<std::vec::Vec<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
    dynamic: ComputeReservationSpecificReservationElDynamic,
}

impl ComputeReservationSpecificReservationEl {
    #[doc = "Set the field `instance_properties`.\n"]
    pub fn set_instance_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeReservationSpecificReservationEl {
    type O = terrars::BlockAssignable<ComputeReservationSpecificReservationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservationSpecificReservationEl {
    #[doc = "The number of resources that are allocated."]
    pub count: terrars::PrimField<f64>,
}

impl BuildComputeReservationSpecificReservationEl {
    pub fn build(self) -> ComputeReservationSpecificReservationEl {
        ComputeReservationSpecificReservationEl {
            count: self.count,
            instance_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeReservationSpecificReservationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationSpecificReservationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservationSpecificReservationElRef {
        ComputeReservationSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationSpecificReservationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nThe number of resources that are allocated."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `in_use_count` after provisioning.\nHow many instances are in use."]
    pub fn in_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_use_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_properties` after provisioning.\n"]
    pub fn instance_properties(
        &self,
    ) -> terrars::ListRef<ComputeReservationSpecificReservationElInstancePropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservationTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservationTimeoutsEl {
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

impl terrars::ToListMappable for ComputeReservationTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeReservationTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservationTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservationTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct ComputeReservationDynamic {
    share_settings: std::option::Option<terrars::DynamicBlock<ComputeReservationShareSettingsEl>>,
    specific_reservation: std::option::Option<terrars::DynamicBlock<ComputeReservationSpecificReservationEl>>,
}
