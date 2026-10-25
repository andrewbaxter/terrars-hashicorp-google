#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeReservationData_ {
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
    delete_at_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_reservation_required: std::option::Option<terrars::PrimField<bool>>,
    zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_after_duration: std::option::Option<std::vec::Vec<ComputeReservation_DeleteAfterDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeReservation_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_sharing_policy: std::option::Option<std::vec::Vec<ComputeReservation_ReservationSharingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_settings: std::option::Option<std::vec::Vec<ComputeReservation_ShareSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_reservation: std::option::Option<std::vec::Vec<ComputeReservation_SpecificReservation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeReservation_Timeouts>,
    dynamic: ComputeReservationDynamic,
}

struct ComputeReservation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeReservationData_>,
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

    #[doc =
        "Set the field `delete_at_time`.\nAbsolute time in future when the reservation will be auto-deleted by Compute Engine. Timestamp is represented in RFC3339 text format.\nCannot be used with delete_after_duration."]
    pub fn set_delete_at_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().delete_at_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `delete_after_duration`.\n"]
    pub fn set_delete_after_duration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservation_DeleteAfterDuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delete_after_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delete_after_duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservation_Params>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reservation_sharing_policy`.\n"]
    pub fn set_reservation_sharing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservation_ReservationSharingPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_sharing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_sharing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `share_settings`.\n"]
    pub fn set_share_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservation_ShareSettings>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservation_SpecificReservation>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeReservation_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `block_names` after provisioning.\nList of all reservation block names in the parent reservation."]
    pub fn block_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_names", self.extract_ref()))
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
        "Get a reference to the value of field `delete_at_time` after provisioning.\nAbsolute time in future when the reservation will be auto-deleted by Compute Engine. Timestamp is represented in RFC3339 text format.\nCannot be used with delete_after_duration."]
    pub fn delete_at_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_at_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservations for reservations."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_commitments` after provisioning.\nFull or partial URL to parent commitments. This field displays for reservations that are tied to multiple commitments."]
    pub fn linked_commitments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_commitments", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> terrars::ListRef<ComputeReservation_ResourceStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `delete_after_duration` after provisioning.\n"]
    pub fn delete_after_duration(&self) -> terrars::ListRef<ComputeReservation_DeleteAfterDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_after_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeReservation_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\n"]
    pub fn reservation_sharing_policy(&self) -> terrars::ListRef<ComputeReservation_ReservationSharingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_sharing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> terrars::ListRef<ComputeReservation_ShareSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> terrars::ListRef<ComputeReservation_SpecificReservationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservation_TimeoutsRef {
        ComputeReservation_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(ComputeReservationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delete_at_time: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                specific_reservation_required: core::default::Default::default(),
                zone: self.zone,
                delete_after_duration: core::default::Default::default(),
                params: core::default::Default::default(),
                reservation_sharing_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `block_names` after provisioning.\nList of all reservation block names in the parent reservation."]
    pub fn block_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_names", self.extract_ref()))
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
        "Get a reference to the value of field `delete_at_time` after provisioning.\nAbsolute time in future when the reservation will be auto-deleted by Compute Engine. Timestamp is represented in RFC3339 text format.\nCannot be used with delete_after_duration."]
    pub fn delete_at_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_at_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservations for reservations."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_commitments` after provisioning.\nFull or partial URL to parent commitments. This field displays for reservations that are tied to multiple commitments."]
    pub fn linked_commitments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_commitments", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> terrars::ListRef<ComputeReservation_ResourceStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `delete_after_duration` after provisioning.\n"]
    pub fn delete_after_duration(&self) -> terrars::ListRef<ComputeReservation_DeleteAfterDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_after_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeReservation_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\n"]
    pub fn reservation_sharing_policy(&self) -> terrars::ListRef<ComputeReservation_ReservationSharingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_sharing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> terrars::ListRef<ComputeReservation_ShareSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> terrars::ListRef<ComputeReservation_SpecificReservationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservation_TimeoutsRef {
        ComputeReservation_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservationDynamic {
    delete_after_duration: std::option::Option<terrars::DynamicBlock<ComputeReservation_DeleteAfterDuration>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeReservation_Params>>,
    reservation_sharing_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeReservation_ReservationSharingPolicy>>,
    share_settings: std::option::Option<terrars::DynamicBlock<ComputeReservation_ShareSettings>>,
    specific_reservation: std::option::Option<terrars::DynamicBlock<ComputeReservation_SpecificReservation>>,
}

#[derive(Serialize)]
pub struct ComputeReservation_ResourceStatus_HealthInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    degraded_block_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_block_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeReservation_ResourceStatus_HealthInfo {
    #[doc = "Set the field `degraded_block_count`.\n"]
    pub fn set_degraded_block_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.degraded_block_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_status`.\n"]
    pub fn set_health_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `healthy_block_count`.\n"]
    pub fn set_healthy_block_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_block_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_ResourceStatus_HealthInfo {
    type O = terrars::BlockAssignable<ComputeReservation_ResourceStatus_HealthInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ResourceStatus_HealthInfo {}

impl BuildComputeReservation_ResourceStatus_HealthInfo {
    pub fn build(self) -> ComputeReservation_ResourceStatus_HealthInfo {
        ComputeReservation_ResourceStatus_HealthInfo {
            degraded_block_count: core::default::Default::default(),
            health_status: core::default::Default::default(),
            healthy_block_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ResourceStatus_HealthInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ResourceStatus_HealthInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_ResourceStatus_HealthInfoRef {
        ComputeReservation_ResourceStatus_HealthInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ResourceStatus_HealthInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `degraded_block_count` after provisioning.\n"]
    pub fn degraded_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.degraded_block_count", self.base))
    }

    #[doc = "Get a reference to the value of field `health_status` after provisioning.\n"]
    pub fn health_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_status", self.base))
    }

    #[doc = "Get a reference to the value of field `healthy_block_count` after provisioning.\n"]
    pub fn healthy_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_block_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    can_reschedule: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_window_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_on_shutdown: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
    #[doc = "Set the field `can_reschedule`.\n"]
    pub fn set_can_reschedule(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.can_reschedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latest_window_start_time`.\n"]
    pub fn set_latest_window_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.latest_window_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_on_shutdown`.\n"]
    pub fn set_maintenance_on_shutdown(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.maintenance_on_shutdown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_reasons`.\n"]
    pub fn set_maintenance_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.maintenance_reasons = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_status`.\n"]
    pub fn set_maintenance_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.maintenance_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window_end_time`.\n"]
    pub fn set_window_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window_end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window_start_time`.\n"]
    pub fn set_window_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
    type O =
        terrars::BlockAssignable<ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {}

impl BuildComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
    pub fn build(self) -> ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
        ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
            can_reschedule: core::default::Default::default(),
            latest_window_start_time: core::default::Default::default(),
            maintenance_on_shutdown: core::default::Default::default(),
            maintenance_reasons: core::default::Default::default(),
            maintenance_status: core::default::Default::default(),
            type_: core::default::Default::default(),
            window_end_time: core::default::Default::default(),
            window_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
        ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `can_reschedule` after provisioning.\n"]
    pub fn can_reschedule(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_reschedule", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_window_start_time` after provisioning.\n"]
    pub fn latest_window_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_window_start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_on_shutdown` after provisioning.\n"]
    pub fn maintenance_on_shutdown(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_on_shutdown", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_reasons` after provisioning.\n"]
    pub fn maintenance_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_reasons", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_status` after provisioning.\n"]
    pub fn maintenance_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `window_end_time` after provisioning.\n"]
    pub fn window_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_ResourceStatus_ReservationMaintenance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_maintenance_ongoing_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_maintenance_pending_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_ongoing_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_pending_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subblock_infra_maintenance_ongoing_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subblock_infra_maintenance_pending_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upcoming_group_maintenance: std
    ::option
    ::Option<terrars::ListField<ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance>>,
}

