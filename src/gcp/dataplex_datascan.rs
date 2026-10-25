#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataplexDatascanData_ {
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
    data_scan_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<std::vec::Vec<DataplexDatascan_Data>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_discovery_spec: std::option::Option<std::vec::Vec<DataplexDatascan_DataDiscoverySpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_documentation_spec: std::option::Option<std::vec::Vec<DataplexDatascan_DataDocumentationSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_profile_spec: std::option::Option<std::vec::Vec<DataplexDatascan_DataProfileSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_quality_spec: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_identity: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionIdentity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_spec: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataplexDatascan_Timeouts>,
    dynamic: DataplexDatascanDynamic,
}

struct DataplexDatascan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataplexDatascanData_>,
}

#[derive(Clone)]
pub struct DataplexDatascan(std::rc::Rc<DataplexDatascan_>);

impl DataplexDatascan {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the scan."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser friendly display name."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for the scan. A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `data`.\n"]
    pub fn set_data(self, v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_Data>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_discovery_spec`.\n"]
    pub fn set_data_discovery_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_discovery_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_discovery_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_documentation_spec`.\n"]
    pub fn set_data_documentation_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataDocumentationSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_documentation_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_documentation_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_profile_spec`.\n"]
    pub fn set_data_profile_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataProfileSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_profile_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_profile_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_quality_spec`.\n"]
    pub fn set_data_quality_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_identity`.\n"]
    pub fn set_execution_identity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_identity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_identity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_spec`.\n"]
    pub fn set_execution_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataplexDatascan_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the scan was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_scan_id` after provisioning.\nDataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter."]
    pub fn data_scan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_scan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the scan."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_status` after provisioning.\nStatus of the data scan execution."]
    pub fn execution_status(&self) -> terrars::ListRef<DataplexDatascan_ExecutionStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the scan. A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the data scan should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the scan, of the form: projects/{project}/locations/{locationId}/dataScans/{datascan_id}, where project refers to a project_id or project_number and locationId refers to a GCP region."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the DataScan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of DataScan."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the scan was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::ListRef<DataplexDatascan_DataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_discovery_spec` after provisioning.\n"]
    pub fn data_discovery_spec(&self) -> terrars::ListRef<DataplexDatascan_DataDiscoverySpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_discovery_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_documentation_spec` after provisioning.\n"]
    pub fn data_documentation_spec(&self) -> terrars::ListRef<DataplexDatascan_DataDocumentationSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_documentation_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_profile_spec` after provisioning.\n"]
    pub fn data_profile_spec(&self) -> terrars::ListRef<DataplexDatascan_DataProfileSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_profile_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_spec` after provisioning.\n"]
    pub fn data_quality_spec(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_quality_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_identity` after provisioning.\n"]
    pub fn execution_identity(&self) -> terrars::ListRef<DataplexDatascan_ExecutionIdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> terrars::ListRef<DataplexDatascan_ExecutionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexDatascan_TimeoutsRef {
        DataplexDatascan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataplexDatascan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataplexDatascan { }

impl terrars::ToListMappable for DataplexDatascan {
    type O = terrars::ListRef<DataplexDatascanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataplexDatascan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataplex_datascan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataplexDatascan {
    pub tf_id: std::string::String,
    #[doc =
        "DataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter."]
    pub data_scan_id: terrars::PrimField<std::string::String>,
    #[doc = "The location where the data scan should reside."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan {
    pub fn build(self, stack: &mut terrars::Stack) -> DataplexDatascan {
        let out = DataplexDatascan(std::rc::Rc::new(DataplexDatascan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataplexDatascanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_scan_id: self.data_scan_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                data: core::default::Default::default(),
                data_discovery_spec: core::default::Default::default(),
                data_documentation_spec: core::default::Default::default(),
                data_profile_spec: core::default::Default::default(),
                data_quality_spec: core::default::Default::default(),
                execution_identity: core::default::Default::default(),
                execution_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataplexDatascanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataplexDatascanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the scan was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_scan_id` after provisioning.\nDataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter."]
    pub fn data_scan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_scan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the scan."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_status` after provisioning.\nStatus of the data scan execution."]
    pub fn execution_status(&self) -> terrars::ListRef<DataplexDatascan_ExecutionStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the scan. A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the data scan should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the scan, of the form: projects/{project}/locations/{locationId}/dataScans/{datascan_id}, where project refers to a project_id or project_number and locationId refers to a GCP region."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the DataScan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of DataScan."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the scan was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::ListRef<DataplexDatascan_DataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_discovery_spec` after provisioning.\n"]
    pub fn data_discovery_spec(&self) -> terrars::ListRef<DataplexDatascan_DataDiscoverySpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_discovery_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_documentation_spec` after provisioning.\n"]
    pub fn data_documentation_spec(&self) -> terrars::ListRef<DataplexDatascan_DataDocumentationSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_documentation_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_profile_spec` after provisioning.\n"]
    pub fn data_profile_spec(&self) -> terrars::ListRef<DataplexDatascan_DataProfileSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_profile_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_spec` after provisioning.\n"]
    pub fn data_quality_spec(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_quality_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_identity` after provisioning.\n"]
    pub fn execution_identity(&self) -> terrars::ListRef<DataplexDatascan_ExecutionIdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> terrars::ListRef<DataplexDatascan_ExecutionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexDatascan_TimeoutsRef {
        DataplexDatascan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascanDynamic {
    data: std::option::Option<terrars::DynamicBlock<DataplexDatascan_Data>>,
    data_discovery_spec: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataDiscoverySpec>>,
    data_documentation_spec: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataDocumentationSpec>>,
    data_profile_spec: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataProfileSpec>>,
    data_quality_spec: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec>>,
    execution_identity: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionIdentity>>,
    execution_spec: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionSpec>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_job_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_job_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_ExecutionStatus {
    #[doc = "Set the field `latest_job_end_time`.\n"]
    pub fn set_latest_job_end_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.latest_job_end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latest_job_start_time`.\n"]
    pub fn set_latest_job_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.latest_job_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_ExecutionStatus {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionStatus {}

impl BuildDataplexDatascan_ExecutionStatus {
    pub fn build(self) -> DataplexDatascan_ExecutionStatus {
        DataplexDatascan_ExecutionStatus {
            latest_job_end_time: core::default::Default::default(),
            latest_job_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_ExecutionStatusRef {
        DataplexDatascan_ExecutionStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `latest_job_end_time` after provisioning.\n"]
    pub fn latest_job_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_job_end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_job_start_time` after provisioning.\n"]
    pub fn latest_job_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_job_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_Data {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_Data {
    #[doc =
        "Set the field `entity`.\nThe Dataplex entity that represents the data source(e.g. BigQuery table) for Datascan."]
    pub fn set_entity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nThe service-qualified full resource name of the cloud resource for a DataScan job to scan against. The field could be:\nCloud Storage bucket (//storage.googleapis.com/projects/PROJECT_ID/buckets/BUCKET_ID) for DataDiscoveryScan OR BigQuery table of type \"TABLE\" (/bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID) for DataProfileScan/DataQualityScan."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_Data {
    type O = terrars::BlockAssignable<DataplexDatascan_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_Data {}

impl BuildDataplexDatascan_Data {
    pub fn build(self) -> DataplexDatascan_Data {
        DataplexDatascan_Data {
            entity: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_DataRef {
        DataplexDatascan_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `entity` after provisioning.\nThe Dataplex entity that represents the data source(e.g. BigQuery table) for Datascan."]
    pub fn entity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nThe service-qualified full resource name of the cloud resource for a DataScan job to scan against. The field could be:\nCloud Storage bucket (//storage.googleapis.com/projects/PROJECT_ID/buckets/BUCKET_ID) for DataDiscoveryScan OR BigQuery table of type \"TABLE\" (/bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID) for DataProfileScan/DataQualityScan."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig {
    #[doc =
        "Set the field `connection`.\nThe BigQuery connection used to create BigLake tables. Must be in the form 'projects/{projectId}/locations/{locationId}/connections/{connection_id}'."]
    pub fn set_connection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe location of the BigQuery dataset to publish BigLake external or non-BigLake external tables to."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project of the BigQuery dataset to publish BigLake external or non-BigLake external tables to. If not specified, the project of the Cloud Storage bucket will be used. The format is \"projects/{project_id_or_number}\"."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `table_type`.\nDetermines whether to publish discovered tables as BigLake external tables or non-BigLake external tables. Possible values: [\"TABLE_TYPE_UNSPECIFIED\", \"EXTERNAL\", \"BIGLAKE\"]"]
    pub fn set_table_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig {
    type O = terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig {}

impl BuildDataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig {
    pub fn build(self) -> DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig {
        DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig {
            connection: core::default::Default::default(),
            location: core::default::Default::default(),
            project: core::default::Default::default(),
            table_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfigRef {
        DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection` after provisioning.\nThe BigQuery connection used to create BigLake tables. Must be in the form 'projects/{projectId}/locations/{locationId}/connections/{connection_id}'."]
    pub fn connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the BigQuery dataset to publish BigLake external or non-BigLake external tables to."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project of the BigQuery dataset to publish BigLake external or non-BigLake external tables to. If not specified, the project of the Cloud Storage bucket will be used. The format is \"projects/{project_id_or_number}\"."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_type` after provisioning.\nDetermines whether to publish discovered tables as BigLake external tables or non-BigLake external tables. Possible values: [\"TABLE_TYPE_UNSPECIFIED\", \"EXTERNAL\", \"BIGLAKE\"]"]
    pub fn table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_rows: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quote: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_inference_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions {
    #[doc = "Set the field `delimiter`.\nThe delimiter that is used to separate values. The default is ',' (comma)."]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding`.\nThe character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `header_rows`.\nThe number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn set_header_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.header_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quote`.\nThe character used to quote column values. Accepts '\"' (double quotation mark) or ''' (single quotation mark). If unspecified, defaults to '\"' (double quotation mark)."]
    pub fn set_quote(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quote = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_inference_disabled`.\nWhether to disable the inference of data types for CSV data. If true, all columns are registered as strings."]
    pub fn set_type_inference_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.type_inference_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions {
    type O = terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions {}

impl BuildDataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions {
    pub fn build(self) -> DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions {
        DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions {
            delimiter: core::default::Default::default(),
            encoding: core::default::Default::default(),
            header_rows: core::default::Default::default(),
            quote: core::default::Default::default(),
            type_inference_disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptionsRef {
        DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delimiter` after provisioning.\nThe delimiter that is used to separate values. The default is ',' (comma)."]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_rows` after provisioning.\nThe number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn header_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_rows", self.base))
    }

    #[doc =
        "Get a reference to the value of field `quote` after provisioning.\nThe character used to quote column values. Accepts '\"' (double quotation mark) or ''' (single quotation mark). If unspecified, defaults to '\"' (double quotation mark)."]
    pub fn quote(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_inference_disabled` after provisioning.\nWhether to disable the inference of data types for CSV data. If true, all columns are registered as strings."]
    pub fn type_inference_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_inference_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_inference_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions {
    #[doc = "Set the field `encoding`.\nThe character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_inference_disabled`.\nWhether to disable the inference of data types for JSON data. If true, all columns are registered as their primitive types (strings, number, or boolean)."]
    pub fn set_type_inference_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.type_inference_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions {
    type O = terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions {}

impl BuildDataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions {
    pub fn build(self) -> DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions {
        DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions {
            encoding: core::default::Default::default(),
            type_inference_disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptionsRef {
        DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_inference_disabled` after provisioning.\nWhether to disable the inference of data types for JSON data. If true, all columns are registered as their primitive types (strings, number, or boolean)."]
    pub fn type_inference_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_inference_disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataDiscoverySpec_StorageConfigDynamic {
    csv_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions>>,
    json_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataDiscoverySpec_StorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_patterns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_patterns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_options: std::option::Option<std::vec::Vec<DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_options: std::option::Option<std::vec::Vec<DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions>>,
    dynamic: DataplexDatascan_DataDiscoverySpec_StorageConfigDynamic,
}

impl DataplexDatascan_DataDiscoverySpec_StorageConfig {
    #[doc =
        "Set the field `exclude_patterns`.\nDefines the data to exclude during discovery. Provide a list of patterns that identify the data to exclude. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn set_exclude_patterns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_patterns`.\nDefines the data to include during discovery when only a subset of the data should be considered. Provide a list of patterns that identify the data to include. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn set_include_patterns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataDiscoverySpec_StorageConfig {
    type O = terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_StorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataDiscoverySpec_StorageConfig {}

impl BuildDataplexDatascan_DataDiscoverySpec_StorageConfig {
    pub fn build(self) -> DataplexDatascan_DataDiscoverySpec_StorageConfig {
        DataplexDatascan_DataDiscoverySpec_StorageConfig {
            exclude_patterns: core::default::Default::default(),
            include_patterns: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            json_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataDiscoverySpec_StorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataDiscoverySpec_StorageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataDiscoverySpec_StorageConfigRef {
        DataplexDatascan_DataDiscoverySpec_StorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataDiscoverySpec_StorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_patterns` after provisioning.\nDefines the data to exclude during discovery. Provide a list of patterns that identify the data to exclude. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn exclude_patterns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_patterns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_patterns` after provisioning.\nDefines the data to include during discovery when only a subset of the data should be considered. Provide a list of patterns that identify the data to include. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn include_patterns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_patterns", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> terrars::ListRef<DataplexDatascan_DataDiscoverySpec_StorageConfig_CsvOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }

    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> terrars::ListRef<DataplexDatascan_DataDiscoverySpec_StorageConfig_JsonOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataDiscoverySpecDynamic {
    bigquery_publishing_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig>>,
    storage_config: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataDiscoverySpec_StorageConfig>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataDiscoverySpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_publishing_config: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_config: std::option::Option<std::vec::Vec<DataplexDatascan_DataDiscoverySpec_StorageConfig>>,
    dynamic: DataplexDatascan_DataDiscoverySpecDynamic,
}

impl DataplexDatascan_DataDiscoverySpec {
    #[doc = "Set the field `bigquery_publishing_config`.\n"]
    pub fn set_bigquery_publishing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_publishing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_publishing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec_StorageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataDiscoverySpec {
    type O = terrars::BlockAssignable<DataplexDatascan_DataDiscoverySpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataDiscoverySpec {}

impl BuildDataplexDatascan_DataDiscoverySpec {
    pub fn build(self) -> DataplexDatascan_DataDiscoverySpec {
        DataplexDatascan_DataDiscoverySpec {
            bigquery_publishing_config: core::default::Default::default(),
            storage_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataDiscoverySpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataDiscoverySpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_DataDiscoverySpecRef {
        DataplexDatascan_DataDiscoverySpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataDiscoverySpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bigquery_publishing_config` after provisioning.\n"]
    pub fn bigquery_publishing_config(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataDiscoverySpec_BigqueryPublishingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_publishing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<DataplexDatascan_DataDiscoverySpec_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataDocumentationSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_publishing_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_dialect: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_DataDocumentationSpec {
    #[doc =
        "Set the field `catalog_publishing_enabled`.\nIf set, the latest DataScan job result will be published to Knowledge Catalog."]
    pub fn set_catalog_publishing_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.catalog_publishing_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sql_dialect`.\nThe SQL dialect to use in the generated SQL queries.\nIf not specified, the default dialect is Google SQL. Possible values: [\"GOOGLE_SQL\", \"SPARK_SQL\"]"]
    pub fn set_sql_dialect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_dialect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataDocumentationSpec {
    type O = terrars::BlockAssignable<DataplexDatascan_DataDocumentationSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataDocumentationSpec {}

impl BuildDataplexDatascan_DataDocumentationSpec {
    pub fn build(self) -> DataplexDatascan_DataDocumentationSpec {
        DataplexDatascan_DataDocumentationSpec {
            catalog_publishing_enabled: core::default::Default::default(),
            sql_dialect: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataDocumentationSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataDocumentationSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_DataDocumentationSpecRef {
        DataplexDatascan_DataDocumentationSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataDocumentationSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `catalog_publishing_enabled` after provisioning.\nIf set, the latest DataScan job result will be published to Knowledge Catalog."]
    pub fn catalog_publishing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_publishing_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sql_dialect` after provisioning.\nThe SQL dialect to use in the generated SQL queries.\nIf not specified, the default dialect is Google SQL. Possible values: [\"GOOGLE_SQL\", \"SPARK_SQL\"]"]
    pub fn sql_dialect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_dialect", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataProfileSpec_ExcludeFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataplexDatascan_DataProfileSpec_ExcludeFields {
    #[doc =
        "Set the field `field_names`.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn set_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.field_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataProfileSpec_ExcludeFields {
    type O = terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_ExcludeFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataProfileSpec_ExcludeFields {}

impl BuildDataplexDatascan_DataProfileSpec_ExcludeFields {
    pub fn build(self) -> DataplexDatascan_DataProfileSpec_ExcludeFields {
        DataplexDatascan_DataProfileSpec_ExcludeFields { field_names: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataProfileSpec_ExcludeFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataProfileSpec_ExcludeFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataProfileSpec_ExcludeFieldsRef {
        DataplexDatascan_DataProfileSpec_ExcludeFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataProfileSpec_ExcludeFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field_names` after provisioning.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataProfileSpec_IncludeFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataplexDatascan_DataProfileSpec_IncludeFields {
    #[doc =
        "Set the field `field_names`.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn set_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.field_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataProfileSpec_IncludeFields {
    type O = terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_IncludeFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataProfileSpec_IncludeFields {}

impl BuildDataplexDatascan_DataProfileSpec_IncludeFields {
    pub fn build(self) -> DataplexDatascan_DataProfileSpec_IncludeFields {
        DataplexDatascan_DataProfileSpec_IncludeFields { field_names: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataProfileSpec_IncludeFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataProfileSpec_IncludeFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataProfileSpec_IncludeFieldsRef {
        DataplexDatascan_DataProfileSpec_IncludeFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataProfileSpec_IncludeFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field_names` after provisioning.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    results_table: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport {
    #[doc =
        "Set the field `results_table`.\nThe BigQuery table to export DataProfileScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn set_results_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.results_table = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport {
    type O = terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport {}

impl BuildDataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport {
    pub fn build(self) -> DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport {
        DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport {
            results_table: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExportRef {
        DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `results_table` after provisioning.\nThe BigQuery table to export DataProfileScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn results_table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.results_table", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataProfileSpec_PostScanActionsDynamic {
    bigquery_export: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataProfileSpec_PostScanActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_export: std::option::Option<std::vec::Vec<DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport>>,
    dynamic: DataplexDatascan_DataProfileSpec_PostScanActionsDynamic,
}

impl DataplexDatascan_DataProfileSpec_PostScanActions {
    #[doc = "Set the field `bigquery_export`.\n"]
    pub fn set_bigquery_export(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExport>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_export = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_export = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataProfileSpec_PostScanActions {
    type O = terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_PostScanActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataProfileSpec_PostScanActions {}

impl BuildDataplexDatascan_DataProfileSpec_PostScanActions {
    pub fn build(self) -> DataplexDatascan_DataProfileSpec_PostScanActions {
        DataplexDatascan_DataProfileSpec_PostScanActions {
            bigquery_export: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataProfileSpec_PostScanActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataProfileSpec_PostScanActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataProfileSpec_PostScanActionsRef {
        DataplexDatascan_DataProfileSpec_PostScanActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataProfileSpec_PostScanActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bigquery_export` after provisioning.\n"]
    pub fn bigquery_export(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataProfileSpec_PostScanActions_BigqueryExportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_export", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataProfileSpecDynamic {
    exclude_fields: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataProfileSpec_ExcludeFields>>,
    include_fields: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataProfileSpec_IncludeFields>>,
    post_scan_actions: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataProfileSpec_PostScanActions>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataProfileSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_publishing_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_fields: std::option::Option<std::vec::Vec<DataplexDatascan_DataProfileSpec_ExcludeFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_fields: std::option::Option<std::vec::Vec<DataplexDatascan_DataProfileSpec_IncludeFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_scan_actions: std::option::Option<std::vec::Vec<DataplexDatascan_DataProfileSpec_PostScanActions>>,
    dynamic: DataplexDatascan_DataProfileSpecDynamic,
}

impl DataplexDatascan_DataProfileSpec {
    #[doc =
        "Set the field `catalog_publishing_enabled`.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn set_catalog_publishing_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.catalog_publishing_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `row_filter`.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn set_row_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.row_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sampling_percent`.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn set_sampling_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sampling_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_fields`.\n"]
    pub fn set_exclude_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_ExcludeFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_fields`.\n"]
    pub fn set_include_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_IncludeFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `post_scan_actions`.\n"]
    pub fn set_post_scan_actions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataProfileSpec_PostScanActions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_scan_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_scan_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataProfileSpec {
    type O = terrars::BlockAssignable<DataplexDatascan_DataProfileSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataProfileSpec {}

impl BuildDataplexDatascan_DataProfileSpec {
    pub fn build(self) -> DataplexDatascan_DataProfileSpec {
        DataplexDatascan_DataProfileSpec {
            catalog_publishing_enabled: core::default::Default::default(),
            row_filter: core::default::Default::default(),
            sampling_percent: core::default::Default::default(),
            exclude_fields: core::default::Default::default(),
            include_fields: core::default::Default::default(),
            post_scan_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataProfileSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataProfileSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_DataProfileSpecRef {
        DataplexDatascan_DataProfileSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataProfileSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `catalog_publishing_enabled` after provisioning.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn catalog_publishing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_publishing_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `row_filter` after provisioning.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn row_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sampling_percent` after provisioning.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn sampling_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_fields` after provisioning.\n"]
    pub fn exclude_fields(&self) -> terrars::ListRef<DataplexDatascan_DataProfileSpec_ExcludeFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `include_fields` after provisioning.\n"]
    pub fn include_fields(&self) -> terrars::ListRef<DataplexDatascan_DataProfileSpec_IncludeFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `post_scan_actions` after provisioning.\n"]
    pub fn post_scan_actions(&self) -> terrars::ListRef<DataplexDatascan_DataProfileSpec_PostScanActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_scan_actions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    results_table: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport {
    #[doc =
        "Set the field `results_table`.\nThe BigQuery table to export DataQualityScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn set_results_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.results_table = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport {}

impl BuildDataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport {
        DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport {
            results_table: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExportRef {
        DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `results_table` after provisioning.\nThe BigQuery table to export DataQualityScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn results_table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.results_table", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger {}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger {
    type O =
        terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger {}

impl BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger {}
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTriggerRef {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger {}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger { }

impl terrars
::ToListMappable for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger {
    type O =
        terrars
        ::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger {}

impl BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger {}
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTriggerRef {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients {
    #[doc = "Set the field `emails`.\nThe email recipients who will receive the DataQualityScan results report."]
    pub fn set_emails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.emails = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients {
    type O =
        terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients {}

impl BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients {
            emails: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_RecipientsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_RecipientsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_RecipientsRef {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_RecipientsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_RecipientsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `emails` after provisioning.\nThe email recipients who will receive the DataQualityScan results report."]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger {
    #[doc = "Set the field `score_threshold`.\nThe score range is in [0,100]."]
    pub fn set_score_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger {
    type O =
        terrars
        ::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger {}

impl BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger {
            score_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTriggerRef {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score_threshold` after provisioning.\nThe score range is in [0,100]."]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportDynamic {
    job_end_trigger: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger>,
    >,
    job_failure_trigger: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger>,
    >,
    recipients: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients>>,
    score_threshold_trigger: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger>,
    >,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_end_trigger: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_failure_trigger: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recipients: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_threshold_trigger: std
    ::option
    ::Option<
        std::vec::Vec<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger>,
    >,
    dynamic: DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportDynamic,
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport {
    #[doc = "Set the field `job_end_trigger`.\n"]
    pub fn set_job_end_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTrigger,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.job_end_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.job_end_trigger = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_failure_trigger`.\n"]
    pub fn set_job_failure_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTrigger,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.job_failure_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.job_failure_trigger = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recipients`.\n"]
    pub fn set_recipients(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_Recipients,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recipients = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recipients = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `score_threshold_trigger`.\n"]
    pub fn set_score_threshold_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTrigger,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.score_threshold_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.score_threshold_trigger = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport {}

impl BuildDataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport {
            job_end_trigger: core::default::Default::default(),
            job_failure_trigger: core::default::Default::default(),
            recipients: core::default::Default::default(),
            score_threshold_trigger: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportRef {
        DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `job_end_trigger` after provisioning.\n"]
    pub fn job_end_trigger(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobEndTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_end_trigger", self.base))
    }

    #[doc = "Get a reference to the value of field `job_failure_trigger` after provisioning.\n"]
    pub fn job_failure_trigger(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_JobFailureTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_failure_trigger", self.base))
    }

    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_RecipientsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recipients", self.base))
    }

    #[doc = "Get a reference to the value of field `score_threshold_trigger` after provisioning.\n"]
    pub fn score_threshold_trigger(
        &self,
    ) -> terrars
    ::ListRef<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport_ScoreThresholdTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.score_threshold_trigger", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataQualitySpec_PostScanActionsDynamic {
    bigquery_export: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport>>,
    notification_report: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_export: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_report: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport>>,
    dynamic: DataplexDatascan_DataQualitySpec_PostScanActionsDynamic,
}

impl DataplexDatascan_DataQualitySpec_PostScanActions {
    #[doc = "Set the field `bigquery_export`.\n"]
    pub fn set_bigquery_export(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExport>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_export = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_export = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_report`.\n"]
    pub fn set_notification_report(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReport>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_report = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_report = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_PostScanActions {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_PostScanActions {}

impl BuildDataplexDatascan_DataQualitySpec_PostScanActions {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_PostScanActions {
        DataplexDatascan_DataQualitySpec_PostScanActions {
            bigquery_export: core::default::Default::default(),
            notification_report: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_PostScanActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_PostScanActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_PostScanActionsRef {
        DataplexDatascan_DataQualitySpec_PostScanActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_PostScanActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bigquery_export` after provisioning.\n"]
    pub fn bigquery_export(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_PostScanActions_BigqueryExportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_export", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_report` after provisioning.\n"]
    pub fn notification_report(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_PostScanActions_NotificationReportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_report", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation {}

impl DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_NonNullExpectation {}

impl BuildDataplexDatascan_DataQualitySpec_Rules_NonNullExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation {
        DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation {}
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_NonNullExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_NonNullExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_NonNullExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_NonNullExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_NonNullExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_RangeExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_max_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_min_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataplexDatascan_DataQualitySpec_Rules_RangeExpectation {
    #[doc =
        "Set the field `max_value`.\nThe maximum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn set_max_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_value`.\nThe minimum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn set_min_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strict_max_enabled`.\nWhether each value needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn set_strict_max_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_max_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strict_min_enabled`.\nWhether each value needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn set_strict_min_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_min_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_RangeExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_RangeExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_RangeExpectation {}

impl BuildDataplexDatascan_DataQualitySpec_Rules_RangeExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_RangeExpectation {
        DataplexDatascan_DataQualitySpec_Rules_RangeExpectation {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_RangeExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_RangeExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_RangeExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_RangeExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_RangeExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_value` after provisioning.\nThe maximum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_value` after provisioning.\nThe minimum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strict_max_enabled` after provisioning.\nWhether each value needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn strict_max_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_max_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strict_min_enabled` after provisioning.\nWhether each value needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn strict_min_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_min_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_RegexExpectation {
    regex: terrars::PrimField<std::string::String>,
}

impl DataplexDatascan_DataQualitySpec_Rules_RegexExpectation { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_RegexExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_RegexExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_RegexExpectation {
    #[doc = "A regular expression the column value is expected to match."]
    pub regex: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_RegexExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_RegexExpectation {
        DataplexDatascan_DataQualitySpec_Rules_RegexExpectation { regex: self.regex }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_RegexExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_RegexExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_RegexExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_RegexExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_RegexExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `regex` after provisioning.\nA regular expression the column value is expected to match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation {
    sql_expression: terrars::PrimField<std::string::String>,
}

impl DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation {
    #[doc = "The SQL expression."]
    pub sql_expression: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation {
        DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation { sql_expression: self.sql_expression }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\nThe SQL expression."]
    pub fn sql_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_SetExpectation {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_DataQualitySpec_Rules_SetExpectation { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_SetExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_SetExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_SetExpectation {
    #[doc = "Expected values for the column value."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_SetExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_SetExpectation {
        DataplexDatascan_DataQualitySpec_Rules_SetExpectation { values: self.values }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_SetExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_SetExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_SetExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_SetExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_SetExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\nExpected values for the column value."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_SqlAssertion {
    sql_statement: terrars::PrimField<std::string::String>,
}

impl DataplexDatascan_DataQualitySpec_Rules_SqlAssertion { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_SqlAssertion {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_SqlAssertion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_SqlAssertion {
    #[doc = "The SQL statement."]
    pub sql_statement: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_SqlAssertion {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_SqlAssertion {
        DataplexDatascan_DataQualitySpec_Rules_SqlAssertion { sql_statement: self.sql_statement }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_SqlAssertionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_SqlAssertionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_SqlAssertionRef {
        DataplexDatascan_DataQualitySpec_Rules_SqlAssertionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_SqlAssertionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sql_statement` after provisioning.\nThe SQL statement."]
    pub fn sql_statement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_statement", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
    statistic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_max_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_min_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation {
    #[doc =
        "Set the field `max_value`.\nThe maximum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn set_max_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_value`.\nThe minimum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn set_min_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strict_max_enabled`.\nWhether column statistic needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn set_strict_max_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_max_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strict_min_enabled`.\nWhether column statistic needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn set_strict_min_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_min_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation {
    #[doc = "column statistics. Possible values: [\"STATISTIC_UNDEFINED\", \"MEAN\", \"MIN\", \"MAX\"]"]
    pub statistic: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation {
        DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            statistic: self.statistic,
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_value` after provisioning.\nThe maximum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_value` after provisioning.\nThe minimum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `statistic` after provisioning.\ncolumn statistics. Possible values: [\"STATISTIC_UNDEFINED\", \"MEAN\", \"MIN\", \"MAX\"]"]
    pub fn statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strict_max_enabled` after provisioning.\nWhether column statistic needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn strict_max_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_max_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strict_min_enabled` after provisioning.\nWhether column statistic needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn strict_min_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_min_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation {
    sql_expression: terrars::PrimField<std::string::String>,
}

impl DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation {
    #[doc = "The SQL expression."]
    pub sql_expression: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation {
        DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation { sql_expression: self.sql_expression }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\nThe SQL expression."]
    pub fn sql_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The string representation of the parameter value."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values {
        DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_TemplateReference_ValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_TemplateReference_ValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_TemplateReference_ValuesRef {
        DataplexDatascan_DataQualitySpec_Rules_TemplateReference_ValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_TemplateReference_ValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe string representation of the parameter value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceDynamic {
    values: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_TemplateReference {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values>>,
    dynamic: DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceDynamic,
}

impl DataplexDatascan_DataQualitySpec_Rules_TemplateReference {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_TemplateReference_Values>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_TemplateReference {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_TemplateReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_TemplateReference {
    #[doc = "The resource name of the template entry."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules_TemplateReference {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_TemplateReference {
        DataplexDatascan_DataQualitySpec_Rules_TemplateReference {
            name: self.name,
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceRef {
        DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the template entry."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation {}

impl DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation { }

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation {}

impl BuildDataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation {
        DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation {}
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectationRef {
        DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataQualitySpec_RulesDynamic {
    non_null_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation>>,
    range_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_RangeExpectation>>,
    regex_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_RegexExpectation>>,
    row_condition_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation>>,
    set_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_SetExpectation>>,
    sql_assertion: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_SqlAssertion>>,
    statistic_range_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation>>,
    table_condition_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation>>,
    template_reference: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_TemplateReference>>,
    uniqueness_expectation: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    dimension: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_null: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_null_expectation: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_expectation: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_RangeExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_expectation: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_RegexExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_condition_expectation: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_expectation: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_SetExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_assertion: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_SqlAssertion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistic_range_expectation: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_condition_expectation: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_reference: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_TemplateReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uniqueness_expectation: std
    ::option
    ::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation>>,
    dynamic: DataplexDatascan_DataQualitySpec_RulesDynamic,
}

impl DataplexDatascan_DataQualitySpec_Rules {
    #[doc = "Set the field `attributes`.\nMap of attribute name and value linked to the rule."]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column`.\nThe unnested column which this rule is evaluated against."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the rule.\nThe maximum length is 1,024 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_null`.\nRows with null values will automatically fail a rule, unless ignoreNull is true. In that case, such null rows are trivially considered passing. Only applicable to ColumnMap rules."]
    pub fn set_ignore_null(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_null = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nA mutable name for the rule.\nThe name must contain only letters (a-z, A-Z), numbers (0-9), or hyphens (-).\nThe maximum length is 63 characters.\nMust start with a letter.\nMust end with a number or a letter."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspended`.\nWhether the Rule is active or suspended. Default = false."]
    pub fn set_suspended(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `threshold`.\nThe minimum ratio of passing_rows / total_rows required to pass this rule, with a range of [0.0, 1.0]. 0 indicates default value (i.e. 1.0)."]
    pub fn set_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_null_expectation`.\n"]
    pub fn set_non_null_expectation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_NonNullExpectation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.non_null_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.non_null_expectation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `range_expectation`.\n"]
    pub fn set_range_expectation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_RangeExpectation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range_expectation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_expectation`.\n"]
    pub fn set_regex_expectation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_RegexExpectation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_expectation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `row_condition_expectation`.\n"]
    pub fn set_row_condition_expectation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.row_condition_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.row_condition_expectation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `set_expectation`.\n"]
    pub fn set_set_expectation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_SetExpectation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.set_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.set_expectation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_assertion`.\n"]
    pub fn set_sql_assertion(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_SqlAssertion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_assertion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_assertion = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `statistic_range_expectation`.\n"]
    pub fn set_statistic_range_expectation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statistic_range_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statistic_range_expectation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_condition_expectation`.\n"]
    pub fn set_table_condition_expectation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_condition_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_condition_expectation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template_reference`.\n"]
    pub fn set_template_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_TemplateReference>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.template_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.template_reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `uniqueness_expectation`.\n"]
    pub fn set_uniqueness_expectation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.uniqueness_expectation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.uniqueness_expectation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec_Rules {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec_Rules {
    #[doc =
        "The dimension name a rule belongs to. Custom dimension name is supported with all uppercase letters and maximum length of 30 characters."]
    pub dimension: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_DataQualitySpec_Rules {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec_Rules {
        DataplexDatascan_DataQualitySpec_Rules {
            attributes: core::default::Default::default(),
            column: core::default::Default::default(),
            description: core::default::Default::default(),
            dimension: self.dimension,
            ignore_null: core::default::Default::default(),
            name: core::default::Default::default(),
            suspended: core::default::Default::default(),
            threshold: core::default::Default::default(),
            non_null_expectation: core::default::Default::default(),
            range_expectation: core::default::Default::default(),
            regex_expectation: core::default::Default::default(),
            row_condition_expectation: core::default::Default::default(),
            set_expectation: core::default::Default::default(),
            sql_assertion: core::default::Default::default(),
            statistic_range_expectation: core::default::Default::default(),
            table_condition_expectation: core::default::Default::default(),
            template_reference: core::default::Default::default(),
            uniqueness_expectation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpec_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpec_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_DataQualitySpec_RulesRef {
        DataplexDatascan_DataQualitySpec_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpec_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes` after provisioning.\nMap of attribute name and value linked to the rule."]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `column` after provisioning.\nThe unnested column which this rule is evaluated against."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the rule.\nThe maximum length is 1,024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dimension` after provisioning.\nThe dimension name a rule belongs to. Custom dimension name is supported with all uppercase letters and maximum length of 30 characters."]
    pub fn dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_null` after provisioning.\nRows with null values will automatically fail a rule, unless ignoreNull is true. In that case, such null rows are trivially considered passing. Only applicable to ColumnMap rules."]
    pub fn ignore_null(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_null", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA mutable name for the rule.\nThe name must contain only letters (a-z, A-Z), numbers (0-9), or hyphens (-).\nThe maximum length is 63 characters.\nMust start with a letter.\nMust end with a number or a letter."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nWhether the Rule is active or suspended. Default = false."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe minimum ratio of passing_rows / total_rows required to pass this rule, with a range of [0.0, 1.0]. 0 indicates default value (i.e. 1.0)."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `non_null_expectation` after provisioning.\n"]
    pub fn non_null_expectation(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_NonNullExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_null_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `range_expectation` after provisioning.\n"]
    pub fn range_expectation(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_RangeExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_expectation` after provisioning.\n"]
    pub fn regex_expectation(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_RegexExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `row_condition_expectation` after provisioning.\n"]
    pub fn row_condition_expectation(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_RowConditionExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_condition_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `set_expectation` after provisioning.\n"]
    pub fn set_expectation(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_SetExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_assertion` after provisioning.\n"]
    pub fn sql_assertion(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_SqlAssertionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_assertion", self.base))
    }

    #[doc = "Get a reference to the value of field `statistic_range_expectation` after provisioning.\n"]
    pub fn statistic_range_expectation(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_StatisticRangeExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statistic_range_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `table_condition_expectation` after provisioning.\n"]
    pub fn table_condition_expectation(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_TableConditionExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_condition_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `template_reference` after provisioning.\n"]
    pub fn template_reference(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_TemplateReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `uniqueness_expectation` after provisioning.\n"]
    pub fn uniqueness_expectation(
        &self,
    ) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_Rules_UniquenessExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uniqueness_expectation", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_DataQualitySpecDynamic {
    post_scan_actions: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_PostScanActions>>,
    rules: std::option::Option<terrars::DynamicBlock<DataplexDatascan_DataQualitySpec_Rules>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_DataQualitySpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_publishing_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_catalog_based_rules: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_scan_actions: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_PostScanActions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<DataplexDatascan_DataQualitySpec_Rules>>,
    dynamic: DataplexDatascan_DataQualitySpecDynamic,
}

impl DataplexDatascan_DataQualitySpec {
    #[doc =
        "Set the field `catalog_publishing_enabled`.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn set_catalog_publishing_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.catalog_publishing_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_catalog_based_rules`.\nIf set to true, the scan will retrieve rules defined in Data Catalog for the resource."]
    pub fn set_enable_catalog_based_rules(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_catalog_based_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\nA filter to selectively run a subset of rules."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `row_filter`.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn set_row_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.row_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sampling_percent`.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn set_sampling_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sampling_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_scan_actions`.\n"]
    pub fn set_post_scan_actions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_PostScanActions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_scan_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_scan_actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_DataQualitySpec_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_DataQualitySpec {
    type O = terrars::BlockAssignable<DataplexDatascan_DataQualitySpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_DataQualitySpec {}

impl BuildDataplexDatascan_DataQualitySpec {
    pub fn build(self) -> DataplexDatascan_DataQualitySpec {
        DataplexDatascan_DataQualitySpec {
            catalog_publishing_enabled: core::default::Default::default(),
            enable_catalog_based_rules: core::default::Default::default(),
            filter: core::default::Default::default(),
            row_filter: core::default::Default::default(),
            sampling_percent: core::default::Default::default(),
            post_scan_actions: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_DataQualitySpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_DataQualitySpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_DataQualitySpecRef {
        DataplexDatascan_DataQualitySpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_DataQualitySpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `catalog_publishing_enabled` after provisioning.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn catalog_publishing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_publishing_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_catalog_based_rules` after provisioning.\nIf set to true, the scan will retrieve rules defined in Data Catalog for the resource."]
    pub fn enable_catalog_based_rules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_catalog_based_rules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA filter to selectively run a subset of rules."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `row_filter` after provisioning.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn row_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sampling_percent` after provisioning.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn sampling_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `post_scan_actions` after provisioning.\n"]
    pub fn post_scan_actions(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_PostScanActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_scan_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataplexDatascan_DataQualitySpec_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionIdentity_DataplexServiceAgent {}

impl DataplexDatascan_ExecutionIdentity_DataplexServiceAgent { }

impl terrars::ToListMappable for DataplexDatascan_ExecutionIdentity_DataplexServiceAgent {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity_DataplexServiceAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionIdentity_DataplexServiceAgent {}

impl BuildDataplexDatascan_ExecutionIdentity_DataplexServiceAgent {
    pub fn build(self) -> DataplexDatascan_ExecutionIdentity_DataplexServiceAgent {
        DataplexDatascan_ExecutionIdentity_DataplexServiceAgent {}
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionIdentity_DataplexServiceAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionIdentity_DataplexServiceAgentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_ExecutionIdentity_DataplexServiceAgentRef {
        DataplexDatascan_ExecutionIdentity_DataplexServiceAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionIdentity_DataplexServiceAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionIdentity_ServiceAccount {
    email: terrars::PrimField<std::string::String>,
}

impl DataplexDatascan_ExecutionIdentity_ServiceAccount { }

impl terrars::ToListMappable for DataplexDatascan_ExecutionIdentity_ServiceAccount {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity_ServiceAccount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionIdentity_ServiceAccount {
    #[doc = "Service account email."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_ExecutionIdentity_ServiceAccount {
    pub fn build(self) -> DataplexDatascan_ExecutionIdentity_ServiceAccount {
        DataplexDatascan_ExecutionIdentity_ServiceAccount { email: self.email }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionIdentity_ServiceAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionIdentity_ServiceAccountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_ExecutionIdentity_ServiceAccountRef {
        DataplexDatascan_ExecutionIdentity_ServiceAccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionIdentity_ServiceAccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nService account email."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionIdentity_UserCredential {}

impl DataplexDatascan_ExecutionIdentity_UserCredential { }

impl terrars::ToListMappable for DataplexDatascan_ExecutionIdentity_UserCredential {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity_UserCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionIdentity_UserCredential {}

impl BuildDataplexDatascan_ExecutionIdentity_UserCredential {
    pub fn build(self) -> DataplexDatascan_ExecutionIdentity_UserCredential {
        DataplexDatascan_ExecutionIdentity_UserCredential {}
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionIdentity_UserCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionIdentity_UserCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_ExecutionIdentity_UserCredentialRef {
        DataplexDatascan_ExecutionIdentity_UserCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionIdentity_UserCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_ExecutionIdentityDynamic {
    dataplex_service_agent: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionIdentity_DataplexServiceAgent>>,
    service_account: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionIdentity_ServiceAccount>>,
    user_credential: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionIdentity_UserCredential>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionIdentity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataplex_service_agent: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionIdentity_DataplexServiceAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionIdentity_ServiceAccount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_credential: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionIdentity_UserCredential>>,
    dynamic: DataplexDatascan_ExecutionIdentityDynamic,
}

impl DataplexDatascan_ExecutionIdentity {
    #[doc = "Set the field `dataplex_service_agent`.\n"]
    pub fn set_dataplex_service_agent(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity_DataplexServiceAgent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataplex_service_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataplex_service_agent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity_ServiceAccount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_credential`.\n"]
    pub fn set_user_credential(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity_UserCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_credential = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_ExecutionIdentity {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionIdentity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionIdentity {}

impl BuildDataplexDatascan_ExecutionIdentity {
    pub fn build(self) -> DataplexDatascan_ExecutionIdentity {
        DataplexDatascan_ExecutionIdentity {
            dataplex_service_agent: core::default::Default::default(),
            service_account: core::default::Default::default(),
            user_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionIdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_ExecutionIdentityRef {
        DataplexDatascan_ExecutionIdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionIdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataplex_service_agent` after provisioning.\n"]
    pub fn dataplex_service_agent(&self) -> terrars::ListRef<DataplexDatascan_ExecutionIdentity_DataplexServiceAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataplex_service_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::ListRef<DataplexDatascan_ExecutionIdentity_ServiceAccountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `user_credential` after provisioning.\n"]
    pub fn user_credential(&self) -> terrars::ListRef<DataplexDatascan_ExecutionIdentity_UserCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionSpec_Trigger_OnDemand {}

impl DataplexDatascan_ExecutionSpec_Trigger_OnDemand { }

impl terrars::ToListMappable for DataplexDatascan_ExecutionSpec_Trigger_OnDemand {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger_OnDemand>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionSpec_Trigger_OnDemand {}

impl BuildDataplexDatascan_ExecutionSpec_Trigger_OnDemand {
    pub fn build(self) -> DataplexDatascan_ExecutionSpec_Trigger_OnDemand {
        DataplexDatascan_ExecutionSpec_Trigger_OnDemand {}
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionSpec_Trigger_OnDemandRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionSpec_Trigger_OnDemandRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_ExecutionSpec_Trigger_OnDemandRef {
        DataplexDatascan_ExecutionSpec_Trigger_OnDemandRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionSpec_Trigger_OnDemandRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionSpec_Trigger_OneTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl_after_scan_completion: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_ExecutionSpec_Trigger_OneTime {
    #[doc =
        "Set the field `ttl_after_scan_completion`.\nTime to live for the DataScan and its results after the one-time run completes. Accepts a string with a unit suffix 's' (e.g., '7200s'). Default is 24 hours. Ranges between 0 and 31536000 seconds (1 year)."]
    pub fn set_ttl_after_scan_completion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ttl_after_scan_completion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_ExecutionSpec_Trigger_OneTime {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger_OneTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionSpec_Trigger_OneTime {}

impl BuildDataplexDatascan_ExecutionSpec_Trigger_OneTime {
    pub fn build(self) -> DataplexDatascan_ExecutionSpec_Trigger_OneTime {
        DataplexDatascan_ExecutionSpec_Trigger_OneTime {
            ttl_after_scan_completion: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionSpec_Trigger_OneTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionSpec_Trigger_OneTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_ExecutionSpec_Trigger_OneTimeRef {
        DataplexDatascan_ExecutionSpec_Trigger_OneTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionSpec_Trigger_OneTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ttl_after_scan_completion` after provisioning.\nTime to live for the DataScan and its results after the one-time run completes. Accepts a string with a unit suffix 's' (e.g., '7200s'). Default is 24 hours. Ranges between 0 and 31536000 seconds (1 year)."]
    pub fn ttl_after_scan_completion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl_after_scan_completion", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionSpec_Trigger_Schedule {
    cron: terrars::PrimField<std::string::String>,
}

impl DataplexDatascan_ExecutionSpec_Trigger_Schedule { }

impl terrars::ToListMappable for DataplexDatascan_ExecutionSpec_Trigger_Schedule {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionSpec_Trigger_Schedule {
    #[doc = "Cron schedule for running scans periodically. This field is required for Schedule scans."]
    pub cron: terrars::PrimField<std::string::String>,
}

impl BuildDataplexDatascan_ExecutionSpec_Trigger_Schedule {
    pub fn build(self) -> DataplexDatascan_ExecutionSpec_Trigger_Schedule {
        DataplexDatascan_ExecutionSpec_Trigger_Schedule { cron: self.cron }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionSpec_Trigger_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionSpec_Trigger_ScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexDatascan_ExecutionSpec_Trigger_ScheduleRef {
        DataplexDatascan_ExecutionSpec_Trigger_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionSpec_Trigger_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cron` after provisioning.\nCron schedule for running scans periodically. This field is required for Schedule scans."]
    pub fn cron(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_ExecutionSpec_TriggerDynamic {
    on_demand: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionSpec_Trigger_OnDemand>>,
    one_time: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionSpec_Trigger_OneTime>>,
    schedule: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionSpec_Trigger_Schedule>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionSpec_Trigger {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionSpec_Trigger_OnDemand>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    one_time: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionSpec_Trigger_OneTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionSpec_Trigger_Schedule>>,
    dynamic: DataplexDatascan_ExecutionSpec_TriggerDynamic,
}

impl DataplexDatascan_ExecutionSpec_Trigger {
    #[doc = "Set the field `on_demand`.\n"]
    pub fn set_on_demand(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger_OnDemand>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_demand = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_demand = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `one_time`.\n"]
    pub fn set_one_time(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger_OneTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.one_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.one_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger_Schedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_ExecutionSpec_Trigger {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionSpec_Trigger {}

impl BuildDataplexDatascan_ExecutionSpec_Trigger {
    pub fn build(self) -> DataplexDatascan_ExecutionSpec_Trigger {
        DataplexDatascan_ExecutionSpec_Trigger {
            on_demand: core::default::Default::default(),
            one_time: core::default::Default::default(),
            schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionSpec_TriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionSpec_TriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_ExecutionSpec_TriggerRef {
        DataplexDatascan_ExecutionSpec_TriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionSpec_TriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand` after provisioning.\n"]
    pub fn on_demand(&self) -> terrars::ListRef<DataplexDatascan_ExecutionSpec_Trigger_OnDemandRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand", self.base))
    }

    #[doc = "Get a reference to the value of field `one_time` after provisioning.\n"]
    pub fn one_time(&self) -> terrars::ListRef<DataplexDatascan_ExecutionSpec_Trigger_OneTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.one_time", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataplexDatascan_ExecutionSpec_Trigger_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexDatascan_ExecutionSpecDynamic {
    trigger: std::option::Option<terrars::DynamicBlock<DataplexDatascan_ExecutionSpec_Trigger>>,
}

#[derive(Serialize)]
pub struct DataplexDatascan_ExecutionSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger: std::option::Option<std::vec::Vec<DataplexDatascan_ExecutionSpec_Trigger>>,
    dynamic: DataplexDatascan_ExecutionSpecDynamic,
}

impl DataplexDatascan_ExecutionSpec {
    #[doc =
        "Set the field `field`.\nThe unnested field (of type Date or Timestamp) that contains values which monotonically increase over time. If not specified, a data scan will run for all data in the table."]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexDatascan_ExecutionSpec_Trigger>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexDatascan_ExecutionSpec {
    type O = terrars::BlockAssignable<DataplexDatascan_ExecutionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_ExecutionSpec {}

impl BuildDataplexDatascan_ExecutionSpec {
    pub fn build(self) -> DataplexDatascan_ExecutionSpec {
        DataplexDatascan_ExecutionSpec {
            field: core::default::Default::default(),
            trigger: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_ExecutionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_ExecutionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_ExecutionSpecRef {
        DataplexDatascan_ExecutionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_ExecutionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nThe unnested field (of type Date or Timestamp) that contains values which monotonically increase over time. If not specified, a data scan will run for all data in the table."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> terrars::ListRef<DataplexDatascan_ExecutionSpec_TriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexDatascan_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexDatascan_Timeouts {
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

impl terrars::ToListMappable for DataplexDatascan_Timeouts {
    type O = terrars::BlockAssignable<DataplexDatascan_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexDatascan_Timeouts {}

impl BuildDataplexDatascan_Timeouts {
    pub fn build(self) -> DataplexDatascan_Timeouts {
        DataplexDatascan_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexDatascan_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexDatascan_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexDatascan_TimeoutsRef {
        DataplexDatascan_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexDatascan_TimeoutsRef {
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
