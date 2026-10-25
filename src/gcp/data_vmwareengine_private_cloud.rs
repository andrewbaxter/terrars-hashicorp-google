#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareenginePrivateCloudData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataVmwareenginePrivateCloud_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVmwareenginePrivateCloudData_>,
}

#[derive(Clone)]
pub struct DataVmwareenginePrivateCloud(std::rc::Rc<DataVmwareenginePrivateCloud_>);

impl DataVmwareenginePrivateCloud {
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
    pub fn hcx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_HcxRef> {
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

    #[doc =
        "Get a reference to the value of field `management_cluster` after provisioning.\nThe management cluster for this private cloud. This used for creating and managing the default cluster."]
    pub fn management_cluster(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_ManagementClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration in the consumer project with which the peering has to be done."]
    pub fn network_config(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_NsxRef> {
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
    pub fn vcenter(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

impl terrars::Referable for DataVmwareenginePrivateCloud {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVmwareenginePrivateCloud { }

impl terrars::ToListMappable for DataVmwareenginePrivateCloud {
    type O = terrars::ListRef<DataVmwareenginePrivateCloudRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVmwareenginePrivateCloud_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vmwareengine_private_cloud".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareenginePrivateCloud {
    pub tf_id: std::string::String,
    #[doc = "The location where the PrivateCloud should reside."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the PrivateCloud."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataVmwareenginePrivateCloud {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVmwareenginePrivateCloud {
        let out = DataVmwareenginePrivateCloud(std::rc::Rc::new(DataVmwareenginePrivateCloud_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVmwareenginePrivateCloudData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVmwareenginePrivateCloudRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloudRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareenginePrivateCloudRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
    pub fn hcx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_HcxRef> {
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

    #[doc =
        "Get a reference to the value of field `management_cluster` after provisioning.\nThe management cluster for this private cloud. This used for creating and managing the default cluster."]
    pub fn management_cluster(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_ManagementClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the PrivateCloud."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration in the consumer project with which the peering has to be done."]
    pub fn network_config(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nsx` after provisioning.\nDetails about a NSX Manager appliance."]
    pub fn nsx(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_NsxRef> {
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
    pub fn vcenter(&self) -> terrars::ListRef<DataVmwareenginePrivateCloud_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_Hcx {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloud_Hcx {
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

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_Hcx {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_Hcx>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_Hcx {}

impl BuildDataVmwareenginePrivateCloud_Hcx {
    pub fn build(self) -> DataVmwareenginePrivateCloud_Hcx {
        DataVmwareenginePrivateCloud_Hcx {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_HcxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_HcxRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareenginePrivateCloud_HcxRef {
        DataVmwareenginePrivateCloud_HcxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_HcxRef {
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
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    type O =
        terrars
        ::BlockAssignable<
            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    pub fn build(
        self,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    type O =
        terrars
        ::BlockAssignable<
            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    pub fn build(
        self,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    type O =
        terrars
        ::BlockAssignable<
            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    pub fn build(
        self,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscale_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumed_memory_thresholds: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_thresholds: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_thresholds: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds,
        >,
    >,
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    #[doc = "Set the field `autoscale_policy_id`.\n"]
    pub fn set_autoscale_policy_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.autoscale_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumed_memory_thresholds`.\n"]
    pub fn set_consumed_memory_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
                        >,
                    >,
    ) -> Self {
        self.consumed_memory_thresholds = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds,
                        >,
                    >,
    ) -> Self {
        self.cpu_thresholds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out_size`.\n"]
    pub fn set_scale_out_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out_size = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds,
                        >,
                    >,
    ) -> Self {
        self.storage_thresholds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    type O =
        terrars
        ::BlockAssignable<DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
    pub fn build(self) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies {
            autoscale_policy_id: core::default::Default::default(),
            consumed_memory_thresholds: core::default::Default::default(),
            cpu_thresholds: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
            scale_out_size: core::default::Default::default(),
            storage_thresholds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscale_policy_id` after provisioning.\n"]
    pub fn autoscale_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscale_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `consumed_memory_thresholds` after provisioning.\n"]
    pub fn consumed_memory_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumed_memory_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_thresholds` after provisioning.\n"]
    pub fn cpu_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_size` after provisioning.\n"]
    pub fn scale_out_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_size", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_thresholds` after provisioning.\n"]
    pub fn storage_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_thresholds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policies: std
    ::option
    ::Option<
        terrars::SetField<DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cool_down_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    #[doc = "Set the field `autoscaling_policies`.\n"]
    pub fn set_autoscaling_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPolicies,
                        >,
                    >,
    ) -> Self {
        self.autoscaling_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cool_down_period`.\n"]
    pub fn set_cool_down_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cool_down_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_cluster_node_count`.\n"]
    pub fn set_max_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cluster_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cluster_node_count`.\n"]
    pub fn set_min_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_cluster_node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
    pub fn build(self) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings {
            autoscaling_policies: core::default::Default::default(),
            cool_down_period: core::default::Default::default(),
            max_cluster_node_count: core::default::Default::default(),
            min_cluster_node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
        DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_policies` after provisioning.\n"]
    pub fn autoscaling_policies(
        &self,
    ) -> terrars::SetRef<DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings_AutoscalingPoliciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.autoscaling_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `cool_down_period` after provisioning.\n"]
    pub fn cool_down_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cool_down_period", self.base))
    }

    #[doc = "Get a reference to the value of field `max_cluster_node_count` after provisioning.\n"]
    pub fn max_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cluster_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_cluster_node_count` after provisioning.\n"]
    pub fn min_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cluster_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    #[doc = "Set the field `custom_core_count`.\n"]
    pub fn set_custom_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
    pub fn build(self) -> DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
        DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs {
            custom_core_count: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
        DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_core_count` after provisioning.\n"]
    pub fn custom_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    #[doc = "Set the field `preferred_location`.\n"]
    pub fn set_preferred_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preferred_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_location`.\n"]
    pub fn set_secondary_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secondary_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
    pub fn build(self) -> DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
        DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig {
            preferred_location: core::default::Default::default(),
            secondary_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
        DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preferred_location` after provisioning.\n"]
    pub fn preferred_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_location", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_location` after provisioning.\n"]
    pub fn secondary_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_ManagementCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_settings: std
    ::option
    ::Option<terrars::ListField<DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_configs: std
    ::option
    ::Option<terrars::SetField<DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stretched_cluster_config: std
    ::option
    ::Option<terrars::ListField<DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig>>,
}

impl DataVmwareenginePrivateCloud_ManagementCluster {
    #[doc = "Set the field `autoscaling_settings`.\n"]
    pub fn set_autoscaling_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettings>>,
    ) -> Self {
        self.autoscaling_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_id`.\n"]
    pub fn set_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigs>>,
    ) -> Self {
        self.node_type_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stretched_cluster_config`.\n"]
    pub fn set_stretched_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfig>>,
    ) -> Self {
        self.stretched_cluster_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_ManagementCluster {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_ManagementCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_ManagementCluster {}

impl BuildDataVmwareenginePrivateCloud_ManagementCluster {
    pub fn build(self) -> DataVmwareenginePrivateCloud_ManagementCluster {
        DataVmwareenginePrivateCloud_ManagementCluster {
            autoscaling_settings: core::default::Default::default(),
            cluster_id: core::default::Default::default(),
            node_type_configs: core::default::Default::default(),
            stretched_cluster_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_ManagementClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_ManagementClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareenginePrivateCloud_ManagementClusterRef {
        DataVmwareenginePrivateCloud_ManagementClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_ManagementClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\n"]
    pub fn autoscaling_settings(
        &self,
    ) -> terrars::ListRef<DataVmwareenginePrivateCloud_ManagementCluster_AutoscalingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_configs` after provisioning.\n"]
    pub fn node_type_configs(
        &self,
    ) -> terrars::SetRef<DataVmwareenginePrivateCloud_ManagementCluster_NodeTypeConfigsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `stretched_cluster_config` after provisioning.\n"]
    pub fn stretched_cluster_config(
        &self,
    ) -> terrars::ListRef<DataVmwareenginePrivateCloud_ManagementCluster_StretchedClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stretched_cluster_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_server_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management_ip_address_layout_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_network_canonical: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloud_NetworkConfig {
    #[doc = "Set the field `dns_server_ip`.\n"]
    pub fn set_dns_server_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_server_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management_cidr`.\n"]
    pub fn set_management_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.management_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management_ip_address_layout_version`.\n"]
    pub fn set_management_ip_address_layout_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.management_ip_address_layout_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vmware_engine_network`.\n"]
    pub fn set_vmware_engine_network(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vmware_engine_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vmware_engine_network_canonical`.\n"]
    pub fn set_vmware_engine_network_canonical(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vmware_engine_network_canonical = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_NetworkConfig {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_NetworkConfig {}

impl BuildDataVmwareenginePrivateCloud_NetworkConfig {
    pub fn build(self) -> DataVmwareenginePrivateCloud_NetworkConfig {
        DataVmwareenginePrivateCloud_NetworkConfig {
            dns_server_ip: core::default::Default::default(),
            management_cidr: core::default::Default::default(),
            management_ip_address_layout_version: core::default::Default::default(),
            vmware_engine_network: core::default::Default::default(),
            vmware_engine_network_canonical: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareenginePrivateCloud_NetworkConfigRef {
        DataVmwareenginePrivateCloud_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_server_ip` after provisioning.\n"]
    pub fn dns_server_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_server_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `management_cidr` after provisioning.\n"]
    pub fn management_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `management_ip_address_layout_version` after provisioning.\n"]
    pub fn management_ip_address_layout_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_ip_address_layout_version", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_engine_network` after provisioning.\n"]
    pub fn vmware_engine_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\n"]
    pub fn vmware_engine_network_canonical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vmware_engine_network_canonical", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareenginePrivateCloud_Nsx {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloud_Nsx {
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

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_Nsx {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_Nsx>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_Nsx {}

impl BuildDataVmwareenginePrivateCloud_Nsx {
    pub fn build(self) -> DataVmwareenginePrivateCloud_Nsx {
        DataVmwareenginePrivateCloud_Nsx {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_NsxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_NsxRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareenginePrivateCloud_NsxRef {
        DataVmwareenginePrivateCloud_NsxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_NsxRef {
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
pub struct DataVmwareenginePrivateCloud_Vcenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareenginePrivateCloud_Vcenter {
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

impl terrars::ToListMappable for DataVmwareenginePrivateCloud_Vcenter {
    type O = terrars::BlockAssignable<DataVmwareenginePrivateCloud_Vcenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareenginePrivateCloud_Vcenter {}

impl BuildDataVmwareenginePrivateCloud_Vcenter {
    pub fn build(self) -> DataVmwareenginePrivateCloud_Vcenter {
        DataVmwareenginePrivateCloud_Vcenter {
            fqdn: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareenginePrivateCloud_VcenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareenginePrivateCloud_VcenterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareenginePrivateCloud_VcenterRef {
        DataVmwareenginePrivateCloud_VcenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareenginePrivateCloud_VcenterRef {
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
