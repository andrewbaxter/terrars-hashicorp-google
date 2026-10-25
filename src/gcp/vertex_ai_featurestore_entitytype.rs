#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiFeaturestoreEntitytypeData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    featurestore: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_config: std::option::Option<std::vec::Vec<VertexAiFeaturestoreEntitytype_MonitoringConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiFeaturestoreEntitytype_Timeouts>,
    dynamic: VertexAiFeaturestoreEntitytypeDynamic,
}

struct VertexAiFeaturestoreEntitytype_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiFeaturestoreEntitytypeData_>,
}

#[derive(Clone)]
pub struct VertexAiFeaturestoreEntitytype(std::rc::Rc<VertexAiFeaturestoreEntitytype_>);

impl VertexAiFeaturestoreEntitytype {
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

    #[doc = "Set the field `description`.\nOptional. Description of the EntityType."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs to assign to this EntityType.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of the EntityType. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_config`.\n"]
    pub fn set_monitoring_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiFeaturestoreEntitytype_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featurestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the EntityType."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `featurestore` after provisioning.\nThe name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}."]
    pub fn featurestore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.featurestore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this EntityType.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the EntityType. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the EntityType."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featurestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<VertexAiFeaturestoreEntitytype_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeaturestoreEntitytype_TimeoutsRef {
        VertexAiFeaturestoreEntitytype_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiFeaturestoreEntitytype {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiFeaturestoreEntitytype { }

impl terrars::ToListMappable for VertexAiFeaturestoreEntitytype {
    type O = terrars::ListRef<VertexAiFeaturestoreEntitytypeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiFeaturestoreEntitytype_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_featurestore_entitytype".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiFeaturestoreEntitytype {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}."]
    pub featurestore: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiFeaturestoreEntitytype {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiFeaturestoreEntitytype {
        let out = VertexAiFeaturestoreEntitytype(std::rc::Rc::new(VertexAiFeaturestoreEntitytype_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiFeaturestoreEntitytypeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                featurestore: self.featurestore,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: core::default::Default::default(),
                monitoring_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiFeaturestoreEntitytypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeaturestoreEntitytypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiFeaturestoreEntitytypeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featurestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the EntityType."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `featurestore` after provisioning.\nThe name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}."]
    pub fn featurestore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.featurestore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this EntityType.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the EntityType. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the EntityType."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featurestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<VertexAiFeaturestoreEntitytype_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeaturestoreEntitytype_TimeoutsRef {
        VertexAiFeaturestoreEntitytype_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiFeaturestoreEntitytypeDynamic {
    monitoring_config: std::option::Option<terrars::DynamicBlock<VertexAiFeaturestoreEntitytype_MonitoringConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig {
    value: terrars::PrimField<f64>,
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig { }

impl terrars::ToListMappable for VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig {
    type O = terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig {
    #[doc =
        "Specify a threshold value that can trigger the alert. For categorical feature, the distribution distance is calculated by L-inifinity norm. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig {
    pub fn build(self) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig { value: self.value }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfigRef {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nSpecify a threshold value that can trigger the alert. For categorical feature, the distribution distance is calculated by L-inifinity norm. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anomaly_detection_baseline: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis {
    #[doc =
        "Set the field `anomaly_detection_baseline`.\nDefines the baseline to do anomaly detection for feature values imported by each [entityTypes.importFeatureValues][] operation. The value must be one of the values below:\n* LATEST_STATS: Choose the later one statistics generated by either most recent snapshot analysis or previous import features analysis. If non of them exists, skip anomaly detection and only generate a statistics.\n* MOST_RECENT_SNAPSHOT_STATS: Use the statistics generated by the most recent snapshot analysis if exists.\n* PREVIOUS_IMPORT_FEATURES_STATS: Use the statistics generated by the previous import features analysis if exists."]
    pub fn set_anomaly_detection_baseline(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.anomaly_detection_baseline = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nWhether to enable / disable / inherite default hebavior for import features analysis. The value must be one of the values below:\n* DEFAULT: The default behavior of whether to enable the monitoring. EntityType-level config: disabled.\n* ENABLED: Explicitly enables import features analysis. EntityType-level config: by default enables import features analysis for all Features under it.\n* DISABLED: Explicitly disables import features analysis. EntityType-level config: by default disables import features analysis for all Features under it."]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis {
    type O = terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis {}

impl BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis {
    pub fn build(self) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis {
            anomaly_detection_baseline: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysisRef {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `anomaly_detection_baseline` after provisioning.\nDefines the baseline to do anomaly detection for feature values imported by each [entityTypes.importFeatureValues][] operation. The value must be one of the values below:\n* LATEST_STATS: Choose the later one statistics generated by either most recent snapshot analysis or previous import features analysis. If non of them exists, skip anomaly detection and only generate a statistics.\n* MOST_RECENT_SNAPSHOT_STATS: Use the statistics generated by the most recent snapshot analysis if exists.\n* PREVIOUS_IMPORT_FEATURES_STATS: Use the statistics generated by the previous import features analysis if exists."]
    pub fn anomaly_detection_baseline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anomaly_detection_baseline", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nWhether to enable / disable / inherite default hebavior for import features analysis. The value must be one of the values below:\n* DEFAULT: The default behavior of whether to enable the monitoring. EntityType-level config: disabled.\n* ENABLED: Explicitly enables import features analysis. EntityType-level config: by default enables import features analysis for all Features under it.\n* DISABLED: Explicitly disables import features analysis. EntityType-level config: by default disables import features analysis for all Features under it."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig {
    value: terrars::PrimField<f64>,
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig { }

impl terrars::ToListMappable for VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig {
    type O = terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig {
    #[doc =
        "Specify a threshold value that can trigger the alert. For numerical feature, the distribution distance is calculated by Jensen–Shannon divergence. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub value: terrars::PrimField<f64>,
}

impl BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig {
    pub fn build(self) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig { value: self.value }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfigRef {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nSpecify a threshold value that can trigger the alert. For numerical feature, the distribution distance is calculated by Jensen–Shannon divergence. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_interval_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staleness_days: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis {
    #[doc =
        "Set the field `disabled`.\nThe monitoring schedule for snapshot analysis. For EntityType-level config: unset / disabled = true indicates disabled by default for Features under it; otherwise by default enable snapshot analysis monitoring with monitoringInterval for Features under it."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitoring_interval_days`.\nConfiguration of the snapshot analysis based monitoring pipeline running interval. The value indicates number of days. The default value is 1.\nIf both FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days and [FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval][] are set when creating/updating EntityTypes/Features, FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days will be used."]
    pub fn set_monitoring_interval_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.monitoring_interval_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `staleness_days`.\nCustomized export features time window for snapshot analysis. Unit is one day. The default value is 21 days. Minimum value is 1 day. Maximum value is 4000 days."]
    pub fn set_staleness_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.staleness_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis {
    type O = terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis {}

impl BuildVertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis {
    pub fn build(self) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis {
            disabled: core::default::Default::default(),
            monitoring_interval_days: core::default::Default::default(),
            staleness_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysisRef {
        VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nThe monitoring schedule for snapshot analysis. For EntityType-level config: unset / disabled = true indicates disabled by default for Features under it; otherwise by default enable snapshot analysis monitoring with monitoringInterval for Features under it."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_interval_days` after provisioning.\nConfiguration of the snapshot analysis based monitoring pipeline running interval. The value indicates number of days. The default value is 1.\nIf both FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days and [FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval][] are set when creating/updating EntityTypes/Features, FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days will be used."]
    pub fn monitoring_interval_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_interval_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `staleness_days` after provisioning.\nCustomized export features time window for snapshot analysis. Unit is one day. The default value is 21 days. Minimum value is 1 day. Maximum value is 4000 days."]
    pub fn staleness_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staleness_days", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiFeaturestoreEntitytype_MonitoringConfigDynamic {
    categorical_threshold_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig>>,
    import_features_analysis: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis>>,
    numerical_threshold_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig>>,
    snapshot_analysis: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis>>,
}

#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categorical_threshold_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    import_features_analysis: std
    ::option
    ::Option<std::vec::Vec<VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    numerical_threshold_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_analysis: std
    ::option
    ::Option<std::vec::Vec<VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis>>,
    dynamic: VertexAiFeaturestoreEntitytype_MonitoringConfigDynamic,
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfig {
    #[doc = "Set the field `categorical_threshold_config`.\n"]
    pub fn set_categorical_threshold_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.categorical_threshold_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.categorical_threshold_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `import_features_analysis`.\n"]
    pub fn set_import_features_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysis>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.import_features_analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.import_features_analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `numerical_threshold_config`.\n"]
    pub fn set_numerical_threshold_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.numerical_threshold_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.numerical_threshold_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snapshot_analysis`.\n"]
    pub fn set_snapshot_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysis>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snapshot_analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snapshot_analysis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiFeaturestoreEntitytype_MonitoringConfig {
    type O = terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_MonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeaturestoreEntitytype_MonitoringConfig {}

impl BuildVertexAiFeaturestoreEntitytype_MonitoringConfig {
    pub fn build(self) -> VertexAiFeaturestoreEntitytype_MonitoringConfig {
        VertexAiFeaturestoreEntitytype_MonitoringConfig {
            categorical_threshold_config: core::default::Default::default(),
            import_features_analysis: core::default::Default::default(),
            numerical_threshold_config: core::default::Default::default(),
            snapshot_analysis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeaturestoreEntitytype_MonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeaturestoreEntitytype_MonitoringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeaturestoreEntitytype_MonitoringConfigRef {
        VertexAiFeaturestoreEntitytype_MonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeaturestoreEntitytype_MonitoringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `categorical_threshold_config` after provisioning.\n"]
    pub fn categorical_threshold_config(
        &self,
    ) -> terrars::ListRef<VertexAiFeaturestoreEntitytype_MonitoringConfig_CategoricalThresholdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categorical_threshold_config", self.base))
    }

    #[doc = "Get a reference to the value of field `import_features_analysis` after provisioning.\n"]
    pub fn import_features_analysis(
        &self,
    ) -> terrars::ListRef<VertexAiFeaturestoreEntitytype_MonitoringConfig_ImportFeaturesAnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.import_features_analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `numerical_threshold_config` after provisioning.\n"]
    pub fn numerical_threshold_config(
        &self,
    ) -> terrars::ListRef<VertexAiFeaturestoreEntitytype_MonitoringConfig_NumericalThresholdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.numerical_threshold_config", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_analysis` after provisioning.\n"]
    pub fn snapshot_analysis(
        &self,
    ) -> terrars::ListRef<VertexAiFeaturestoreEntitytype_MonitoringConfig_SnapshotAnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_analysis", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytype_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiFeaturestoreEntitytype_Timeouts {
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

impl terrars::ToListMappable for VertexAiFeaturestoreEntitytype_Timeouts {
    type O = terrars::BlockAssignable<VertexAiFeaturestoreEntitytype_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeaturestoreEntitytype_Timeouts {}

impl BuildVertexAiFeaturestoreEntitytype_Timeouts {
    pub fn build(self) -> VertexAiFeaturestoreEntitytype_Timeouts {
        VertexAiFeaturestoreEntitytype_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeaturestoreEntitytype_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeaturestoreEntitytype_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiFeaturestoreEntitytype_TimeoutsRef {
        VertexAiFeaturestoreEntitytype_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeaturestoreEntitytype_TimeoutsRef {
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
