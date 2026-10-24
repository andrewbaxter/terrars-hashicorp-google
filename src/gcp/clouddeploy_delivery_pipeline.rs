use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddeployDeliveryPipelineData {
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
    serial_pipeline: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddeployDeliveryPipelineTimeoutsEl>,
    dynamic: ClouddeployDeliveryPipelineDynamic,
}

struct ClouddeployDeliveryPipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddeployDeliveryPipelineData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddeployDeliveryPipelineTimeoutsEl>) -> Self {
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
    pub fn condition(&self) -> terrars::ListRef<ClouddeployDeliveryPipelineConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the pipeline was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nName of the `DeliveryPipeline`. Format is [a-z][a-z0-9\\-]{0,62}."]
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
    pub fn serial_pipeline(&self) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serial_pipeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeliveryPipelineTimeoutsElRef {
        ClouddeployDeliveryPipelineTimeoutsElRef::new(
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
    #[doc = "Name of the `DeliveryPipeline`. Format is [a-z][a-z0-9\\-]{0,62}."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddeployDeliveryPipeline {
        let out = ClouddeployDeliveryPipeline(std::rc::Rc::new(ClouddeployDeliveryPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddeployDeliveryPipelineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
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
    pub fn condition(&self) -> terrars::ListRef<ClouddeployDeliveryPipelineConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the pipeline was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nName of the `DeliveryPipeline`. Format is [a-z][a-z0-9\\-]{0,62}."]
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
    pub fn serial_pipeline(&self) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serial_pipeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeliveryPipelineTimeoutsElRef {
        ClouddeployDeliveryPipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {}

impl BuildClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
        ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
            status: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
        ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
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
pub struct ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    missing_targets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {}

impl BuildClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
        ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
            missing_targets: core::default::Default::default(),
            status: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
        ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
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
pub struct ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<bool>>,
}

impl ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {}

impl BuildClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
        ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
            error_details: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
        ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
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
pub struct ClouddeployDeliveryPipelineConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_ready_condition: std
    ::option
    ::Option<terrars::ListField<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets_present_condition: std
    ::option
    ::Option<terrars::ListField<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets_type_condition: std
    ::option
    ::Option<terrars::ListField<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl>>,
}

impl ClouddeployDeliveryPipelineConditionEl {
    #[doc = "Set the field `pipeline_ready_condition`.\n"]
    pub fn set_pipeline_ready_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl>>,
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
                    ::Into<terrars::ListField<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl>>,
    ) -> Self {
        self.targets_present_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `targets_type_condition`.\n"]
    pub fn set_targets_type_condition(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl>>,
    ) -> Self {
        self.targets_type_condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeliveryPipelineConditionEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineConditionEl {}

impl BuildClouddeployDeliveryPipelineConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionEl {
        ClouddeployDeliveryPipelineConditionEl {
            pipeline_ready_condition: core::default::Default::default(),
            targets_present_condition: core::default::Default::default(),
            targets_type_condition: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeliveryPipelineConditionElRef {
        ClouddeployDeliveryPipelineConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pipeline_ready_condition` after provisioning.\n"]
    pub fn pipeline_ready_condition(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pipeline_ready_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `targets_present_condition` after provisioning.\n"]
    pub fn targets_present_condition(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets_present_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `targets_type_condition` after provisioning.\n"]
    pub fn targets_type_condition(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets_type_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_target_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    values: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    #[doc = "Required. Values are deploy parameters in key-value pairs."]
    pub values: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
            match_target_labels: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
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
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl {
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
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl {
            actions: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef {
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
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl {
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
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl {
            actions: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef {
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
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElDynamic {
    postdeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl,
        >,
    >,
    predeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
    percentages: terrars::ListField<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postdeploy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predeploy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl>,
    >,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
    #[doc = "Set the field `verify`.\nWhether to run verify tests after each percentage deployment."]
    pub fn set_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify = std::option::Option::Some(v.into());
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl,
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl,
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
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
    #[doc =
        "Required. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100."]
    pub percentages: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
            percentages: self.percentages,
            verify: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef {
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

    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]
    pub fn postdeploy(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]
    pub fn predeploy(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl {
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
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl {
            actions: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef {
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
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl {
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
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl {
            actions: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef {
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
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElDynamic {
    postdeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl,
        >,
    >,
    predeploy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl {
    percentage: terrars::PrimField<f64>,
    phase_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postdeploy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predeploy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl,
        >,
    >,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl {
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl,
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl,
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
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl {
    #[doc = "Required. Percentage deployment for the phase."]
    pub percentage: terrars::PrimField<f64>,
    #[doc =
        "Required. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`."]
    pub phase_id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl {
            percentage: self.percentage,
            phase_id: self.phase_id,
            profiles: core::default::Default::default(),
            verify: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef {
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

    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]
    pub fn postdeploy(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]
    pub fn predeploy(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElDynamic {
    phase_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl,
        >,
    >,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl,
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
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {
            phase_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phase_configs` after provisioning.\n"]
    pub fn phase_configs(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phase_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_traffic_control: std::option::Option<terrars::PrimField<bool>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl {
    #[doc =
        "Set the field `automatic_traffic_control`.\nWhether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments."]
    pub fn set_automatic_traffic_control(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_traffic_control = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl {
            automatic_traffic_control: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `automatic_traffic_control` after provisioning.\nWhether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments."]
    pub fn automatic_traffic_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_traffic_control", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl {
    deployment: terrars::PrimField<std::string::String>,
    http_route: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_update_wait_time: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl {
    #[doc =
        "Set the field `route_update_wait_time`.\nOptional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time."]
    pub fn set_route_update_wait_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.route_update_wait_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl {
    #[doc =
        "Required. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service."]
    pub deployment: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the Gateway API HTTPRoute."]
    pub http_route: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the Kubernetes Service."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl {
            deployment: self.deployment,
            http_route: self.http_route,
            route_update_wait_time: core::default::Default::default(),
            service: self.service,
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef {
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
        "Get a reference to the value of field `route_update_wait_time` after provisioning.\nOptional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time."]
    pub fn route_update_wait_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_update_wait_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the Kubernetes Service."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl {
    deployment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_pod_overprovisioning: std::option::Option<terrars::PrimField<bool>>,
    service: terrars::PrimField<std::string::String>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl {
    #[doc =
        "Set the field `disable_pod_overprovisioning`.\nOptional. Whether to disable Pod overprovisioning. If Pod overprovisioning is disabled then Cloud Deploy will limit the number of total Pods used for the deployment strategy to the number of Pods the Deployment has on the cluster."]
    pub fn set_disable_pod_overprovisioning(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_pod_overprovisioning = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl {
    #[doc = "Required. Name of the Kubernetes Deployment whose traffic is managed by the specified Service."]
    pub deployment: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the Kubernetes Service."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl {
            deployment: self.deployment,
            disable_pod_overprovisioning: core::default::Default::default(),
            service: self.service,
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef {
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
        "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the Kubernetes Service."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElDynamic {
    gateway_service_mesh: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl,
        >,
    >,
    service_networking: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_service_mesh: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_networking: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl,
        >,
    >,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl {
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl,
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl,
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
::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl {
    type O =
        terrars
        ::BlockAssignable<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl {
            gateway_service_mesh: core::default::Default::default(),
            service_networking: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gateway_service_mesh` after provisioning.\n"]
    pub fn gateway_service_mesh(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_service_mesh", self.base))
    }

    #[doc = "Get a reference to the value of field `service_networking` after provisioning.\n"]
    pub fn service_networking(
        &self,
    ) -> terrars
    ::ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_networking", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElDynamic {
    cloud_run: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl>,
    >,
    kubernetes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_run: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl>,
    >,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl,
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl,
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
            cloud_run: core::default::Default::default(),
            kubernetes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\n"]
    pub fn cloud_run(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run", self.base))
    }

    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElDynamic {
    canary_deployment: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl>,
    >,
    custom_canary_deployment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl>,
    >,
    runtime_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_deployment: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_canary_deployment: std
    ::option
    ::Option<
        std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_config: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl,
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl,
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl,
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
            canary_deployment: core::default::Default::default(),
            custom_canary_deployment: core::default::Default::default(),
            runtime_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canary_deployment` after provisioning.\n"]
    pub fn canary_deployment(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_canary_deployment` after provisioning.\n"]
    pub fn custom_canary_deployment(
        &self,
    ) -> terrars
    ::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_canary_deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
            actions: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
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
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
    type O =
        terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
            actions: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
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
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElDynamic {
    postdeploy: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl>,
    >,
    predeploy: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postdeploy: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predeploy: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    #[doc = "Set the field `verify`.\nWhether to verify a deployment."]
    pub fn set_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify = std::option::Option::Some(v.into());
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl,
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
                            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl,
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
}

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
            verify: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `verify` after provisioning.\nWhether to verify a deployment."]
    pub fn verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }

    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]
    pub fn postdeploy(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }

    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]
    pub fn predeploy(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElDynamic {
    canary: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>>,
    standard: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    #[doc = "Set the field `canary`.\n"]
    pub fn set_canary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>,
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
                        terrars
                        ::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl>,
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
            canary: core::default::Default::default(),
            standard: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canary` after provisioning.\n"]
    pub fn canary(&self) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary", self.base))
    }

    #[doc = "Get a reference to the value of field `standard` after provisioning.\n"]
    pub fn standard(
        &self,
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElDynamic {
    deploy_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>>,
    strategy: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_parameters: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
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
                        terrars
                        ::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>,
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
                    ::Into<terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>>,
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
            profiles: core::default::Default::default(),
            target_id: core::default::Default::default(),
            deploy_parameters: core::default::Default::default(),
            strategy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
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
    ) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElDynamic {
    stages: std::option::Option<terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stages: std::option::Option<std::vec::Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElDynamic,
}

impl ClouddeployDeliveryPipelineSerialPipelineEl {
    #[doc = "Set the field `stages`.\n"]
    pub fn set_stages(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>>,
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineSerialPipelineEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineSerialPipelineEl {}

impl BuildClouddeployDeliveryPipelineSerialPipelineEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineEl {
        ClouddeployDeliveryPipelineSerialPipelineEl {
            stages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineSerialPipelineElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineSerialPipelineElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElRef {
        ClouddeployDeliveryPipelineSerialPipelineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineSerialPipelineElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> terrars::ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stages", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeliveryPipelineTimeoutsEl {
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

impl terrars::ToListMappable for ClouddeployDeliveryPipelineTimeoutsEl {
    type O = terrars::BlockAssignable<ClouddeployDeliveryPipelineTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeliveryPipelineTimeoutsEl {}

impl BuildClouddeployDeliveryPipelineTimeoutsEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineTimeoutsEl {
        ClouddeployDeliveryPipelineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployDeliveryPipelineTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeliveryPipelineTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeliveryPipelineTimeoutsElRef {
        ClouddeployDeliveryPipelineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeliveryPipelineTimeoutsElRef {
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
struct ClouddeployDeliveryPipelineDynamic {
    serial_pipeline: std::option::Option<terrars::DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineEl>>,
}
