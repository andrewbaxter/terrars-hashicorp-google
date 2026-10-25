#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetappVolumeReplicationData_ {
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
    delete_destination_volume: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_stopping: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_enabled: std::option::Option<terrars::PrimField<bool>>,
    replication_schedule: terrars::PrimField<std::string::String>,
    volume_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_mirror: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_volume_parameters: std
    ::option
    ::Option<std::vec::Vec<NetappVolumeReplication_DestinationVolumeParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetappVolumeReplication_Timeouts>,
    dynamic: NetappVolumeReplicationDynamic,
}

struct NetappVolumeReplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetappVolumeReplicationData_>,
}

#[derive(Clone)]
pub struct NetappVolumeReplication(std::rc::Rc<NetappVolumeReplication_>);

impl NetappVolumeReplication {
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
        "Set the field `delete_destination_volume`.\nA destination volume is created as part of replication creation. The destination volume will not became\nunder Terraform management unless you import it manually. If you delete the replication, this volume\nwill remain.\nSetting this parameter to true will delete the *current* destination volume when destroying the\nreplication. If you reversed the replication direction, this will be your former source volume!\nFor production use, it is recommended to keep this parameter false to avoid accidental volume\ndeletion. Handle with care. Default is false."]
    pub fn set_delete_destination_volume(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_destination_volume = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn description of this resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_stopping`.\nOnly replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING\ncurrently receives an update and stopping the update might be undesirable. Set this parameter to true\nto stop anyway. All data transferred to the destination will be discarded and content of destination\nvolume will remain at the state of the last successful update. Default is false."]
    pub fn set_force_stopping(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_stopping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc =
        "Set the field `replication_enabled`.\nSet to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write\nand act independently from the source volume.\nSet to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes\ndone to the destination volume with the content of the source volume."]
    pub fn set_replication_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replication_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for_mirror`.\nReplication resource state is independent of mirror_state. With enough data, it can take many hours\nfor mirror_state to reach MIRRORED. If you want Terraform to wait for the mirror to finish on\ncreate/stop/resume operations, set this parameter to true. Default is false."]
    pub fn set_wait_for_mirror(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_mirror = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_volume_parameters`.\n"]
    pub fn set_destination_volume_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolumeReplication_DestinationVolumeParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_volume_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_volume_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetappVolumeReplication_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the active directory. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_destination_volume` after provisioning.\nA destination volume is created as part of replication creation. The destination volume will not became\nunder Terraform management unless you import it manually. If you delete the replication, this volume\nwill remain.\nSetting this parameter to true will delete the *current* destination volume when destroying the\nreplication. If you reversed the replication direction, this will be your former source volume!\nFor production use, it is recommended to keep this parameter false to avoid accidental volume\ndeletion. Handle with care. Default is false."]
    pub fn delete_destination_volume(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_destination_volume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_volume` after provisioning.\nFull resource name of destination volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn destination_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_volume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_stopping` after provisioning.\nOnly replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING\ncurrently receives an update and stopping the update might be undesirable. Set this parameter to true\nto stop anyway. All data transferred to the destination will be discarded and content of destination\nvolume will remain at the state of the last successful update. Default is false."]
    pub fn force_stopping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_stopping", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthy` after provisioning.\nCondition of the relationship. Can be one of the following:\n  - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer.\n  - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer."]
    pub fn healthy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_peering_details` after provisioning.\nHybridPeeringDetails contains details about the hybrid peering."]
    pub fn hybrid_peering_details(&self) -> terrars::ListRef<NetappVolumeReplication_HybridPeeringDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hybrid_peering_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_replication_type` after provisioning.\nHybrid replication type."]
    pub fn hybrid_replication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hybrid_replication_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_replication_user_commands` after provisioning.\nCopy pastable snapmirror commands to be executed on onprem cluster by the customer."]
    pub fn hybrid_replication_user_commands(
        &self,
    ) -> terrars::ListRef<NetappVolumeReplication_HybridReplicationUserCommandsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_user_commands", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nName of region for this resource. The resource needs to be created in the region of the destination volume."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mirror_state` after provisioning.\nIndicates the state of the mirror between source and destination volumes. Depending on the amount of data\nin your source volume, PREPARING phase can take hours or days. mirrorState = MIRRORED indicates your baseline\ntransfer ended and destination volume became accessible read-only. TRANSFERRING means a MIRRORED volume\ncurrently receives an update. Updated every 5 minutes."]
    pub fn mirror_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirror_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the replication. Needs to be unique per location."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replication_enabled` after provisioning.\nSet to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write\nand act independently from the source volume.\nSet to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes\ndone to the destination volume with the content of the source volume."]
    pub fn replication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replication_schedule` after provisioning.\nSpecifies the replication interval. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn replication_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nReverting a replication can swap source and destination volume roles. This field indicates if the 'location' hosts\nthe source or destination volume. For resume and revert and resume operations it is critical to understand\nwhich volume is the source volume, since it will overwrite changes done to the destination volume."]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_volume` after provisioning.\nFull resource name of source volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn source_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_volume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nIndicates the state of replication resource. State of the mirror itself is indicated in mirrorState."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_details` after provisioning.\nState details of the replication resource."]
    pub fn state_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_stats` after provisioning.\nReplication transfer statistics. All statistics are updated every 5 minutes."]
    pub fn transfer_stats(&self) -> terrars::ListRef<NetappVolumeReplication_TransferStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_stats", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `volume_name` after provisioning.\nThe name of the existing source volume."]
    pub fn volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_mirror` after provisioning.\nReplication resource state is independent of mirror_state. With enough data, it can take many hours\nfor mirror_state to reach MIRRORED. If you want Terraform to wait for the mirror to finish on\ncreate/stop/resume operations, set this parameter to true. Default is false."]
    pub fn wait_for_mirror(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_mirror", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_volume_parameters` after provisioning.\n"]
    pub fn destination_volume_parameters(
        &self,
    ) -> terrars::ListRef<NetappVolumeReplication_DestinationVolumeParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_volume_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolumeReplication_TimeoutsRef {
        NetappVolumeReplication_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for NetappVolumeReplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetappVolumeReplication { }

impl terrars::ToListMappable for NetappVolumeReplication {
    type O = terrars::ListRef<NetappVolumeReplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetappVolumeReplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_netapp_volume_replication".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetappVolumeReplication {
    pub tf_id: std::string::String,
    #[doc =
        "Name of region for this resource. The resource needs to be created in the region of the destination volume."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the replication. Needs to be unique per location."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Specifies the replication interval. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub replication_schedule: terrars::PrimField<std::string::String>,
    #[doc = "The name of the existing source volume."]
    pub volume_name: terrars::PrimField<std::string::String>,
}

impl BuildNetappVolumeReplication {
    pub fn build(self, stack: &mut terrars::Stack) -> NetappVolumeReplication {
        let out = NetappVolumeReplication(std::rc::Rc::new(NetappVolumeReplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetappVolumeReplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delete_destination_volume: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                force_stopping: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                replication_enabled: core::default::Default::default(),
                replication_schedule: self.replication_schedule,
                volume_name: self.volume_name,
                wait_for_mirror: core::default::Default::default(),
                destination_volume_parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetappVolumeReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetappVolumeReplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the active directory. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_destination_volume` after provisioning.\nA destination volume is created as part of replication creation. The destination volume will not became\nunder Terraform management unless you import it manually. If you delete the replication, this volume\nwill remain.\nSetting this parameter to true will delete the *current* destination volume when destroying the\nreplication. If you reversed the replication direction, this will be your former source volume!\nFor production use, it is recommended to keep this parameter false to avoid accidental volume\ndeletion. Handle with care. Default is false."]
    pub fn delete_destination_volume(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_destination_volume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_volume` after provisioning.\nFull resource name of destination volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn destination_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_volume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_stopping` after provisioning.\nOnly replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING\ncurrently receives an update and stopping the update might be undesirable. Set this parameter to true\nto stop anyway. All data transferred to the destination will be discarded and content of destination\nvolume will remain at the state of the last successful update. Default is false."]
    pub fn force_stopping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_stopping", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthy` after provisioning.\nCondition of the relationship. Can be one of the following:\n  - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer.\n  - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer."]
    pub fn healthy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_peering_details` after provisioning.\nHybridPeeringDetails contains details about the hybrid peering."]
    pub fn hybrid_peering_details(&self) -> terrars::ListRef<NetappVolumeReplication_HybridPeeringDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hybrid_peering_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_replication_type` after provisioning.\nHybrid replication type."]
    pub fn hybrid_replication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hybrid_replication_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_replication_user_commands` after provisioning.\nCopy pastable snapmirror commands to be executed on onprem cluster by the customer."]
    pub fn hybrid_replication_user_commands(
        &self,
    ) -> terrars::ListRef<NetappVolumeReplication_HybridReplicationUserCommandsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_user_commands", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nName of region for this resource. The resource needs to be created in the region of the destination volume."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mirror_state` after provisioning.\nIndicates the state of the mirror between source and destination volumes. Depending on the amount of data\nin your source volume, PREPARING phase can take hours or days. mirrorState = MIRRORED indicates your baseline\ntransfer ended and destination volume became accessible read-only. TRANSFERRING means a MIRRORED volume\ncurrently receives an update. Updated every 5 minutes."]
    pub fn mirror_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirror_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the replication. Needs to be unique per location."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replication_enabled` after provisioning.\nSet to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write\nand act independently from the source volume.\nSet to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes\ndone to the destination volume with the content of the source volume."]
    pub fn replication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replication_schedule` after provisioning.\nSpecifies the replication interval. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn replication_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nReverting a replication can swap source and destination volume roles. This field indicates if the 'location' hosts\nthe source or destination volume. For resume and revert and resume operations it is critical to understand\nwhich volume is the source volume, since it will overwrite changes done to the destination volume."]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_volume` after provisioning.\nFull resource name of source volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn source_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_volume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nIndicates the state of replication resource. State of the mirror itself is indicated in mirrorState."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_details` after provisioning.\nState details of the replication resource."]
    pub fn state_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_stats` after provisioning.\nReplication transfer statistics. All statistics are updated every 5 minutes."]
    pub fn transfer_stats(&self) -> terrars::ListRef<NetappVolumeReplication_TransferStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_stats", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `volume_name` after provisioning.\nThe name of the existing source volume."]
    pub fn volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_mirror` after provisioning.\nReplication resource state is independent of mirror_state. With enough data, it can take many hours\nfor mirror_state to reach MIRRORED. If you want Terraform to wait for the mirror to finish on\ncreate/stop/resume operations, set this parameter to true. Default is false."]
    pub fn wait_for_mirror(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_mirror", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_volume_parameters` after provisioning.\n"]
    pub fn destination_volume_parameters(
        &self,
    ) -> terrars::ListRef<NetappVolumeReplication_DestinationVolumeParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_volume_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolumeReplication_TimeoutsRef {
        NetappVolumeReplication_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetappVolumeReplicationDynamic {
    destination_volume_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<NetappVolumeReplication_DestinationVolumeParameters>>,
}

#[derive(Serialize)]
pub struct NetappVolumeReplication_HybridPeeringDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command_expiry_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passphrase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_svm_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_volume_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolumeReplication_HybridPeeringDetails {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command_expiry_time`.\n"]
    pub fn set_command_expiry_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.command_expiry_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `passphrase`.\n"]
    pub fn set_passphrase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.passphrase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_cluster_name`.\n"]
    pub fn set_peer_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_svm_name`.\n"]
    pub fn set_peer_svm_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_svm_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_volume_name`.\n"]
    pub fn set_peer_volume_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_volume_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ip`.\n"]
    pub fn set_subnet_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolumeReplication_HybridPeeringDetails {
    type O = terrars::BlockAssignable<NetappVolumeReplication_HybridPeeringDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolumeReplication_HybridPeeringDetails {}

impl BuildNetappVolumeReplication_HybridPeeringDetails {
    pub fn build(self) -> NetappVolumeReplication_HybridPeeringDetails {
        NetappVolumeReplication_HybridPeeringDetails {
            command: core::default::Default::default(),
            command_expiry_time: core::default::Default::default(),
            passphrase: core::default::Default::default(),
            peer_cluster_name: core::default::Default::default(),
            peer_svm_name: core::default::Default::default(),
            peer_volume_name: core::default::Default::default(),
            subnet_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolumeReplication_HybridPeeringDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeReplication_HybridPeeringDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetappVolumeReplication_HybridPeeringDetailsRef {
        NetappVolumeReplication_HybridPeeringDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolumeReplication_HybridPeeringDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `command_expiry_time` after provisioning.\n"]
    pub fn command_expiry_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command_expiry_time", self.base))
    }

    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\n"]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_cluster_name` after provisioning.\n"]
    pub fn peer_cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_svm_name` after provisioning.\n"]
    pub fn peer_svm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_svm_name", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_volume_name` after provisioning.\n"]
    pub fn peer_volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_volume_name", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ip` after provisioning.\n"]
    pub fn subnet_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolumeReplication_HybridReplicationUserCommands {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commands: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetappVolumeReplication_HybridReplicationUserCommands {
    #[doc = "Set the field `commands`.\n"]
    pub fn set_commands(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.commands = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolumeReplication_HybridReplicationUserCommands {
    type O = terrars::BlockAssignable<NetappVolumeReplication_HybridReplicationUserCommands>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolumeReplication_HybridReplicationUserCommands {}

impl BuildNetappVolumeReplication_HybridReplicationUserCommands {
    pub fn build(self) -> NetappVolumeReplication_HybridReplicationUserCommands {
        NetappVolumeReplication_HybridReplicationUserCommands { commands: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetappVolumeReplication_HybridReplicationUserCommandsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeReplication_HybridReplicationUserCommandsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetappVolumeReplication_HybridReplicationUserCommandsRef {
        NetappVolumeReplication_HybridReplicationUserCommandsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolumeReplication_HybridReplicationUserCommandsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commands` after provisioning.\n"]
    pub fn commands(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.commands", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolumeReplication_TransferStats {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lag_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transfer_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transfer_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transfer_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transfer_error: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_transfer_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolumeReplication_TransferStats {
    #[doc = "Set the field `lag_duration`.\n"]
    pub fn set_lag_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lag_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_transfer_bytes`.\n"]
    pub fn set_last_transfer_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_transfer_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_transfer_duration`.\n"]
    pub fn set_last_transfer_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_transfer_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_transfer_end_time`.\n"]
    pub fn set_last_transfer_end_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_transfer_end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_transfer_error`.\n"]
    pub fn set_last_transfer_error(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_transfer_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_transfer_duration`.\n"]
    pub fn set_total_transfer_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_transfer_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_bytes`.\n"]
    pub fn set_transfer_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.transfer_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolumeReplication_TransferStats {
    type O = terrars::BlockAssignable<NetappVolumeReplication_TransferStats>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolumeReplication_TransferStats {}

impl BuildNetappVolumeReplication_TransferStats {
    pub fn build(self) -> NetappVolumeReplication_TransferStats {
        NetappVolumeReplication_TransferStats {
            lag_duration: core::default::Default::default(),
            last_transfer_bytes: core::default::Default::default(),
            last_transfer_duration: core::default::Default::default(),
            last_transfer_end_time: core::default::Default::default(),
            last_transfer_error: core::default::Default::default(),
            total_transfer_duration: core::default::Default::default(),
            transfer_bytes: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolumeReplication_TransferStatsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeReplication_TransferStatsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolumeReplication_TransferStatsRef {
        NetappVolumeReplication_TransferStatsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolumeReplication_TransferStatsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lag_duration` after provisioning.\n"]
    pub fn lag_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lag_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `last_transfer_bytes` after provisioning.\n"]
    pub fn last_transfer_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_transfer_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `last_transfer_duration` after provisioning.\n"]
    pub fn last_transfer_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_transfer_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `last_transfer_end_time` after provisioning.\n"]
    pub fn last_transfer_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_transfer_end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `last_transfer_error` after provisioning.\n"]
    pub fn last_transfer_error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_transfer_error", self.base))
    }

    #[doc = "Get a reference to the value of field `total_transfer_duration` after provisioning.\n"]
    pub fn total_transfer_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_transfer_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_bytes` after provisioning.\n"]
    pub fn transfer_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooling_threshold_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier_action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy {
    #[doc =
        "Set the field `cooling_threshold_days`.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn set_cooling_threshold_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cooling_threshold_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tier_action`.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn set_tier_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy {
    type O = terrars::BlockAssignable<NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolumeReplication_DestinationVolumeParameters_TieringPolicy {}

impl BuildNetappVolumeReplication_DestinationVolumeParameters_TieringPolicy {
    pub fn build(self) -> NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy {
        NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy {
            cooling_threshold_days: core::default::Default::default(),
            tier_action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolumeReplication_DestinationVolumeParameters_TieringPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeReplication_DestinationVolumeParameters_TieringPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetappVolumeReplication_DestinationVolumeParameters_TieringPolicyRef {
        NetappVolumeReplication_DestinationVolumeParameters_TieringPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolumeReplication_DestinationVolumeParameters_TieringPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cooling_threshold_days` after provisioning.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn cooling_threshold_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooling_threshold_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tier_action` after provisioning.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn tier_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetappVolumeReplication_DestinationVolumeParametersDynamic {
    tiering_policy: std
    ::option
    ::Option<terrars::DynamicBlock<NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy>>,
}

#[derive(Serialize)]
pub struct NetappVolumeReplication_DestinationVolumeParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_name: std::option::Option<terrars::PrimField<std::string::String>>,
    storage_pool: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiering_policy: std::option::Option<std::vec::Vec<NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy>>,
    dynamic: NetappVolumeReplication_DestinationVolumeParametersDynamic,
}

impl NetappVolumeReplication_DestinationVolumeParameters {
    #[doc = "Set the field `description`.\nDescription for the destination volume."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `share_name`.\nShare name for destination volume. If not specified, name of source volume's share name will be used."]
    pub fn set_share_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_id`.\nName for the destination volume to be created. If not specified, the name of the source volume will be used."]
    pub fn set_volume_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tiering_policy`.\n"]
    pub fn set_tiering_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetappVolumeReplication_DestinationVolumeParameters_TieringPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tiering_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tiering_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetappVolumeReplication_DestinationVolumeParameters {
    type O = terrars::BlockAssignable<NetappVolumeReplication_DestinationVolumeParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolumeReplication_DestinationVolumeParameters {
    #[doc =
        "Name of an existing storage pool for the destination volume with format: 'projects/{{project}}/locations/{{location}}/storagePools/{{poolId}}'"]
    pub storage_pool: terrars::PrimField<std::string::String>,
}

impl BuildNetappVolumeReplication_DestinationVolumeParameters {
    pub fn build(self) -> NetappVolumeReplication_DestinationVolumeParameters {
        NetappVolumeReplication_DestinationVolumeParameters {
            description: core::default::Default::default(),
            share_name: core::default::Default::default(),
            storage_pool: self.storage_pool,
            volume_id: core::default::Default::default(),
            tiering_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolumeReplication_DestinationVolumeParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeReplication_DestinationVolumeParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetappVolumeReplication_DestinationVolumeParametersRef {
        NetappVolumeReplication_DestinationVolumeParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolumeReplication_DestinationVolumeParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription for the destination volume."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `share_name` after provisioning.\nShare name for destination volume. If not specified, name of source volume's share name will be used."]
    pub fn share_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool` after provisioning.\nName of an existing storage pool for the destination volume with format: 'projects/{{project}}/locations/{{location}}/storagePools/{{poolId}}'"]
    pub fn storage_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_id` after provisioning.\nName for the destination volume to be created. If not specified, the name of the source volume will be used."]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(
        &self,
    ) -> terrars::ListRef<NetappVolumeReplication_DestinationVolumeParameters_TieringPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiering_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolumeReplication_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolumeReplication_Timeouts {
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

impl terrars::ToListMappable for NetappVolumeReplication_Timeouts {
    type O = terrars::BlockAssignable<NetappVolumeReplication_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolumeReplication_Timeouts {}

impl BuildNetappVolumeReplication_Timeouts {
    pub fn build(self) -> NetappVolumeReplication_Timeouts {
        NetappVolumeReplication_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolumeReplication_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeReplication_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolumeReplication_TimeoutsRef {
        NetappVolumeReplication_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolumeReplication_TimeoutsRef {
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
