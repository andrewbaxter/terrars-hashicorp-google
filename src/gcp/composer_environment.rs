#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComposerEnvironmentData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_config: std::option::Option<std::vec::Vec<ComposerEnvironment_StorageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComposerEnvironment_Timeouts>,
    dynamic: ComposerEnvironmentDynamic,
}

struct ComposerEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComposerEnvironmentData_>,
}

#[derive(Clone)]
pub struct ComposerEnvironment(std::rc::Rc<ComposerEnvironment_>);

impl ComposerEnvironment {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_StorageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComposerEnvironment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComposerEnvironment_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<ComposerEnvironment_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComposerEnvironment_TimeoutsRef {
        ComposerEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComposerEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComposerEnvironment { }

impl terrars::ToListMappable for ComposerEnvironment {
    type O = terrars::ListRef<ComposerEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComposerEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_composer_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComposerEnvironment {
    pub tf_id: std::string::String,
    #[doc = "Name of the environment."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> ComposerEnvironment {
        let out = ComposerEnvironment(std::rc::Rc::new(ComposerEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComposerEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                config: core::default::Default::default(),
                storage_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComposerEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComposerEnvironmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComposerEnvironment_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<ComposerEnvironment_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComposerEnvironment_TimeoutsRef {
        ComposerEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironmentDynamic {
    config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config>>,
    storage_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_StorageConfig>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    #[doc =
        "Set the field `retention_days`.\nHow many days data should be retained for. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn set_retention_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retention_mode`.\nWhether database retention is enabled or not. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn set_retention_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {}

impl BuildComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
    pub fn build(self) -> ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
        ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig {
            retention_days: core::default::Default::default(),
            retention_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
        ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `retention_days` after provisioning.\nHow many days data should be retained for. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_mode` after provisioning.\nWhether database retention is enabled or not. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn retention_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    #[doc =
        "Set the field `storage_mode`.\nWhether logs in cloud logging only is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4 and newer but not in composer-3*"]
    pub fn set_storage_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {}

impl BuildComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
    pub fn build(self) -> ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
        ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig {
            storage_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
        ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `storage_mode` after provisioning.\nWhether logs in cloud logging only is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4 and newer but not in composer-3*"]
    pub fn storage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_Config_DataRetentionConfigDynamic {
    airflow_metadata_retention_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig>>,
    task_logs_retention_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_DataRetentionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airflow_metadata_retention_config: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_logs_retention_config: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig>>,
    dynamic: ComposerEnvironment_Config_DataRetentionConfigDynamic,
}

impl ComposerEnvironment_Config_DataRetentionConfig {
    #[doc = "Set the field `airflow_metadata_retention_config`.\n"]
    pub fn set_airflow_metadata_retention_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.airflow_metadata_retention_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.airflow_metadata_retention_config = std::option::Option::Some(d);
            },
        }
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
                        terrars
                        ::BlockAssignable<ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.task_logs_retention_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.task_logs_retention_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_DataRetentionConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_DataRetentionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_DataRetentionConfig {}

impl BuildComposerEnvironment_Config_DataRetentionConfig {
    pub fn build(self) -> ComposerEnvironment_Config_DataRetentionConfig {
        ComposerEnvironment_Config_DataRetentionConfig {
            airflow_metadata_retention_config: core::default::Default::default(),
            task_logs_retention_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_DataRetentionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_DataRetentionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_DataRetentionConfigRef {
        ComposerEnvironment_Config_DataRetentionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_DataRetentionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `airflow_metadata_retention_config` after provisioning.\n"]
    pub fn airflow_metadata_retention_config(
        &self,
    ) -> terrars::ListRef<ComposerEnvironment_Config_DataRetentionConfig_AirflowMetadataRetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.airflow_metadata_retention_config", self.base))
    }

    #[doc = "Get a reference to the value of field `task_logs_retention_config` after provisioning.\n"]
    pub fn task_logs_retention_config(
        &self,
    ) -> terrars::ListRef<ComposerEnvironment_Config_DataRetentionConfig_TaskLogsRetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_logs_retention_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_DatabaseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironment_Config_DatabaseConfig {
    #[doc =
        "Set the field `machine_type`.\nOptional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nOptional. Cloud SQL database preferred zone."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_DatabaseConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_DatabaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_DatabaseConfig {}

impl BuildComposerEnvironment_Config_DatabaseConfig {
    pub fn build(self) -> ComposerEnvironment_Config_DatabaseConfig {
        ComposerEnvironment_Config_DatabaseConfig {
            machine_type: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_DatabaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_DatabaseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_Config_DatabaseConfigRef {
        ComposerEnvironment_Config_DatabaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_DatabaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nOptional. Cloud SQL database preferred zone."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_EncryptionConfig {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironment_Config_EncryptionConfig { }

impl terrars::ToListMappable for ComposerEnvironment_Config_EncryptionConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_EncryptionConfig {
    #[doc =
        "Optional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment_Config_EncryptionConfig {
    pub fn build(self) -> ComposerEnvironment_Config_EncryptionConfig {
        ComposerEnvironment_Config_EncryptionConfig { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_EncryptionConfigRef {
        ComposerEnvironment_Config_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_MaintenanceWindow {
    end_time: terrars::PrimField<std::string::String>,
    recurrence: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironment_Config_MaintenanceWindow { }

impl terrars::ToListMappable for ComposerEnvironment_Config_MaintenanceWindow {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_MaintenanceWindow {
    #[doc =
        "Maintenance window end time. It is used only to calculate the duration of the maintenance window. The value for end-time must be in the future, relative to 'start_time'."]
    pub end_time: terrars::PrimField<std::string::String>,
    #[doc =
        "Maintenance window recurrence. Format is a subset of RFC-5545 (https://tools.ietf.org/html/rfc5545) 'RRULE'. The only allowed values for 'FREQ' field are 'FREQ=DAILY' and 'FREQ=WEEKLY;BYDAY=...'. Example values: 'FREQ=WEEKLY;BYDAY=TU,WE', 'FREQ=DAILY'."]
    pub recurrence: terrars::PrimField<std::string::String>,
    #[doc = "Start time of the first recurrence of the maintenance window."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment_Config_MaintenanceWindow {
    pub fn build(self) -> ComposerEnvironment_Config_MaintenanceWindow {
        ComposerEnvironment_Config_MaintenanceWindow {
            end_time: self.end_time,
            recurrence: self.recurrence,
            start_time: self.start_time,
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_MaintenanceWindowRef {
        ComposerEnvironment_Config_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nMaintenance window end time. It is used only to calculate the duration of the maintenance window. The value for end-time must be in the future, relative to 'start_time'."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recurrence` after provisioning.\nMaintenance window recurrence. Format is a subset of RFC-5545 (https://tools.ietf.org/html/rfc5545) 'RRULE'. The only allowed values for 'FREQ' field are 'FREQ=DAILY' and 'FREQ=WEEKLY;BYDAY=...'. Example values: 'FREQ=WEEKLY;BYDAY=TU,WE', 'FREQ=DAILY'."]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nStart time of the first recurrence of the maintenance window."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    cidr_block: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[doc = "Set the field `display_name`.\ndisplay_name is a field for users to identify CIDR blocks."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[doc = "cidr_block must be specified in CIDR notation."]
    pub cidr_block: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
    pub fn build(self) -> ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
        ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks {
            cidr_block: self.cidr_block,
            display_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
        ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cidr_block` after provisioning.\ncidr_block must be specified in CIDR notation."]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\ndisplay_name is a field for users to identify CIDR blocks."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_Config_MasterAuthorizedNetworksConfigDynamic {
    cidr_blocks: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks>>,
    dynamic: ComposerEnvironment_Config_MasterAuthorizedNetworksConfigDynamic,
}

impl ComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig_CidrBlocks>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cidr_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cidr_blocks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    #[doc = "Whether or not master authorized networks is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
    pub fn build(self) -> ComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
        ComposerEnvironment_Config_MasterAuthorizedNetworksConfig {
            enabled: self.enabled,
            cidr_blocks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
        ComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not master authorized networks is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
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

impl ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
    #[doc =
        "Set the field `cluster_ipv4_cidr_block`.\nThe IP address range used to allocate IP addresses to pods in the cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both."]
    pub fn set_cluster_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_secondary_range_name`.\nThe name of the cluster's secondary range used to allocate IP addresses to pods. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn set_cluster_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `services_ipv4_cidr_block`.\nThe IP address range used to allocate IP addresses in this cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both."]
    pub fn set_services_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `services_secondary_range_name`.\nThe name of the services' secondary range used to allocate IP addresses to the cluster. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn set_services_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_ip_aliases`.\nWhether or not to enable Alias IPs in the GKE cluster. If true, a VPC-native cluster is created. Defaults to true if the ip_allocation_policy block is present in config. This field is only supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use VPC-native GKE clusters."]
    pub fn set_use_ip_aliases(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_ip_aliases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {}

impl BuildComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
    pub fn build(self) -> ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
        ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy {
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            use_ip_aliases: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
        ComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\nThe IP address range used to allocate IP addresses to pods in the cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both."]
    pub fn cluster_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\nThe name of the cluster's secondary range used to allocate IP addresses to pods. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn cluster_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_secondary_range_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\nThe IP address range used to allocate IP addresses in this cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both."]
    pub fn services_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services_secondary_range_name` after provisioning.\nThe name of the services' secondary range used to allocate IP addresses to the cluster. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn services_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_secondary_range_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_ip_aliases` after provisioning.\nWhether or not to enable Alias IPs in the GKE cluster. If true, a VPC-native cluster is created. Defaults to true if the ip_allocation_policy block is present in config. This field is only supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use VPC-native GKE clusters."]
    pub fn use_ip_aliases(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ip_aliases", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_Config_NodeConfigDynamic {
    ip_allocation_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composer_internal_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composer_network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ip_masq_agent: std::option::Option<terrars::PrimField<bool>>,
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_allocation_policy: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy>>,
    dynamic: ComposerEnvironment_Config_NodeConfigDynamic,
}

impl ComposerEnvironment_Config_NodeConfig {
    #[doc =
        "Set the field `composer_internal_ipv4_cidr_block`.\nIPv4 cidr range that will be used by Composer internal components."]
    pub fn set_composer_internal_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.composer_internal_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `composer_network_attachment`.\nPSC (Private Service Connect) Network entry point. Customers can pre-create the Network Attachment and point Cloud Composer environment to use. It is possible to share network attachment among many environments, provided enough IP addresses are available."]
    pub fn set_composer_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.composer_network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_size_gb`.\nThe disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_ip_masq_agent`.\nDeploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent"]
    pub fn set_enable_ip_masq_agent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ip_masq_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. The network must belong to the environment's project. If unspecified, the \"default\" network ID in the environment's project is used. If a Custom Subnet Network is provided, subnetwork must also be provided."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `oauth_scopes`.\nThe set of Google API scopes to be made available on all node VMs. Cannot be updated. If empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_oauth_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated. If given, note that the service account must have roles/composer.worker for any GCP resources created under the Cloud Composer Environment."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe Compute Engine subnetwork to be used for machine communications, specified as a self-link, relative resource name (e.g. \"projects/{project}/regions/{region}/subnetworks/{subnetwork}\"), or by name. If subnetwork is provided, network must also be provided and the subnetwork must belong to the enclosing environment's project and region."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nThe list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with RFC1035. Cannot be updated."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nThe Compute Engine zone in which to deploy the VMs running the Apache Airflow software, specified as the zone name or relative resource name (e.g. \"projects/{project}/zones/{zone}\"). Must belong to the enclosing environment's project and region. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_allocation_policy`.\n"]
    pub fn set_ip_allocation_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironment_Config_NodeConfig_IpAllocationPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_allocation_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_allocation_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_NodeConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_NodeConfig {}

impl BuildComposerEnvironment_Config_NodeConfig {
    pub fn build(self) -> ComposerEnvironment_Config_NodeConfig {
        ComposerEnvironment_Config_NodeConfig {
            composer_internal_ipv4_cidr_block: core::default::Default::default(),
            composer_network_attachment: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            enable_ip_masq_agent: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            network: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
            ip_allocation_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_Config_NodeConfigRef {
        ComposerEnvironment_Config_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_NodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `composer_internal_ipv4_cidr_block` after provisioning.\nIPv4 cidr range that will be used by Composer internal components."]
    pub fn composer_internal_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.composer_internal_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `composer_network_attachment` after provisioning.\nPSC (Private Service Connect) Network entry point. Customers can pre-create the Network Attachment and point Cloud Composer environment to use. It is possible to share network attachment among many environments, provided enough IP addresses are available."]
    pub fn composer_network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.composer_network_attachment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_ip_masq_agent` after provisioning.\nDeploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent"]
    pub fn enable_ip_masq_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ip_masq_agent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. The network must belong to the environment's project. If unspecified, the \"default\" network ID in the environment's project is used. If a Custom Subnet Network is provided, subnetwork must also be provided."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_scopes` after provisioning.\nThe set of Google API scopes to be made available on all node VMs. Cannot be updated. If empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn oauth_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated. If given, note that the service account must have roles/composer.worker for any GCP resources created under the Cloud Composer Environment."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe Compute Engine subnetwork to be used for machine communications, specified as a self-link, relative resource name (e.g. \"projects/{project}/regions/{region}/subnetworks/{subnetwork}\"), or by name. If subnetwork is provided, network must also be provided and the subnetwork must belong to the enclosing environment's project and region."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with RFC1035. Cannot be updated."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe Compute Engine zone in which to deploy the VMs running the Apache Airflow software, specified as the zone name or relative resource name (e.g. \"projects/{project}/zones/{zone}\"). Must belong to the enclosing environment's project and region. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(&self) -> terrars::ListRef<ComposerEnvironment_Config_NodeConfig_IpAllocationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_PrivateEnvironmentConfig {
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

impl ComposerEnvironment_Config_PrivateEnvironmentConfig {
    #[doc =
        "Set the field `cloud_composer_connection_subnetwork`.\nWhen specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_cloud_composer_connection_subnetwork(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_composer_connection_subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cloud_composer_network_ipv4_cidr_block`.\nThe CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_cloud_composer_network_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_composer_network_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cloud_sql_ipv4_cidr_block`.\nThe CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block."]
    pub fn set_cloud_sql_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_sql_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_type`.\nMode of internal communication within the Composer environment. Must be one of \"VPC_PEERING\" or \"PRIVATE_SERVICE_CONNECT\"."]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_private_endpoint`.\nIf true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_enable_private_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_privately_used_public_ips`.\nWhen enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block."]
    pub fn set_enable_privately_used_public_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_privately_used_public_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `master_ipv4_cidr_block`.\nThe IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used."]
    pub fn set_master_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `web_server_ipv4_cidr_block`.\nThe CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_web_server_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.web_server_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_PrivateEnvironmentConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_PrivateEnvironmentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_PrivateEnvironmentConfig {}

impl BuildComposerEnvironment_Config_PrivateEnvironmentConfig {
    pub fn build(self) -> ComposerEnvironment_Config_PrivateEnvironmentConfig {
        ComposerEnvironment_Config_PrivateEnvironmentConfig {
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
pub struct ComposerEnvironment_Config_PrivateEnvironmentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_PrivateEnvironmentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_PrivateEnvironmentConfigRef {
        ComposerEnvironment_Config_PrivateEnvironmentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_PrivateEnvironmentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_composer_connection_subnetwork` after provisioning.\nWhen specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn cloud_composer_connection_subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_composer_connection_subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloud_composer_network_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn cloud_composer_network_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_composer_network_ipv4_cidr_block", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `cloud_sql_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block."]
    pub fn cloud_sql_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_type` after provisioning.\nMode of internal communication within the Composer environment. Must be one of \"VPC_PEERING\" or \"PRIVATE_SERVICE_CONNECT\"."]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_endpoint` after provisioning.\nIf true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn enable_private_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_privately_used_public_ips` after provisioning.\nWhen enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block."]
    pub fn enable_privately_used_public_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_privately_used_public_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\nThe IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used."]
    pub fn master_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `web_server_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn web_server_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_server_ipv4_cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_creation_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    #[doc = "Set the field `snapshot_creation_schedule`.\nSnapshot schedule, in the unix-cron format."]
    pub fn set_snapshot_creation_schedule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.snapshot_creation_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_location`.\nthe URI of a bucket folder where to save the snapshot."]
    pub fn set_snapshot_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nA time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    #[doc = "When enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
    pub fn build(self) -> ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
        ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig {
            enabled: self.enabled,
            snapshot_creation_schedule: core::default::Default::default(),
            snapshot_location: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
        ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_creation_schedule` after provisioning.\nSnapshot schedule, in the unix-cron format."]
    pub fn snapshot_creation_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_creation_schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_location` after provisioning.\nthe URI of a bucket folder where to save the snapshot."]
    pub fn snapshot_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nA time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_Config_RecoveryConfigDynamic {
    scheduled_snapshots_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_RecoveryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_snapshots_config: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig>>,
    dynamic: ComposerEnvironment_Config_RecoveryConfigDynamic,
}

impl ComposerEnvironment_Config_RecoveryConfig {
    #[doc = "Set the field `scheduled_snapshots_config`.\n"]
    pub fn set_scheduled_snapshots_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduled_snapshots_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduled_snapshots_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_RecoveryConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_RecoveryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_RecoveryConfig {}

impl BuildComposerEnvironment_Config_RecoveryConfig {
    pub fn build(self) -> ComposerEnvironment_Config_RecoveryConfig {
        ComposerEnvironment_Config_RecoveryConfig {
            scheduled_snapshots_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_RecoveryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_RecoveryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_Config_RecoveryConfigRef {
        ComposerEnvironment_Config_RecoveryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_RecoveryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheduled_snapshots_config` after provisioning.\n"]
    pub fn scheduled_snapshots_config(
        &self,
    ) -> terrars::ListRef<ComposerEnvironment_Config_RecoveryConfig_ScheduledSnapshotsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_snapshots_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    enabled: terrars::PrimField<bool>,
}

impl ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration { }

impl terrars::ToListMappable for ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    #[doc = "Whether or not Cloud Data Lineage integration is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
    pub fn build(self) -> ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration {
        ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
        ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not Cloud Data Lineage integration is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_Config_SoftwareConfigDynamic {
    cloud_data_lineage_integration: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_SoftwareConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airflow_config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_data_lineage_integration: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration>>,
    dynamic: ComposerEnvironment_Config_SoftwareConfigDynamic,
}

impl ComposerEnvironment_Config_SoftwareConfig {
    #[doc =
        "Set the field `airflow_config_overrides`.\nApache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and cannot contain \"=\" or \";\". Section and property names cannot contain characters: \".\" Apache Airflow configuration property names must be written in snake_case. Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are blacklisted, and cannot be overridden."]
    pub fn set_airflow_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.airflow_config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `env_variables`.\nAdditional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression [a-zA-Z_][a-zA-Z0-9_]*. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+), and they cannot match any of the following reserved names: AIRFLOW_HOME C_FORCE_ROOT CONTAINER_NAME DAGS_FOLDER GCP_PROJECT GCS_BUCKET GKE_CLUSTER_NAME SQL_DATABASE SQL_INSTANCE SQL_PASSWORD SQL_PROJECT SQL_REGION SQL_USER."]
    pub fn set_env_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_version`.\nThe version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression composer-([0-9]+(\\.[0-9]+\\.[0-9]+(-preview\\.[0-9]+)?)?|latest)-airflow-([0-9]+(\\.[0-9]+(\\.[0-9]+)?)?). The Cloud Composer portion of the image version is a full semantic version, or an alias in the form of major version number or 'latest'. The Apache Airflow portion of the image version is a full semantic version that points to one of the supported Apache Airflow versions, or an alias in the form of only major or major.minor versions specified. See documentation for more details and version list."]
    pub fn set_image_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pypi_packages`.\nCustom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name (e.g. \"numpy\"). Values are the lowercase extras and version specifier (e.g. \"==1.12.0\", \"[devel,gcp_api]\", \"[devel]>=1.8.2, <1.9.2\"). To specify a package without pinning it to a version specifier, use the empty string as the value."]
    pub fn set_pypi_packages(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pypi_packages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_version`.\nThe major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use Python major version 3."]
    pub fn set_python_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scheduler_count`.\nThe number of schedulers for Airflow. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-2.*.*."]
    pub fn set_scheduler_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scheduler_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `web_server_plugins_mode`.\nShould be either 'ENABLED' or 'DISABLED'. Defaults to 'ENABLED'. Used in Composer 3."]
    pub fn set_web_server_plugins_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.web_server_plugins_mode = std::option::Option::Some(v.into());
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
                        terrars
                        ::BlockAssignable<ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_data_lineage_integration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_data_lineage_integration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_SoftwareConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_SoftwareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_SoftwareConfig {}

impl BuildComposerEnvironment_Config_SoftwareConfig {
    pub fn build(self) -> ComposerEnvironment_Config_SoftwareConfig {
        ComposerEnvironment_Config_SoftwareConfig {
            airflow_config_overrides: core::default::Default::default(),
            env_variables: core::default::Default::default(),
            image_version: core::default::Default::default(),
            pypi_packages: core::default::Default::default(),
            python_version: core::default::Default::default(),
            scheduler_count: core::default::Default::default(),
            web_server_plugins_mode: core::default::Default::default(),
            cloud_data_lineage_integration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_SoftwareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_SoftwareConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_Config_SoftwareConfigRef {
        ComposerEnvironment_Config_SoftwareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_SoftwareConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `airflow_config_overrides` after provisioning.\nApache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and cannot contain \"=\" or \";\". Section and property names cannot contain characters: \".\" Apache Airflow configuration property names must be written in snake_case. Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are blacklisted, and cannot be overridden."]
    pub fn airflow_config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.airflow_config_overrides", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env_variables` after provisioning.\nAdditional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression [a-zA-Z_][a-zA-Z0-9_]*. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+), and they cannot match any of the following reserved names: AIRFLOW_HOME C_FORCE_ROOT CONTAINER_NAME DAGS_FOLDER GCP_PROJECT GCS_BUCKET GKE_CLUSTER_NAME SQL_DATABASE SQL_INSTANCE SQL_PASSWORD SQL_PROJECT SQL_REGION SQL_USER."]
    pub fn env_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_variables", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_version` after provisioning.\nThe version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression composer-([0-9]+(\\.[0-9]+\\.[0-9]+(-preview\\.[0-9]+)?)?|latest)-airflow-([0-9]+(\\.[0-9]+(\\.[0-9]+)?)?). The Cloud Composer portion of the image version is a full semantic version, or an alias in the form of major version number or 'latest'. The Apache Airflow portion of the image version is a full semantic version that points to one of the supported Apache Airflow versions, or an alias in the form of only major or major.minor versions specified. See documentation for more details and version list."]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pypi_packages` after provisioning.\nCustom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name (e.g. \"numpy\"). Values are the lowercase extras and version specifier (e.g. \"==1.12.0\", \"[devel,gcp_api]\", \"[devel]>=1.8.2, <1.9.2\"). To specify a package without pinning it to a version specifier, use the empty string as the value."]
    pub fn pypi_packages(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pypi_packages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_version` after provisioning.\nThe major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use Python major version 3."]
    pub fn python_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheduler_count` after provisioning.\nThe number of schedulers for Airflow. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-2.*.*."]
    pub fn scheduler_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduler_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `web_server_plugins_mode` after provisioning.\nShould be either 'ENABLED' or 'DISABLED'. Defaults to 'ENABLED'. Used in Composer 3."]
    pub fn web_server_plugins_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_server_plugins_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_data_lineage_integration` after provisioning.\n"]
    pub fn cloud_data_lineage_integration(
        &self,
    ) -> terrars::ListRef<ComposerEnvironment_Config_SoftwareConfig_CloudDataLineageIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_data_lineage_integration", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WebServerConfig {
    machine_type: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironment_Config_WebServerConfig { }

impl terrars::ToListMappable for ComposerEnvironment_Config_WebServerConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WebServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WebServerConfig {
    #[doc =
        "Optional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
    pub machine_type: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment_Config_WebServerConfig {
    pub fn build(self) -> ComposerEnvironment_Config_WebServerConfig {
        ComposerEnvironment_Config_WebServerConfig { machine_type: self.machine_type }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WebServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WebServerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_Config_WebServerConfigRef {
        ComposerEnvironment_Config_WebServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WebServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    #[doc = "Set the field `description`.\nA description of this ip range."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    #[doc =
        "IP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. IP range prefixes should be properly truncated. For example, 1.2.3.4/24 should be truncated to 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 should be truncated to 2001:db8::/32."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
    pub fn build(self) -> ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
        ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange {
            description: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
        ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this ip range."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nIP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. IP range prefixes should be properly truncated. For example, 1.2.3.4/24 should be truncated to 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 should be truncated to 2001:db8::/32."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_Config_WebServerNetworkAccessControlDynamic {
    allowed_ip_range: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WebServerNetworkAccessControl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ip_range: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange>>,
    dynamic: ComposerEnvironment_Config_WebServerNetworkAccessControlDynamic,
}

impl ComposerEnvironment_Config_WebServerNetworkAccessControl {
    #[doc = "Set the field `allowed_ip_range`.\n"]
    pub fn set_allowed_ip_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComposerEnvironment_Config_WebServerNetworkAccessControl_AllowedIpRange>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_ip_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_ip_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_WebServerNetworkAccessControl {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WebServerNetworkAccessControl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WebServerNetworkAccessControl {}

impl BuildComposerEnvironment_Config_WebServerNetworkAccessControl {
    pub fn build(self) -> ComposerEnvironment_Config_WebServerNetworkAccessControl {
        ComposerEnvironment_Config_WebServerNetworkAccessControl {
            allowed_ip_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WebServerNetworkAccessControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WebServerNetworkAccessControlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_WebServerNetworkAccessControlRef {
        ComposerEnvironment_Config_WebServerNetworkAccessControlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WebServerNetworkAccessControlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    #[doc = "Set the field `count`.\nNumber of DAG processors."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\nCPU request and limit for DAG processor."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for DAG processor."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for DAG processor."]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_DagProcessor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WorkloadsConfig_DagProcessor {}

impl BuildComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
    pub fn build(self) -> ComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
        ComposerEnvironment_Config_WorkloadsConfig_DagProcessor {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
        ComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of DAG processors."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for DAG processor."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for DAG processor."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for DAG processor."]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    #[doc = "Set the field `count`.\nThe number of schedulers."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\nCPU request and limit for a single Airflow scheduler replica"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for a single Airflow scheduler replica."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for a single Airflow scheduler replica."]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_Scheduler>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WorkloadsConfig_Scheduler {}

impl BuildComposerEnvironment_Config_WorkloadsConfig_Scheduler {
    pub fn build(self) -> ComposerEnvironment_Config_WorkloadsConfig_Scheduler {
        ComposerEnvironment_Config_WorkloadsConfig_Scheduler {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
        ComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WorkloadsConfig_SchedulerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of schedulers."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow scheduler replica"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow scheduler replica."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for a single Airflow scheduler replica."]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    count: terrars::PrimField<f64>,
    cpu: terrars::PrimField<f64>,
    memory_gb: terrars::PrimField<f64>,
}

impl ComposerEnvironment_Config_WorkloadsConfig_Triggerer { }

impl terrars::ToListMappable for ComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_Triggerer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    #[doc = "The number of triggerers."]
    pub count: terrars::PrimField<f64>,
    #[doc = "CPU request and limit for a single Airflow triggerer replica."]
    pub cpu: terrars::PrimField<f64>,
    #[doc = "Memory (GB) request and limit for a single Airflow triggerer replica."]
    pub memory_gb: terrars::PrimField<f64>,
}

impl BuildComposerEnvironment_Config_WorkloadsConfig_Triggerer {
    pub fn build(self) -> ComposerEnvironment_Config_WorkloadsConfig_Triggerer {
        ComposerEnvironment_Config_WorkloadsConfig_Triggerer {
            count: self.count,
            cpu: self.cpu,
            memory_gb: self.memory_gb,
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
        ComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WorkloadsConfig_TriggererRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of triggerers."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow triggerer replica."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow triggerer replica."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_WebServer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ComposerEnvironment_Config_WorkloadsConfig_WebServer {
    #[doc = "Set the field `cpu`.\nCPU request and limit for Airflow web server."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for Airflow web server."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for Airflow web server."]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_WorkloadsConfig_WebServer {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_WebServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WorkloadsConfig_WebServer {}

impl BuildComposerEnvironment_Config_WorkloadsConfig_WebServer {
    pub fn build(self) -> ComposerEnvironment_Config_WorkloadsConfig_WebServer {
        ComposerEnvironment_Config_WorkloadsConfig_WebServer {
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
        ComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WorkloadsConfig_WebServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for Airflow web server."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for Airflow web server."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for Airflow web server."]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_Worker {
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

impl ComposerEnvironment_Config_WorkloadsConfig_Worker {
    #[doc = "Set the field `cpu`.\nCPU request and limit for a single Airflow worker replica."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_count`.\nMaximum number of workers for autoscaling."]
    pub fn set_max_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for a single Airflow worker replica."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_count`.\nMinimum number of workers for autoscaling."]
    pub fn set_min_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for a single Airflow worker replica."]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_WorkloadsConfig_Worker {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_Worker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WorkloadsConfig_Worker {}

impl BuildComposerEnvironment_Config_WorkloadsConfig_Worker {
    pub fn build(self) -> ComposerEnvironment_Config_WorkloadsConfig_Worker {
        ComposerEnvironment_Config_WorkloadsConfig_Worker {
            cpu: core::default::Default::default(),
            max_count: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            min_count: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
        ComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WorkloadsConfig_WorkerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow worker replica."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_count` after provisioning.\nMaximum number of workers for autoscaling."]
    pub fn max_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow worker replica."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_count` after provisioning.\nMinimum number of workers for autoscaling."]
    pub fn min_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for a single Airflow worker replica."]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_Config_WorkloadsConfigDynamic {
    dag_processor: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WorkloadsConfig_DagProcessor>>,
    scheduler: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WorkloadsConfig_Scheduler>>,
    triggerer: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WorkloadsConfig_Triggerer>>,
    web_server: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WorkloadsConfig_WebServer>>,
    worker: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WorkloadsConfig_Worker>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config_WorkloadsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dag_processor: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_WorkloadsConfig_DagProcessor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_WorkloadsConfig_Scheduler>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggerer: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_WorkloadsConfig_Triggerer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_WorkloadsConfig_WebServer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_WorkloadsConfig_Worker>>,
    dynamic: ComposerEnvironment_Config_WorkloadsConfigDynamic,
}

impl ComposerEnvironment_Config_WorkloadsConfig {
    #[doc = "Set the field `dag_processor`.\n"]
    pub fn set_dag_processor(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_DagProcessor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dag_processor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dag_processor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduler`.\n"]
    pub fn set_scheduler(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_Scheduler>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduler = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduler = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `triggerer`.\n"]
    pub fn set_triggerer(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_Triggerer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.triggerer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.triggerer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_server`.\n"]
    pub fn set_web_server(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_WebServer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig_Worker>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config_WorkloadsConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config_WorkloadsConfig {}

impl BuildComposerEnvironment_Config_WorkloadsConfig {
    pub fn build(self) -> ComposerEnvironment_Config_WorkloadsConfig {
        ComposerEnvironment_Config_WorkloadsConfig {
            dag_processor: core::default::Default::default(),
            scheduler: core::default::Default::default(),
            triggerer: core::default::Default::default(),
            web_server: core::default::Default::default(),
            worker: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_Config_WorkloadsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_Config_WorkloadsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_Config_WorkloadsConfigRef {
        ComposerEnvironment_Config_WorkloadsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_Config_WorkloadsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dag_processor` after provisioning.\n"]
    pub fn dag_processor(&self) -> terrars::ListRef<ComposerEnvironment_Config_WorkloadsConfig_DagProcessorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_processor", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduler` after provisioning.\n"]
    pub fn scheduler(&self) -> terrars::ListRef<ComposerEnvironment_Config_WorkloadsConfig_SchedulerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduler", self.base))
    }

    #[doc = "Get a reference to the value of field `triggerer` after provisioning.\n"]
    pub fn triggerer(&self) -> terrars::ListRef<ComposerEnvironment_Config_WorkloadsConfig_TriggererRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggerer", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server` after provisioning.\n"]
    pub fn web_server(&self) -> terrars::ListRef<ComposerEnvironment_Config_WorkloadsConfig_WebServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> terrars::ListRef<ComposerEnvironment_Config_WorkloadsConfig_WorkerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComposerEnvironment_ConfigDynamic {
    data_retention_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_DataRetentionConfig>>,
    database_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_DatabaseConfig>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_EncryptionConfig>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_MaintenanceWindow>>,
    master_authorized_networks_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig>>,
    node_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_NodeConfig>>,
    private_environment_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_PrivateEnvironmentConfig>>,
    recovery_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_RecoveryConfig>>,
    software_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_SoftwareConfig>>,
    web_server_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WebServerConfig>>,
    web_server_network_access_control: std
    ::option
    ::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WebServerNetworkAccessControl>>,
    workloads_config: std::option::Option<terrars::DynamicBlock<ComposerEnvironment_Config_WorkloadsConfig>>,
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_builds_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_environment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resilience_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_retention_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_DataRetentionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_DatabaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_authorized_networks_config: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_environment_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_PrivateEnvironmentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_RecoveryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_SoftwareConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_WebServerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_server_network_access_control: std
    ::option
    ::Option<std::vec::Vec<ComposerEnvironment_Config_WebServerNetworkAccessControl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workloads_config: std::option::Option<std::vec::Vec<ComposerEnvironment_Config_WorkloadsConfig>>,
    dynamic: ComposerEnvironment_ConfigDynamic,
}

impl ComposerEnvironment_Config {
    #[doc =
        "Set the field `enable_private_builds_only`.\nOptional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet."]
    pub fn set_enable_private_builds_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_builds_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_private_environment`.\nOptional. If true, a private Composer environment will be created."]
    pub fn set_enable_private_environment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_environment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `environment_size`.\nThe size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_environment_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_count`.\nThe number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resilience_mode`.\nWhether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer."]
    pub fn set_resilience_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resilience_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_retention_config`.\n"]
    pub fn set_data_retention_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_DataRetentionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_retention_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_retention_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database_config`.\n"]
    pub fn set_database_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_DatabaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_MaintenanceWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_authorized_networks_config`.\n"]
    pub fn set_master_authorized_networks_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironment_Config_MasterAuthorizedNetworksConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.master_authorized_networks_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.master_authorized_networks_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_NodeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_environment_config`.\n"]
    pub fn set_private_environment_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_PrivateEnvironmentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_environment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_environment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recovery_config`.\n"]
    pub fn set_recovery_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_RecoveryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recovery_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recovery_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_SoftwareConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.software_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.software_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_server_config`.\n"]
    pub fn set_web_server_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WebServerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_server_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_server_network_access_control`.\n"]
    pub fn set_web_server_network_access_control(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WebServerNetworkAccessControl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_server_network_access_control = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server_network_access_control = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workloads_config`.\n"]
    pub fn set_workloads_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComposerEnvironment_Config_WorkloadsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workloads_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workloads_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComposerEnvironment_Config {
    type O = terrars::BlockAssignable<ComposerEnvironment_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Config {}

impl BuildComposerEnvironment_Config {
    pub fn build(self) -> ComposerEnvironment_Config {
        ComposerEnvironment_Config {
            enable_private_builds_only: core::default::Default::default(),
            enable_private_environment: core::default::Default::default(),
            environment_size: core::default::Default::default(),
            node_count: core::default::Default::default(),
            resilience_mode: core::default::Default::default(),
            data_retention_config: core::default::Default::default(),
            database_config: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            master_authorized_networks_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            private_environment_config: core::default::Default::default(),
            recovery_config: core::default::Default::default(),
            software_config: core::default::Default::default(),
            web_server_config: core::default::Default::default(),
            web_server_network_access_control: core::default::Default::default(),
            workloads_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_ConfigRef {
        ComposerEnvironment_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `airflow_uri` after provisioning.\nThe URI of the Apache Airflow Web UI hosted within this environment."]
    pub fn airflow_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.airflow_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dag_gcs_prefix` after provisioning.\nThe Cloud Storage prefix of the DAGs for this environment. Although Cloud Storage objects reside in a flat namespace, a hierarchical file tree can be simulated using '/'-delimited object name prefixes. DAG objects for this environment reside in a simulated directory with this prefix."]
    pub fn dag_gcs_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dag_gcs_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_builds_only` after provisioning.\nOptional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet."]
    pub fn enable_private_builds_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_builds_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_environment` after provisioning.\nOptional. If true, a private Composer environment will be created."]
    pub fn enable_private_environment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_size` after provisioning.\nThe size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn environment_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gke_cluster` after provisioning.\nThe Kubernetes Engine cluster used to run this environment."]
    pub fn gke_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resilience_mode` after provisioning.\nWhether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer."]
    pub fn resilience_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resilience_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `data_retention_config` after provisioning.\n"]
    pub fn data_retention_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_DataRetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_retention_config", self.base))
    }

    #[doc = "Get a reference to the value of field `database_config` after provisioning.\n"]
    pub fn database_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_DatabaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_config", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<ComposerEnvironment_Config_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<ComposerEnvironment_Config_MasterAuthorizedNetworksConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_authorized_networks_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `private_environment_config` after provisioning.\n"]
    pub fn private_environment_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_PrivateEnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_environment_config", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_config` after provisioning.\n"]
    pub fn recovery_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_RecoveryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recovery_config", self.base))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_SoftwareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_config` after provisioning.\n"]
    pub fn web_server_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_WebServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server_config", self.base))
    }

    #[doc = "Get a reference to the value of field `web_server_network_access_control` after provisioning.\n"]
    pub fn web_server_network_access_control(
        &self,
    ) -> terrars::ListRef<ComposerEnvironment_Config_WebServerNetworkAccessControlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_server_network_access_control", self.base))
    }

    #[doc = "Get a reference to the value of field `workloads_config` after provisioning.\n"]
    pub fn workloads_config(&self) -> terrars::ListRef<ComposerEnvironment_Config_WorkloadsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workloads_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_StorageConfig {
    bucket: terrars::PrimField<std::string::String>,
}

impl ComposerEnvironment_StorageConfig { }

impl terrars::ToListMappable for ComposerEnvironment_StorageConfig {
    type O = terrars::BlockAssignable<ComposerEnvironment_StorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_StorageConfig {
    #[doc = "Optional. Name of an existing Cloud Storage bucket to be used by the environment."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildComposerEnvironment_StorageConfig {
    pub fn build(self) -> ComposerEnvironment_StorageConfig {
        ComposerEnvironment_StorageConfig { bucket: self.bucket }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_StorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_StorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_StorageConfigRef {
        ComposerEnvironment_StorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_StorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nOptional. Name of an existing Cloud Storage bucket to be used by the environment."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct ComposerEnvironment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComposerEnvironment_Timeouts {
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

impl terrars::ToListMappable for ComposerEnvironment_Timeouts {
    type O = terrars::BlockAssignable<ComposerEnvironment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComposerEnvironment_Timeouts {}

impl BuildComposerEnvironment_Timeouts {
    pub fn build(self) -> ComposerEnvironment_Timeouts {
        ComposerEnvironment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComposerEnvironment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposerEnvironment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComposerEnvironment_TimeoutsRef {
        ComposerEnvironment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComposerEnvironment_TimeoutsRef {
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