impl ComputeReservation_ResourceStatus_ReservationMaintenance {
    #[doc = "Set the field `instance_maintenance_ongoing_count`.\n"]
    pub fn set_instance_maintenance_ongoing_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.instance_maintenance_ongoing_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_maintenance_pending_count`.\n"]
    pub fn set_instance_maintenance_pending_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.instance_maintenance_pending_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_ongoing_count`.\n"]
    pub fn set_maintenance_ongoing_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maintenance_ongoing_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_pending_count`.\n"]
    pub fn set_maintenance_pending_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maintenance_pending_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_type`.\n"]
    pub fn set_scheduling_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheduling_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subblock_infra_maintenance_ongoing_count`.\n"]
    pub fn set_subblock_infra_maintenance_ongoing_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_ongoing_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subblock_infra_maintenance_pending_count`.\n"]
    pub fn set_subblock_infra_maintenance_pending_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_pending_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upcoming_group_maintenance`.\n"]
    pub fn set_upcoming_group_maintenance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance>,
                    >,
    ) -> Self {
        self.upcoming_group_maintenance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_ResourceStatus_ReservationMaintenance {
    type O = terrars::BlockAssignable<ComputeReservation_ResourceStatus_ReservationMaintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ResourceStatus_ReservationMaintenance {}

impl BuildComputeReservation_ResourceStatus_ReservationMaintenance {
    pub fn build(self) -> ComputeReservation_ResourceStatus_ReservationMaintenance {
        ComputeReservation_ResourceStatus_ReservationMaintenance {
            instance_maintenance_ongoing_count: core::default::Default::default(),
            instance_maintenance_pending_count: core::default::Default::default(),
            maintenance_ongoing_count: core::default::Default::default(),
            maintenance_pending_count: core::default::Default::default(),
            scheduling_type: core::default::Default::default(),
            subblock_infra_maintenance_ongoing_count: core::default::Default::default(),
            subblock_infra_maintenance_pending_count: core::default::Default::default(),
            upcoming_group_maintenance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ResourceStatus_ReservationMaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ResourceStatus_ReservationMaintenanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_ResourceStatus_ReservationMaintenanceRef {
        ComputeReservation_ResourceStatus_ReservationMaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ResourceStatus_ReservationMaintenanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_ongoing_count` after provisioning.\n"]
    pub fn instance_maintenance_ongoing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_maintenance_ongoing_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_pending_count` after provisioning.\n"]
    pub fn instance_maintenance_pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_maintenance_pending_count", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_ongoing_count` after provisioning.\n"]
    pub fn maintenance_ongoing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_ongoing_count", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_pending_count` after provisioning.\n"]
    pub fn maintenance_pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_pending_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling_type` after provisioning.\n"]
    pub fn scheduling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_ongoing_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_ongoing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_ongoing_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_pending_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_pending_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `upcoming_group_maintenance` after provisioning.\n"]
    pub fn upcoming_group_maintenance(
        &self,
    ) -> terrars::ListRef<ComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upcoming_group_maintenance", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_ResourceStatus_SpecificSkuAllocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_instance_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    utilizations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeReservation_ResourceStatus_SpecificSkuAllocation {
    #[doc = "Set the field `source_instance_template_id`.\n"]
    pub fn set_source_instance_template_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_instance_template_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `utilizations`.\n"]
    pub fn set_utilizations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.utilizations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_ResourceStatus_SpecificSkuAllocation {
    type O = terrars::BlockAssignable<ComputeReservation_ResourceStatus_SpecificSkuAllocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ResourceStatus_SpecificSkuAllocation {}

impl BuildComputeReservation_ResourceStatus_SpecificSkuAllocation {
    pub fn build(self) -> ComputeReservation_ResourceStatus_SpecificSkuAllocation {
        ComputeReservation_ResourceStatus_SpecificSkuAllocation {
            source_instance_template_id: core::default::Default::default(),
            utilizations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
        ComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_instance_template_id` after provisioning.\n"]
    pub fn source_instance_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `utilizations` after provisioning.\n"]
    pub fn utilizations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.utilizations", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_ResourceStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_info: std::option::Option<terrars::ListField<ComputeReservation_ResourceStatus_HealthInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_block_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_maintenance: std
    ::option
    ::Option<terrars::ListField<ComputeReservation_ResourceStatus_ReservationMaintenance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_sku_allocation: std
    ::option
    ::Option<terrars::ListField<ComputeReservation_ResourceStatus_SpecificSkuAllocation>>,
}

