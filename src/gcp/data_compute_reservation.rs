#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeReservationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    zone: terrars::PrimField<std::string::String>,
}

struct DataComputeReservation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeReservationData_>,
}

#[derive(Clone)]
pub struct DataComputeReservation(std::rc::Rc<DataComputeReservation_>);

impl DataComputeReservation {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `delete_after_duration` after provisioning.\nDuration after which the reservation will be auto-deleted by Compute Engine. Cannot be used with delete_at_time."]
    pub fn delete_after_duration(&self) -> terrars::ListRef<DataComputeReservation_DeleteAfterDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_after_duration", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> terrars::ListRef<DataComputeReservation_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\nSharing policy for reservations with Google Cloud managed services."]
    pub fn reservation_sharing_policy(&self) -> terrars::ListRef<DataComputeReservation_ReservationSharingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_sharing_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> terrars::ListRef<DataComputeReservation_ResourceStatusRef> {
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
        "Get a reference to the value of field `share_settings` after provisioning.\nThe share setting for reservations."]
    pub fn share_settings(&self) -> terrars::ListRef<DataComputeReservation_ShareSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `specific_reservation` after provisioning.\nReservation for instances with specific machine shapes."]
    pub fn specific_reservation(&self) -> terrars::ListRef<DataComputeReservation_SpecificReservationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
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
}

impl terrars::Referable for DataComputeReservation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeReservation { }

