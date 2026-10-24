use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SpannerInstanceData {
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
    config: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_config: std::option::Option<std::vec::Vec<SpannerInstanceAutoscalingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SpannerInstanceTimeoutsEl>,
    dynamic: SpannerInstanceDynamic,
}

struct SpannerInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpannerInstanceData>,
}

#[derive(Clone)]
pub struct SpannerInstance(std::rc::Rc<SpannerInstance_>);

impl SpannerInstance {
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
        "Set the field `force_destroy`.\nWhen deleting a spanner instance, this boolean option will delete all backups of this instance.\nThis must be set to true if you created a backup manually in the console."]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\n\n\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_nodes`.\nThe number of nodes allocated to this instance. Exactly one of either node_count or processing_units\nmust be present in terraform."]
    pub fn set_num_nodes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().num_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `processing_units`.\nThe number of processing units allocated to this instance. Exactly one of processing_units\nor node_count must be present in terraform."]
    pub fn set_processing_units(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().processing_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerInstanceAutoscalingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SpannerInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\n\n\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_nodes` after provisioning.\nThe number of nodes allocated to this instance. Exactly one of either node_count or processing_units\nmust be present in terraform."]
    pub fn num_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance. Exactly one of processing_units\nor node_count must be present in terraform."]
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

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> terrars::ListRef<SpannerInstanceAutoscalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstanceTimeoutsElRef {
        SpannerInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SpannerInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SpannerInstance { }

impl terrars::ToListMappable for SpannerInstance {
    type O = terrars::ListRef<SpannerInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SpannerInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_spanner_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpannerInstance {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub config: terrars::PrimField<std::string::String>,
    #[doc =
        "The descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildSpannerInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> SpannerInstance {
        let out = SpannerInstance(std::rc::Rc::new(SpannerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpannerInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                config: self.config,
                display_name: self.display_name,
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: core::default::Default::default(),
                num_nodes: core::default::Default::default(),
                processing_units: core::default::Default::default(),
                project: core::default::Default::default(),
                autoscaling_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpannerInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpannerInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\n\n\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_nodes` after provisioning.\nThe number of nodes allocated to this instance. Exactly one of either node_count or processing_units\nmust be present in terraform."]
    pub fn num_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance. Exactly one of processing_units\nor node_count must be present in terraform."]
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

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> terrars::ListRef<SpannerInstanceAutoscalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstanceTimeoutsElRef {
        SpannerInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_processing_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_processing_units: std::option::Option<terrars::PrimField<f64>>,
}

impl SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    #[doc =
        "Set the field `max_nodes`.\nSpecifies maximum number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn set_max_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_processing_units`.\nSpecifies maximum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn set_max_processing_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_processing_units = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_nodes`.\nSpecifies number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to 1."]
    pub fn set_min_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_processing_units`.\nSpecifies minimum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000."]
    pub fn set_min_processing_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_processing_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    type O = terrars::BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {}

impl BuildSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    pub fn build(self) -> SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
        SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
            max_nodes: core::default::Default::default(),
            max_processing_units: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            min_processing_units: core::default::Default::default(),
        }
    }
}

pub struct SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
        SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_nodes` after provisioning.\nSpecifies maximum number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn max_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_processing_units` after provisioning.\nSpecifies maximum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn max_processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_processing_units", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_nodes` after provisioning.\nSpecifies number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to 1."]
    pub fn min_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_processing_units` after provisioning.\nSpecifies minimum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000."]
    pub fn min_processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_processing_units", self.base))
    }
}

#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high_priority_cpu_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    #[doc =
        "Set the field `high_priority_cpu_utilization_percent`.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization).."]
    pub fn set_high_priority_cpu_utilization_percent(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.high_priority_cpu_utilization_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_utilization_percent`.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_storage_utilization_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_utilization_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    type O = terrars::BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {}

impl BuildSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    pub fn build(self) -> SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
        SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
            high_priority_cpu_utilization_percent: core::default::Default::default(),
            storage_utilization_percent: core::default::Default::default(),
        }
    }
}

pub struct SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
        SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `high_priority_cpu_utilization_percent` after provisioning.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization).."]
    pub fn high_priority_cpu_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.high_priority_cpu_utilization_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_utilization_percent` after provisioning.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn storage_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_utilization_percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpannerInstanceAutoscalingConfigElDynamic {
    autoscaling_limits: std
    ::option
    ::Option<terrars::DynamicBlock<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    autoscaling_targets: std
    ::option
    ::Option<terrars::DynamicBlock<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
}

#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_limits: std::option::Option<std::vec::Vec<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_targets: std::option::Option<std::vec::Vec<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
    dynamic: SpannerInstanceAutoscalingConfigElDynamic,
}

impl SpannerInstanceAutoscalingConfigEl {
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `autoscaling_targets`.\n"]
    pub fn set_autoscaling_targets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_targets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpannerInstanceAutoscalingConfigEl {
    type O = terrars::BlockAssignable<SpannerInstanceAutoscalingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstanceAutoscalingConfigEl {}

impl BuildSpannerInstanceAutoscalingConfigEl {
    pub fn build(self) -> SpannerInstanceAutoscalingConfigEl {
        SpannerInstanceAutoscalingConfigEl {
            autoscaling_limits: core::default::Default::default(),
            autoscaling_targets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SpannerInstanceAutoscalingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstanceAutoscalingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerInstanceAutoscalingConfigElRef {
        SpannerInstanceAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstanceAutoscalingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]
    pub fn autoscaling_limits(&self) -> terrars::ListRef<SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_limits", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_targets` after provisioning.\n"]
    pub fn autoscaling_targets(&self) -> terrars::ListRef<SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_targets", self.base))
    }
}

#[derive(Serialize)]
pub struct SpannerInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpannerInstanceTimeoutsEl {
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

impl terrars::ToListMappable for SpannerInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<SpannerInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstanceTimeoutsEl {}

impl BuildSpannerInstanceTimeoutsEl {
    pub fn build(self) -> SpannerInstanceTimeoutsEl {
        SpannerInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SpannerInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerInstanceTimeoutsElRef {
        SpannerInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstanceTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct SpannerInstanceDynamic {
    autoscaling_config: std::option::Option<terrars::DynamicBlock<SpannerInstanceAutoscalingConfigEl>>,
}
