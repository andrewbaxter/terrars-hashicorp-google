#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct TranscoderJobData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<TranscoderJob_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<TranscoderJob_Timeouts>,
    dynamic: TranscoderJobDynamic,
}

struct TranscoderJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TranscoderJobData_>,
}

#[derive(Clone)]
pub struct TranscoderJob(std::rc::Rc<TranscoderJob_>);

impl TranscoderJob {
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
        "Set the field `labels`.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        "Set the field `template_id`.\nSpecify the templateId to use for populating Job.config.\nThe default is preset/web-hd, which is the only supported preset."]
    pub fn set_template_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().template_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<TranscoderJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the job was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe time the transcoding finished."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the transcoding job resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe time the transcoding started."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the job."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template_id` after provisioning.\nSpecify the templateId to use for populating Job.config.\nThe default is preset/web-hd, which is the only supported preset."]
    pub fn template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<TranscoderJob_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscoderJob_TimeoutsRef {
        TranscoderJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for TranscoderJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TranscoderJob { }

impl terrars::ToListMappable for TranscoderJob {
    type O = terrars::ListRef<TranscoderJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TranscoderJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_transcoder_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTranscoderJob {
    pub tf_id: std::string::String,
    #[doc = "The location of the transcoding job resource."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildTranscoderJob {
    pub fn build(self, stack: &mut terrars::Stack) -> TranscoderJob {
        let out = TranscoderJob(std::rc::Rc::new(TranscoderJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TranscoderJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                template_id: core::default::Default::default(),
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TranscoderJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TranscoderJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the job was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe time the transcoding finished."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the transcoding job resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe time the transcoding started."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the job."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template_id` after provisioning.\nSpecify the templateId to use for populating Job.config.\nThe default is preset/web-hd, which is the only supported preset."]
    pub fn template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<TranscoderJob_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscoderJob_TimeoutsRef {
        TranscoderJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJobDynamic {
    config: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_AdBreaks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Config_AdBreaks {
    #[doc =
        "Set the field `start_time_offset`.\nStart time in seconds for the ad break, relative to the output file timeline"]
    pub fn set_start_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_AdBreaks {
    type O = terrars::BlockAssignable<TranscoderJob_Config_AdBreaks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_AdBreaks {}

impl BuildTranscoderJob_Config_AdBreaks {
    pub fn build(self) -> TranscoderJob_Config_AdBreaks {
        TranscoderJob_Config_AdBreaks { start_time_offset: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_AdBreaksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_AdBreaksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_AdBreaksRef {
        TranscoderJob_Config_AdBreaksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_AdBreaksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `start_time_offset` after provisioning.\nStart time in seconds for the ad break, relative to the output file timeline"]
    pub fn start_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time_offset", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_EditList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inputs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Config_EditList {
    #[doc = "Set the field `inputs`.\nList of values identifying files that should be used in this atom."]
    pub fn set_inputs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inputs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nA unique key for this atom."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_time_offset`.\nStart time in seconds for the atom, relative to the input file timeline. The default is '0s'."]
    pub fn set_start_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_EditList {
    type O = terrars::BlockAssignable<TranscoderJob_Config_EditList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_EditList {}

impl BuildTranscoderJob_Config_EditList {
    pub fn build(self) -> TranscoderJob_Config_EditList {
        TranscoderJob_Config_EditList {
            inputs: core::default::Default::default(),
            key: core::default::Default::default(),
            start_time_offset: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_EditListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_EditListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_EditListRef {
        TranscoderJob_Config_EditListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_EditListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inputs` after provisioning.\nList of values identifying files that should be used in this atom."]
    pub fn inputs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inputs", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nA unique key for this atom."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time_offset` after provisioning.\nStart time in seconds for the atom, relative to the input file timeline. The default is '0s'."]
    pub fn start_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time_offset", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_ElementaryStreams_AudioStream {
    bitrate_bps: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_layout: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate_hertz: std::option::Option<terrars::PrimField<f64>>,
}

impl TranscoderJob_Config_ElementaryStreams_AudioStream {
    #[doc = "Set the field `channel_count`.\nNumber of audio channels. The default is '2'."]
    pub fn set_channel_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.channel_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `channel_layout`.\nA list of channel names specifying layout of the audio channels. The default is [\"fl\", \"fr\"]."]
    pub fn set_channel_layout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.channel_layout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `codec`.\nThe codec for this audio stream. The default is 'aac'."]
    pub fn set_codec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.codec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate_hertz`.\nThe audio sample rate in Hertz. The default is '48000'."]
    pub fn set_sample_rate_hertz(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate_hertz = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_ElementaryStreams_AudioStream {
    type O = terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_AudioStream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_ElementaryStreams_AudioStream {
    #[doc = "Audio bitrate in bits per second."]
    pub bitrate_bps: terrars::PrimField<f64>,
}

impl BuildTranscoderJob_Config_ElementaryStreams_AudioStream {
    pub fn build(self) -> TranscoderJob_Config_ElementaryStreams_AudioStream {
        TranscoderJob_Config_ElementaryStreams_AudioStream {
            bitrate_bps: self.bitrate_bps,
            channel_count: core::default::Default::default(),
            channel_layout: core::default::Default::default(),
            codec: core::default::Default::default(),
            sample_rate_hertz: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_ElementaryStreams_AudioStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_ElementaryStreams_AudioStreamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_ElementaryStreams_AudioStreamRef {
        TranscoderJob_Config_ElementaryStreams_AudioStreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_ElementaryStreams_AudioStreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bitrate_bps` after provisioning.\nAudio bitrate in bits per second."]
    pub fn bitrate_bps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate_bps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `channel_count` after provisioning.\nNumber of audio channels. The default is '2'."]
    pub fn channel_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `channel_layout` after provisioning.\nA list of channel names specifying layout of the audio channels. The default is [\"fl\", \"fr\"]."]
    pub fn channel_layout(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.channel_layout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `codec` after provisioning.\nThe codec for this audio stream. The default is 'aac'."]
    pub fn codec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_rate_hertz` after provisioning.\nThe audio sample rate in Hertz. The default is '48000'."]
    pub fn sample_rate_hertz(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate_hertz", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg {}

impl TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg { }

impl terrars::ToListMappable for TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg {
    type O = terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg {}

impl BuildTranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg {
    pub fn build(self) -> TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg {
        TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStream_H264_HlgRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_ElementaryStreams_VideoStream_H264_HlgRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_ElementaryStreams_VideoStream_H264_HlgRef {
        TranscoderJob_Config_ElementaryStreams_VideoStream_H264_HlgRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_ElementaryStreams_VideoStream_H264_HlgRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr {}

impl TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr { }

impl terrars::ToListMappable for TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr {
    type O = terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr {}

impl BuildTranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr {
    pub fn build(self) -> TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr {
        TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStream_H264_SdrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_ElementaryStreams_VideoStream_H264_SdrRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_ElementaryStreams_VideoStream_H264_SdrRef {
        TranscoderJob_Config_ElementaryStreams_VideoStream_H264_SdrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_ElementaryStreams_VideoStream_H264_SdrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_ElementaryStreams_VideoStream_H264Dynamic {
    hlg: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg>>,
    sdr: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStream_H264 {
    bitrate_bps: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crf_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entropy_coder: std::option::Option<terrars::PrimField<std::string::String>>,
    frame_rate: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    height_pixels: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pixel_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_control_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vbv_fullness_bits: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vbv_size_bits: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    width_pixels: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hlg: std::option::Option<std::vec::Vec<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sdr: std::option::Option<std::vec::Vec<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr>>,
    dynamic: TranscoderJob_Config_ElementaryStreams_VideoStream_H264Dynamic,
}

impl TranscoderJob_Config_ElementaryStreams_VideoStream_H264 {
    #[doc = "Set the field `crf_level`.\nTarget CRF level. The default is '21'."]
    pub fn set_crf_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.crf_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entropy_coder`.\nThe entropy coder to use. The default is 'cabac'."]
    pub fn set_entropy_coder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entropy_coder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_duration`.\nSelect the GOP size based on the specified duration. The default is '3s'."]
    pub fn set_gop_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gop_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `height_pixels`.\nThe height of the video in pixels."]
    pub fn set_height_pixels(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.height_pixels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pixel_format`.\nPixel format to use. The default is 'yuv420p'."]
    pub fn set_pixel_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pixel_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preset`.\nEnforces the specified codec preset. The default is 'veryfast'."]
    pub fn set_preset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile`.\nEnforces the specified codec profile."]
    pub fn set_profile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_control_mode`.\nSpecify the mode. The default is 'vbr'."]
    pub fn set_rate_control_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rate_control_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vbv_fullness_bits`.\nInitial fullness of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn set_vbv_fullness_bits(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vbv_fullness_bits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vbv_size_bits`.\nSize of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn set_vbv_size_bits(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vbv_size_bits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `width_pixels`.\nThe width of the video in pixels."]
    pub fn set_width_pixels(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.width_pixels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hlg`.\n"]
    pub fn set_hlg(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Hlg>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hlg = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hlg = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sdr`.\n"]
    pub fn set_sdr(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_Sdr>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sdr = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sdr = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_ElementaryStreams_VideoStream_H264 {
    type O = terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream_H264>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_ElementaryStreams_VideoStream_H264 {
    #[doc = "The video bitrate in bits per second."]
    pub bitrate_bps: terrars::PrimField<f64>,
    #[doc = "The target video frame rate in frames per second (FPS)."]
    pub frame_rate: terrars::PrimField<f64>,
}

impl BuildTranscoderJob_Config_ElementaryStreams_VideoStream_H264 {
    pub fn build(self) -> TranscoderJob_Config_ElementaryStreams_VideoStream_H264 {
        TranscoderJob_Config_ElementaryStreams_VideoStream_H264 {
            bitrate_bps: self.bitrate_bps,
            crf_level: core::default::Default::default(),
            entropy_coder: core::default::Default::default(),
            frame_rate: self.frame_rate,
            gop_duration: core::default::Default::default(),
            height_pixels: core::default::Default::default(),
            pixel_format: core::default::Default::default(),
            preset: core::default::Default::default(),
            profile: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            vbv_fullness_bits: core::default::Default::default(),
            vbv_size_bits: core::default::Default::default(),
            width_pixels: core::default::Default::default(),
            hlg: core::default::Default::default(),
            sdr: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStream_H264Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_ElementaryStreams_VideoStream_H264Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_ElementaryStreams_VideoStream_H264Ref {
        TranscoderJob_Config_ElementaryStreams_VideoStream_H264Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_ElementaryStreams_VideoStream_H264Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bitrate_bps` after provisioning.\nThe video bitrate in bits per second."]
    pub fn bitrate_bps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate_bps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `crf_level` after provisioning.\nTarget CRF level. The default is '21'."]
    pub fn crf_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crf_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entropy_coder` after provisioning.\nThe entropy coder to use. The default is 'cabac'."]
    pub fn entropy_coder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entropy_coder", self.base))
    }

    #[doc =
        "Get a reference to the value of field `frame_rate` after provisioning.\nThe target video frame rate in frames per second (FPS)."]
    pub fn frame_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frame_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gop_duration` after provisioning.\nSelect the GOP size based on the specified duration. The default is '3s'."]
    pub fn gop_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `height_pixels` after provisioning.\nThe height of the video in pixels."]
    pub fn height_pixels(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height_pixels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pixel_format` after provisioning.\nPixel format to use. The default is 'yuv420p'."]
    pub fn pixel_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pixel_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preset` after provisioning.\nEnforces the specified codec preset. The default is 'veryfast'."]
    pub fn preset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `profile` after provisioning.\nEnforces the specified codec profile."]
    pub fn profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rate_control_mode` after provisioning.\nSpecify the mode. The default is 'vbr'."]
    pub fn rate_control_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_control_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vbv_fullness_bits` after provisioning.\nInitial fullness of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn vbv_fullness_bits(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vbv_fullness_bits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vbv_size_bits` after provisioning.\nSize of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn vbv_size_bits(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vbv_size_bits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `width_pixels` after provisioning.\nThe width of the video in pixels."]
    pub fn width_pixels(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width_pixels", self.base))
    }

    #[doc = "Get a reference to the value of field `hlg` after provisioning.\n"]
    pub fn hlg(&self) -> terrars::ListRef<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_HlgRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hlg", self.base))
    }

    #[doc = "Get a reference to the value of field `sdr` after provisioning.\n"]
    pub fn sdr(&self) -> terrars::ListRef<TranscoderJob_Config_ElementaryStreams_VideoStream_H264_SdrRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sdr", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_ElementaryStreams_VideoStreamDynamic {
    h264: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_ElementaryStreams_VideoStream_H264>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStream {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h264: std::option::Option<std::vec::Vec<TranscoderJob_Config_ElementaryStreams_VideoStream_H264>>,
    dynamic: TranscoderJob_Config_ElementaryStreams_VideoStreamDynamic,
}

impl TranscoderJob_Config_ElementaryStreams_VideoStream {
    #[doc = "Set the field `h264`.\n"]
    pub fn set_h264(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream_H264>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.h264 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.h264 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_ElementaryStreams_VideoStream {
    type O = terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_ElementaryStreams_VideoStream {}

impl BuildTranscoderJob_Config_ElementaryStreams_VideoStream {
    pub fn build(self) -> TranscoderJob_Config_ElementaryStreams_VideoStream {
        TranscoderJob_Config_ElementaryStreams_VideoStream {
            h264: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_ElementaryStreams_VideoStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_ElementaryStreams_VideoStreamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_ElementaryStreams_VideoStreamRef {
        TranscoderJob_Config_ElementaryStreams_VideoStreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_ElementaryStreams_VideoStreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `h264` after provisioning.\n"]
    pub fn h264(&self) -> terrars::ListRef<TranscoderJob_Config_ElementaryStreams_VideoStream_H264Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.h264", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_ElementaryStreamsDynamic {
    audio_stream: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_ElementaryStreams_AudioStream>>,
    video_stream: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_ElementaryStreams_VideoStream>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_ElementaryStreams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_stream: std::option::Option<std::vec::Vec<TranscoderJob_Config_ElementaryStreams_AudioStream>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_stream: std::option::Option<std::vec::Vec<TranscoderJob_Config_ElementaryStreams_VideoStream>>,
    dynamic: TranscoderJob_Config_ElementaryStreamsDynamic,
}

impl TranscoderJob_Config_ElementaryStreams {
    #[doc = "Set the field `key`.\nA unique key for this atom."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_stream`.\n"]
    pub fn set_audio_stream(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_AudioStream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_stream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_stream`.\n"]
    pub fn set_video_stream(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams_VideoStream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video_stream = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_ElementaryStreams {
    type O = terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_ElementaryStreams {}

impl BuildTranscoderJob_Config_ElementaryStreams {
    pub fn build(self) -> TranscoderJob_Config_ElementaryStreams {
        TranscoderJob_Config_ElementaryStreams {
            key: core::default::Default::default(),
            audio_stream: core::default::Default::default(),
            video_stream: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_ElementaryStreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_ElementaryStreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_ElementaryStreamsRef {
        TranscoderJob_Config_ElementaryStreamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_ElementaryStreamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nA unique key for this atom."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_stream` after provisioning.\n"]
    pub fn audio_stream(&self) -> terrars::ListRef<TranscoderJob_Config_ElementaryStreams_AudioStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `video_stream` after provisioning.\n"]
    pub fn video_stream(&self) -> terrars::ListRef<TranscoderJob_Config_ElementaryStreams_VideoStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_stream", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_Aes128 {}

impl TranscoderJob_Config_Encryptions_Aes128 { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_Aes128 {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_Aes128>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_Aes128 {}

impl BuildTranscoderJob_Config_Encryptions_Aes128 {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_Aes128 {
        TranscoderJob_Config_Encryptions_Aes128 {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_Aes128Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_Aes128Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_Encryptions_Aes128Ref {
        TranscoderJob_Config_Encryptions_Aes128Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_Aes128Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_Clearkey {}

impl TranscoderJob_Config_Encryptions_DrmSystems_Clearkey { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_DrmSystems_Clearkey {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Clearkey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_DrmSystems_Clearkey {}

impl BuildTranscoderJob_Config_Encryptions_DrmSystems_Clearkey {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_DrmSystems_Clearkey {
        TranscoderJob_Config_Encryptions_DrmSystems_Clearkey {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_ClearkeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_DrmSystems_ClearkeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Encryptions_DrmSystems_ClearkeyRef {
        TranscoderJob_Config_Encryptions_DrmSystems_ClearkeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_DrmSystems_ClearkeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_Fairplay {}

impl TranscoderJob_Config_Encryptions_DrmSystems_Fairplay { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_DrmSystems_Fairplay {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Fairplay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_DrmSystems_Fairplay {}

impl BuildTranscoderJob_Config_Encryptions_DrmSystems_Fairplay {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_DrmSystems_Fairplay {
        TranscoderJob_Config_Encryptions_DrmSystems_Fairplay {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_FairplayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_DrmSystems_FairplayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Encryptions_DrmSystems_FairplayRef {
        TranscoderJob_Config_Encryptions_DrmSystems_FairplayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_DrmSystems_FairplayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_Playready {}

impl TranscoderJob_Config_Encryptions_DrmSystems_Playready { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_DrmSystems_Playready {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Playready>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_DrmSystems_Playready {}

impl BuildTranscoderJob_Config_Encryptions_DrmSystems_Playready {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_DrmSystems_Playready {
        TranscoderJob_Config_Encryptions_DrmSystems_Playready {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_PlayreadyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_DrmSystems_PlayreadyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Encryptions_DrmSystems_PlayreadyRef {
        TranscoderJob_Config_Encryptions_DrmSystems_PlayreadyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_DrmSystems_PlayreadyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_Widevine {}

impl TranscoderJob_Config_Encryptions_DrmSystems_Widevine { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_DrmSystems_Widevine {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Widevine>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_DrmSystems_Widevine {}

impl BuildTranscoderJob_Config_Encryptions_DrmSystems_Widevine {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_DrmSystems_Widevine {
        TranscoderJob_Config_Encryptions_DrmSystems_Widevine {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems_WidevineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_DrmSystems_WidevineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Encryptions_DrmSystems_WidevineRef {
        TranscoderJob_Config_Encryptions_DrmSystems_WidevineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_DrmSystems_WidevineRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_Encryptions_DrmSystemsDynamic {
    clearkey: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_DrmSystems_Clearkey>>,
    fairplay: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_DrmSystems_Fairplay>>,
    playready: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_DrmSystems_Playready>>,
    widevine: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_DrmSystems_Widevine>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_DrmSystems {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clearkey: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_DrmSystems_Clearkey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fairplay: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_DrmSystems_Fairplay>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    playready: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_DrmSystems_Playready>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    widevine: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_DrmSystems_Widevine>>,
    dynamic: TranscoderJob_Config_Encryptions_DrmSystemsDynamic,
}

impl TranscoderJob_Config_Encryptions_DrmSystems {
    #[doc = "Set the field `clearkey`.\n"]
    pub fn set_clearkey(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Clearkey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.clearkey = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.clearkey = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fairplay`.\n"]
    pub fn set_fairplay(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Fairplay>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fairplay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fairplay = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `playready`.\n"]
    pub fn set_playready(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Playready>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.playready = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.playready = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `widevine`.\n"]
    pub fn set_widevine(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems_Widevine>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.widevine = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.widevine = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_DrmSystems {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_DrmSystems {}

impl BuildTranscoderJob_Config_Encryptions_DrmSystems {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_DrmSystems {
        TranscoderJob_Config_Encryptions_DrmSystems {
            clearkey: core::default::Default::default(),
            fairplay: core::default::Default::default(),
            playready: core::default::Default::default(),
            widevine: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_DrmSystemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_DrmSystemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Encryptions_DrmSystemsRef {
        TranscoderJob_Config_Encryptions_DrmSystemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_DrmSystemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clearkey` after provisioning.\n"]
    pub fn clearkey(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_DrmSystems_ClearkeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clearkey", self.base))
    }

    #[doc = "Get a reference to the value of field `fairplay` after provisioning.\n"]
    pub fn fairplay(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_DrmSystems_FairplayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fairplay", self.base))
    }

    #[doc = "Get a reference to the value of field `playready` after provisioning.\n"]
    pub fn playready(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_DrmSystems_PlayreadyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.playready", self.base))
    }

    #[doc = "Get a reference to the value of field `widevine` after provisioning.\n"]
    pub fn widevine(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_DrmSystems_WidevineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.widevine", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_MpegCenc {
    scheme: terrars::PrimField<std::string::String>,
}

impl TranscoderJob_Config_Encryptions_MpegCenc { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_MpegCenc {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_MpegCenc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_MpegCenc {
    #[doc = "Specify the encryption scheme."]
    pub scheme: terrars::PrimField<std::string::String>,
}

impl BuildTranscoderJob_Config_Encryptions_MpegCenc {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_MpegCenc {
        TranscoderJob_Config_Encryptions_MpegCenc { scheme: self.scheme }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_MpegCencRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_MpegCencRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_Encryptions_MpegCencRef {
        TranscoderJob_Config_Encryptions_MpegCencRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_MpegCencRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\nSpecify the encryption scheme."]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_SampleAes {}

impl TranscoderJob_Config_Encryptions_SampleAes { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_SampleAes {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_SampleAes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_SampleAes {}

impl BuildTranscoderJob_Config_Encryptions_SampleAes {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_SampleAes {
        TranscoderJob_Config_Encryptions_SampleAes {}
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_SampleAesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_SampleAesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_Encryptions_SampleAesRef {
        TranscoderJob_Config_Encryptions_SampleAesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_SampleAesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions_SecretManagerKeySource {
    secret_version: terrars::PrimField<std::string::String>,
}

impl TranscoderJob_Config_Encryptions_SecretManagerKeySource { }

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions_SecretManagerKeySource {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions_SecretManagerKeySource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions_SecretManagerKeySource {
    #[doc =
        "The name of the Secret Version containing the encryption key in the following format: projects/{project}/secrets/{secret_id}/versions/{version_number}."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildTranscoderJob_Config_Encryptions_SecretManagerKeySource {
    pub fn build(self) -> TranscoderJob_Config_Encryptions_SecretManagerKeySource {
        TranscoderJob_Config_Encryptions_SecretManagerKeySource { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Encryptions_SecretManagerKeySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Encryptions_SecretManagerKeySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Encryptions_SecretManagerKeySourceRef {
        TranscoderJob_Config_Encryptions_SecretManagerKeySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Encryptions_SecretManagerKeySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe name of the Secret Version containing the encryption key in the following format: projects/{project}/secrets/{secret_id}/versions/{version_number}."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_EncryptionsDynamic {
    aes128: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_Aes128>>,
    drm_systems: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_DrmSystems>>,
    mpeg_cenc: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_MpegCenc>>,
    sample_aes: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_SampleAes>>,
    secret_manager_key_source: std
    ::option
    ::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions_SecretManagerKeySource>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Encryptions {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aes128: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_Aes128>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drm_systems: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_DrmSystems>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mpeg_cenc: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_MpegCenc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_aes: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_SampleAes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_key_source: std
    ::option
    ::Option<std::vec::Vec<TranscoderJob_Config_Encryptions_SecretManagerKeySource>>,
    dynamic: TranscoderJob_Config_EncryptionsDynamic,
}

impl TranscoderJob_Config_Encryptions {
    #[doc = "Set the field `aes128`.\n"]
    pub fn set_aes128(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_Aes128>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aes128 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aes128 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `drm_systems`.\n"]
    pub fn set_drm_systems(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_DrmSystems>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.drm_systems = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.drm_systems = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mpeg_cenc`.\n"]
    pub fn set_mpeg_cenc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_MpegCenc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mpeg_cenc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mpeg_cenc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sample_aes`.\n"]
    pub fn set_sample_aes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_SampleAes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sample_aes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_aes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_manager_key_source`.\n"]
    pub fn set_secret_manager_key_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions_SecretManagerKeySource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_manager_key_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_manager_key_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Encryptions {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Encryptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Encryptions {
    #[doc = "Identifier for this set of encryption options."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildTranscoderJob_Config_Encryptions {
    pub fn build(self) -> TranscoderJob_Config_Encryptions {
        TranscoderJob_Config_Encryptions {
            id: self.id,
            aes128: core::default::Default::default(),
            drm_systems: core::default::Default::default(),
            mpeg_cenc: core::default::Default::default(),
            sample_aes: core::default::Default::default(),
            secret_manager_key_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_EncryptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_EncryptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_EncryptionsRef {
        TranscoderJob_Config_EncryptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_EncryptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIdentifier for this set of encryption options."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `aes128` after provisioning.\n"]
    pub fn aes128(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_Aes128Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aes128", self.base))
    }

    #[doc = "Get a reference to the value of field `drm_systems` after provisioning.\n"]
    pub fn drm_systems(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_DrmSystemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.drm_systems", self.base))
    }

    #[doc = "Get a reference to the value of field `mpeg_cenc` after provisioning.\n"]
    pub fn mpeg_cenc(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_MpegCencRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mpeg_cenc", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_aes` after provisioning.\n"]
    pub fn sample_aes(&self) -> terrars::ListRef<TranscoderJob_Config_Encryptions_SampleAesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_aes", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_manager_key_source` after provisioning.\n"]
    pub fn secret_manager_key_source(
        &self,
    ) -> terrars::ListRef<TranscoderJob_Config_Encryptions_SecretManagerKeySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager_key_source", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Inputs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Config_Inputs {
    #[doc =
        "Set the field `key`.\nA unique key for this input. Must be specified when using advanced mapping and edit lists."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `uri`.\nURI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, gs://bucket/inputs/file.mp4).\nIf empty, the value is populated from Job.input_uri."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Inputs {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Inputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Inputs {}

impl BuildTranscoderJob_Config_Inputs {
    pub fn build(self) -> TranscoderJob_Config_Inputs {
        TranscoderJob_Config_Inputs {
            key: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_InputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_InputsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_InputsRef {
        TranscoderJob_Config_InputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_InputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nA unique key for this input. Must be specified when using advanced mapping and edit lists."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, gs://bucket/inputs/file.mp4).\nIf empty, the value is populated from Job.input_uri."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Manifests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mux_streams: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Config_Manifests {
    #[doc = "Set the field `file_name`.\nThe name of the generated file. The default is 'manifest'."]
    pub fn set_file_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mux_streams`.\nList of user supplied MuxStream.key values that should appear in this manifest."]
    pub fn set_mux_streams(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.mux_streams = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nType of the manifest. Possible values: [\"MANIFEST_TYPE_UNSPECIFIED\", \"HLS\", \"DASH\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Manifests {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Manifests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Manifests {}

impl BuildTranscoderJob_Config_Manifests {
    pub fn build(self) -> TranscoderJob_Config_Manifests {
        TranscoderJob_Config_Manifests {
            file_name: core::default::Default::default(),
            mux_streams: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_ManifestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_ManifestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_ManifestsRef {
        TranscoderJob_Config_ManifestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_ManifestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_name` after provisioning.\nThe name of the generated file. The default is 'manifest'."]
    pub fn file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mux_streams` after provisioning.\nList of user supplied MuxStream.key values that should appear in this manifest."]
    pub fn mux_streams(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mux_streams", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of the manifest. Possible values: [\"MANIFEST_TYPE_UNSPECIFIED\", \"HLS\", \"DASH\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_MuxStreams_SegmentSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Config_MuxStreams_SegmentSettings {
    #[doc = "Set the field `segment_duration`.\nDuration of the segments in seconds. The default is '6.0s'."]
    pub fn set_segment_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segment_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_MuxStreams_SegmentSettings {
    type O = terrars::BlockAssignable<TranscoderJob_Config_MuxStreams_SegmentSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_MuxStreams_SegmentSettings {}

impl BuildTranscoderJob_Config_MuxStreams_SegmentSettings {
    pub fn build(self) -> TranscoderJob_Config_MuxStreams_SegmentSettings {
        TranscoderJob_Config_MuxStreams_SegmentSettings { segment_duration: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_MuxStreams_SegmentSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_MuxStreams_SegmentSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_MuxStreams_SegmentSettingsRef {
        TranscoderJob_Config_MuxStreams_SegmentSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_MuxStreams_SegmentSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `segment_duration` after provisioning.\nDuration of the segments in seconds. The default is '6.0s'."]
    pub fn segment_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_MuxStreamsDynamic {
    segment_settings: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_MuxStreams_SegmentSettings>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_MuxStreams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elementary_streams: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_settings: std::option::Option<std::vec::Vec<TranscoderJob_Config_MuxStreams_SegmentSettings>>,
    dynamic: TranscoderJob_Config_MuxStreamsDynamic,
}

impl TranscoderJob_Config_MuxStreams {
    #[doc = "Set the field `container`.\nThe container format. The default is 'mp4'."]
    pub fn set_container(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elementary_streams`.\nList of ElementaryStream.key values multiplexed in this stream."]
    pub fn set_elementary_streams(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.elementary_streams = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_id`.\nIdentifier of the encryption configuration to use."]
    pub fn set_encryption_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_name`.\nThe name of the generated file."]
    pub fn set_file_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nA unique key for this multiplexed stream."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_settings`.\n"]
    pub fn set_segment_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_MuxStreams_SegmentSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.segment_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.segment_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_MuxStreams {
    type O = terrars::BlockAssignable<TranscoderJob_Config_MuxStreams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_MuxStreams {}

impl BuildTranscoderJob_Config_MuxStreams {
    pub fn build(self) -> TranscoderJob_Config_MuxStreams {
        TranscoderJob_Config_MuxStreams {
            container: core::default::Default::default(),
            elementary_streams: core::default::Default::default(),
            encryption_id: core::default::Default::default(),
            file_name: core::default::Default::default(),
            key: core::default::Default::default(),
            segment_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_MuxStreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_MuxStreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_MuxStreamsRef {
        TranscoderJob_Config_MuxStreamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_MuxStreamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container` after provisioning.\nThe container format. The default is 'mp4'."]
    pub fn container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc =
        "Get a reference to the value of field `elementary_streams` after provisioning.\nList of ElementaryStream.key values multiplexed in this stream."]
    pub fn elementary_streams(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elementary_streams", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encryption_id` after provisioning.\nIdentifier of the encryption configuration to use."]
    pub fn encryption_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_name` after provisioning.\nThe name of the generated file."]
    pub fn file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nA unique key for this multiplexed stream."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_settings` after provisioning.\n"]
    pub fn segment_settings(&self) -> terrars::ListRef<TranscoderJob_Config_MuxStreams_SegmentSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segment_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Output {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Config_Output {
    #[doc = "Set the field `uri`.\nURI for the output file(s). For example, gs://my-bucket/outputs/."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Output {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Output>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Output {}

impl BuildTranscoderJob_Config_Output {
    pub fn build(self) -> TranscoderJob_Config_Output {
        TranscoderJob_Config_Output { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_OutputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_OutputRef {
        TranscoderJob_Config_OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_OutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI for the output file(s). For example, gs://my-bucket/outputs/."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    y: std::option::Option<terrars::PrimField<f64>>,
}

impl TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy {
    #[doc = "Set the field `x`.\nNormalized x coordinate."]
    pub fn set_x(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.x = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `y`.\nNormalized y coordinate."]
    pub fn set_y(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.y = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Overlays_Animations_AnimationFade_Xy {}

impl BuildTranscoderJob_Config_Overlays_Animations_AnimationFade_Xy {
    pub fn build(self) -> TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy {
        TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy {
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Overlays_Animations_AnimationFade_XyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Overlays_Animations_AnimationFade_XyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Overlays_Animations_AnimationFade_XyRef {
        TranscoderJob_Config_Overlays_Animations_AnimationFade_XyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Overlays_Animations_AnimationFade_XyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `x` after provisioning.\nNormalized x coordinate."]
    pub fn x(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }

    #[doc = "Get a reference to the value of field `y` after provisioning.\nNormalized y coordinate."]
    pub fn y(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_Overlays_Animations_AnimationFadeDynamic {
    xy: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Overlays_Animations_AnimationFade {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    fade_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xy: std::option::Option<std::vec::Vec<TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy>>,
    dynamic: TranscoderJob_Config_Overlays_Animations_AnimationFadeDynamic,
}

impl TranscoderJob_Config_Overlays_Animations_AnimationFade {
    #[doc = "Set the field `end_time_offset`.\nThe time to end the fade animation, in seconds."]
    pub fn set_end_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time_offset`.\nThe time to start the fade animation, in seconds."]
    pub fn set_start_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xy`.\n"]
    pub fn set_xy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TranscoderJob_Config_Overlays_Animations_AnimationFade_Xy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.xy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.xy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Overlays_Animations_AnimationFade {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Overlays_Animations_AnimationFade>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Overlays_Animations_AnimationFade {
    #[doc =
        "Required. Type of fade animation: 'FADE_IN' or 'FADE_OUT'.\nThe possible values are:\n\n* 'FADE_TYPE_UNSPECIFIED': The fade type is not specified.\n\n* 'FADE_IN': Fade the overlay object into view.\n\n* 'FADE_OUT': Fade the overlay object out of view. Possible values: [\"FADE_TYPE_UNSPECIFIED\", \"FADE_IN\", \"FADE_OUT\"]"]
    pub fade_type: terrars::PrimField<std::string::String>,
}

impl BuildTranscoderJob_Config_Overlays_Animations_AnimationFade {
    pub fn build(self) -> TranscoderJob_Config_Overlays_Animations_AnimationFade {
        TranscoderJob_Config_Overlays_Animations_AnimationFade {
            end_time_offset: core::default::Default::default(),
            fade_type: self.fade_type,
            start_time_offset: core::default::Default::default(),
            xy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Overlays_Animations_AnimationFadeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Overlays_Animations_AnimationFadeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TranscoderJob_Config_Overlays_Animations_AnimationFadeRef {
        TranscoderJob_Config_Overlays_Animations_AnimationFadeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Overlays_Animations_AnimationFadeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time_offset` after provisioning.\nThe time to end the fade animation, in seconds."]
    pub fn end_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time_offset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fade_type` after provisioning.\nRequired. Type of fade animation: 'FADE_IN' or 'FADE_OUT'.\nThe possible values are:\n\n* 'FADE_TYPE_UNSPECIFIED': The fade type is not specified.\n\n* 'FADE_IN': Fade the overlay object into view.\n\n* 'FADE_OUT': Fade the overlay object out of view. Possible values: [\"FADE_TYPE_UNSPECIFIED\", \"FADE_IN\", \"FADE_OUT\"]"]
    pub fn fade_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fade_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time_offset` after provisioning.\nThe time to start the fade animation, in seconds."]
    pub fn start_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `xy` after provisioning.\n"]
    pub fn xy(&self) -> terrars::ListRef<TranscoderJob_Config_Overlays_Animations_AnimationFade_XyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xy", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_Overlays_AnimationsDynamic {
    animation_fade: std
    ::option
    ::Option<terrars::DynamicBlock<TranscoderJob_Config_Overlays_Animations_AnimationFade>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Overlays_Animations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    animation_fade: std::option::Option<std::vec::Vec<TranscoderJob_Config_Overlays_Animations_AnimationFade>>,
    dynamic: TranscoderJob_Config_Overlays_AnimationsDynamic,
}

impl TranscoderJob_Config_Overlays_Animations {
    #[doc = "Set the field `animation_fade`.\n"]
    pub fn set_animation_fade(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Overlays_Animations_AnimationFade>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.animation_fade = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.animation_fade = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Overlays_Animations {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Overlays_Animations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Overlays_Animations {}

impl BuildTranscoderJob_Config_Overlays_Animations {
    pub fn build(self) -> TranscoderJob_Config_Overlays_Animations {
        TranscoderJob_Config_Overlays_Animations {
            animation_fade: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Overlays_AnimationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Overlays_AnimationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_Overlays_AnimationsRef {
        TranscoderJob_Config_Overlays_AnimationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Overlays_AnimationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `animation_fade` after provisioning.\n"]
    pub fn animation_fade(&self) -> terrars::ListRef<TranscoderJob_Config_Overlays_Animations_AnimationFadeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.animation_fade", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Overlays_Image {
    uri: terrars::PrimField<std::string::String>,
}

impl TranscoderJob_Config_Overlays_Image { }

impl terrars::ToListMappable for TranscoderJob_Config_Overlays_Image {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Overlays_Image>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Overlays_Image {
    #[doc = "URI of the image in Cloud Storage. For example, gs://bucket/inputs/image.png."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildTranscoderJob_Config_Overlays_Image {
    pub fn build(self) -> TranscoderJob_Config_Overlays_Image {
        TranscoderJob_Config_Overlays_Image { uri: self.uri }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_Overlays_ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_Overlays_ImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_Overlays_ImageRef {
        TranscoderJob_Config_Overlays_ImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_Overlays_ImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI of the image in Cloud Storage. For example, gs://bucket/inputs/image.png."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_Config_OverlaysDynamic {
    animations: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Overlays_Animations>>,
    image: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Overlays_Image>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_Overlays {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    animations: std::option::Option<std::vec::Vec<TranscoderJob_Config_Overlays_Animations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<TranscoderJob_Config_Overlays_Image>>,
    dynamic: TranscoderJob_Config_OverlaysDynamic,
}

impl TranscoderJob_Config_Overlays {
    #[doc = "Set the field `animations`.\n"]
    pub fn set_animations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Overlays_Animations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.animations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.animations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Overlays_Image>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_Overlays {
    type O = terrars::BlockAssignable<TranscoderJob_Config_Overlays>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_Overlays {}

impl BuildTranscoderJob_Config_Overlays {
    pub fn build(self) -> TranscoderJob_Config_Overlays {
        TranscoderJob_Config_Overlays {
            animations: core::default::Default::default(),
            image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_OverlaysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_OverlaysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_OverlaysRef {
        TranscoderJob_Config_OverlaysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_OverlaysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `animations` after provisioning.\n"]
    pub fn animations(&self) -> terrars::ListRef<TranscoderJob_Config_Overlays_AnimationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.animations", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<TranscoderJob_Config_Overlays_ImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Config_PubsubDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Config_PubsubDestination {
    #[doc =
        "Set the field `topic`.\nThe name of the Pub/Sub topic to publish job completion notification to. For example: projects/{project}/topics/{topic}."]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config_PubsubDestination {
    type O = terrars::BlockAssignable<TranscoderJob_Config_PubsubDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config_PubsubDestination {}

impl BuildTranscoderJob_Config_PubsubDestination {
    pub fn build(self) -> TranscoderJob_Config_PubsubDestination {
        TranscoderJob_Config_PubsubDestination { topic: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_Config_PubsubDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_Config_PubsubDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_Config_PubsubDestinationRef {
        TranscoderJob_Config_PubsubDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_Config_PubsubDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nThe name of the Pub/Sub topic to publish job completion notification to. For example: projects/{project}/topics/{topic}."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscoderJob_ConfigDynamic {
    ad_breaks: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_AdBreaks>>,
    edit_list: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_EditList>>,
    elementary_streams: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_ElementaryStreams>>,
    encryptions: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Encryptions>>,
    inputs: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Inputs>>,
    manifests: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Manifests>>,
    mux_streams: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_MuxStreams>>,
    output: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Output>>,
    overlays: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_Overlays>>,
    pubsub_destination: std::option::Option<terrars::DynamicBlock<TranscoderJob_Config_PubsubDestination>>,
}

#[derive(Serialize)]
pub struct TranscoderJob_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ad_breaks: std::option::Option<std::vec::Vec<TranscoderJob_Config_AdBreaks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edit_list: std::option::Option<std::vec::Vec<TranscoderJob_Config_EditList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elementary_streams: std::option::Option<std::vec::Vec<TranscoderJob_Config_ElementaryStreams>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryptions: std::option::Option<std::vec::Vec<TranscoderJob_Config_Encryptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inputs: std::option::Option<std::vec::Vec<TranscoderJob_Config_Inputs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifests: std::option::Option<std::vec::Vec<TranscoderJob_Config_Manifests>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mux_streams: std::option::Option<std::vec::Vec<TranscoderJob_Config_MuxStreams>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std::option::Option<std::vec::Vec<TranscoderJob_Config_Output>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overlays: std::option::Option<std::vec::Vec<TranscoderJob_Config_Overlays>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_destination: std::option::Option<std::vec::Vec<TranscoderJob_Config_PubsubDestination>>,
    dynamic: TranscoderJob_ConfigDynamic,
}

impl TranscoderJob_Config {
    #[doc = "Set the field `ad_breaks`.\n"]
    pub fn set_ad_breaks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_AdBreaks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ad_breaks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ad_breaks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `edit_list`.\n"]
    pub fn set_edit_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_EditList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.edit_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.edit_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elementary_streams`.\n"]
    pub fn set_elementary_streams(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_ElementaryStreams>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elementary_streams = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elementary_streams = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryptions`.\n"]
    pub fn set_encryptions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Encryptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryptions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryptions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inputs`.\n"]
    pub fn set_inputs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Inputs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inputs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `manifests`.\n"]
    pub fn set_manifests(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Manifests>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.manifests = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.manifests = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mux_streams`.\n"]
    pub fn set_mux_streams(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_MuxStreams>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mux_streams = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mux_streams = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Output>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `overlays`.\n"]
    pub fn set_overlays(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_Overlays>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.overlays = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.overlays = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pubsub_destination`.\n"]
    pub fn set_pubsub_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscoderJob_Config_PubsubDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TranscoderJob_Config {
    type O = terrars::BlockAssignable<TranscoderJob_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Config {}

impl BuildTranscoderJob_Config {
    pub fn build(self) -> TranscoderJob_Config {
        TranscoderJob_Config {
            ad_breaks: core::default::Default::default(),
            edit_list: core::default::Default::default(),
            elementary_streams: core::default::Default::default(),
            encryptions: core::default::Default::default(),
            inputs: core::default::Default::default(),
            manifests: core::default::Default::default(),
            mux_streams: core::default::Default::default(),
            output: core::default::Default::default(),
            overlays: core::default::Default::default(),
            pubsub_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_ConfigRef {
        TranscoderJob_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ad_breaks` after provisioning.\n"]
    pub fn ad_breaks(&self) -> terrars::ListRef<TranscoderJob_Config_AdBreaksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ad_breaks", self.base))
    }

    #[doc = "Get a reference to the value of field `edit_list` after provisioning.\n"]
    pub fn edit_list(&self) -> terrars::ListRef<TranscoderJob_Config_EditListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edit_list", self.base))
    }

    #[doc = "Get a reference to the value of field `elementary_streams` after provisioning.\n"]
    pub fn elementary_streams(&self) -> terrars::ListRef<TranscoderJob_Config_ElementaryStreamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elementary_streams", self.base))
    }

    #[doc = "Get a reference to the value of field `encryptions` after provisioning.\n"]
    pub fn encryptions(&self) -> terrars::ListRef<TranscoderJob_Config_EncryptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryptions", self.base))
    }

    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> terrars::ListRef<TranscoderJob_Config_InputsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inputs", self.base))
    }

    #[doc = "Get a reference to the value of field `manifests` after provisioning.\n"]
    pub fn manifests(&self) -> terrars::ListRef<TranscoderJob_Config_ManifestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manifests", self.base))
    }

    #[doc = "Get a reference to the value of field `mux_streams` after provisioning.\n"]
    pub fn mux_streams(&self) -> terrars::ListRef<TranscoderJob_Config_MuxStreamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mux_streams", self.base))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> terrars::ListRef<TranscoderJob_Config_OutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }

    #[doc = "Get a reference to the value of field `overlays` after provisioning.\n"]
    pub fn overlays(&self) -> terrars::ListRef<TranscoderJob_Config_OverlaysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overlays", self.base))
    }

    #[doc = "Get a reference to the value of field `pubsub_destination` after provisioning.\n"]
    pub fn pubsub_destination(&self) -> terrars::ListRef<TranscoderJob_Config_PubsubDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscoderJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscoderJob_Timeouts {
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

impl terrars::ToListMappable for TranscoderJob_Timeouts {
    type O = terrars::BlockAssignable<TranscoderJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscoderJob_Timeouts {}

impl BuildTranscoderJob_Timeouts {
    pub fn build(self) -> TranscoderJob_Timeouts {
        TranscoderJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TranscoderJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscoderJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscoderJob_TimeoutsRef {
        TranscoderJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscoderJob_TimeoutsRef {
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
