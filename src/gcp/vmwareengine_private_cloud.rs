#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VmwareenginePrivateCloudData_ {
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
    deletion_delay_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_deletion_delay_hours_if_zero: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management_cluster: std::option::Option<std::vec::Vec<VmwareenginePrivateCloud_ManagementCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<VmwareenginePrivateCloud_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VmwareenginePrivateCloud_Timeouts>,
    dynamic: VmwareenginePrivateCloudDynamic,
}

struct VmwareenginePrivateCloud_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VmwareenginePrivateCloudData_>,
}

#[derive(Clone)]
pub struct VmwareenginePrivateCloud(std::rc::Rc<VmwareenginePrivateCloud_>);

impl VmwareenginePrivateCloud {
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
        "Set the field `deletion_delay_hours`.\nThe number of hours to delay this request. You can set this value to an hour between 0 to 8, where setting it to 0 starts the deletion request immediately. If no value is set, a default value is set at the API Level."]
    pub fn set_deletion_delay_hours(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deletion_delay_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nUser-provided description for this private cloud."]
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
        "Set the field `send_deletion_delay_hours_if_zero`.\nWhile set true, deletion_delay_hours value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the deletion_delay_hours field. It can be used both alone and together with deletion_delay_hours."]
    pub fn set_send_deletion_delay_hours_if_zero(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().send_deletion_delay_hours_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nInitial type of the private cloud. Possible values: [\"STANDARD\", \"TIME_LIMITED\", \"STRETCHED\"]"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management_cluster`.\n"]
    pub fn set_management_cluster(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VmwareenginePrivateCloud_NetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VmwareenginePrivateCloud_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nTime when the resource was scheduled for deletion.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_delay_hours` after provisioning.\nThe number of hours to delay this request. You can set this value to an hour between 0 to 8, where setting it to 0 starts the deletion request immediately. If no value is set, a default value is set at the API Level."]
    pub fn deletion_delay_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_delay_hours", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nTime when the resource will be irreversibly deleted.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> terrars::ListRef<VmwareenginePrivateCloud_HcxRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> terrars::ListRef<VmwareenginePrivateCloud_NsxRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nsx", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `send_deletion_delay_hours_if_zero` after provisioning.\nWhile set true, deletion_delay_hours value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the deletion_delay_hours field. It can be used both alone and together with deletion_delay_hours."]
    pub fn send_deletion_delay_hours_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_deletion_delay_hours_if_zero", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of the resource. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nInitial type of the private cloud. Possible values: [\"STANDARD\", \"TIME_LIMITED\", \"STRETCHED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast update time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vcenter` after provisioning.\nDetails about a vCenter Server management appliance."]
    pub fn vcenter(&self) -> terrars::ListRef<VmwareenginePrivateCloud_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management_cluster` after provisioning.\n"]
    pub fn management_cluster(&self) -> terrars::ListRef<VmwareenginePrivateCloud_ManagementClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<VmwareenginePrivateCloud_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareenginePrivateCloud_TimeoutsRef {
        VmwareenginePrivateCloud_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VmwareenginePrivateCloud {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VmwareenginePrivateCloud { }

impl terrars::ToListMappable for VmwareenginePrivateCloud {
    type O = terrars::ListRef<VmwareenginePrivateCloudRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VmwareenginePrivateCloud_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vmwareengine_private_cloud".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVmwareenginePrivateCloud {
    pub tf_id: std::string::String,
    #[doc = "The location where the PrivateCloud should reside."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the PrivateCloud."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVmwareenginePrivateCloud {
    pub fn build(self, stack: &mut terrars::Stack) -> VmwareenginePrivateCloud {
        let out = VmwareenginePrivateCloud(std::rc::Rc::new(VmwareenginePrivateCloud_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VmwareenginePrivateCloudData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_delay_hours: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                send_deletion_delay_hours_if_zero: core::default::Default::default(),
                type_: core::default::Default::default(),
                management_cluster: core::default::Default::default(),
                network_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VmwareenginePrivateCloudRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloudRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VmwareenginePrivateCloudRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nTime when the resource was scheduled for deletion.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_delay_hours` after provisioning.\nThe number of hours to delay this request. You can set this value to an hour between 0 to 8, where setting it to 0 starts the deletion request immediately. If no value is set, a default value is set at the API Level."]
    pub fn deletion_delay_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_delay_hours", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this private cloud."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nTime when the resource will be irreversibly deleted.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hcx` after provisioning.\nDetails about a HCX Cloud Manager appliance."]
    pub fn hcx(&self) -> terrars::ListRef<VmwareenginePrivateCloud_HcxRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> terrars::ListRef<VmwareenginePrivateCloud_NsxRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nsx", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `send_deletion_delay_hours_if_zero` after provisioning.\nWhile set true, deletion_delay_hours value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the deletion_delay_hours field. It can be used both alone and together with deletion_delay_hours."]
    pub fn send_deletion_delay_hours_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_deletion_delay_hours_if_zero", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of the resource. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nInitial type of the private cloud. Possible values: [\"STANDARD\", \"TIME_LIMITED\", \"STRETCHED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast update time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vcenter` after provisioning.\nDetails about a vCenter Server management appliance."]
    pub fn vcenter(&self) -> terrars::ListRef<VmwareenginePrivateCloud_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management_cluster` after provisioning.\n"]
    pub fn management_cluster(&self) -> terrars::ListRef<VmwareenginePrivateCloud_ManagementClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<VmwareenginePrivateCloud_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareenginePrivateCloud_TimeoutsRef {
        VmwareenginePrivateCloud_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VmwareenginePrivateCloudDynamic {
    management_cluster: std::option::Option<terrars::DynamicBlock<VmwareenginePrivateCloud_ManagementCluster>>,
    network_config: std::option::Option<terrars::DynamicBlock<VmwareenginePrivateCloud_NetworkConfig>>,
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_Hcx {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareenginePrivateCloud_Hcx {
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

impl terrars::ToListMappable for VmwareenginePrivateCloud_Hcx {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_Hcx>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_Hcx {}

impl BuildVmwareenginePrivateCloud_Hcx {
    pub fn build(self) -> VmwareenginePrivateCloud_Hcx {
        VmwareenginePrivateCloud_Hcx {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_HcxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_HcxRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareenginePrivateCloud_HcxRef {
        VmwareenginePrivateCloud_HcxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_HcxRef {
    #[allow(dead_code)]
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
pub struct VmwareenginePrivateCloud_Nsx {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareenginePrivateCloud_Nsx {
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

impl terrars::ToListMappable for VmwareenginePrivateCloud_Nsx {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_Nsx>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_Nsx {}

impl BuildVmwareenginePrivateCloud_Nsx {
    pub fn build(self) -> VmwareenginePrivateCloud_Nsx {
        VmwareenginePrivateCloud_Nsx {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_NsxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_NsxRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareenginePrivateCloud_NsxRef {
        VmwareenginePrivateCloud_NsxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_NsxRef {
    #[allow(dead_code)]
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
pub struct VmwareenginePrivateCloud_Vcenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareenginePrivateCloud_Vcenter {
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

impl terrars::ToListMappable for VmwareenginePrivateCloud_Vcenter {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_Vcenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_Vcenter {}

impl BuildVmwareenginePrivateCloud_Vcenter {
    pub fn build(self) -> VmwareenginePrivateCloud_Vcenter {
        VmwareenginePrivateCloud_Vcenter {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_VcenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_VcenterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareenginePrivateCloud_VcenterRef {
        VmwareenginePrivateCloud_VcenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_VcenterRef {
    #[allow(dead_code)]
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
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    scale_in: terrars::PrimField<f64>,
    scale_out: terrars::PrimField<f64>,
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds { }

impl terrars
::ToListMappable for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    type O =
        terrars
        ::BlockAssignable<
            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: terrars::PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: terrars::PrimField<f64>,
}

impl BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    pub fn build(
        self,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    scale_in: terrars::PrimField<f64>,
    scale_out: terrars::PrimField<f64>,
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds { }

impl terrars
::ToListMappable for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    type O =
        terrars
        ::BlockAssignable<
            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: terrars::PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: terrars::PrimField<f64>,
}

impl BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    pub fn build(
        self,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    scale_in: terrars::PrimField<f64>,
    scale_out: terrars::PrimField<f64>,
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds { }

impl terrars
::ToListMappable for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    type O =
        terrars
        ::BlockAssignable<
            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: terrars::PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: terrars::PrimField<f64>,
}

impl BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    pub fn build(
        self,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesDynamic {
    consumed_memory_thresholds: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
        >,
    >,
    cpu_thresholds: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds,
        >,
    >,
    storage_thresholds: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds,
        >,
    >,
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    autoscale_policy_id: terrars::PrimField<std::string::String>,
    node_type_id: terrars::PrimField<std::string::String>,
    scale_out_size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumed_memory_thresholds: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_thresholds: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_thresholds: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds>,
    >,
    dynamic: VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesDynamic,
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    #[doc = "Set the field `consumed_memory_thresholds`.\n"]
    pub fn set_consumed_memory_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consumed_memory_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consumed_memory_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cpu_thresholds`.\n"]
    pub fn set_cpu_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cpu_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_thresholds`.\n"]
    pub fn set_storage_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    type O =
        terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    #[doc = ""]
    pub autoscale_policy_id: terrars::PrimField<std::string::String>,
    #[doc = "The canonical identifier of the node type to add or remove."]
    pub node_type_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Number of nodes to add to a cluster during a scale-out operation.\nMust be divisible by 2 for stretched clusters."]
    pub scale_out_size: terrars::PrimField<f64>,
}

impl BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    pub fn build(self) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
            autoscale_policy_id: self.autoscale_policy_id,
            node_type_id: self.node_type_id,
            scale_out_size: self.scale_out_size,
            consumed_memory_thresholds: core::default::Default::default(),
            cpu_thresholds: core::default::Default::default(),
            storage_thresholds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscale_policy_id` after provisioning.\n"]
    pub fn autoscale_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscale_policy_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_type_id` after provisioning.\nThe canonical identifier of the node type to add or remove."]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out_size` after provisioning.\nNumber of nodes to add to a cluster during a scale-out operation.\nMust be divisible by 2 for stretched clusters."]
    pub fn scale_out_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_size", self.base))
    }

    #[doc = "Get a reference to the value of field `consumed_memory_thresholds` after provisioning.\n"]
    pub fn consumed_memory_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumed_memory_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_thresholds` after provisioning.\n"]
    pub fn cpu_thresholds(
        &self,
    ) -> terrars
    ::ListRef<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_thresholds` after provisioning.\n"]
    pub fn storage_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_thresholds", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsDynamic {
    autoscaling_policies: std
    ::option
    ::Option<
        terrars::DynamicBlock<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies>,
    >,
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cool_down_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policies: std
    ::option
    ::Option<std::vec::Vec<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies>>,
    dynamic: VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsDynamic,
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    #[doc =
        "Set the field `cool_down_period`.\nThe minimum duration between consecutive autoscale operations.\nIt starts once addition or removal of nodes is fully completed.\nMinimum cool down period is 30m.\nCool down period must be in whole minutes (for example, 30m, 31m, 50m).\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_cool_down_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cool_down_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_cluster_node_count`.\nMaximum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_max_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cluster_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cluster_node_count`.\nMinimum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_min_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_cluster_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_policies`.\n"]
    pub fn set_autoscaling_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_policies = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {}

