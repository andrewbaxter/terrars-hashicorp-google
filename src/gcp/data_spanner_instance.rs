#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSpannerInstanceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSpannerInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSpannerInstanceData_>,
}

#[derive(Clone)]
pub struct DataSpannerInstance(std::rc::Rc<DataSpannerInstance_>);

impl DataSpannerInstance {
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

    #[doc =
        "Set the field `config`.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn set_config(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `autoscaling_config` after provisioning.\nThe autoscaling configuration. Autoscaling is enabled if this field is set.\nExactly one of either num_nodes, processing_units or autoscaling_config must be\npresent in terraform except when instance_type = FREE_INSTANCE.\nWhen autoscaling is enabled, num_nodes and processing_units are treated as,\nOUTPUT_ONLY fields and reflect the current compute capacity allocated to\nthe instance."]
    pub fn autoscaling_config(&self) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_backup_schedule_type` after provisioning.\nControls the default backup behavior for new databases within the instance.\nNote that 'AUTOMATIC' is not permitted for free instances, as backups and backup schedules are not allowed for free instances.\nif unset or NONE, no default backup schedule will be created for new databases within the instance. Possible values: [\"NONE\", \"AUTOMATIC\"]"]
    pub fn default_backup_schedule_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_backup_schedule_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edition` after provisioning.\nThe edition selected for this instance. Different editions provide different capabilities at different price points. Possible values: [\"EDITION_UNSPECIFIED\", \"STANDARD\", \"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen deleting a spanner instance, this boolean option will delete all backups of this instance.\nThis must be set to true if you created a backup manually in the console."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of this instance. The type can be used to distinguish product variants, that can affect aspects like:\nusage restrictions, quotas and billing. Currently this is used to distinguish FREE_INSTANCE vs PROVISIONED instances.\nWhen configured as FREE_INSTANCE, the field 'edition' should not be configured. Possible values: [\"PROVISIONED\", \"FREE_INSTANCE\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_nodes` after provisioning.\nThe number of nodes allocated to this instance. Exactly one of either num_nodes, processing_units or\nautoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn num_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance. Exactly one of either num_nodes,\nprocessing_units or autoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nInstance status: 'CREATING' or 'READY'."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl terrars::Referable for DataSpannerInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSpannerInstance { }

impl terrars::ToListMappable for DataSpannerInstance {
    type O = terrars::ListRef<DataSpannerInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSpannerInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_spanner_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSpannerInstance {
    pub tf_id: std::string::String,
    #[doc =
        "A unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataSpannerInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSpannerInstance {
        let out = DataSpannerInstance(std::rc::Rc::new(DataSpannerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSpannerInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                config: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSpannerInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSpannerInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_config` after provisioning.\nThe autoscaling configuration. Autoscaling is enabled if this field is set.\nExactly one of either num_nodes, processing_units or autoscaling_config must be\npresent in terraform except when instance_type = FREE_INSTANCE.\nWhen autoscaling is enabled, num_nodes and processing_units are treated as,\nOUTPUT_ONLY fields and reflect the current compute capacity allocated to\nthe instance."]
    pub fn autoscaling_config(&self) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_backup_schedule_type` after provisioning.\nControls the default backup behavior for new databases within the instance.\nNote that 'AUTOMATIC' is not permitted for free instances, as backups and backup schedules are not allowed for free instances.\nif unset or NONE, no default backup schedule will be created for new databases within the instance. Possible values: [\"NONE\", \"AUTOMATIC\"]"]
    pub fn default_backup_schedule_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_backup_schedule_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edition` after provisioning.\nThe edition selected for this instance. Different editions provide different capabilities at different price points. Possible values: [\"EDITION_UNSPECIFIED\", \"STANDARD\", \"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen deleting a spanner instance, this boolean option will delete all backups of this instance.\nThis must be set to true if you created a backup manually in the console."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of this instance. The type can be used to distinguish product variants, that can affect aspects like:\nusage restrictions, quotas and billing. Currently this is used to distinguish FREE_INSTANCE vs PROVISIONED instances.\nWhen configured as FREE_INSTANCE, the field 'edition' should not be configured. Possible values: [\"PROVISIONED\", \"FREE_INSTANCE\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_nodes` after provisioning.\nThe number of nodes allocated to this instance. Exactly one of either num_nodes, processing_units or\nautoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn num_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance. Exactly one of either num_nodes,\nprocessing_units or autoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nInstance status: 'CREATING' or 'READY'."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSpannerInstance_AutoscalingConfig_AutoscalingLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_processing_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_processing_units: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSpannerInstance_AutoscalingConfig_AutoscalingLimits {
    #[doc = "Set the field `max_nodes`.\n"]
    pub fn set_max_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_processing_units`.\n"]
    pub fn set_max_processing_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_processing_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_nodes`.\n"]
    pub fn set_min_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_processing_units`.\n"]
    pub fn set_min_processing_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_processing_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpannerInstance_AutoscalingConfig_AutoscalingLimits {
    type O = terrars::BlockAssignable<DataSpannerInstance_AutoscalingConfig_AutoscalingLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpannerInstance_AutoscalingConfig_AutoscalingLimits {}

impl BuildDataSpannerInstance_AutoscalingConfig_AutoscalingLimits {
    pub fn build(self) -> DataSpannerInstance_AutoscalingConfig_AutoscalingLimits {
        DataSpannerInstance_AutoscalingConfig_AutoscalingLimits {
            max_nodes: core::default::Default::default(),
            max_processing_units: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            min_processing_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpannerInstance_AutoscalingConfig_AutoscalingLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerInstance_AutoscalingConfig_AutoscalingLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSpannerInstance_AutoscalingConfig_AutoscalingLimitsRef {
        DataSpannerInstance_AutoscalingConfig_AutoscalingLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpannerInstance_AutoscalingConfig_AutoscalingLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_nodes` after provisioning.\n"]
    pub fn max_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `max_processing_units` after provisioning.\n"]
    pub fn max_processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_processing_units", self.base))
    }

    #[doc = "Get a reference to the value of field `min_nodes` after provisioning.\n"]
    pub fn min_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_processing_units` after provisioning.\n"]
    pub fn min_processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_processing_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_limits: std::option::Option<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AutoscalingLimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_target_high_priority_cpu_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_target_total_cpu_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_high_priority_cpu_autoscaling: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_total_cpu_autoscaling: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides {
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AutoscalingLimits>>,
    ) -> Self {
        self.autoscaling_limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_target_high_priority_cpu_utilization_percent`.\n"]
    pub fn set_autoscaling_target_high_priority_cpu_utilization_percent(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.autoscaling_target_high_priority_cpu_utilization_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_target_total_cpu_utilization_percent`.\n"]
    pub fn set_autoscaling_target_total_cpu_utilization_percent(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.autoscaling_target_total_cpu_utilization_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_high_priority_cpu_autoscaling`.\n"]
    pub fn set_disable_high_priority_cpu_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_high_priority_cpu_autoscaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_total_cpu_autoscaling`.\n"]
    pub fn set_disable_total_cpu_autoscaling(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_total_cpu_autoscaling = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides {
    type O = terrars::BlockAssignable<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides {}

impl BuildDataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides {
    pub fn build(self) -> DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides {
        DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides {
            autoscaling_limits: core::default::Default::default(),
            autoscaling_target_high_priority_cpu_utilization_percent: core::default::Default::default(),
            autoscaling_target_total_cpu_utilization_percent: core::default::Default::default(),
            disable_high_priority_cpu_autoscaling: core::default::Default::default(),
            disable_total_cpu_autoscaling: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_OverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_OverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_OverridesRef {
        DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_OverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_OverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]
    pub fn autoscaling_limits(&self) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfig_AutoscalingLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_target_high_priority_cpu_utilization_percent` after provisioning.\n"]
    pub fn autoscaling_target_high_priority_cpu_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscaling_target_high_priority_cpu_utilization_percent", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_target_total_cpu_utilization_percent` after provisioning.\n"]
    pub fn autoscaling_target_total_cpu_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscaling_target_total_cpu_utilization_percent", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `disable_high_priority_cpu_autoscaling` after provisioning.\n"]
    pub fn disable_high_priority_cpu_autoscaling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_high_priority_cpu_autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_total_cpu_autoscaling` after provisioning.\n"]
    pub fn disable_total_cpu_autoscaling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_total_cpu_autoscaling", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection {
    type O =
        terrars
        ::BlockAssignable<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection {}

impl BuildDataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection {
    pub fn build(self) -> DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection {
        DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection {
            location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelectionRef {
        DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std
    ::option
    ::Option<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_selection: std
    ::option
    ::Option<
        terrars::ListField<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection>,
    >,
}

impl DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions {
    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_Overrides>,
                    >,
    ) -> Self {
        self.overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_selection`.\n"]
    pub fn set_replica_selection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelection,
                        >,
                    >,
    ) -> Self {
        self.replica_selection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions {
    type O = terrars::BlockAssignable<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions {}

impl BuildDataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions {
    pub fn build(self) -> DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions {
        DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions {
            overrides: core::default::Default::default(),
            replica_selection: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptionsRef {
        DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(
        &self,
    ) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_OverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_selection` after provisioning.\n"]
    pub fn replica_selection(
        &self,
    ) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions_ReplicaSelectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_selection", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSpannerInstance_AutoscalingConfig_AutoscalingTargets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high_priority_cpu_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_cpu_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSpannerInstance_AutoscalingConfig_AutoscalingTargets {
    #[doc = "Set the field `high_priority_cpu_utilization_percent`.\n"]
    pub fn set_high_priority_cpu_utilization_percent(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.high_priority_cpu_utilization_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_utilization_percent`.\n"]
    pub fn set_storage_utilization_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_utilization_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_cpu_utilization_percent`.\n"]
    pub fn set_total_cpu_utilization_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_cpu_utilization_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpannerInstance_AutoscalingConfig_AutoscalingTargets {
    type O = terrars::BlockAssignable<DataSpannerInstance_AutoscalingConfig_AutoscalingTargets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpannerInstance_AutoscalingConfig_AutoscalingTargets {}

impl BuildDataSpannerInstance_AutoscalingConfig_AutoscalingTargets {
    pub fn build(self) -> DataSpannerInstance_AutoscalingConfig_AutoscalingTargets {
        DataSpannerInstance_AutoscalingConfig_AutoscalingTargets {
            high_priority_cpu_utilization_percent: core::default::Default::default(),
            storage_utilization_percent: core::default::Default::default(),
            total_cpu_utilization_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpannerInstance_AutoscalingConfig_AutoscalingTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerInstance_AutoscalingConfig_AutoscalingTargetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSpannerInstance_AutoscalingConfig_AutoscalingTargetsRef {
        DataSpannerInstance_AutoscalingConfig_AutoscalingTargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpannerInstance_AutoscalingConfig_AutoscalingTargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `high_priority_cpu_utilization_percent` after provisioning.\n"]
    pub fn high_priority_cpu_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.high_priority_cpu_utilization_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_utilization_percent` after provisioning.\n"]
    pub fn storage_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_utilization_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `total_cpu_utilization_percent` after provisioning.\n"]
    pub fn total_cpu_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_cpu_utilization_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSpannerInstance_AutoscalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asymmetric_autoscaling_options: std
    ::option
    ::Option<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_limits: std::option::Option<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AutoscalingLimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_targets: std
    ::option
    ::Option<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AutoscalingTargets>>,
}

impl DataSpannerInstance_AutoscalingConfig {
    #[doc = "Set the field `asymmetric_autoscaling_options`.\n"]
    pub fn set_asymmetric_autoscaling_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptions>>,
    ) -> Self {
        self.asymmetric_autoscaling_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AutoscalingLimits>>,
    ) -> Self {
        self.autoscaling_limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_targets`.\n"]
    pub fn set_autoscaling_targets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSpannerInstance_AutoscalingConfig_AutoscalingTargets>>,
    ) -> Self {
        self.autoscaling_targets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpannerInstance_AutoscalingConfig {
    type O = terrars::BlockAssignable<DataSpannerInstance_AutoscalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpannerInstance_AutoscalingConfig {}

impl BuildDataSpannerInstance_AutoscalingConfig {
    pub fn build(self) -> DataSpannerInstance_AutoscalingConfig {
        DataSpannerInstance_AutoscalingConfig {
            asymmetric_autoscaling_options: core::default::Default::default(),
            autoscaling_limits: core::default::Default::default(),
            autoscaling_targets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpannerInstance_AutoscalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerInstance_AutoscalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpannerInstance_AutoscalingConfigRef {
        DataSpannerInstance_AutoscalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpannerInstance_AutoscalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asymmetric_autoscaling_options` after provisioning.\n"]
    pub fn asymmetric_autoscaling_options(
        &self,
    ) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfig_AsymmetricAutoscalingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asymmetric_autoscaling_options", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]
    pub fn autoscaling_limits(&self) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfig_AutoscalingLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_limits", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_targets` after provisioning.\n"]
    pub fn autoscaling_targets(&self) -> terrars::ListRef<DataSpannerInstance_AutoscalingConfig_AutoscalingTargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_targets", self.base))
    }
}
