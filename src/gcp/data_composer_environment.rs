#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComposerEnvironmentData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComposerEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComposerEnvironmentData_>,
}

#[derive(Clone)]
pub struct DataComposerEnvironment(std::rc::Rc<DataComposerEnvironment_>);

impl DataComposerEnvironment {
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

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe location or Compute Engine region for the environment."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nConfiguration parameters for this environment."]
    pub fn config(&self) -> terrars::ListRef<DataComposerEnvironment_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe location or Compute Engine region for the environment."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_config` after provisioning.\nConfiguration options for storage used by Composer environment."]
    pub fn storage_config(&self) -> terrars::ListRef<DataComposerEnvironment_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl terrars::Referable for DataComposerEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComposerEnvironment { }

impl terrars::ToListMappable for DataComposerEnvironment {
    type O = terrars::ListRef<DataComposerEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComposerEnvironment_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_composer_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComposerEnvironment {
    pub tf_id: std::string::String,
    #[doc = "Name of the environment."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComposerEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComposerEnvironment {
        let out = DataComposerEnvironment(std::rc::Rc::new(DataComposerEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComposerEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComposerEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComposerEnvironmentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nConfiguration parameters for this environment."]
    pub fn config(&self) -> terrars::ListRef<DataComposerEnvironment_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe location or Compute Engine region for the environment."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_config` after provisioning.\nConfiguration options for storage used by Composer environment."]
    pub fn storage_config(&self) -> terrars::ListRef<DataComposerEnvironment_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    #[doc = "Set the field `retention_days`.\n"]
    pub fn set_retention_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_mode`.\n"]
    pub fn set_retention_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    type O =
        terrars::BlockAssignable<DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {}

impl BuildDataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
        DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
            retention_days: core::default::Default::default(),
            retention_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
        DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_days` after provisioning.\n"]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_mode` after provisioning.\n"]
    pub fn retention_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    #[doc = "Set the field `storage_mode`.\n"]
    pub fn set_storage_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {}

impl BuildDataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
        DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
            storage_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
        DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_mode` after provisioning.\n"]
    pub fn storage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_DataRetentionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airflow_metadata_retention_config: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_logs_retention_config: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig>>,
}

impl DataComposerEnvironment_Config_DataRetentionConfig {
    #[doc = "Set the field `airflow_metadata_retention_config`.\n"]
    pub fn set_airflow_metadata_retention_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig>,
                    >,
    ) -> Self {
        self.airflow_metadata_retention_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_logs_retention_config`.\n"]
    pub fn set_task_logs_retention_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig>,
                    >,
    ) -> Self {
        self.task_logs_retention_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_DataRetentionConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_DataRetentionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_DataRetentionConfig {}

impl BuildDataComposerEnvironment_Config_DataRetentionConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_DataRetentionConfig {
        DataComposerEnvironment_Config_DataRetentionConfig {
            airflow_metadata_retention_config: core::default::Default::default(),
            task_logs_retention_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_DataRetentionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_DataRetentionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_DataRetentionConfigRef {
        DataComposerEnvironment_Config_DataRetentionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_DataRetentionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `airflow_metadata_retention_config` after provisioning.\n"]
    pub fn airflow_metadata_retention_config(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.airflow_metadata_retention_config", self.base))
    }

    #[doc = "Get a reference to the value of field `task_logs_retention_config` after provisioning.\n"]
    pub fn task_logs_retention_config(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_logs_retention_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_DatabaseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_DatabaseConfig {
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_DatabaseConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_DatabaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_DatabaseConfig {}

impl BuildDataComposerEnvironment_Config_DatabaseConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_DatabaseConfig {
        DataComposerEnvironment_Config_DatabaseConfig {
            machine_type: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_DatabaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_DatabaseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_DatabaseConfigRef {
        DataComposerEnvironment_Config_DatabaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_DatabaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_EncryptionConfig {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_EncryptionConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_EncryptionConfig {}

impl BuildDataComposerEnvironment_Config_EncryptionConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_EncryptionConfig {
        DataComposerEnvironment_Config_EncryptionConfig { kms_key_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_EncryptionConfigRef {
        DataComposerEnvironment_Config_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_MaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_MaintenanceWindow {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recurrence`.\n"]
    pub fn set_recurrence(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recurrence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_MaintenanceWindow {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_MaintenanceWindow {}

impl BuildDataComposerEnvironment_Config_MaintenanceWindow {
    pub fn build(self) -> DataComposerEnvironment_Config_MaintenanceWindow {
        DataComposerEnvironment_Config_MaintenanceWindow {
            end_time: core::default::Default::default(),
            recurrence: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_MaintenanceWindowRef {
        DataComposerEnvironment_Config_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {}

impl BuildDataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    pub fn build(self) -> DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
        DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
            cidr_block: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
        DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std
    ::option
    ::Option<terrars::SetField<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks>,
                    >,
    ) -> Self {
        self.cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_MasterAuthorizedNetworksConfig {}

impl BuildDataComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
        DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
            cidr_blocks: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
        DataComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(
        &self,
    ) -> terrars::SetRef<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_ip_aliases: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
    #[doc = "Set the field `cluster_ipv4_cidr_block`.\n"]
    pub fn set_cluster_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_secondary_range_name`.\n"]
    pub fn set_cluster_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services_ipv4_cidr_block`.\n"]
    pub fn set_services_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services_secondary_range_name`.\n"]
    pub fn set_services_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_ip_aliases`.\n"]
    pub fn set_use_ip_aliases(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_ip_aliases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {}

impl BuildDataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
    pub fn build(self) -> DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
        DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            use_ip_aliases: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
        DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\n"]
    pub fn cluster_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\n"]
    pub fn cluster_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_secondary_range_name", self.base))
    }

    #[doc = "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\n"]
    pub fn services_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `services_secondary_range_name` after provisioning.\n"]
    pub fn services_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_secondary_range_name", self.base))
    }

    #[doc = "Get a reference to the value of field `use_ip_aliases` after provisioning.\n"]
    pub fn use_ip_aliases(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ip_aliases", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composer_internal_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composer_network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ip_masq_agent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_allocation_policy: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_NodeConfig {
    #[doc = "Set the field `composer_internal_ipv4_cidr_block`.\n"]
    pub fn set_composer_internal_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.composer_internal_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `composer_network_attachment`.\n"]
    pub fn set_composer_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.composer_network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ip_masq_agent`.\n"]
    pub fn set_enable_ip_masq_agent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ip_masq_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_allocation_policy`.\n"]
    pub fn set_ip_allocation_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicy>>,
    ) -> Self {
        self.ip_allocation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_NodeConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_NodeConfig {}

impl BuildDataComposerEnvironment_Config_NodeConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_NodeConfig {
        DataComposerEnvironment_Config_NodeConfig {
            composer_internal_ipv4_cidr_block: core::default::Default::default(),
            composer_network_attachment: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            enable_ip_masq_agent: core::default::Default::default(),
            ip_allocation_policy: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            network: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComposerEnvironment_Config_NodeConfigRef {
        DataComposerEnvironment_Config_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_NodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `composer_internal_ipv4_cidr_block` after provisioning.\n"]
    pub fn composer_internal_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.composer_internal_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `composer_network_attachment` after provisioning.\n"]
    pub fn composer_network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.composer_network_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_ip_masq_agent` after provisioning.\n"]
    pub fn enable_ip_masq_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ip_masq_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_PrivateEnvironmentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_composer_connection_subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_composer_network_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_privately_used_public_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_PrivateEnvironmentConfig {
    #[doc = "Set the field `cloud_composer_connection_subnetwork`.\n"]
    pub fn set_cloud_composer_connection_subnetwork(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_composer_connection_subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_composer_network_ipv4_cidr_block`.\n"]
    pub fn set_cloud_composer_network_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_composer_network_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_sql_ipv4_cidr_block`.\n"]
    pub fn set_cloud_sql_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_sql_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_endpoint`.\n"]
    pub fn set_enable_private_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_privately_used_public_ips`.\n"]
    pub fn set_enable_privately_used_public_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_privately_used_public_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_ipv4_cidr_block`.\n"]
    pub fn set_master_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_server_ipv4_cidr_block`.\n"]
    pub fn set_web_server_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.web_server_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_PrivateEnvironmentConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_PrivateEnvironmentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_PrivateEnvironmentConfig {}

impl BuildDataComposerEnvironment_Config_PrivateEnvironmentConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_PrivateEnvironmentConfig {
        DataComposerEnvironment_Config_PrivateEnvironmentConfig {
            cloud_composer_connection_subnetwork: core::default::Default::default(),
            cloud_composer_network_ipv4_cidr_block: core::default::Default::default(),
            cloud_sql_ipv4_cidr_block: core::default::Default::default(),
            connection_type: core::default::Default::default(),
            enable_private_endpoint: core::default::Default::default(),
            enable_privately_used_public_ips: core::default::Default::default(),
            master_ipv4_cidr_block: core::default::Default::default(),
            web_server_ipv4_cidr_block: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_PrivateEnvironmentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_PrivateEnvironmentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_PrivateEnvironmentConfigRef {
        DataComposerEnvironment_Config_PrivateEnvironmentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_PrivateEnvironmentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_composer_connection_subnetwork` after provisioning.\n"]
    pub fn cloud_composer_connection_subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_composer_connection_subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_composer_network_ipv4_cidr_block` after provisioning.\n"]
    pub fn cloud_composer_network_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_composer_network_ipv4_cidr_block", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloud_sql_ipv4_cidr_block` after provisioning.\n"]
    pub fn cloud_sql_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_private_endpoint` after provisioning.\n"]
    pub fn enable_private_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_privately_used_public_ips` after provisioning.\n"]
    pub fn enable_privately_used_public_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_privately_used_public_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\n"]
    pub fn master_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_ipv4_cidr_block` after provisioning.\n"]
    pub fn web_server_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_server_ipv4_cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_creation_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_creation_schedule`.\n"]
    pub fn set_snapshot_creation_schedule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.snapshot_creation_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_location`.\n"]
    pub fn set_snapshot_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {}

impl BuildDataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
        DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
            enabled: core::default::Default::default(),
            snapshot_creation_schedule: core::default::Default::default(),
            snapshot_location: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
        DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_creation_schedule` after provisioning.\n"]
    pub fn snapshot_creation_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_creation_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_location` after provisioning.\n"]
    pub fn snapshot_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_location", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_RecoveryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_snapshots_config: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig>>,
}

impl DataComposerEnvironment_Config_RecoveryConfig {
    #[doc = "Set the field `scheduled_snapshots_config`.\n"]
    pub fn set_scheduled_snapshots_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig>,
                    >,
    ) -> Self {
        self.scheduled_snapshots_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_RecoveryConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_RecoveryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_RecoveryConfig {}

impl BuildDataComposerEnvironment_Config_RecoveryConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_RecoveryConfig {
        DataComposerEnvironment_Config_RecoveryConfig {
            scheduled_snapshots_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_RecoveryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_RecoveryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_RecoveryConfigRef {
        DataComposerEnvironment_Config_RecoveryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_RecoveryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheduled_snapshots_config` after provisioning.\n"]
    pub fn scheduled_snapshots_config(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_snapshots_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {}

impl BuildDataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    pub fn build(self) -> DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
        DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
        DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_SoftwareConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airflow_config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_data_lineage_integration: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pypi_packages: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_plugins_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_SoftwareConfig {
    #[doc = "Set the field `airflow_config_overrides`.\n"]
    pub fn set_airflow_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.airflow_config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_data_lineage_integration`.\n"]
    pub fn set_cloud_data_lineage_integration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration>,
                    >,
    ) -> Self {
        self.cloud_data_lineage_integration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env_variables`.\n"]
    pub fn set_env_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_version`.\n"]
    pub fn set_image_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pypi_packages`.\n"]
    pub fn set_pypi_packages(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pypi_packages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_version`.\n"]
    pub fn set_python_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduler_count`.\n"]
    pub fn set_scheduler_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scheduler_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_server_plugins_mode`.\n"]
    pub fn set_web_server_plugins_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.web_server_plugins_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_SoftwareConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_SoftwareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_SoftwareConfig {}

impl BuildDataComposerEnvironment_Config_SoftwareConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_SoftwareConfig {
        DataComposerEnvironment_Config_SoftwareConfig {
            airflow_config_overrides: core::default::Default::default(),
            cloud_data_lineage_integration: core::default::Default::default(),
            env_variables: core::default::Default::default(),
            image_version: core::default::Default::default(),
            pypi_packages: core::default::Default::default(),
            python_version: core::default::Default::default(),
            scheduler_count: core::default::Default::default(),
            web_server_plugins_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_SoftwareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_SoftwareConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_SoftwareConfigRef {
        DataComposerEnvironment_Config_SoftwareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_SoftwareConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `airflow_config_overrides` after provisioning.\n"]
    pub fn airflow_config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.airflow_config_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_data_lineage_integration` after provisioning.\n"]
    pub fn cloud_data_lineage_integration(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_data_lineage_integration", self.base))
    }

    #[doc = "Get a reference to the value of field `env_variables` after provisioning.\n"]
    pub fn env_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version` after provisioning.\n"]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.base))
    }

    #[doc = "Get a reference to the value of field `pypi_packages` after provisioning.\n"]
    pub fn pypi_packages(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pypi_packages", self.base))
    }

    #[doc = "Get a reference to the value of field `python_version` after provisioning.\n"]
    pub fn python_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_version", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduler_count` after provisioning.\n"]
    pub fn scheduler_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduler_count", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_plugins_mode` after provisioning.\n"]
    pub fn web_server_plugins_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_server_plugins_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WebServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_WebServerConfig {
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WebServerConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WebServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WebServerConfig {}

impl BuildDataComposerEnvironment_Config_WebServerConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_WebServerConfig {
        DataComposerEnvironment_Config_WebServerConfig { machine_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WebServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WebServerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WebServerConfigRef {
        DataComposerEnvironment_Config_WebServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WebServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {}

impl BuildDataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    pub fn build(self) -> DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
        DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
            description: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
        DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WebServerNetworkAccessControl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ip_range: std
    ::option
    ::Option<terrars::SetField<DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange>>,
}

impl DataComposerEnvironment_Config_WebServerNetworkAccessControl {
    #[doc = "Set the field `allowed_ip_range`.\n"]
    pub fn set_allowed_ip_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange>,
                    >,
    ) -> Self {
        self.allowed_ip_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WebServerNetworkAccessControl {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WebServerNetworkAccessControl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WebServerNetworkAccessControl {}

impl BuildDataComposerEnvironment_Config_WebServerNetworkAccessControl {
    pub fn build(self) -> DataComposerEnvironment_Config_WebServerNetworkAccessControl {
        DataComposerEnvironment_Config_WebServerNetworkAccessControl {
            allowed_ip_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WebServerNetworkAccessControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WebServerNetworkAccessControlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WebServerNetworkAccessControlRef {
        DataComposerEnvironment_Config_WebServerNetworkAccessControlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WebServerNetworkAccessControlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_ip_range` after provisioning.\n"]
    pub fn allowed_ip_range(
        &self,
    ) -> terrars::SetRef<DataComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_ip_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WorkloadsConfig_DagProcessor {}

impl BuildDataComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    pub fn build(self) -> DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
        DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
        DataComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WorkloadsConfig_Scheduler>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WorkloadsConfig_Scheduler {}

impl BuildDataComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    pub fn build(self) -> DataComposerEnvironment_Config_WorkloadsConfig_Scheduler {
        DataComposerEnvironment_Config_WorkloadsConfig_Scheduler {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
        DataComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WorkloadsConfig_Triggerer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WorkloadsConfig_Triggerer {}

impl BuildDataComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    pub fn build(self) -> DataComposerEnvironment_Config_WorkloadsConfig_Triggerer {
        DataComposerEnvironment_Config_WorkloadsConfig_Triggerer {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
        DataComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_WebServer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComposerEnvironment_Config_WorkloadsConfig_WebServer {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WorkloadsConfig_WebServer {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WorkloadsConfig_WebServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WorkloadsConfig_WebServer {}

impl BuildDataComposerEnvironment_Config_WorkloadsConfig_WebServer {
    pub fn build(self) -> DataComposerEnvironment_Config_WorkloadsConfig_WebServer {
        DataComposerEnvironment_Config_WorkloadsConfig_WebServer {
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
        DataComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_Worker {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComposerEnvironment_Config_WorkloadsConfig_Worker {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_count`.\n"]
    pub fn set_max_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_count`.\n"]
    pub fn set_min_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WorkloadsConfig_Worker {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WorkloadsConfig_Worker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WorkloadsConfig_Worker {}

impl BuildDataComposerEnvironment_Config_WorkloadsConfig_Worker {
    pub fn build(self) -> DataComposerEnvironment_Config_WorkloadsConfig_Worker {
        DataComposerEnvironment_Config_WorkloadsConfig_Worker {
            cpu: core::default::Default::default(),
            max_count: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            min_count: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
        DataComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `max_count` after provisioning.\n"]
    pub fn max_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `min_count` after provisioning.\n"]
    pub fn min_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_count", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config_WorkloadsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dag_processor: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_Scheduler>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggerer: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_Triggerer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_WebServer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_Worker>>,
}

impl DataComposerEnvironment_Config_WorkloadsConfig {
    #[doc = "Set the field `dag_processor`.\n"]
    pub fn set_dag_processor(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_DagProcessor>>,
    ) -> Self {
        self.dag_processor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduler`.\n"]
    pub fn set_scheduler(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_Scheduler>>,
    ) -> Self {
        self.scheduler = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `triggerer`.\n"]
    pub fn set_triggerer(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_Triggerer>>,
    ) -> Self {
        self.triggerer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_server`.\n"]
    pub fn set_web_server(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_WebServer>>,
    ) -> Self {
        self.web_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig_Worker>>,
    ) -> Self {
        self.worker = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config_WorkloadsConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config_WorkloadsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config_WorkloadsConfig {}

impl BuildDataComposerEnvironment_Config_WorkloadsConfig {
    pub fn build(self) -> DataComposerEnvironment_Config_WorkloadsConfig {
        DataComposerEnvironment_Config_WorkloadsConfig {
            dag_processor: core::default::Default::default(),
            scheduler: core::default::Default::default(),
            triggerer: core::default::Default::default(),
            web_server: core::default::Default::default(),
            worker: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_Config_WorkloadsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_Config_WorkloadsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComposerEnvironment_Config_WorkloadsConfigRef {
        DataComposerEnvironment_Config_WorkloadsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_Config_WorkloadsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dag_processor` after provisioning.\n"]
    pub fn dag_processor(&self) -> terrars::ListRef<DataComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_processor", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduler` after provisioning.\n"]
    pub fn scheduler(&self) -> terrars::ListRef<DataComposerEnvironment_Config_WorkloadsConfig_SchedulerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduler", self.base))
    }

    #[doc = "Get a reference to the value of field `triggerer` after provisioning.\n"]
    pub fn triggerer(&self) -> terrars::ListRef<DataComposerEnvironment_Config_WorkloadsConfig_TriggererRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggerer", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server` after provisioning.\n"]
    pub fn web_server(&self) -> terrars::ListRef<DataComposerEnvironment_Config_WorkloadsConfig_WebServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> terrars::ListRef<DataComposerEnvironment_Config_WorkloadsConfig_WorkerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airflow_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dag_gcs_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_retention_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_DataRetentionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_DatabaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_builds_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_environment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_authorized_networks_config: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_environment_config: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_PrivateEnvironmentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_RecoveryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resilience_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_SoftwareConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_WebServerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_network_access_control: std
    ::option
    ::Option<terrars::ListField<DataComposerEnvironment_Config_WebServerNetworkAccessControl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workloads_config: std::option::Option<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig>>,
}

impl DataComposerEnvironment_Config {
    #[doc = "Set the field `airflow_uri`.\n"]
    pub fn set_airflow_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.airflow_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dag_gcs_prefix`.\n"]
    pub fn set_dag_gcs_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dag_gcs_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_retention_config`.\n"]
    pub fn set_data_retention_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_DataRetentionConfig>>,
    ) -> Self {
        self.data_retention_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_config`.\n"]
    pub fn set_database_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_DatabaseConfig>>,
    ) -> Self {
        self.database_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_builds_only`.\n"]
    pub fn set_enable_private_builds_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_builds_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_environment`.\n"]
    pub fn set_enable_private_environment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_EncryptionConfig>>,
    ) -> Self {
        self.encryption_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_size`.\n"]
    pub fn set_environment_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_cluster`.\n"]
    pub fn set_gke_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gke_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_MaintenanceWindow>>,
    ) -> Self {
        self.maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_authorized_networks_config`.\n"]
    pub fn set_master_authorized_networks_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfig>>,
    ) -> Self {
        self.master_authorized_networks_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_NodeConfig>>,
    ) -> Self {
        self.node_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_environment_config`.\n"]
    pub fn set_private_environment_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_PrivateEnvironmentConfig>>,
    ) -> Self {
        self.private_environment_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_config`.\n"]
    pub fn set_recovery_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_RecoveryConfig>>,
    ) -> Self {
        self.recovery_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resilience_mode`.\n"]
    pub fn set_resilience_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resilience_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_SoftwareConfig>>,
    ) -> Self {
        self.software_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_server_config`.\n"]
    pub fn set_web_server_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WebServerConfig>>,
    ) -> Self {
        self.web_server_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_server_network_access_control`.\n"]
    pub fn set_web_server_network_access_control(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WebServerNetworkAccessControl>>,
    ) -> Self {
        self.web_server_network_access_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workloads_config`.\n"]
    pub fn set_workloads_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComposerEnvironment_Config_WorkloadsConfig>>,
    ) -> Self {
        self.workloads_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_Config {
    type O = terrars::BlockAssignable<DataComposerEnvironment_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_Config {}

impl BuildDataComposerEnvironment_Config {
    pub fn build(self) -> DataComposerEnvironment_Config {
        DataComposerEnvironment_Config {
            airflow_uri: core::default::Default::default(),
            dag_gcs_prefix: core::default::Default::default(),
            data_retention_config: core::default::Default::default(),
            database_config: core::default::Default::default(),
            enable_private_builds_only: core::default::Default::default(),
            enable_private_environment: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            environment_size: core::default::Default::default(),
            gke_cluster: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            master_authorized_networks_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            node_count: core::default::Default::default(),
            private_environment_config: core::default::Default::default(),
            recovery_config: core::default::Default::default(),
            resilience_mode: core::default::Default::default(),
            software_config: core::default::Default::default(),
            web_server_config: core::default::Default::default(),
            web_server_network_access_control: core::default::Default::default(),
            workloads_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComposerEnvironment_ConfigRef {
        DataComposerEnvironment_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `airflow_uri` after provisioning.\n"]
    pub fn airflow_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.airflow_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `dag_gcs_prefix` after provisioning.\n"]
    pub fn dag_gcs_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dag_gcs_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `data_retention_config` after provisioning.\n"]
    pub fn data_retention_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_DataRetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_retention_config", self.base))
    }

    #[doc = "Get a reference to the value of field `database_config` after provisioning.\n"]
    pub fn database_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_DatabaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_config", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_private_builds_only` after provisioning.\n"]
    pub fn enable_private_builds_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_builds_only", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_private_environment` after provisioning.\n"]
    pub fn enable_private_environment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_environment", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_size` after provisioning.\n"]
    pub fn environment_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_size", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_cluster` after provisioning.\n"]
    pub fn gke_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataComposerEnvironment_Config_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_authorized_networks_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `private_environment_config` after provisioning.\n"]
    pub fn private_environment_config(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_PrivateEnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_environment_config", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_config` after provisioning.\n"]
    pub fn recovery_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_RecoveryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recovery_config", self.base))
    }

    #[doc = "Get a reference to the value of field `resilience_mode` after provisioning.\n"]
    pub fn resilience_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resilience_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_SoftwareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_config` after provisioning.\n"]
    pub fn web_server_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_WebServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server_config", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_network_access_control` after provisioning.\n"]
    pub fn web_server_network_access_control(
        &self,
    ) -> terrars::ListRef<DataComposerEnvironment_Config_WebServerNetworkAccessControlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server_network_access_control", self.base))
    }

    #[doc = "Get a reference to the value of field `workloads_config` after provisioning.\n"]
    pub fn workloads_config(&self) -> terrars::ListRef<DataComposerEnvironment_Config_WorkloadsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workloads_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComposerEnvironment_StorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComposerEnvironment_StorageConfig {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComposerEnvironment_StorageConfig {
    type O = terrars::BlockAssignable<DataComposerEnvironment_StorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComposerEnvironment_StorageConfig {}

impl BuildDataComposerEnvironment_StorageConfig {
    pub fn build(self) -> DataComposerEnvironment_StorageConfig {
        DataComposerEnvironment_StorageConfig { bucket: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComposerEnvironment_StorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComposerEnvironment_StorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComposerEnvironment_StorageConfigRef {
        DataComposerEnvironment_StorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComposerEnvironment_StorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}
