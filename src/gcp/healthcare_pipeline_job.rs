#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct HealthcarePipelineJobData_ {
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
    dataset: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_lineage: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backfill_pipeline_job: std::option::Option<std::vec::Vec<HealthcarePipelineJob_BackfillPipelineJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_pipeline_job: std::option::Option<std::vec::Vec<HealthcarePipelineJob_MappingPipelineJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reconciliation_pipeline_job: std::option::Option<std::vec::Vec<HealthcarePipelineJob_ReconciliationPipelineJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<HealthcarePipelineJob_Timeouts>,
    dynamic: HealthcarePipelineJobDynamic,
}

struct HealthcarePipelineJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<HealthcarePipelineJobData_>,
}

#[derive(Clone)]
pub struct HealthcarePipelineJob(std::rc::Rc<HealthcarePipelineJob_>);

impl HealthcarePipelineJob {
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

    #[doc = "Set the field `disable_lineage`.\nIf true, disables writing lineage for the pipeline."]
    pub fn set_disable_lineage(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_lineage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-supplied key-value pairs used to organize Pipeline Jobs.\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, and must conform to the following PCRE regular expression:\n[\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\nLabel values are optional, must be between 1 and 63 characters long, have a\nUTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE\nregular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\nNo more than 64 labels can be associated with a given pipeline.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backfill_pipeline_job`.\n"]
    pub fn set_backfill_pipeline_job(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcarePipelineJob_BackfillPipelineJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backfill_pipeline_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backfill_pipeline_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mapping_pipeline_job`.\n"]
    pub fn set_mapping_pipeline_job(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mapping_pipeline_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mapping_pipeline_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reconciliation_pipeline_job`.\n"]
    pub fn set_reconciliation_pipeline_job(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcarePipelineJob_ReconciliationPipelineJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reconciliation_pipeline_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reconciliation_pipeline_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<HealthcarePipelineJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nHealthcare Dataset under which the Pipeline Job is to run"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_lineage` after provisioning.\nIf true, disables writing lineage for the pipeline."]
    pub fn disable_lineage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_lineage", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize Pipeline Jobs.\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, and must conform to the following PCRE regular expression:\n[\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\nLabel values are optional, must be between 1 and 63 characters long, have a\nUTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE\nregular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\nNo more than 64 labels can be associated with a given pipeline.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation where the Pipeline Job is to run"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the pipeline job. This field is user-assigned."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_pipeline_job` after provisioning.\n"]
    pub fn backfill_pipeline_job(&self) -> terrars::ListRef<HealthcarePipelineJob_BackfillPipelineJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_pipeline_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mapping_pipeline_job` after provisioning.\n"]
    pub fn mapping_pipeline_job(&self) -> terrars::ListRef<HealthcarePipelineJob_MappingPipelineJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_pipeline_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reconciliation_pipeline_job` after provisioning.\n"]
    pub fn reconciliation_pipeline_job(&self) -> terrars::ListRef<HealthcarePipelineJob_ReconciliationPipelineJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reconciliation_pipeline_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcarePipelineJob_TimeoutsRef {
        HealthcarePipelineJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for HealthcarePipelineJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for HealthcarePipelineJob { }

impl terrars::ToListMappable for HealthcarePipelineJob {
    type O = terrars::ListRef<HealthcarePipelineJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for HealthcarePipelineJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_healthcare_pipeline_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildHealthcarePipelineJob {
    pub tf_id: std::string::String,
    #[doc = "Healthcare Dataset under which the Pipeline Job is to run"]
    pub dataset: terrars::PrimField<std::string::String>,
    #[doc = "Location where the Pipeline Job is to run"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Specifies the name of the pipeline job. This field is user-assigned."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildHealthcarePipelineJob {
    pub fn build(self, stack: &mut terrars::Stack) -> HealthcarePipelineJob {
        let out = HealthcarePipelineJob(std::rc::Rc::new(HealthcarePipelineJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(HealthcarePipelineJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dataset: self.dataset,
                deletion_policy: core::default::Default::default(),
                disable_lineage: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                backfill_pipeline_job: core::default::Default::default(),
                mapping_pipeline_job: core::default::Default::default(),
                reconciliation_pipeline_job: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct HealthcarePipelineJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl HealthcarePipelineJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nHealthcare Dataset under which the Pipeline Job is to run"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_lineage` after provisioning.\nIf true, disables writing lineage for the pipeline."]
    pub fn disable_lineage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_lineage", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize Pipeline Jobs.\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, and must conform to the following PCRE regular expression:\n[\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\nLabel values are optional, must be between 1 and 63 characters long, have a\nUTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE\nregular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\nNo more than 64 labels can be associated with a given pipeline.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation where the Pipeline Job is to run"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the pipeline job. This field is user-assigned."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_pipeline_job` after provisioning.\n"]
    pub fn backfill_pipeline_job(&self) -> terrars::ListRef<HealthcarePipelineJob_BackfillPipelineJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_pipeline_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mapping_pipeline_job` after provisioning.\n"]
    pub fn mapping_pipeline_job(&self) -> terrars::ListRef<HealthcarePipelineJob_MappingPipelineJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_pipeline_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reconciliation_pipeline_job` after provisioning.\n"]
    pub fn reconciliation_pipeline_job(&self) -> terrars::ListRef<HealthcarePipelineJob_ReconciliationPipelineJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reconciliation_pipeline_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcarePipelineJob_TimeoutsRef {
        HealthcarePipelineJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct HealthcarePipelineJobDynamic {
    backfill_pipeline_job: std::option::Option<terrars::DynamicBlock<HealthcarePipelineJob_BackfillPipelineJob>>,
    mapping_pipeline_job: std::option::Option<terrars::DynamicBlock<HealthcarePipelineJob_MappingPipelineJob>>,
    reconciliation_pipeline_job: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcarePipelineJob_ReconciliationPipelineJob>>,
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_BackfillPipelineJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_pipeline_job: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HealthcarePipelineJob_BackfillPipelineJob {
    #[doc =
        "Set the field `mapping_pipeline_job`.\nSpecifies the mapping pipeline job to backfill, the name format\nshould follow: projects/{projectId}/locations/{locationId}/datasets/{datasetId}/pipelineJobs/{pipelineJobId}."]
    pub fn set_mapping_pipeline_job(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mapping_pipeline_job = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HealthcarePipelineJob_BackfillPipelineJob {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_BackfillPipelineJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_BackfillPipelineJob {}

impl BuildHealthcarePipelineJob_BackfillPipelineJob {
    pub fn build(self) -> HealthcarePipelineJob_BackfillPipelineJob {
        HealthcarePipelineJob_BackfillPipelineJob { mapping_pipeline_job: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_BackfillPipelineJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_BackfillPipelineJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcarePipelineJob_BackfillPipelineJobRef {
        HealthcarePipelineJob_BackfillPipelineJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_BackfillPipelineJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mapping_pipeline_job` after provisioning.\nSpecifies the mapping pipeline job to backfill, the name format\nshould follow: projects/{projectId}/locations/{locationId}/datasets/{datasetId}/pipelineJobs/{pipelineJobId}."]
    pub fn mapping_pipeline_job(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mapping_pipeline_job", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    fhir_store: terrars::PrimField<std::string::String>,
}

impl HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource {
    #[doc = "Set the field `description`.\nDescribes the streaming FHIR data source."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource {
    #[doc =
        "The path to the FHIR store in the format projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}."]
    pub fhir_store: terrars::PrimField<std::string::String>,
}

impl BuildHealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource {
    pub fn build(self) -> HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource {
        HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource {
            description: core::default::Default::default(),
            fhir_store: self.fhir_store,
        }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSourceRef {
        HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescribes the streaming FHIR data source."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fhir_store` after provisioning.\nThe path to the FHIR store in the format projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}."]
    pub fn fhir_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fhir_store", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource {
    import_uri_prefix: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource { }

impl terrars::ToListMappable for HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource {
    #[doc =
        "Directory path where all the Whistle files are located.\nExample: gs://{bucket-id}/{path/to/import-root/dir}"]
    pub import_uri_prefix: terrars::PrimField<std::string::String>,
    #[doc =
        "Main configuration file which has the entrypoint or the root function.\nExample: gs://{bucket-id}/{path/to/import-root/dir}/entrypoint-file-name.wstl."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildHealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource {
    pub fn build(self) -> HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource {
        HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource {
            import_uri_prefix: self.import_uri_prefix,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSourceRef {
        HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `import_uri_prefix` after provisioning.\nDirectory path where all the Whistle files are located.\nExample: gs://{bucket-id}/{path/to/import-root/dir}"]
    pub fn import_uri_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_uri_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nMain configuration file which has the entrypoint or the root function.\nExample: gs://{bucket-id}/{path/to/import-root/dir}/entrypoint-file-name.wstl."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcarePipelineJob_MappingPipelineJob_MappingConfigDynamic {
    whistle_config_source: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource>>,
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_MappingPipelineJob_MappingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whistle_config_source: std
    ::option
    ::Option<std::vec::Vec<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource>>,
    dynamic: HealthcarePipelineJob_MappingPipelineJob_MappingConfigDynamic,
}

impl HealthcarePipelineJob_MappingPipelineJob_MappingConfig {
    #[doc = "Set the field `description`.\nDescribes the mapping configuration."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `whistle_config_source`.\n"]
    pub fn set_whistle_config_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.whistle_config_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.whistle_config_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HealthcarePipelineJob_MappingPipelineJob_MappingConfig {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob_MappingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_MappingPipelineJob_MappingConfig {}

impl BuildHealthcarePipelineJob_MappingPipelineJob_MappingConfig {
    pub fn build(self) -> HealthcarePipelineJob_MappingPipelineJob_MappingConfig {
        HealthcarePipelineJob_MappingPipelineJob_MappingConfig {
            description: core::default::Default::default(),
            whistle_config_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_MappingPipelineJob_MappingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_MappingPipelineJob_MappingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcarePipelineJob_MappingPipelineJob_MappingConfigRef {
        HealthcarePipelineJob_MappingPipelineJob_MappingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_MappingPipelineJob_MappingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescribes the mapping configuration."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `whistle_config_source` after provisioning.\n"]
    pub fn whistle_config_source(
        &self,
    ) -> terrars::ListRef<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.whistle_config_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcarePipelineJob_MappingPipelineJobDynamic {
    fhir_streaming_source: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource>>,
    mapping_config: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcarePipelineJob_MappingPipelineJob_MappingConfig>>,
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_MappingPipelineJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fhir_store_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reconciliation_destination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fhir_streaming_source: std
    ::option
    ::Option<std::vec::Vec<HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_config: std::option::Option<std::vec::Vec<HealthcarePipelineJob_MappingPipelineJob_MappingConfig>>,
    dynamic: HealthcarePipelineJob_MappingPipelineJobDynamic,
}

impl HealthcarePipelineJob_MappingPipelineJob {
    #[doc =
        "Set the field `fhir_store_destination`.\nIf set, the mapping pipeline will write snapshots to this\nFHIR store without assigning stable IDs. You must\ngrant your pipeline project's Cloud Healthcare Service\nAgent serviceaccount healthcare.fhirResources.executeBundle\nand healthcare.fhirResources.create permissions on the\ndestination store. The destination store must set\n[disableReferentialIntegrity][FhirStore.disable_referential_integrity]\nto true. The destination store must use FHIR version R4.\nFormat: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{fhirStoreID}."]
    pub fn set_fhir_store_destination(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.fhir_store_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reconciliation_destination`.\nIf set to true, a mapping pipeline will send output snapshots\nto the reconciliation pipeline in its dataset. A reconciliation\npipeline must exist in this dataset before a mapping pipeline\nwith a reconciliation destination can be created."]
    pub fn set_reconciliation_destination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reconciliation_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fhir_streaming_source`.\n"]
    pub fn set_fhir_streaming_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fhir_streaming_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fhir_streaming_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mapping_config`.\n"]
    pub fn set_mapping_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob_MappingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mapping_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HealthcarePipelineJob_MappingPipelineJob {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_MappingPipelineJob {}

impl BuildHealthcarePipelineJob_MappingPipelineJob {
    pub fn build(self) -> HealthcarePipelineJob_MappingPipelineJob {
        HealthcarePipelineJob_MappingPipelineJob {
            fhir_store_destination: core::default::Default::default(),
            reconciliation_destination: core::default::Default::default(),
            fhir_streaming_source: core::default::Default::default(),
            mapping_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_MappingPipelineJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_MappingPipelineJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcarePipelineJob_MappingPipelineJobRef {
        HealthcarePipelineJob_MappingPipelineJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_MappingPipelineJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fhir_store_destination` after provisioning.\nIf set, the mapping pipeline will write snapshots to this\nFHIR store without assigning stable IDs. You must\ngrant your pipeline project's Cloud Healthcare Service\nAgent serviceaccount healthcare.fhirResources.executeBundle\nand healthcare.fhirResources.create permissions on the\ndestination store. The destination store must set\n[disableReferentialIntegrity][FhirStore.disable_referential_integrity]\nto true. The destination store must use FHIR version R4.\nFormat: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{fhirStoreID}."]
    pub fn fhir_store_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fhir_store_destination", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reconciliation_destination` after provisioning.\nIf set to true, a mapping pipeline will send output snapshots\nto the reconciliation pipeline in its dataset. A reconciliation\npipeline must exist in this dataset before a mapping pipeline\nwith a reconciliation destination can be created."]
    pub fn reconciliation_destination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciliation_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `fhir_streaming_source` after provisioning.\n"]
    pub fn fhir_streaming_source(
        &self,
    ) -> terrars::ListRef<HealthcarePipelineJob_MappingPipelineJob_FhirStreamingSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fhir_streaming_source", self.base))
    }

    #[doc = "Get a reference to the value of field `mapping_config` after provisioning.\n"]
    pub fn mapping_config(&self) -> terrars::ListRef<HealthcarePipelineJob_MappingPipelineJob_MappingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigDynamic {
    whistle_config_source: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource>>,
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whistle_config_source: std
    ::option
    ::Option<std::vec::Vec<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource>>,
    dynamic: HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigDynamic,
}

impl HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig {
    #[doc = "Set the field `description`.\nDescribes the mapping configuration."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `whistle_config_source`.\n"]
    pub fn set_whistle_config_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.whistle_config_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.whistle_config_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig {}

impl BuildHealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig {
    pub fn build(self) -> HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig {
        HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig {
            description: core::default::Default::default(),
            whistle_config_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigRef {
        HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescribes the mapping configuration."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `whistle_config_source` after provisioning.\n"]
    pub fn whistle_config_source(
        &self,
    ) -> terrars::ListRef<HealthcarePipelineJob_MappingPipelineJob_MappingConfig_WhistleConfigSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.whistle_config_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcarePipelineJob_ReconciliationPipelineJobDynamic {
    merge_config: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig>>,
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_ReconciliationPipelineJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fhir_store_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    matching_uri_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    merge_config: std::option::Option<std::vec::Vec<HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig>>,
    dynamic: HealthcarePipelineJob_ReconciliationPipelineJobDynamic,
}

impl HealthcarePipelineJob_ReconciliationPipelineJob {
    #[doc =
        "Set the field `fhir_store_destination`.\nThe harmonized FHIR store to write harmonized FHIR resources to,\nin the format of: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{id}"]
    pub fn set_fhir_store_destination(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.fhir_store_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `merge_config`.\n"]
    pub fn set_merge_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.merge_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.merge_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HealthcarePipelineJob_ReconciliationPipelineJob {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_ReconciliationPipelineJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_ReconciliationPipelineJob {
    #[doc =
        "Specifies the top level directory of the matching configs used\nin all mapping pipelines, which extract properties for resources\nto be matched on.\nExample: gs://{bucket-id}/{path/to/matching/configs}"]
    pub matching_uri_prefix: terrars::PrimField<std::string::String>,
}

impl BuildHealthcarePipelineJob_ReconciliationPipelineJob {
    pub fn build(self) -> HealthcarePipelineJob_ReconciliationPipelineJob {
        HealthcarePipelineJob_ReconciliationPipelineJob {
            fhir_store_destination: core::default::Default::default(),
            matching_uri_prefix: self.matching_uri_prefix,
            merge_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_ReconciliationPipelineJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_ReconciliationPipelineJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcarePipelineJob_ReconciliationPipelineJobRef {
        HealthcarePipelineJob_ReconciliationPipelineJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_ReconciliationPipelineJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fhir_store_destination` after provisioning.\nThe harmonized FHIR store to write harmonized FHIR resources to,\nin the format of: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{id}"]
    pub fn fhir_store_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fhir_store_destination", self.base))
    }

    #[doc =
        "Get a reference to the value of field `matching_uri_prefix` after provisioning.\nSpecifies the top level directory of the matching configs used\nin all mapping pipelines, which extract properties for resources\nto be matched on.\nExample: gs://{bucket-id}/{path/to/matching/configs}"]
    pub fn matching_uri_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matching_uri_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `merge_config` after provisioning.\n"]
    pub fn merge_config(&self) -> terrars::ListRef<HealthcarePipelineJob_ReconciliationPipelineJob_MergeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.merge_config", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcarePipelineJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HealthcarePipelineJob_Timeouts {
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

impl terrars::ToListMappable for HealthcarePipelineJob_Timeouts {
    type O = terrars::BlockAssignable<HealthcarePipelineJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcarePipelineJob_Timeouts {}

impl BuildHealthcarePipelineJob_Timeouts {
    pub fn build(self) -> HealthcarePipelineJob_Timeouts {
        HealthcarePipelineJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcarePipelineJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcarePipelineJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcarePipelineJob_TimeoutsRef {
        HealthcarePipelineJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcarePipelineJob_TimeoutsRef {
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
