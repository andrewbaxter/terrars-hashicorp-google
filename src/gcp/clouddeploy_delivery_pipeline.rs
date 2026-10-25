#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddeployDeliveryPipelineData_ {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serial_pipeline: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddeployDeliveryPipeline_Timeouts>,
    dynamic: ClouddeployDeliveryPipelineDynamic,
}

struct ClouddeployDeliveryPipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddeployDeliveryPipelineData_>,
}

#[derive(Clone)]
pub struct ClouddeployDeliveryPipeline(std::rc::Rc<ClouddeployDeliveryPipeline_>);

impl ClouddeployDeliveryPipeline {
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
        "Set the field `annotations`.\nUser annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the `DeliveryPipeline`. Max length is 255 characters."]
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
        "Set the field `labels`.\nLabels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suspended`.\nWhen suspended, no new releases or rollouts can be created, but in-progress ones will complete."]
    pub fn set_suspended(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serial_pipeline`.\n"]
    pub fn set_serial_pipeline(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serial_pipeline = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.serial_pipeline = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddeployDeliveryPipeline_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nOutput only. Information around the state of the Delivery Pipeline."]
    pub fn condition(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the pipeline was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the `DeliveryPipeline`. Max length is 255 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the `DeliveryPipeline`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, no new releases or rollouts can be created, but in-progress ones will complete."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `DeliveryPipeline`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the pipeline was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial_pipeline` after provisioning.\n"]
    pub fn serial_pipeline(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipelineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serial_pipeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeliveryPipeline_TimeoutsRef {
        ClouddeployDeliveryPipeline_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ClouddeployDeliveryPipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClouddeployDeliveryPipeline { }

impl terrars::ToListMappable for ClouddeployDeliveryPipeline {
    type O = terrars::ListRef<ClouddeployDeliveryPipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClouddeployDeliveryPipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_clouddeploy_delivery_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClouddeployDeliveryPipeline {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the `DeliveryPipeline`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddeployDeliveryPipeline {
        let out = ClouddeployDeliveryPipeline(std::rc::Rc::new(ClouddeployDeliveryPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddeployDeliveryPipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                suspended: core::default::Default::default(),
                serial_pipeline: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClouddeployDeliveryPipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClouddeployDeliveryPipelineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nOutput only. Information around the state of the Delivery Pipeline."]
    pub fn condition(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the pipeline was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the `DeliveryPipeline`. Max length is 255 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the `DeliveryPipeline`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, no new releases or rollouts can be created, but in-progress ones will complete."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `DeliveryPipeline`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the pipeline was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial_pipeline` after provisioning.\n"]
    pub fn serial_pipeline(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipelineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serial_pipeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeliveryPipeline_TimeoutsRef {
        ClouddeployDeliveryPipeline_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineDynamic {
    serial_pipeline: std::option::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_Condition_PipelineReadyCondition {}

impl BuildClouddeployDeliveryPipeline_Condition_PipelineReadyCondition {
    pub fn build(self) -> ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition {
        ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition {
            status: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_Condition_PipelineReadyConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_Condition_PipelineReadyConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_Condition_PipelineReadyConditionRef {
        ClouddeployDeliveryPipeline_Condition_PipelineReadyConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_Condition_PipelineReadyConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    missing_targets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition {
    #[doc = "Set the field `missing_targets`.\n"]
    pub fn set_missing_targets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.missing_targets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_Condition_TargetsPresentCondition {}

impl BuildClouddeployDeliveryPipeline_Condition_TargetsPresentCondition {
    pub fn build(self) -> ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition {
        ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition {
            missing_targets: core::default::Default::default(),
            status: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_Condition_TargetsPresentConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_Condition_TargetsPresentConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_Condition_TargetsPresentConditionRef {
        ClouddeployDeliveryPipeline_Condition_TargetsPresentConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_Condition_TargetsPresentConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `missing_targets` after provisioning.\n"]
    pub fn missing_targets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.missing_targets", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<bool>>,
}

impl ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition {
    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_Condition_TargetsTypeCondition {}

impl BuildClouddeployDeliveryPipeline_Condition_TargetsTypeCondition {
    pub fn build(self) -> ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition {
        ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition {
            error_details: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_Condition_TargetsTypeConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_Condition_TargetsTypeConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_Condition_TargetsTypeConditionRef {
        ClouddeployDeliveryPipeline_Condition_TargetsTypeConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_Condition_TargetsTypeConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_details", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_ready_condition: std
    ::option
    ::Option<terrars::ListField<ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets_present_condition: std
    ::option
    ::Option<terrars::ListField<ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets_type_condition: std
    ::option
    ::Option<terrars::ListField<ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition>>,
}

impl ClouddeployDeliveryPipeline_Condition {
    #[doc = "Set the field `pipeline_ready_condition`.\n"]
    pub fn set_pipeline_ready_condition(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ClouddeployDeliveryPipeline_Condition_PipelineReadyCondition>>,
    ) -> Self {
        self.pipeline_ready_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `targets_present_condition`.\n"]
    pub fn set_targets_present_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ClouddeployDeliveryPipeline_Condition_TargetsPresentCondition>>,
    ) -> Self {
        self.targets_present_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `targets_type_condition`.\n"]
    pub fn set_targets_type_condition(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ClouddeployDeliveryPipeline_Condition_TargetsTypeCondition>>,
    ) -> Self {
        self.targets_type_condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_Condition {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_Condition {}

impl BuildClouddeployDeliveryPipeline_Condition {
    pub fn build(self) -> ClouddeployDeliveryPipeline_Condition {
        ClouddeployDeliveryPipeline_Condition {
            pipeline_ready_condition: core::default::Default::default(),
            targets_present_condition: core::default::Default::default(),
            targets_type_condition: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeliveryPipeline_ConditionRef {
        ClouddeployDeliveryPipeline_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pipeline_ready_condition` after provisioning.\n"]
    pub fn pipeline_ready_condition(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_Condition_PipelineReadyConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pipeline_ready_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `targets_present_condition` after provisioning.\n"]
    pub fn targets_present_condition(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_Condition_TargetsPresentConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets_present_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `targets_type_condition` after provisioning.\n"]
    pub fn targets_type_condition(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_Condition_TargetsTypeConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets_type_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_target_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    values: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters {
    #[doc =
        "Set the field `match_target_labels`.\nOptional. Deploy parameters are applied to targets with match labels. If unspecified, deploy parameters are applied to all targets (including child targets of a multi-target)."]
    pub fn set_match_target_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_target_labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters {
    #[doc = "Required. Values are deploy parameters in key-value pairs."]
    pub values: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters {
            match_target_labels: core::default::Default::default(),
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParametersRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `match_target_labels` after provisioning.\nOptional. Deploy parameters are applied to targets with match labels. If unspecified, deploy parameters are applied to all targets (including child targets of a multi-target)."]
    pub fn match_target_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.match_target_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nRequired. Values are deploy parameters in key-value pairs."]
    pub fn values(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container {
    #[doc =
        "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container {
    #[doc = "Required. Image is the container image to use."]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: self.image,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_ContainerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_ContainerRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_ContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."]
    pub fn env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskDynamic {
    container: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container>,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task {
            container: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksDynamic {
    task: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task>,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks {
    #[doc = "Set the field `frequency`.\nOptional. Frequency of the custom check."]
    pub fn set_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task`.\n"]
    pub fn set_task(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_Task,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.task = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.task = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks {
    #[doc = "Required. Unique identifier for the custom check."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks {
            frequency: core::default::Default::default(),
            id: self.id,
            task: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nOptional. Frequency of the custom check."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the custom check."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `task` after provisioning.\n"]
    pub fn task(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks_TaskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks {
    alert_policies: terrars::ListField<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks {
    #[doc = "Set the field `labels`.\nOptional. Labels to filter the alert policies."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks {
    #[doc =
        "Required. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."]
    pub alert_policies: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Required. Unique identifier for the alert policy check."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks {
            alert_policies: self.alert_policies,
            id: self.id,
            labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecksRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alert_policies` after provisioning.\nRequired. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."]
    pub fn alert_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert_policies", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the alert policy check."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to filter the alert policies."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudDynamic {
    alert_policy_checks: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert_policy_checks: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks,
        >,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud {
    #[doc = "Set the field `alert_policy_checks`.\n"]
    pub fn set_alert_policy_checks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecks,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alert_policy_checks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alert_policy_checks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud {
            alert_policy_checks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert_policy_checks` after provisioning.\n"]
    pub fn alert_policy_checks(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud_AlertPolicyChecksRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert_policy_checks", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisDynamic {
    custom_checks: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks>,
    >,
    google_cloud: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis {
    duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_checks: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis {
    #[doc = "Set the field `custom_checks`.\n"]
    pub fn set_custom_checks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecks,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_checks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_checks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud`.\n"]
    pub fn set_google_cloud(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloud,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis {
    #[doc = "Required. Duration of the analysis."]
    pub duration: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis {
            duration: self.duration,
            custom_checks: core::default::Default::default(),
            google_cloud: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\nRequired. Duration of the analysis."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_checks` after provisioning.\n"]
    pub fn custom_checks(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_CustomChecksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_checks", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud` after provisioning.\n"]
    pub fn google_cloud(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis_GoogleCloudRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_cloud", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy {
    #[doc =
        "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy {
            actions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PostdeployRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PostdeployRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PostdeployRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PostdeployRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PostdeployRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy {
    #[doc =
        "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy {
            actions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PredeployRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PredeployRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PredeployRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PredeployRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PredeployRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksDynamic {
    container: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container>,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_Container,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks {
            container: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigDynamic {
    tasks: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tasks: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig {
    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tasks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tasks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig {
    type O =
        terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig {
            tasks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]
    pub fn tasks(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tasks", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentDynamic {
    analysis: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis>>,
    postdeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy>,
    >,
    predeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy>,
    >,
    verify_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment {
    percentages: terrars::ListField<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postdeploy: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predeploy: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_config: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment {
    #[doc = "Set the field `verify`.\nWhether to run verify tests after each percentage deployment."]
    pub fn set_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postdeploy`.\n"]
    pub fn set_postdeploy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postdeploy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postdeploy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predeploy`.\n"]
    pub fn set_predeploy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predeploy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predeploy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verify_config`.\n"]
    pub fn set_verify_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verify_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verify_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment {
    type O =
        terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment {
    #[doc =
        "Required. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100."]
    pub percentages: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment {
            percentages: self.percentages,
            verify: core::default::Default::default(),
            analysis: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            verify_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percentages` after provisioning.\nRequired. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100."]
    pub fn percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.percentages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `verify` after provisioning.\nWhether to run verify tests after each percentage deployment."]
    pub fn verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]
    pub fn postdeploy(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PostdeployRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]
    pub fn predeploy(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PredeployRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_config` after provisioning.\n"]
    pub fn verify_config(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verify_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsDynamic {
    analysis: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis>>,
    postdeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy>,
    >,
    predeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy>,
    >,
    verify_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs {
    percentage: terrars::PrimField<f64>,
    phase_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postdeploy: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predeploy: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_config: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs {
    #[doc =
        "Set the field `profiles`.\nSkaffold profiles to use when rendering the manifest for this phase. These are in addition to the profiles list specified in the `DeliveryPipeline` stage."]
    pub fn set_profiles(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.profiles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verify`.\nWhether to run verify tests after the deployment."]
    pub fn set_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postdeploy`.\n"]
    pub fn set_postdeploy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Postdeploy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postdeploy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postdeploy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predeploy`.\n"]
    pub fn set_predeploy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_Predeploy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predeploy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predeploy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verify_config`.\n"]
    pub fn set_verify_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verify_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verify_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs {
    #[doc = "Required. Percentage deployment for the phase."]
    pub percentage: terrars::PrimField<f64>,
    #[doc =
        "Required. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`."]
    pub phase_id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs {
            percentage: self.percentage,
            phase_id: self.phase_id,
            profiles: core::default::Default::default(),
            verify: core::default::Default::default(),
            analysis: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            verify_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nRequired. Percentage deployment for the phase."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phase_id` after provisioning.\nRequired. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`."]
    pub fn phase_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `profiles` after provisioning.\nSkaffold profiles to use when rendering the manifest for this phase. These are in addition to the profiles list specified in the `DeliveryPipeline` stage."]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }

    #[doc =
        "Get a reference to the value of field `verify` after provisioning.\nWhether to run verify tests after the deployment."]
    pub fn verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]
    pub fn postdeploy(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PostdeployRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]
    pub fn predeploy(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment_PredeployRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_config` after provisioning.\n"]
    pub fn verify_config(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verify_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentDynamic {
    phase_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs>,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment {
    #[doc = "Set the field `phase_configs`.\n"]
    pub fn set_phase_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phase_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phase_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment {
            phase_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phase_configs` after provisioning.\n"]
    pub fn phase_configs(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment_PhaseConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phase_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_traffic_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_revision_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prior_revision_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stable_revision_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun {
    #[doc =
        "Set the field `automatic_traffic_control`.\nWhether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments."]
    pub fn set_automatic_traffic_control(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_traffic_control = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `canary_revision_tags`.\nOptional. A list of tags that are added to the canary revision while the canary phase is in progress."]
    pub fn set_canary_revision_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.canary_revision_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prior_revision_tags`.\nOptional. A list of tags that are added to the prior revision while the canary phase is in progress."]
    pub fn set_prior_revision_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prior_revision_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stable_revision_tags`.\nOptional. A list of tags that are added to the final stable revision when the stable phase is applied."]
    pub fn set_stable_revision_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stable_revision_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun {
            automatic_traffic_control: core::default::Default::default(),
            canary_revision_tags: core::default::Default::default(),
            prior_revision_tags: core::default::Default::default(),
            stable_revision_tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRunRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRunRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRunRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRunRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRunRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `automatic_traffic_control` after provisioning.\nWhether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments."]
    pub fn automatic_traffic_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_traffic_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `canary_revision_tags` after provisioning.\nOptional. A list of tags that are added to the canary revision while the canary phase is in progress."]
    pub fn canary_revision_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_revision_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prior_revision_tags` after provisioning.\nOptional. A list of tags that are added to the prior revision while the canary phase is in progress."]
    pub fn prior_revision_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prior_revision_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stable_revision_tags` after provisioning.\nOptional. A list of tags that are added to the final stable revision when the stable phase is applied."]
    pub fn stable_revision_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stable_revision_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations {
    destination_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_service: std::option::Option<terrars::PrimField<bool>>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations {
    #[doc =
        "Set the field `propagate_service`.\nOptional. Whether to propagate the Kubernetes Service to the route destination clusters. The Service will always be deployed to the Target cluster even if the HTTPRoute is not. This option may be used to facilitiate successful DNS lookup in the route destination clusters. Can only be set to true if destinations are specified."]
    pub fn set_propagate_service(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.propagate_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations {
    #[doc =
        "Required. The clusters where the Gateway API HTTPRoute resource will be deployed to. Valid entries include the associated entities IDs configured in the Target resource and \"@self\" to include the Target cluster."]
    pub destination_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations {
            destination_ids: self.destination_ids,
            propagate_service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinationsRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destination_ids` after provisioning.\nRequired. The clusters where the Gateway API HTTPRoute resource will be deployed to. Valid entries include the associated entities IDs configured in the Target resource and \"@self\" to include the Target cluster."]
    pub fn destination_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `propagate_service` after provisioning.\nOptional. Whether to propagate the Kubernetes Service to the route destination clusters. The Service will always be deployed to the Target cluster even if the HTTPRoute is not. This option may be used to facilitiate successful DNS lookup in the route destination clusters. Can only be set to true if destinations are specified."]
    pub fn propagate_service(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshDynamic {
    route_destinations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh {
    deployment: terrars::PrimField<std::string::String>,
    http_route: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_selector_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_update_wait_time: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stable_cutback_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_destinations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations,
        >,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh {
    #[doc =
        "Set the field `pod_selector_label`.\nOptional. The label to use when selecting Pods for the Deployment and Service resources. This label must already be present in both resources."]
    pub fn set_pod_selector_label(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pod_selector_label = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `route_update_wait_time`.\nOptional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time."]
    pub fn set_route_update_wait_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.route_update_wait_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stable_cutback_duration`.\nOptional. The amount of time to migrate traffic back from the canary Service to the original Service during the stable phase deployment. If specified, must be between 15s and 3600s. If unspecified, there is no cutback time."]
    pub fn set_stable_cutback_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.stable_cutback_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_destinations`.\n"]
    pub fn set_route_destinations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinations,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_destinations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh {
    #[doc =
        "Required. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service."]
    pub deployment: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the Gateway API HTTPRoute."]
    pub http_route: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the Kubernetes Service."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh {
            deployment: self.deployment,
            http_route: self.http_route,
            pod_selector_label: core::default::Default::default(),
            route_update_wait_time: core::default::Default::default(),
            service: self.service,
            stable_cutback_duration: core::default::Default::default(),
            route_destinations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deployment` after provisioning.\nRequired. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service."]
    pub fn deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_route` after provisioning.\nRequired. Name of the Gateway API HTTPRoute."]
    pub fn http_route(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_route", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_selector_label` after provisioning.\nOptional. The label to use when selecting Pods for the Deployment and Service resources. This label must already be present in both resources."]
    pub fn pod_selector_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_selector_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `route_update_wait_time` after provisioning.\nOptional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time."]
    pub fn route_update_wait_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_update_wait_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the Kubernetes Service."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stable_cutback_duration` after provisioning.\nOptional. The amount of time to migrate traffic back from the canary Service to the original Service during the stable phase deployment. If specified, must be between 15s and 3600s. If unspecified, there is no cutback time."]
    pub fn stable_cutback_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stable_cutback_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `route_destinations` after provisioning.\n"]
    pub fn route_destinations(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh_RouteDestinationsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_destinations", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking {
    deployment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_pod_overprovisioning: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_selector_label: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking {
    #[doc =
        "Set the field `disable_pod_overprovisioning`.\nOptional. Whether to disable Pod overprovisioning. If Pod overprovisioning is disabled then Cloud Deploy will limit the number of total Pods used for the deployment strategy to the number of Pods the Deployment has on the cluster."]
    pub fn set_disable_pod_overprovisioning(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_pod_overprovisioning = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pod_selector_label`.\nOptional. The label to use when selecting Pods for the Deployment resource. This label must already be present in the Deployment."]
    pub fn set_pod_selector_label(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pod_selector_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking {
    #[doc = "Required. Name of the Kubernetes Deployment whose traffic is managed by the specified Service."]
    pub deployment: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the Kubernetes Service."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking {
            deployment: self.deployment,
            disable_pod_overprovisioning: core::default::Default::default(),
            pod_selector_label: core::default::Default::default(),
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworkingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworkingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworkingRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworkingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworkingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deployment` after provisioning.\nRequired. Name of the Kubernetes Deployment whose traffic is managed by the specified Service."]
    pub fn deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_pod_overprovisioning` after provisioning.\nOptional. Whether to disable Pod overprovisioning. If Pod overprovisioning is disabled then Cloud Deploy will limit the number of total Pods used for the deployment strategy to the number of Pods the Deployment has on the cluster."]
    pub fn disable_pod_overprovisioning(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_pod_overprovisioning", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_selector_label` after provisioning.\nOptional. The label to use when selecting Pods for the Deployment resource. This label must already be present in the Deployment."]
    pub fn pod_selector_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_selector_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the Kubernetes Service."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesDynamic {
    gateway_service_mesh: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh,
        >,
    >,
    service_networking: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_service_mesh: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_networking: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking,
        >,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes {
    #[doc = "Set the field `gateway_service_mesh`.\n"]
    pub fn set_gateway_service_mesh(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMesh,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gateway_service_mesh = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gateway_service_mesh = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_networking`.\n"]
    pub fn set_service_networking(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworking,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_networking = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_networking = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes {
            gateway_service_mesh: core::default::Default::default(),
            service_networking: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gateway_service_mesh` after provisioning.\n"]
    pub fn gateway_service_mesh(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_GatewayServiceMeshRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_service_mesh", self.base))
    }

    #[doc = "Get a reference to the value of field `service_networking` after provisioning.\n"]
    pub fn service_networking(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes_ServiceNetworkingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_networking", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigDynamic {
    cloud_run: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun>,
    >,
    kubernetes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_run: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes>,
    >,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig {
    #[doc = "Set the field `cloud_run`.\n"]
    pub fn set_cloud_run(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRun,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_run = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_run = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes`.\n"]
    pub fn set_kubernetes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_Kubernetes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kubernetes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig {
    type O =
        terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig {
            cloud_run: core::default::Default::default(),
            kubernetes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\n"]
    pub fn cloud_run(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_CloudRunRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run", self.base))
    }

    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig_KubernetesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryDynamic {
    canary_deployment: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment>,
    >,
    custom_canary_deployment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment>,
    >,
    runtime_config: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_deployment: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_canary_deployment: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_config: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary {
    #[doc = "Set the field `canary_deployment`.\n"]
    pub fn set_canary_deployment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeployment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.canary_deployment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.canary_deployment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_canary_deployment`.\n"]
    pub fn set_custom_canary_deployment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeployment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_canary_deployment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_canary_deployment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.runtime_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.runtime_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary {
            canary_deployment: core::default::Default::default(),
            custom_canary_deployment: core::default::Default::default(),
            runtime_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canary_deployment` after provisioning.\n"]
    pub fn canary_deployment(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CanaryDeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_canary_deployment` after provisioning.\n"]
    pub fn custom_canary_deployment(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_CustomCanaryDeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_canary_deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployDynamic {
    tasks: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tasks: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy {
    #[doc =
        "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tasks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tasks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy {
    type O =
        terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy {
            actions: core::default::Default::default(),
            tasks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]
    pub fn tasks(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tasks", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployDynamic {
    tasks: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tasks: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy {
    #[doc =
        "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_Tasks,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tasks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tasks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy {
            actions: core::default::Default::default(),
            tasks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]
    pub fn tasks(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy_TasksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tasks", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardDynamic {
    analysis: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis>>,
    postdeploy: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy>>,
    predeploy: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy>>,
    verify_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postdeploy: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predeploy: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_config: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard {
    #[doc = "Set the field `verify`.\nWhether to verify a deployment."]
    pub fn set_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Analysis,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postdeploy`.\n"]
    pub fn set_postdeploy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Postdeploy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postdeploy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postdeploy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predeploy`.\n"]
    pub fn set_predeploy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_Predeploy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predeploy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predeploy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verify_config`.\n"]
    pub fn set_verify_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verify_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verify_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard {
            verify: core::default::Default::default(),
            analysis: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            verify_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `verify` after provisioning.\nWhether to verify a deployment."]
    pub fn verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]
    pub fn postdeploy(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PostdeployRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]
    pub fn predeploy(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_PredeployRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_config` after provisioning.\n"]
    pub fn verify_config(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard_VerifyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verify_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyDynamic {
    canary: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary>>,
    standard: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy {
    #[doc = "Set the field `canary`.\n"]
    pub fn set_canary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Canary>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.canary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.canary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `standard`.\n"]
    pub fn set_standard(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_Standard>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.standard = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.standard = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy {
            canary: core::default::Default::default(),
            standard: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyRef {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canary` after provisioning.\n"]
    pub fn canary(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_CanaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary", self.base))
    }

    #[doc = "Get a reference to the value of field `standard` after provisioning.\n"]
    pub fn standard(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy_StandardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipeline_StagesDynamic {
    deploy_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters>>,
    strategy: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_Stages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_parameters: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipeline_StagesDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline_Stages {
    #[doc =
        "Set the field `profiles`.\nSkaffold profiles to use when rendering the manifest for this stage's `Target`."]
    pub fn set_profiles(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.profiles = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_id`.\nThe target_id to which this stage points. This field refers exclusively to the last segment of a target name. For example, this field would just be `my-target` (rather than `projects/project/locations/location/targets/my-target`). The location of the `Target` is inferred to be the same as the location of the `DeliveryPipeline` that contains this `Stage`."]
    pub fn set_target_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_parameters`.\n"]
    pub fn set_deploy_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deploy_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages_Strategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.strategy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline_Stages {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline_Stages {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline_Stages {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline_Stages {
        ClouddeployDeliveryPipeline_SerialPipeline_Stages {
            profiles: core::default::Default::default(),
            target_id: core::default::Default::default(),
            deploy_parameters: core::default::Default::default(),
            strategy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline_StagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipeline_StagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipeline_SerialPipeline_StagesRef {
        ClouddeployDeliveryPipeline_SerialPipeline_StagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipeline_StagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `profiles` after provisioning.\nSkaffold profiles to use when rendering the manifest for this stage's `Target`."]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_id` after provisioning.\nThe target_id to which this stage points. This field refers exclusively to the last segment of a target name. For example, this field would just be `my-target` (rather than `projects/project/locations/location/targets/my-target`). The location of the `Target` is inferred to be the same as the location of the `DeliveryPipeline` that contains this `Stage`."]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_parameters` after provisioning.\n"]
    pub fn deploy_parameters(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_DeployParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_Stages_StrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipeline_SerialPipelineDynamic {
    stages: std::option::Option<terrars::DynamicBlock<ClouddeployDeliveryPipeline_SerialPipeline_Stages>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_SerialPipeline {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stages: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipeline_SerialPipeline_Stages>>,
    dynamic: ClouddeployDeliveryPipeline_SerialPipelineDynamic,
}

impl ClouddeployDeliveryPipeline_SerialPipeline {
    #[doc = "Set the field `stages`.\n"]
    pub fn set_stages(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline_Stages>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stages = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_SerialPipeline {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_SerialPipeline>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_SerialPipeline {}

impl BuildClouddeployDeliveryPipeline_SerialPipeline {
    pub fn build(self) -> ClouddeployDeliveryPipeline_SerialPipeline {
        ClouddeployDeliveryPipeline_SerialPipeline {
            stages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_SerialPipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_SerialPipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeliveryPipeline_SerialPipelineRef {
        ClouddeployDeliveryPipeline_SerialPipelineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_SerialPipelineRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> terrars::ListRef<ClouddeployDeliveryPipeline_SerialPipeline_StagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stages", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipeline_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipeline_Timeouts {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipeline_Timeouts {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipeline_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipeline_Timeouts {}

impl BuildClouddeployDeliveryPipeline_Timeouts {
    pub fn build(self) -> ClouddeployDeliveryPipeline_Timeouts {
        ClouddeployDeliveryPipeline_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeliveryPipeline_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipeline_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeliveryPipeline_TimeoutsRef {
        ClouddeployDeliveryPipeline_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipeline_TimeoutsRef {
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