impl BuildVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    pub fn build(self) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
            cool_down_period: core::default::Default::default(),
            max_cluster_node_count: core::default::Default::default(),
            min_cluster_node_count: core::default::Default::default(),
            autoscaling_policies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
        VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cool_down_period` after provisioning.\nThe minimum duration between consecutive autoscale operations.\nIt starts once addition or removal of nodes is fully completed.\nMinimum cool down period is 30m.\nCool down period must be in whole minutes (for example, 30m, 31m, 50m).\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn cool_down_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cool_down_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_cluster_node_count` after provisioning.\nMaximum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn max_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cluster_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cluster_node_count` after provisioning.\nMinimum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn min_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cluster_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_core_count: std::option::Option<terrars::PrimField<f64>>,
    node_count: terrars::PrimField<f64>,
    node_type_id: terrars::PrimField<std::string::String>,
}

impl VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    #[doc =
        "Set the field `custom_core_count`.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nThis cannot be changed once the PrivateCloud is created."]
    pub fn set_custom_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_core_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    #[doc = "The number of nodes of this type in the cluster."]
    pub node_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub node_type_id: terrars::PrimField<std::string::String>,
}

impl BuildVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    pub fn build(self) -> VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
        VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
            custom_core_count: core::default::Default::default(),
            node_count: self.node_count,
            node_type_id: self.node_type_id,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
        VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_core_count` after provisioning.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nThis cannot be changed once the PrivateCloud is created."]
    pub fn custom_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes of this type in the cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    #[doc =
        "Set the field `preferred_location`.\nZone that will remain operational when connection between the two zones is lost.\nSpecify the zone in the following format: projects/{project}/locations/{location}."]
    pub fn set_preferred_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preferred_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secondary_location`.\nAdditional zone for a higher level of availability and load balancing.\nSpecify the zone in the following format: projects/{project}/locations/{location}."]
    pub fn set_secondary_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secondary_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {}

impl BuildVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    pub fn build(self) -> VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
        VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
            preferred_location: core::default::Default::default(),
            secondary_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
        VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `preferred_location` after provisioning.\nZone that will remain operational when connection between the two zones is lost.\nSpecify the zone in the following format: projects/{project}/locations/{location}."]
    pub fn preferred_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secondary_location` after provisioning.\nAdditional zone for a higher level of availability and load balancing.\nSpecify the zone in the following format: projects/{project}/locations/{location}."]
    pub fn secondary_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareenginePrivateCloud_ManagementClusterDynamic {
    autoscaling_settings: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings>>,
    node_type_configs: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs>>,
    stretched_cluster_config: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig>>,
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_ManagementCluster {
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_settings: std
    ::option
    ::Option<std::vec::Vec<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_configs: std::option::Option<std::vec::Vec<VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stretched_cluster_config: std
    ::option
    ::Option<std::vec::Vec<VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig>>,
    dynamic: VmwareenginePrivateCloud_ManagementClusterDynamic,
}

impl VmwareenginePrivateCloud_ManagementCluster {
    #[doc = "Set the field `autoscaling_settings`.\n"]
    pub fn set_autoscaling_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_type_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_type_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stretched_cluster_config`.\n"]
    pub fn set_stretched_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stretched_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stretched_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VmwareenginePrivateCloud_ManagementCluster {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_ManagementCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_ManagementCluster {
    #[doc =
        "The user-provided identifier of the new Cluster. The identifier must meet the following requirements:\n  * Only contains 1-63 alphanumeric characters and hyphens\n  * Begins with an alphabetical character\n  * Ends with a non-hyphen character\n  * Not formatted as a UUID\n  * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)"]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildVmwareenginePrivateCloud_ManagementCluster {
    pub fn build(self) -> VmwareenginePrivateCloud_ManagementCluster {
        VmwareenginePrivateCloud_ManagementCluster {
            cluster_id: self.cluster_id,
            autoscaling_settings: core::default::Default::default(),
            node_type_configs: core::default::Default::default(),
            stretched_cluster_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_ManagementClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_ManagementClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareenginePrivateCloud_ManagementClusterRef {
        VmwareenginePrivateCloud_ManagementClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_ManagementClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nThe user-provided identifier of the new Cluster. The identifier must meet the following requirements:\n  * Only contains 1-63 alphanumeric characters and hyphens\n  * Begins with an alphabetical character\n  * Ends with a non-hyphen character\n  * Not formatted as a UUID\n  * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\n"]
    pub fn autoscaling_settings(
        &self,
    ) -> terrars::ListRef<VmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `stretched_cluster_config` after provisioning.\n"]
    pub fn stretched_cluster_config(
        &self,
    ) -> terrars::ListRef<VmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stretched_cluster_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_NetworkConfig {
    management_cidr: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareenginePrivateCloud_NetworkConfig {
    #[doc =
        "Set the field `vmware_engine_network`.\nThe relative resource name of the VMware Engine network attached to the private cloud.\nSpecify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}\nwhere {project} can either be a project number or a project ID."]
    pub fn set_vmware_engine_network(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vmware_engine_network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VmwareenginePrivateCloud_NetworkConfig {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_NetworkConfig {
    #[doc = "Management CIDR used by VMware management appliances."]
    pub management_cidr: terrars::PrimField<std::string::String>,
}

impl BuildVmwareenginePrivateCloud_NetworkConfig {
    pub fn build(self) -> VmwareenginePrivateCloud_NetworkConfig {
        VmwareenginePrivateCloud_NetworkConfig {
            management_cidr: self.management_cidr,
            vmware_engine_network: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareenginePrivateCloud_NetworkConfigRef {
        VmwareenginePrivateCloud_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dns_server_ip` after provisioning.\nDNS Server IP of the Private Cloud."]
    pub fn dns_server_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_server_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `management_cidr` after provisioning.\nManagement CIDR used by VMware management appliances."]
    pub fn management_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_cidr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `management_ip_address_layout_version` after provisioning.\nThe IP address layout version of the management IP address range.\nPossible versions include:\n* managementIpAddressLayoutVersion=1: Indicates the legacy IP address layout used by some existing private clouds. This is no longer supported for new private clouds\nas it does not support all features.\n* managementIpAddressLayoutVersion=2: Indicates the latest IP address layout\nused by all newly created private clouds. This version supports all current features."]
    pub fn management_ip_address_layout_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_ip_address_layout_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vmware_engine_network` after provisioning.\nThe relative resource name of the VMware Engine network attached to the private cloud.\nSpecify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}\nwhere {project} can either be a project number or a project ID."]
    pub fn vmware_engine_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\nThe canonical name of the VMware Engine network in\nthe form: projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}"]
    pub fn vmware_engine_network_canonical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network_canonical", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareenginePrivateCloud_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareenginePrivateCloud_Timeouts {
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

impl terrars::ToListMappable for VmwareenginePrivateCloud_Timeouts {
    type O = terrars::BlockAssignable<VmwareenginePrivateCloud_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareenginePrivateCloud_Timeouts {}

impl BuildVmwareenginePrivateCloud_Timeouts {
    pub fn build(self) -> VmwareenginePrivateCloud_Timeouts {
        VmwareenginePrivateCloud_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareenginePrivateCloud_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareenginePrivateCloud_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareenginePrivateCloud_TimeoutsRef {
        VmwareenginePrivateCloud_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareenginePrivateCloud_TimeoutsRef {
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