impl ComputeReservation_ResourceStatus {
    #[doc = "Set the field `health_info`.\n"]
    pub fn set_health_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeReservation_ResourceStatus_HealthInfo>>,
    ) -> Self {
        self.health_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reservation_block_count`.\n"]
    pub fn set_reservation_block_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.reservation_block_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reservation_maintenance`.\n"]
    pub fn set_reservation_maintenance(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeReservation_ResourceStatus_ReservationMaintenance>>,
    ) -> Self {
        self.reservation_maintenance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `specific_sku_allocation`.\n"]
    pub fn set_specific_sku_allocation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeReservation_ResourceStatus_SpecificSkuAllocation>>,
    ) -> Self {
        self.specific_sku_allocation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_ResourceStatus {
    type O = terrars::BlockAssignable<ComputeReservation_ResourceStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ResourceStatus {}

impl BuildComputeReservation_ResourceStatus {
    pub fn build(self) -> ComputeReservation_ResourceStatus {
        ComputeReservation_ResourceStatus {
            health_info: core::default::Default::default(),
            reservation_block_count: core::default::Default::default(),
            reservation_maintenance: core::default::Default::default(),
            specific_sku_allocation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ResourceStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ResourceStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservation_ResourceStatusRef {
        ComputeReservation_ResourceStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ResourceStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `health_info` after provisioning.\n"]
    pub fn health_info(&self) -> terrars::ListRef<ComputeReservation_ResourceStatus_HealthInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_info", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_block_count` after provisioning.\n"]
    pub fn reservation_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_block_count", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_maintenance` after provisioning.\n"]
    pub fn reservation_maintenance(
        &self,
    ) -> terrars::ListRef<ComputeReservation_ResourceStatus_ReservationMaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_maintenance", self.base))
    }

    #[doc = "Get a reference to the value of field `specific_sku_allocation` after provisioning.\n"]
    pub fn specific_sku_allocation(
        &self,
    ) -> terrars::ListRef<ComputeReservation_ResourceStatus_SpecificSkuAllocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_sku_allocation", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_DeleteAfterDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservation_DeleteAfterDuration {
    #[doc = "Set the field `nanos`.\nNumber of nanoseconds for the auto-delete duration."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nNumber of seconds for the auto-delete duration."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_DeleteAfterDuration {
    type O = terrars::BlockAssignable<ComputeReservation_DeleteAfterDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_DeleteAfterDuration {}

impl BuildComputeReservation_DeleteAfterDuration {
    pub fn build(self) -> ComputeReservation_DeleteAfterDuration {
        ComputeReservation_DeleteAfterDuration {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_DeleteAfterDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_DeleteAfterDurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservation_DeleteAfterDurationRef {
        ComputeReservation_DeleteAfterDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_DeleteAfterDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nNumber of nanoseconds for the auto-delete duration."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nNumber of seconds for the auto-delete duration."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeReservation_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the reservation. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_Params {
    type O = terrars::BlockAssignable<ComputeReservation_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_Params {}

impl BuildComputeReservation_Params {
    pub fn build(self) -> ComputeReservation_Params {
        ComputeReservation_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservation_ParamsRef {
        ComputeReservation_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the reservation. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_ReservationSharingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_share_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservation_ReservationSharingPolicy {
    #[doc =
        "Set the field `service_share_type`.\nSharing config for all Google Cloud services. Possible values: [\"ALLOW_ALL\", \"DISALLOW_ALL\"]"]
    pub fn set_service_share_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_share_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_ReservationSharingPolicy {
    type O = terrars::BlockAssignable<ComputeReservation_ReservationSharingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ReservationSharingPolicy {}

impl BuildComputeReservation_ReservationSharingPolicy {
    pub fn build(self) -> ComputeReservation_ReservationSharingPolicy {
        ComputeReservation_ReservationSharingPolicy { service_share_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ReservationSharingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ReservationSharingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_ReservationSharingPolicyRef {
        ComputeReservation_ReservationSharingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ReservationSharingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_share_type` after provisioning.\nSharing config for all Google Cloud services. Possible values: [\"ALLOW_ALL\", \"DISALLOW_ALL\"]"]
    pub fn service_share_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_share_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_ShareSettings_ProjectMap {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservation_ShareSettings_ProjectMap {
    #[doc =
        "Set the field `project_id`.\nThe project id/number, should be same as the key of this project config in the project map."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_ShareSettings_ProjectMap {
    type O = terrars::BlockAssignable<ComputeReservation_ShareSettings_ProjectMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ShareSettings_ProjectMap {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildComputeReservation_ShareSettings_ProjectMap {
    pub fn build(self) -> ComputeReservation_ShareSettings_ProjectMap {
        ComputeReservation_ShareSettings_ProjectMap {
            id: self.id,
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ShareSettings_ProjectMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ShareSettings_ProjectMapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_ShareSettings_ProjectMapRef {
        ComputeReservation_ShareSettings_ProjectMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ShareSettings_ProjectMapRef {
    #[allow(dead_code)]
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
struct ComputeReservation_ShareSettingsDynamic {
    project_map: std::option::Option<terrars::DynamicBlock<ComputeReservation_ShareSettings_ProjectMap>>,
}

#[derive(Serialize)]
pub struct ComputeReservation_ShareSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_map: std::option::Option<std::vec::Vec<ComputeReservation_ShareSettings_ProjectMap>>,
    dynamic: ComputeReservation_ShareSettingsDynamic,
}

impl ComputeReservation_ShareSettings {
    #[doc =
        "Set the field `share_type`.\nType of sharing for this shared-reservation Possible values: [\"LOCAL\", \"SPECIFIC_PROJECTS\"]"]
    pub fn set_share_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeReservation_ShareSettings_ProjectMap>>,
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

impl terrars::ToListMappable for ComputeReservation_ShareSettings {
    type O = terrars::BlockAssignable<ComputeReservation_ShareSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_ShareSettings {}

impl BuildComputeReservation_ShareSettings {
    pub fn build(self) -> ComputeReservation_ShareSettings {
        ComputeReservation_ShareSettings {
            share_type: core::default::Default::default(),
            project_map: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_ShareSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_ShareSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservation_ShareSettingsRef {
        ComputeReservation_ShareSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_ShareSettingsRef {
    #[allow(dead_code)]
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
pub struct ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    accelerator_count: terrars::PrimField<f64>,
    accelerator_type: terrars::PrimField<std::string::String>,
}

impl ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators { }

impl terrars::ToListMappable for ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    type O = terrars::BlockAssignable<ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    #[doc = "The number of the guest accelerator cards exposed to\nthis instance."]
    pub accelerator_count: terrars::PrimField<f64>,
    #[doc =
        "The full or partial URL of the accelerator type to\nattach to this instance. For example:\n'projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100'\n\nIf you are creating an instance template, specify only the accelerator name."]
    pub accelerator_type: terrars::PrimField<std::string::String>,
}

impl BuildComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    pub fn build(self) -> ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
        ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
            accelerator_count: self.accelerator_count,
            accelerator_type: self.accelerator_type,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
        ComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
    #[allow(dead_code)]
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
pub struct ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    disk_size_gb: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    #[doc =
        "Set the field `interface`.\nThe disk interface to use for attaching this disk. Default value: \"SCSI\" Possible values: [\"SCSI\", \"NVME\"]"]
    pub fn set_interface(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    type O = terrars::BlockAssignable<ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    #[doc = "The size of the disk in base-2 GB."]
    pub disk_size_gb: terrars::PrimField<f64>,
}

impl BuildComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    pub fn build(self) -> ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
        ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
            disk_size_gb: self.disk_size_gb,
            interface: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
        ComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
    #[allow(dead_code)]
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
struct ComputeReservation_SpecificReservation_InstancePropertiesDynamic {
    guest_accelerators: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators>>,
    local_ssds: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds>>,
}

#[derive(Serialize)]
pub struct ComputeReservation_SpecificReservation_InstanceProperties {
    machine_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerators: std
    ::option
    ::Option<std::vec::Vec<ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssds: std::option::Option<std::vec::Vec<ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds>>,
    dynamic: ComputeReservation_SpecificReservation_InstancePropertiesDynamic,
}

impl ComputeReservation_SpecificReservation_InstanceProperties {
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
                        ::BlockAssignable<ComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators>,
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
                        terrars::BlockAssignable<ComputeReservation_SpecificReservation_InstanceProperties_LocalSsds>,
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

impl terrars::ToListMappable for ComputeReservation_SpecificReservation_InstanceProperties {
    type O = terrars::BlockAssignable<ComputeReservation_SpecificReservation_InstanceProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_SpecificReservation_InstanceProperties {
    #[doc = "The name of the machine type to reserve."]
    pub machine_type: terrars::PrimField<std::string::String>,
}

impl BuildComputeReservation_SpecificReservation_InstanceProperties {
    pub fn build(self) -> ComputeReservation_SpecificReservation_InstanceProperties {
        ComputeReservation_SpecificReservation_InstanceProperties {
            machine_type: self.machine_type,
            min_cpu_platform: core::default::Default::default(),
            guest_accelerators: core::default::Default::default(),
            local_ssds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_SpecificReservation_InstancePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_SpecificReservation_InstancePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeReservation_SpecificReservation_InstancePropertiesRef {
        ComputeReservation_SpecificReservation_InstancePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_SpecificReservation_InstancePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location_hint` after provisioning.\nAn opaque location hint used to place the allocation close to other resources. This field is for use by internal tools that use the public API."]
    pub fn location_hint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_hint", self.base))
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
    ) -> terrars::ListRef<ComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssds` after provisioning.\n"]
    pub fn local_ssds(&self) -> terrars::ListRef<ComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_ssds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeReservation_SpecificReservationDynamic {
    instance_properties: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeReservation_SpecificReservation_InstanceProperties>>,
}

#[derive(Serialize)]
pub struct ComputeReservation_SpecificReservation {
    count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_instance_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_properties: std::option::Option<std::vec::Vec<ComputeReservation_SpecificReservation_InstanceProperties>>,
    dynamic: ComputeReservation_SpecificReservationDynamic,
}

impl ComputeReservation_SpecificReservation {
    #[doc =
        "Set the field `source_instance_template`.\nSpecifies the instance template to create the reservation. If you use this field, you must exclude the\ninstanceProperties field."]
    pub fn set_source_instance_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_instance_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_properties`.\n"]
    pub fn set_instance_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeReservation_SpecificReservation_InstanceProperties>>,
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

impl terrars::ToListMappable for ComputeReservation_SpecificReservation {
    type O = terrars::BlockAssignable<ComputeReservation_SpecificReservation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_SpecificReservation {
    #[doc = "The number of resources that are allocated."]
    pub count: terrars::PrimField<f64>,
}

impl BuildComputeReservation_SpecificReservation {
    pub fn build(self) -> ComputeReservation_SpecificReservation {
        ComputeReservation_SpecificReservation {
            count: self.count,
            source_instance_template: core::default::Default::default(),
            instance_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_SpecificReservationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_SpecificReservationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservation_SpecificReservationRef {
        ComputeReservation_SpecificReservationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_SpecificReservationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `assured_count` after provisioning.\nIndicates how many instances are actually usable currently."]
    pub fn assured_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assured_count", self.base))
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

    #[doc =
        "Get a reference to the value of field `source_instance_template` after provisioning.\nSpecifies the instance template to create the reservation. If you use this field, you must exclude the\ninstanceProperties field."]
    pub fn source_instance_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_template", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_properties` after provisioning.\n"]
    pub fn instance_properties(&self) -> terrars::ListRef<ComputeReservation_SpecificReservation_InstancePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeReservation_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeReservation_Timeouts {
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

impl terrars::ToListMappable for ComputeReservation_Timeouts {
    type O = terrars::BlockAssignable<ComputeReservation_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeReservation_Timeouts {}

impl BuildComputeReservation_Timeouts {
    pub fn build(self) -> ComputeReservation_Timeouts {
        ComputeReservation_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeReservation_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeReservation_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeReservation_TimeoutsRef {
        ComputeReservation_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeReservation_TimeoutsRef {
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