impl terrars::ToListMappable for DataComputeReservation {
    type O = terrars::ListRef<DataComputeReservationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeReservation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_reservation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeReservation {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The zone where the reservation is made."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeReservation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeReservation {
        let out = DataComputeReservation(std::rc::Rc::new(DataComputeReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeReservationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                project: core::default::Default::default(),
                zone: self.zone,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeReservationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeReservationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `delete_after_duration` after provisioning.\nDuration after which the reservation will be auto-deleted by Compute Engine. Cannot be used with delete_at_time."]
    pub fn delete_after_duration(&self) -> terrars::ListRef<DataComputeReservation_DeleteAfterDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_after_duration", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> terrars::ListRef<DataComputeReservation_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\nSharing policy for reservations with Google Cloud managed services."]
    pub fn reservation_sharing_policy(&self) -> terrars::ListRef<DataComputeReservation_ReservationSharingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_sharing_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> terrars::ListRef<DataComputeReservation_ResourceStatusRef> {
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
        "Get a reference to the value of field `share_settings` after provisioning.\nThe share setting for reservations."]
    pub fn share_settings(&self) -> terrars::ListRef<DataComputeReservation_ShareSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `specific_reservation` after provisioning.\nReservation for instances with specific machine shapes."]
    pub fn specific_reservation(&self) -> terrars::ListRef<DataComputeReservation_SpecificReservationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataComputeReservation_DeleteAfterDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_DeleteAfterDuration {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_DeleteAfterDuration {
    type O = terrars::BlockAssignable<DataComputeReservation_DeleteAfterDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_DeleteAfterDuration {}

impl BuildDataComputeReservation_DeleteAfterDuration {
    pub fn build(self) -> DataComputeReservation_DeleteAfterDuration {
        DataComputeReservation_DeleteAfterDuration {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_DeleteAfterDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_DeleteAfterDurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeReservation_DeleteAfterDurationRef {
        DataComputeReservation_DeleteAfterDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_DeleteAfterDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeReservation_Params {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_Params {
    type O = terrars::BlockAssignable<DataComputeReservation_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_Params {}

impl BuildDataComputeReservation_Params {
    pub fn build(self) -> DataComputeReservation_Params {
        DataComputeReservation_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeReservation_ParamsRef {
        DataComputeReservation_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_ReservationSharingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_share_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_ReservationSharingPolicy {
    #[doc = "Set the field `service_share_type`.\n"]
    pub fn set_service_share_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_share_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_ReservationSharingPolicy {
    type O = terrars::BlockAssignable<DataComputeReservation_ReservationSharingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ReservationSharingPolicy {}

impl BuildDataComputeReservation_ReservationSharingPolicy {
    pub fn build(self) -> DataComputeReservation_ReservationSharingPolicy {
        DataComputeReservation_ReservationSharingPolicy { service_share_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_ReservationSharingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ReservationSharingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_ReservationSharingPolicyRef {
        DataComputeReservation_ReservationSharingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ReservationSharingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_share_type` after provisioning.\n"]
    pub fn service_share_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_share_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_ResourceStatus_HealthInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    degraded_block_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_block_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeReservation_ResourceStatus_HealthInfo {
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

impl terrars::ToListMappable for DataComputeReservation_ResourceStatus_HealthInfo {
    type O = terrars::BlockAssignable<DataComputeReservation_ResourceStatus_HealthInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ResourceStatus_HealthInfo {}

impl BuildDataComputeReservation_ResourceStatus_HealthInfo {
    pub fn build(self) -> DataComputeReservation_ResourceStatus_HealthInfo {
        DataComputeReservation_ResourceStatus_HealthInfo {
            degraded_block_count: core::default::Default::default(),
            health_status: core::default::Default::default(),
            healthy_block_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_ResourceStatus_HealthInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ResourceStatus_HealthInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_ResourceStatus_HealthInfoRef {
        DataComputeReservation_ResourceStatus_HealthInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ResourceStatus_HealthInfoRef {
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
pub struct DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
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

impl DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
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

impl terrars
::ToListMappable for DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
    type O =
        terrars
        ::BlockAssignable<DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {}

impl BuildDataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
    pub fn build(self) -> DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
        DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance {
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
pub struct DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
        DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef {
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
pub struct DataComputeReservation_ResourceStatus_ReservationMaintenance {
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
    ::Option<
        terrars::ListField<DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance>,
    >,
}

impl DataComputeReservation_ResourceStatus_ReservationMaintenance {
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
                        ::ListField<
                            DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenance,
                        >,
                    >,
    ) -> Self {
        self.upcoming_group_maintenance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_ResourceStatus_ReservationMaintenance {
    type O = terrars::BlockAssignable<DataComputeReservation_ResourceStatus_ReservationMaintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ResourceStatus_ReservationMaintenance {}

impl BuildDataComputeReservation_ResourceStatus_ReservationMaintenance {
    pub fn build(self) -> DataComputeReservation_ResourceStatus_ReservationMaintenance {
        DataComputeReservation_ResourceStatus_ReservationMaintenance {
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
pub struct DataComputeReservation_ResourceStatus_ReservationMaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ResourceStatus_ReservationMaintenanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_ResourceStatus_ReservationMaintenanceRef {
        DataComputeReservation_ResourceStatus_ReservationMaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ResourceStatus_ReservationMaintenanceRef {
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
    ) -> terrars::ListRef<DataComputeReservation_ResourceStatus_ReservationMaintenance_UpcomingGroupMaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upcoming_group_maintenance", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_ResourceStatus_SpecificSkuAllocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_instance_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    utilizations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeReservation_ResourceStatus_SpecificSkuAllocation {
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

impl terrars::ToListMappable for DataComputeReservation_ResourceStatus_SpecificSkuAllocation {
    type O = terrars::BlockAssignable<DataComputeReservation_ResourceStatus_SpecificSkuAllocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ResourceStatus_SpecificSkuAllocation {}

impl BuildDataComputeReservation_ResourceStatus_SpecificSkuAllocation {
    pub fn build(self) -> DataComputeReservation_ResourceStatus_SpecificSkuAllocation {
        DataComputeReservation_ResourceStatus_SpecificSkuAllocation {
            source_instance_template_id: core::default::Default::default(),
            utilizations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
        DataComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ResourceStatus_SpecificSkuAllocationRef {
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
pub struct DataComputeReservation_ResourceStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_info: std::option::Option<terrars::ListField<DataComputeReservation_ResourceStatus_HealthInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_block_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_maintenance: std
    ::option
    ::Option<terrars::ListField<DataComputeReservation_ResourceStatus_ReservationMaintenance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_sku_allocation: std
    ::option
    ::Option<terrars::ListField<DataComputeReservation_ResourceStatus_SpecificSkuAllocation>>,
}

impl DataComputeReservation_ResourceStatus {
    #[doc = "Set the field `health_info`.\n"]
    pub fn set_health_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeReservation_ResourceStatus_HealthInfo>>,
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
        v: impl std::convert::Into<terrars::ListField<DataComputeReservation_ResourceStatus_ReservationMaintenance>>,
    ) -> Self {
        self.reservation_maintenance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `specific_sku_allocation`.\n"]
    pub fn set_specific_sku_allocation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeReservation_ResourceStatus_SpecificSkuAllocation>>,
    ) -> Self {
        self.specific_sku_allocation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_ResourceStatus {
    type O = terrars::BlockAssignable<DataComputeReservation_ResourceStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ResourceStatus {}

impl BuildDataComputeReservation_ResourceStatus {
    pub fn build(self) -> DataComputeReservation_ResourceStatus {
        DataComputeReservation_ResourceStatus {
            health_info: core::default::Default::default(),
            reservation_block_count: core::default::Default::default(),
            reservation_maintenance: core::default::Default::default(),
            specific_sku_allocation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_ResourceStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ResourceStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeReservation_ResourceStatusRef {
        DataComputeReservation_ResourceStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ResourceStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `health_info` after provisioning.\n"]
    pub fn health_info(&self) -> terrars::ListRef<DataComputeReservation_ResourceStatus_HealthInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_info", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_block_count` after provisioning.\n"]
    pub fn reservation_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_block_count", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_maintenance` after provisioning.\n"]
    pub fn reservation_maintenance(
        &self,
    ) -> terrars::ListRef<DataComputeReservation_ResourceStatus_ReservationMaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_maintenance", self.base))
    }

    #[doc = "Get a reference to the value of field `specific_sku_allocation` after provisioning.\n"]
    pub fn specific_sku_allocation(
        &self,
    ) -> terrars::ListRef<DataComputeReservation_ResourceStatus_SpecificSkuAllocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_sku_allocation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_ShareSettings_ProjectMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_ShareSettings_ProjectMap {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_ShareSettings_ProjectMap {
    type O = terrars::BlockAssignable<DataComputeReservation_ShareSettings_ProjectMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ShareSettings_ProjectMap {}

impl BuildDataComputeReservation_ShareSettings_ProjectMap {
    pub fn build(self) -> DataComputeReservation_ShareSettings_ProjectMap {
        DataComputeReservation_ShareSettings_ProjectMap {
            id: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_ShareSettings_ProjectMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ShareSettings_ProjectMapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_ShareSettings_ProjectMapRef {
        DataComputeReservation_ShareSettings_ProjectMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ShareSettings_ProjectMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_ShareSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_map: std::option::Option<terrars::SetField<DataComputeReservation_ShareSettings_ProjectMap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_ShareSettings {
    #[doc = "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataComputeReservation_ShareSettings_ProjectMap>>,
    ) -> Self {
        self.project_map = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_type`.\n"]
    pub fn set_share_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_ShareSettings {
    type O = terrars::BlockAssignable<DataComputeReservation_ShareSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_ShareSettings {}

impl BuildDataComputeReservation_ShareSettings {
    pub fn build(self) -> DataComputeReservation_ShareSettings {
        DataComputeReservation_ShareSettings {
            project_map: core::default::Default::default(),
            share_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_ShareSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_ShareSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeReservation_ShareSettingsRef {
        DataComputeReservation_ShareSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_ShareSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `project_map` after provisioning.\n"]
    pub fn project_map(&self) -> terrars::SetRef<DataComputeReservation_ShareSettings_ProjectMapRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.project_map", self.base))
    }

    #[doc = "Get a reference to the value of field `share_type` after provisioning.\n"]
    pub fn share_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    type O =
        terrars::BlockAssignable<DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {}

impl BuildDataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
    pub fn build(self) -> DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
        DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
        DataComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface`.\n"]
    pub fn set_interface(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    type O = terrars::BlockAssignable<DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {}

impl BuildDataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
    pub fn build(self) -> DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
        DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds {
            disk_size_gb: core::default::Default::default(),
            interface: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
        DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `interface` after provisioning.\n"]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_SpecificReservation_InstanceProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerators: std
    ::option
    ::Option<terrars::ListField<DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssds: std
    ::option
    ::Option<terrars::ListField<DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_hint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_SpecificReservation_InstanceProperties {
    #[doc = "Set the field `guest_accelerators`.\n"]
    pub fn set_guest_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataComputeReservation_SpecificReservation_InstanceProperties_GuestAccelerators>,
                    >,
    ) -> Self {
        self.guest_accelerators = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsds>,
                    >,
    ) -> Self {
        self.local_ssds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location_hint`.\n"]
    pub fn set_location_hint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_hint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_SpecificReservation_InstanceProperties {
    type O = terrars::BlockAssignable<DataComputeReservation_SpecificReservation_InstanceProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_SpecificReservation_InstanceProperties {}

impl BuildDataComputeReservation_SpecificReservation_InstanceProperties {
    pub fn build(self) -> DataComputeReservation_SpecificReservation_InstanceProperties {
        DataComputeReservation_SpecificReservation_InstanceProperties {
            guest_accelerators: core::default::Default::default(),
            local_ssds: core::default::Default::default(),
            location_hint: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_SpecificReservation_InstancePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_SpecificReservation_InstancePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservation_SpecificReservation_InstancePropertiesRef {
        DataComputeReservation_SpecificReservation_InstancePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_SpecificReservation_InstancePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guest_accelerators` after provisioning.\n"]
    pub fn guest_accelerators(
        &self,
    ) -> terrars::ListRef<DataComputeReservation_SpecificReservation_InstanceProperties_GuestAcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssds` after provisioning.\n"]
    pub fn local_ssds(
        &self,
    ) -> terrars::ListRef<DataComputeReservation_SpecificReservation_InstanceProperties_LocalSsdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_ssds", self.base))
    }

    #[doc = "Get a reference to the value of field `location_hint` after provisioning.\n"]
    pub fn location_hint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_hint", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservation_SpecificReservation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assured_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    in_use_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_properties: std
    ::option
    ::Option<terrars::ListField<DataComputeReservation_SpecificReservation_InstanceProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_instance_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservation_SpecificReservation {
    #[doc = "Set the field `assured_count`.\n"]
    pub fn set_assured_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.assured_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `in_use_count`.\n"]
    pub fn set_in_use_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.in_use_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_properties`.\n"]
    pub fn set_instance_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeReservation_SpecificReservation_InstanceProperties>>,
    ) -> Self {
        self.instance_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_instance_template`.\n"]
    pub fn set_source_instance_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_instance_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservation_SpecificReservation {
    type O = terrars::BlockAssignable<DataComputeReservation_SpecificReservation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservation_SpecificReservation {}

impl BuildDataComputeReservation_SpecificReservation {
    pub fn build(self) -> DataComputeReservation_SpecificReservation {
        DataComputeReservation_SpecificReservation {
            assured_count: core::default::Default::default(),
            count: core::default::Default::default(),
            in_use_count: core::default::Default::default(),
            instance_properties: core::default::Default::default(),
            source_instance_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservation_SpecificReservationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservation_SpecificReservationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeReservation_SpecificReservationRef {
        DataComputeReservation_SpecificReservationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservation_SpecificReservationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assured_count` after provisioning.\n"]
    pub fn assured_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assured_count", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `in_use_count` after provisioning.\n"]
    pub fn in_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_use_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_properties` after provisioning.\n"]
    pub fn instance_properties(
        &self,
    ) -> terrars::ListRef<DataComputeReservation_SpecificReservation_InstancePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `source_instance_template` after provisioning.\n"]
    pub fn source_instance_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_template", self.base))
    }
}
