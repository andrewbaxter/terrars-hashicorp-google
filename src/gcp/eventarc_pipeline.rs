#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EventarcPipelineData_ {
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
    crypto_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    pipeline_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<EventarcPipeline_Destinations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_payload_format: std::option::Option<std::vec::Vec<EventarcPipeline_InputPayloadFormat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<EventarcPipeline_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mediations: std::option::Option<std::vec::Vec<EventarcPipeline_Mediations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<EventarcPipeline_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EventarcPipeline_Timeouts>,
    dynamic: EventarcPipelineDynamic,
}

struct EventarcPipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EventarcPipelineData_>,
}

#[derive(Clone)]
pub struct EventarcPipeline(std::rc::Rc<EventarcPipeline_>);

impl EventarcPipeline {
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
        "Set the field `annotations`.\nUser-defined annotations. See https://google.aip.dev/128#annotations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `crypto_key_name`.\nResource name of a KMS crypto key (managed by the user) used to\nencrypt/decrypt the event data. If not set, an internal Google-owned key\nwill be used to encrypt messages. It must match the pattern\n\"projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}\"."]
    pub fn set_crypto_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().crypto_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nDisplay name of resource."]
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
        "Set the field `labels`.\nUser labels attached to the Pipeline that can be used to group\nresources. An object containing a list of \"key\": value pairs. Example: {\n\"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_payload_format`.\n"]
    pub fn set_input_payload_format(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_payload_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_payload_format = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mediations`.\n"]
    pub fn set_mediations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_Mediations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mediations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mediations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_RetryPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EventarcPipeline_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser-defined annotations. See https://google.aip.dev/128#annotations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to\nencrypt/decrypt the event data. If not set, an internal Google-owned key\nwill be used to encrypt messages. It must match the pattern\n\"projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}\"."]
    pub fn crypto_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of\nother fields, and might be sent only on create requests to ensure that the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser labels attached to the Pipeline that can be used to group\nresources. An object containing a list of \"key\": value pairs. Example: {\n\"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Pipeline. Must be unique within the\nlocation of the project and must be in\n'projects/{project}/locations/{location}/pipelines/{pipeline}' format."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_id` after provisioning.\nThe user-provided ID to be assigned to the Pipeline. It should match the\nformat '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer-assigned unique identifier for the Pipeline. The value\nis a UUID4 string and guaranteed to remain unchanged until the resource is\ndeleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::ListRef<EventarcPipeline_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_payload_format` after provisioning.\n"]
    pub fn input_payload_format(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_payload_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<EventarcPipeline_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mediations` after provisioning.\n"]
    pub fn mediations(&self) -> terrars::ListRef<EventarcPipeline_MediationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mediations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<EventarcPipeline_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcPipeline_TimeoutsRef {
        EventarcPipeline_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EventarcPipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EventarcPipeline { }

impl terrars::ToListMappable for EventarcPipeline {
    type O = terrars::ListRef<EventarcPipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EventarcPipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_eventarc_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEventarcPipeline {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The user-provided ID to be assigned to the Pipeline. It should match the\nformat '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub pipeline_id: terrars::PrimField<std::string::String>,
}

impl BuildEventarcPipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> EventarcPipeline {
        let out = EventarcPipeline(std::rc::Rc::new(EventarcPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EventarcPipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                crypto_key_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                pipeline_id: self.pipeline_id,
                project: core::default::Default::default(),
                destinations: core::default::Default::default(),
                input_payload_format: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                mediations: core::default::Default::default(),
                retry_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EventarcPipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EventarcPipelineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser-defined annotations. See https://google.aip.dev/128#annotations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to\nencrypt/decrypt the event data. If not set, an internal Google-owned key\nwill be used to encrypt messages. It must match the pattern\n\"projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}\"."]
    pub fn crypto_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of\nother fields, and might be sent only on create requests to ensure that the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser labels attached to the Pipeline that can be used to group\nresources. An object containing a list of \"key\": value pairs. Example: {\n\"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Pipeline. Must be unique within the\nlocation of the project and must be in\n'projects/{project}/locations/{location}/pipelines/{pipeline}' format."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_id` after provisioning.\nThe user-provided ID to be assigned to the Pipeline. It should match the\nformat '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer-assigned unique identifier for the Pipeline. The value\nis a UUID4 string and guaranteed to remain unchanged until the resource is\ndeleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::ListRef<EventarcPipeline_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_payload_format` after provisioning.\n"]
    pub fn input_payload_format(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_payload_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<EventarcPipeline_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mediations` after provisioning.\n"]
    pub fn mediations(&self) -> terrars::ListRef<EventarcPipeline_MediationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mediations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<EventarcPipeline_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcPipeline_TimeoutsRef {
        EventarcPipeline_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EventarcPipelineDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<EventarcPipeline_Destinations>>,
    input_payload_format: std::option::Option<terrars::DynamicBlock<EventarcPipeline_InputPayloadFormat>>,
    logging_config: std::option::Option<terrars::DynamicBlock<EventarcPipeline_LoggingConfig>>,
    mediations: std::option::Option<terrars::DynamicBlock<EventarcPipeline_Mediations>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<EventarcPipeline_RetryPolicy>>,
}

#[derive(Serialize)]
pub struct EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account: terrars::PrimField<std::string::String>,
}

impl EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc {
    #[doc =
        "Set the field `audience`.\nAudience to be used to generate the OIDC Token. The audience claim\nidentifies the recipient that the JWT is intended for. If\nunspecified, the destination URI will be used."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc {
    type O = terrars::BlockAssignable<EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc {
    #[doc =
        "Service account email used to generate the OIDC Token.\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow the\nPipeline to create OpenID tokens for authenticated requests."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildEventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc {
    pub fn build(self) -> EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc {
        EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc {
            audience: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidcRef {
        EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audience` after provisioning.\nAudience to be used to generate the OIDC Token. The audience claim\nidentifies the recipient that the JWT is intended for. If\nunspecified, the destination URI will be used."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account email used to generate the OIDC Token.\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow the\nPipeline to create OpenID tokens for authenticated requests."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_Destinations_AuthenticationConfig_OauthToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account: terrars::PrimField<std::string::String>,
}

impl EventarcPipeline_Destinations_AuthenticationConfig_OauthToken {
    #[doc =
        "Set the field `scope`.\nOAuth scope to be used for generating OAuth access token. If not\nspecified, \"https://www.googleapis.com/auth/cloud-platform\" will be\nused."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Destinations_AuthenticationConfig_OauthToken {
    type O = terrars::BlockAssignable<EventarcPipeline_Destinations_AuthenticationConfig_OauthToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Destinations_AuthenticationConfig_OauthToken {
    #[doc =
        "Service account email used to generate the [OAuth\ntoken](https://developers.google.com/identity/protocols/OAuth2).\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow Pipeline\nto create OAuth2 tokens for authenticated requests."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildEventarcPipeline_Destinations_AuthenticationConfig_OauthToken {
    pub fn build(self) -> EventarcPipeline_Destinations_AuthenticationConfig_OauthToken {
        EventarcPipeline_Destinations_AuthenticationConfig_OauthToken {
            scope: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_Destinations_AuthenticationConfig_OauthTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_Destinations_AuthenticationConfig_OauthTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EventarcPipeline_Destinations_AuthenticationConfig_OauthTokenRef {
        EventarcPipeline_Destinations_AuthenticationConfig_OauthTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_Destinations_AuthenticationConfig_OauthTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nOAuth scope to be used for generating OAuth access token. If not\nspecified, \"https://www.googleapis.com/auth/cloud-platform\" will be\nused."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account email used to generate the [OAuth\ntoken](https://developers.google.com/identity/protocols/OAuth2).\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow Pipeline\nto create OAuth2 tokens for authenticated requests."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcPipeline_Destinations_AuthenticationConfigDynamic {
    google_oidc: std
    ::option
    ::Option<terrars::DynamicBlock<EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc>>,
    oauth_token: std
    ::option
    ::Option<terrars::DynamicBlock<EventarcPipeline_Destinations_AuthenticationConfig_OauthToken>>,
}

#[derive(Serialize)]
pub struct EventarcPipeline_Destinations_AuthenticationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_oidc: std::option::Option<std::vec::Vec<EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_token: std::option::Option<std::vec::Vec<EventarcPipeline_Destinations_AuthenticationConfig_OauthToken>>,
    dynamic: EventarcPipeline_Destinations_AuthenticationConfigDynamic,
}

impl EventarcPipeline_Destinations_AuthenticationConfig {
    #[doc = "Set the field `google_oidc`.\n"]
    pub fn set_google_oidc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_oidc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_oidc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_token`.\n"]
    pub fn set_oauth_token(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EventarcPipeline_Destinations_AuthenticationConfig_OauthToken>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_token = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_token = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Destinations_AuthenticationConfig {
    type O = terrars::BlockAssignable<EventarcPipeline_Destinations_AuthenticationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Destinations_AuthenticationConfig {}

impl BuildEventarcPipeline_Destinations_AuthenticationConfig {
    pub fn build(self) -> EventarcPipeline_Destinations_AuthenticationConfig {
        EventarcPipeline_Destinations_AuthenticationConfig {
            google_oidc: core::default::Default::default(),
            oauth_token: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_Destinations_AuthenticationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_Destinations_AuthenticationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EventarcPipeline_Destinations_AuthenticationConfigRef {
        EventarcPipeline_Destinations_AuthenticationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_Destinations_AuthenticationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `google_oidc` after provisioning.\n"]
    pub fn google_oidc(&self) -> terrars::ListRef<EventarcPipeline_Destinations_AuthenticationConfig_GoogleOidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> terrars::ListRef<EventarcPipeline_Destinations_AuthenticationConfig_OauthTokenRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_token", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_Destinations_HttpEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_binding_template: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl EventarcPipeline_Destinations_HttpEndpoint {
    #[doc =
        "Set the field `message_binding_template`.\nThe CEL expression used to modify how the destination-bound HTTP\nrequest is constructed.\n\nIf a binding expression is not specified here, the message\nis treated as a CloudEvent and is mapped to the HTTP request according\nto the CloudEvent HTTP Protocol Binding Binary Content Mode\n(https://github.com/cloudevents/spec/blob/main/cloudevents/bindings/http-protocol-binding.md#31-binary-content-mode).\nIn this representation, all fields except the 'data' and\n'datacontenttype' field on the message are mapped to HTTP request\nheaders with a prefix of 'ce-'.\n\nTo construct the HTTP request payload and the value of the content-type\nHTTP header, the payload format is defined as follows:\n1) Use the output_payload_format_type on the Pipeline.Destination if it\nis set, else:\n2) Use the input_payload_format_type on the Pipeline if it is set,\nelse:\n3) Treat the payload as opaque binary data.\n\nThe 'data' field of the message is converted to the payload format or\nleft as-is for case 3) and then attached as the payload of the HTTP\nrequest. The 'content-type' header on the HTTP request is set to the\npayload format type or left empty for case 3). However, if a mediation\nhas updated the 'datacontenttype' field on the message so that it is\nnot the same as the payload format type but it is still a prefix of the\npayload format type, then the 'content-type' header on the HTTP request\nis set to this 'datacontenttype' value. For example, if the\n'datacontenttype' is \"application/json\" and the payload format type is\n\"application/json; charset=utf-8\", then the 'content-type' header on\nthe HTTP request is set to \"application/json; charset=utf-8\".\n\nIf a non-empty binding expression is specified then this expression is\nused to modify the default CloudEvent HTTP Protocol Binding Binary\nContent representation.\nThe result of the CEL expression must be a map of key/value pairs\nwhich is used as follows:\n- If a map named 'headers' exists on the result of the expression,\nthen its key/value pairs are directly mapped to the HTTP request\nheaders. The headers values are constructed from the corresponding\nvalue type's canonical representation. If the 'headers' field doesn't\nexist then the resulting HTTP request will be the headers of the\nCloudEvent HTTP Binding Binary Content Mode representation of the final\nmessage. Note: If the specified binding expression, has updated the\n'datacontenttype' field on the message so that it is not the same as\nthe payload format type but it is still a prefix of the payload format\ntype, then the 'content-type' header in the 'headers' map is set to\nthis 'datacontenttype' value.\n- If a field named 'body' exists on the result of the expression then\nits value is directly mapped to the body of the request. If the value\nof the 'body' field is of type bytes or string then it is used for\nthe HTTP request body as-is, with no conversion. If the body field is\nof any other type then it is converted to a JSON string. If the body\nfield does not exist then the resulting payload of the HTTP request\nwill be data value of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message as described earlier.\n- Any other fields in the resulting expression will be ignored.\n\nThe CEL expression may access the incoming CloudEvent message in its\ndefinition, as follows:\n- The 'data' field of the incoming CloudEvent message can be accessed\nusing the 'message.data' value. Subfields of 'message.data' may also be\naccessed if an input_payload_format has been specified on the Pipeline.\n- Each attribute of the incoming CloudEvent message can be accessed\nusing the 'message.' value, where  is replaced with the\nname of the attribute.\n- Existing headers can be accessed in the CEL expression using the\n'headers' variable. The 'headers' variable defines a map of key/value\npairs corresponding to the HTTP headers of the CloudEvent HTTP Binding\nBinary Content Mode representation of the final message as described\nearlier. For example, the following CEL expression can be used to\nconstruct an HTTP request by adding an additional header to the HTTP\nheaders of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message and by overwriting the body of the\nrequest:\n\n'''\n{\n\"headers\": headers.merge({\"new-header-key\": \"new-header-value\"}),\n\"body\": \"new-body\"\n}\n'''\n- The default binding for the message payload can be accessed using the\n'body' variable. It conatins a string representation of the message\npayload in the format specified by the 'output_payload_format' field.\nIf the 'input_payload_format' field is not set, the 'body'\nvariable contains the same message payload bytes that were published.\n\nAdditionally, the following CEL extension functions are provided for\nuse in this CEL expression:\n- toBase64Url:\nmap.toBase64Url() -> string\n- Converts a CelValue to a base64url encoded string\n- toJsonString: map.toJsonString() -> string\n- Converts a CelValue to a JSON string\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents.\n- It converts 'data' to destination payload format\nspecified in 'output_payload_format'. If 'output_payload_format' is\nnot set, the data will remain unchanged.\n- It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\n'output_payload_format'. If no\n'output_payload_format' is set it will use the value of the\n\"datacontenttype\" attribute on the CloudEvent if present, else\nremove \"datacontenttype\" attribute.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function.\n\nThe Pipeline expects that the message it receives adheres to the\nstandard CloudEvent format. If it doesn't then the outgoing message\nrequest may fail with a persistent error."]
    pub fn set_message_binding_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.message_binding_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Destinations_HttpEndpoint {
    type O = terrars::BlockAssignable<EventarcPipeline_Destinations_HttpEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Destinations_HttpEndpoint {
    #[doc =
        "The URI of the HTTP enpdoint.\n\nThe value must be a RFC2396 URI string.\nExamples: 'https://svc.us-central1.p.local:8080/route'.\nOnly the HTTPS protocol is supported."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildEventarcPipeline_Destinations_HttpEndpoint {
    pub fn build(self) -> EventarcPipeline_Destinations_HttpEndpoint {
        EventarcPipeline_Destinations_HttpEndpoint {
            message_binding_template: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_Destinations_HttpEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_Destinations_HttpEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_Destinations_HttpEndpointRef {
        EventarcPipeline_Destinations_HttpEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_Destinations_HttpEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `message_binding_template` after provisioning.\nThe CEL expression used to modify how the destination-bound HTTP\nrequest is constructed.\n\nIf a binding expression is not specified here, the message\nis treated as a CloudEvent and is mapped to the HTTP request according\nto the CloudEvent HTTP Protocol Binding Binary Content Mode\n(https://github.com/cloudevents/spec/blob/main/cloudevents/bindings/http-protocol-binding.md#31-binary-content-mode).\nIn this representation, all fields except the 'data' and\n'datacontenttype' field on the message are mapped to HTTP request\nheaders with a prefix of 'ce-'.\n\nTo construct the HTTP request payload and the value of the content-type\nHTTP header, the payload format is defined as follows:\n1) Use the output_payload_format_type on the Pipeline.Destination if it\nis set, else:\n2) Use the input_payload_format_type on the Pipeline if it is set,\nelse:\n3) Treat the payload as opaque binary data.\n\nThe 'data' field of the message is converted to the payload format or\nleft as-is for case 3) and then attached as the payload of the HTTP\nrequest. The 'content-type' header on the HTTP request is set to the\npayload format type or left empty for case 3). However, if a mediation\nhas updated the 'datacontenttype' field on the message so that it is\nnot the same as the payload format type but it is still a prefix of the\npayload format type, then the 'content-type' header on the HTTP request\nis set to this 'datacontenttype' value. For example, if the\n'datacontenttype' is \"application/json\" and the payload format type is\n\"application/json; charset=utf-8\", then the 'content-type' header on\nthe HTTP request is set to \"application/json; charset=utf-8\".\n\nIf a non-empty binding expression is specified then this expression is\nused to modify the default CloudEvent HTTP Protocol Binding Binary\nContent representation.\nThe result of the CEL expression must be a map of key/value pairs\nwhich is used as follows:\n- If a map named 'headers' exists on the result of the expression,\nthen its key/value pairs are directly mapped to the HTTP request\nheaders. The headers values are constructed from the corresponding\nvalue type's canonical representation. If the 'headers' field doesn't\nexist then the resulting HTTP request will be the headers of the\nCloudEvent HTTP Binding Binary Content Mode representation of the final\nmessage. Note: If the specified binding expression, has updated the\n'datacontenttype' field on the message so that it is not the same as\nthe payload format type but it is still a prefix of the payload format\ntype, then the 'content-type' header in the 'headers' map is set to\nthis 'datacontenttype' value.\n- If a field named 'body' exists on the result of the expression then\nits value is directly mapped to the body of the request. If the value\nof the 'body' field is of type bytes or string then it is used for\nthe HTTP request body as-is, with no conversion. If the body field is\nof any other type then it is converted to a JSON string. If the body\nfield does not exist then the resulting payload of the HTTP request\nwill be data value of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message as described earlier.\n- Any other fields in the resulting expression will be ignored.\n\nThe CEL expression may access the incoming CloudEvent message in its\ndefinition, as follows:\n- The 'data' field of the incoming CloudEvent message can be accessed\nusing the 'message.data' value. Subfields of 'message.data' may also be\naccessed if an input_payload_format has been specified on the Pipeline.\n- Each attribute of the incoming CloudEvent message can be accessed\nusing the 'message.' value, where  is replaced with the\nname of the attribute.\n- Existing headers can be accessed in the CEL expression using the\n'headers' variable. The 'headers' variable defines a map of key/value\npairs corresponding to the HTTP headers of the CloudEvent HTTP Binding\nBinary Content Mode representation of the final message as described\nearlier. For example, the following CEL expression can be used to\nconstruct an HTTP request by adding an additional header to the HTTP\nheaders of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message and by overwriting the body of the\nrequest:\n\n'''\n{\n\"headers\": headers.merge({\"new-header-key\": \"new-header-value\"}),\n\"body\": \"new-body\"\n}\n'''\n- The default binding for the message payload can be accessed using the\n'body' variable. It conatins a string representation of the message\npayload in the format specified by the 'output_payload_format' field.\nIf the 'input_payload_format' field is not set, the 'body'\nvariable contains the same message payload bytes that were published.\n\nAdditionally, the following CEL extension functions are provided for\nuse in this CEL expression:\n- toBase64Url:\nmap.toBase64Url() -> string\n- Converts a CelValue to a base64url encoded string\n- toJsonString: map.toJsonString() -> string\n- Converts a CelValue to a JSON string\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents.\n- It converts 'data' to destination payload format\nspecified in 'output_payload_format'. If 'output_payload_format' is\nnot set, the data will remain unchanged.\n- It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\n'output_payload_format'. If no\n'output_payload_format' is set it will use the value of the\n\"datacontenttype\" attribute on the CloudEvent if present, else\nremove \"datacontenttype\" attribute.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function.\n\nThe Pipeline expects that the message it receives adheres to the\nstandard CloudEvent format. If it doesn't then the outgoing message\nrequest may fail with a persistent error."]
    pub fn message_binding_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_binding_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe URI of the HTTP enpdoint.\n\nThe value must be a RFC2396 URI string.\nExamples: 'https://svc.us-central1.p.local:8080/route'.\nOnly the HTTPS protocol is supported."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_Destinations_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcPipeline_Destinations_NetworkConfig {
    #[doc =
        "Set the field `network_attachment`.\nName of the NetworkAttachment that allows access to the consumer VPC.\n\nFormat:\n'projects/{PROJECT_ID}/regions/{REGION}/networkAttachments/{NETWORK_ATTACHMENT_NAME}'\n\nRequired for HTTP endpoint destinations. Must not be specified for\nWorkflows, MessageBus, or Topic destinations."]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Destinations_NetworkConfig {
    type O = terrars::BlockAssignable<EventarcPipeline_Destinations_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Destinations_NetworkConfig {}

impl BuildEventarcPipeline_Destinations_NetworkConfig {
    pub fn build(self) -> EventarcPipeline_Destinations_NetworkConfig {
        EventarcPipeline_Destinations_NetworkConfig { network_attachment: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_Destinations_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_Destinations_NetworkConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EventarcPipeline_Destinations_NetworkConfigRef {
        EventarcPipeline_Destinations_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_Destinations_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nName of the NetworkAttachment that allows access to the consumer VPC.\n\nFormat:\n'projects/{PROJECT_ID}/regions/{REGION}/networkAttachments/{NETWORK_ATTACHMENT_NAME}'\n\nRequired for HTTP endpoint destinations. Must not be specified for\nWorkflows, MessageBus, or Topic destinations."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_InputPayloadFormat_Avro {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_definition: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcPipeline_InputPayloadFormat_Avro {
    #[doc = "Set the field `schema_definition`.\nThe entire schema definition is stored in this field."]
    pub fn set_schema_definition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_definition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_InputPayloadFormat_Avro {
    type O = terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Avro>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_InputPayloadFormat_Avro {}

impl BuildEventarcPipeline_InputPayloadFormat_Avro {
    pub fn build(self) -> EventarcPipeline_InputPayloadFormat_Avro {
        EventarcPipeline_InputPayloadFormat_Avro { schema_definition: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_InputPayloadFormat_AvroRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_InputPayloadFormat_AvroRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_InputPayloadFormat_AvroRef {
        EventarcPipeline_InputPayloadFormat_AvroRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_InputPayloadFormat_AvroRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `schema_definition` after provisioning.\nThe entire schema definition is stored in this field."]
    pub fn schema_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_InputPayloadFormat_Json {}

impl EventarcPipeline_InputPayloadFormat_Json { }

impl terrars::ToListMappable for EventarcPipeline_InputPayloadFormat_Json {
    type O = terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Json>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_InputPayloadFormat_Json {}

impl BuildEventarcPipeline_InputPayloadFormat_Json {
    pub fn build(self) -> EventarcPipeline_InputPayloadFormat_Json {
        EventarcPipeline_InputPayloadFormat_Json {}
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_InputPayloadFormat_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_InputPayloadFormat_JsonRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_InputPayloadFormat_JsonRef {
        EventarcPipeline_InputPayloadFormat_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_InputPayloadFormat_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_InputPayloadFormat_Protobuf {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_definition: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcPipeline_InputPayloadFormat_Protobuf {
    #[doc = "Set the field `schema_definition`.\nThe entire schema definition is stored in this field."]
    pub fn set_schema_definition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_definition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_InputPayloadFormat_Protobuf {
    type O = terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Protobuf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_InputPayloadFormat_Protobuf {}

impl BuildEventarcPipeline_InputPayloadFormat_Protobuf {
    pub fn build(self) -> EventarcPipeline_InputPayloadFormat_Protobuf {
        EventarcPipeline_InputPayloadFormat_Protobuf { schema_definition: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_InputPayloadFormat_ProtobufRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_InputPayloadFormat_ProtobufRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EventarcPipeline_InputPayloadFormat_ProtobufRef {
        EventarcPipeline_InputPayloadFormat_ProtobufRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_InputPayloadFormat_ProtobufRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `schema_definition` after provisioning.\nThe entire schema definition is stored in this field."]
    pub fn schema_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcPipeline_Destinations_OutputPayloadFormatDynamic {
    avro: std::option::Option<terrars::DynamicBlock<EventarcPipeline_InputPayloadFormat_Avro>>,
    json: std::option::Option<terrars::DynamicBlock<EventarcPipeline_InputPayloadFormat_Json>>,
    protobuf: std::option::Option<terrars::DynamicBlock<EventarcPipeline_InputPayloadFormat_Protobuf>>,
}

#[derive(Serialize)]
pub struct EventarcPipeline_Destinations_OutputPayloadFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro: std::option::Option<std::vec::Vec<EventarcPipeline_InputPayloadFormat_Avro>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std::option::Option<std::vec::Vec<EventarcPipeline_InputPayloadFormat_Json>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protobuf: std::option::Option<std::vec::Vec<EventarcPipeline_InputPayloadFormat_Protobuf>>,
    dynamic: EventarcPipeline_Destinations_OutputPayloadFormatDynamic,
}

impl EventarcPipeline_Destinations_OutputPayloadFormat {
    #[doc = "Set the field `avro`.\n"]
    pub fn set_avro(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Avro>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avro = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avro = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Json>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `protobuf`.\n"]
    pub fn set_protobuf(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Protobuf>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.protobuf = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.protobuf = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Destinations_OutputPayloadFormat {
    type O = terrars::BlockAssignable<EventarcPipeline_Destinations_OutputPayloadFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Destinations_OutputPayloadFormat {}

impl BuildEventarcPipeline_Destinations_OutputPayloadFormat {
    pub fn build(self) -> EventarcPipeline_Destinations_OutputPayloadFormat {
        EventarcPipeline_Destinations_OutputPayloadFormat {
            avro: core::default::Default::default(),
            json: core::default::Default::default(),
            protobuf: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_Destinations_OutputPayloadFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_Destinations_OutputPayloadFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EventarcPipeline_Destinations_OutputPayloadFormatRef {
        EventarcPipeline_Destinations_OutputPayloadFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_Destinations_OutputPayloadFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `avro` after provisioning.\n"]
    pub fn avro(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormat_AvroRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormat_JsonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc = "Get a reference to the value of field `protobuf` after provisioning.\n"]
    pub fn protobuf(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormat_ProtobufRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protobuf", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcPipeline_DestinationsDynamic {
    authentication_config: std
    ::option
    ::Option<terrars::DynamicBlock<EventarcPipeline_Destinations_AuthenticationConfig>>,
    http_endpoint: std::option::Option<terrars::DynamicBlock<EventarcPipeline_Destinations_HttpEndpoint>>,
    network_config: std::option::Option<terrars::DynamicBlock<EventarcPipeline_Destinations_NetworkConfig>>,
    output_payload_format: std
    ::option
    ::Option<terrars::DynamicBlock<EventarcPipeline_Destinations_OutputPayloadFormat>>,
}

#[derive(Serialize)]
pub struct EventarcPipeline_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_bus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_config: std::option::Option<std::vec::Vec<EventarcPipeline_Destinations_AuthenticationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint: std::option::Option<std::vec::Vec<EventarcPipeline_Destinations_HttpEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<EventarcPipeline_Destinations_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_payload_format: std::option::Option<std::vec::Vec<EventarcPipeline_Destinations_OutputPayloadFormat>>,
    dynamic: EventarcPipeline_DestinationsDynamic,
}

impl EventarcPipeline_Destinations {
    #[doc =
        "Set the field `message_bus`.\nThe resource name of the Message Bus to which events should be\npublished. The Message Bus resource should exist in the same project as\nthe Pipeline. Format:\n'projects/{project}/locations/{location}/messageBuses/{message_bus}'"]
    pub fn set_message_bus(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_bus = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `topic`.\nThe resource name of the Pub/Sub topic to which events should be\npublished. Format:\n'projects/{project}/locations/{location}/topics/{topic}'"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `workflow`.\nThe resource name of the Workflow whose Executions are triggered by\nthe events. The Workflow resource should be deployed in the same\nproject as the Pipeline. Format:\n'projects/{project}/locations/{location}/workflows/{workflow}'"]
    pub fn set_workflow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workflow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_Destinations_AuthenticationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_Destinations_HttpEndpoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_Destinations_NetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_payload_format`.\n"]
    pub fn set_output_payload_format(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_Destinations_OutputPayloadFormat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_payload_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_payload_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Destinations {
    type O = terrars::BlockAssignable<EventarcPipeline_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Destinations {}

impl BuildEventarcPipeline_Destinations {
    pub fn build(self) -> EventarcPipeline_Destinations {
        EventarcPipeline_Destinations {
            message_bus: core::default::Default::default(),
            topic: core::default::Default::default(),
            workflow: core::default::Default::default(),
            authentication_config: core::default::Default::default(),
            http_endpoint: core::default::Default::default(),
            network_config: core::default::Default::default(),
            output_payload_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_DestinationsRef {
        EventarcPipeline_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `message_bus` after provisioning.\nThe resource name of the Message Bus to which events should be\npublished. The Message Bus resource should exist in the same project as\nthe Pipeline. Format:\n'projects/{project}/locations/{location}/messageBuses/{message_bus}'"]
    pub fn message_bus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_bus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nThe resource name of the Pub/Sub topic to which events should be\npublished. Format:\n'projects/{project}/locations/{location}/topics/{topic}'"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workflow` after provisioning.\nThe resource name of the Workflow whose Executions are triggered by\nthe events. The Workflow resource should be deployed in the same\nproject as the Pipeline. Format:\n'projects/{project}/locations/{location}/workflows/{workflow}'"]
    pub fn workflow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(&self) -> terrars::ListRef<EventarcPipeline_Destinations_AuthenticationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_config", self.base))
    }

    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> terrars::ListRef<EventarcPipeline_Destinations_HttpEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<EventarcPipeline_Destinations_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.base))
    }

    #[doc = "Get a reference to the value of field `output_payload_format` after provisioning.\n"]
    pub fn output_payload_format(&self) -> terrars::ListRef<EventarcPipeline_Destinations_OutputPayloadFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_payload_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcPipeline_InputPayloadFormatDynamic {
    avro: std::option::Option<terrars::DynamicBlock<EventarcPipeline_InputPayloadFormat_Avro>>,
    json: std::option::Option<terrars::DynamicBlock<EventarcPipeline_InputPayloadFormat_Json>>,
    protobuf: std::option::Option<terrars::DynamicBlock<EventarcPipeline_InputPayloadFormat_Protobuf>>,
}

#[derive(Serialize)]
pub struct EventarcPipeline_InputPayloadFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro: std::option::Option<std::vec::Vec<EventarcPipeline_InputPayloadFormat_Avro>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std::option::Option<std::vec::Vec<EventarcPipeline_InputPayloadFormat_Json>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protobuf: std::option::Option<std::vec::Vec<EventarcPipeline_InputPayloadFormat_Protobuf>>,
    dynamic: EventarcPipeline_InputPayloadFormatDynamic,
}

impl EventarcPipeline_InputPayloadFormat {
    #[doc = "Set the field `avro`.\n"]
    pub fn set_avro(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Avro>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avro = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avro = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Json>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `protobuf`.\n"]
    pub fn set_protobuf(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat_Protobuf>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.protobuf = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.protobuf = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_InputPayloadFormat {
    type O = terrars::BlockAssignable<EventarcPipeline_InputPayloadFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_InputPayloadFormat {}

impl BuildEventarcPipeline_InputPayloadFormat {
    pub fn build(self) -> EventarcPipeline_InputPayloadFormat {
        EventarcPipeline_InputPayloadFormat {
            avro: core::default::Default::default(),
            json: core::default::Default::default(),
            protobuf: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_InputPayloadFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_InputPayloadFormatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_InputPayloadFormatRef {
        EventarcPipeline_InputPayloadFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_InputPayloadFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `avro` after provisioning.\n"]
    pub fn avro(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormat_AvroRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormat_JsonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc = "Get a reference to the value of field `protobuf` after provisioning.\n"]
    pub fn protobuf(&self) -> terrars::ListRef<EventarcPipeline_InputPayloadFormat_ProtobufRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protobuf", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_severity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcPipeline_LoggingConfig {
    #[doc =
        "Set the field `log_severity`.\nThe minimum severity of logs that will be sent to Stackdriver/Platform\nTelemetry. Logs at severitiy ≥ this value will be sent, unless it is NONE. Possible values: [\"NONE\", \"DEBUG\", \"INFO\", \"NOTICE\", \"WARNING\", \"ERROR\", \"CRITICAL\", \"ALERT\", \"EMERGENCY\"]"]
    pub fn set_log_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_severity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_LoggingConfig {
    type O = terrars::BlockAssignable<EventarcPipeline_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_LoggingConfig {}

impl BuildEventarcPipeline_LoggingConfig {
    pub fn build(self) -> EventarcPipeline_LoggingConfig {
        EventarcPipeline_LoggingConfig { log_severity: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_LoggingConfigRef {
        EventarcPipeline_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log_severity` after provisioning.\nThe minimum severity of logs that will be sent to Stackdriver/Platform\nTelemetry. Logs at severitiy ≥ this value will be sent, unless it is NONE. Possible values: [\"NONE\", \"DEBUG\", \"INFO\", \"NOTICE\", \"WARNING\", \"ERROR\", \"CRITICAL\", \"ALERT\", \"EMERGENCY\"]"]
    pub fn log_severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_severity", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_Mediations_Transformation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcPipeline_Mediations_Transformation {
    #[doc =
        "Set the field `transformation_template`.\nThe CEL expression template to apply to transform messages.\nThe following CEL extension functions are provided for\nuse in this CEL expression:\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toDestinationPayloadFormat():\nmessage.data.toDestinationPayloadFormat() -> string or bytes\n- Converts the message data to the destination payload format\nspecified in Pipeline.Destination.output_payload_format\n- This function is meant to be applied to the message.data field.\n- If the destination payload format is not set, the function will\nreturn the message data unchanged.\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents\n- This function applies toDestinationPayloadFormat() to the\nmessage data. It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\nPipeline.Destination.output_payload_format. If no\noutput_payload_format is set it will use the existing\ndatacontenttype on the CloudEvent if present, else leave\ndatacontenttype absent.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function."]
    pub fn set_transformation_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transformation_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Mediations_Transformation {
    type O = terrars::BlockAssignable<EventarcPipeline_Mediations_Transformation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Mediations_Transformation {}

impl BuildEventarcPipeline_Mediations_Transformation {
    pub fn build(self) -> EventarcPipeline_Mediations_Transformation {
        EventarcPipeline_Mediations_Transformation { transformation_template: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_Mediations_TransformationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_Mediations_TransformationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_Mediations_TransformationRef {
        EventarcPipeline_Mediations_TransformationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_Mediations_TransformationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `transformation_template` after provisioning.\nThe CEL expression template to apply to transform messages.\nThe following CEL extension functions are provided for\nuse in this CEL expression:\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toDestinationPayloadFormat():\nmessage.data.toDestinationPayloadFormat() -> string or bytes\n- Converts the message data to the destination payload format\nspecified in Pipeline.Destination.output_payload_format\n- This function is meant to be applied to the message.data field.\n- If the destination payload format is not set, the function will\nreturn the message data unchanged.\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents\n- This function applies toDestinationPayloadFormat() to the\nmessage data. It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\nPipeline.Destination.output_payload_format. If no\noutput_payload_format is set it will use the existing\ndatacontenttype on the CloudEvent if present, else leave\ndatacontenttype absent.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function."]
    pub fn transformation_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transformation_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcPipeline_MediationsDynamic {
    transformation: std::option::Option<terrars::DynamicBlock<EventarcPipeline_Mediations_Transformation>>,
}

#[derive(Serialize)]
pub struct EventarcPipeline_Mediations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation: std::option::Option<std::vec::Vec<EventarcPipeline_Mediations_Transformation>>,
    dynamic: EventarcPipeline_MediationsDynamic,
}

impl EventarcPipeline_Mediations {
    #[doc = "Set the field `transformation`.\n"]
    pub fn set_transformation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcPipeline_Mediations_Transformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_Mediations {
    type O = terrars::BlockAssignable<EventarcPipeline_Mediations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Mediations {}

impl BuildEventarcPipeline_Mediations {
    pub fn build(self) -> EventarcPipeline_Mediations {
        EventarcPipeline_Mediations {
            transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_MediationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_MediationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_MediationsRef {
        EventarcPipeline_MediationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_MediationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `transformation` after provisioning.\n"]
    pub fn transformation(&self) -> terrars::ListRef<EventarcPipeline_Mediations_TransformationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retry_delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_retry_delay: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcPipeline_RetryPolicy {
    #[doc =
        "Set the field `max_attempts`.\nThe maximum number of delivery attempts for any message. The value must\nbe between 1 and 100.\nThe default value for this field is 5."]
    pub fn set_max_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_retry_delay`.\nThe maximum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 60."]
    pub fn set_max_retry_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_retry_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_retry_delay`.\nThe minimum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 5."]
    pub fn set_min_retry_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_retry_delay = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcPipeline_RetryPolicy {
    type O = terrars::BlockAssignable<EventarcPipeline_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_RetryPolicy {}

impl BuildEventarcPipeline_RetryPolicy {
    pub fn build(self) -> EventarcPipeline_RetryPolicy {
        EventarcPipeline_RetryPolicy {
            max_attempts: core::default::Default::default(),
            max_retry_delay: core::default::Default::default(),
            min_retry_delay: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_RetryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_RetryPolicyRef {
        EventarcPipeline_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_attempts` after provisioning.\nThe maximum number of delivery attempts for any message. The value must\nbe between 1 and 100.\nThe default value for this field is 5."]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_retry_delay` after provisioning.\nThe maximum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 60."]
    pub fn max_retry_delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retry_delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_retry_delay` after provisioning.\nThe minimum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 5."]
    pub fn min_retry_delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_retry_delay", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcPipeline_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcPipeline_Timeouts {
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

impl terrars::ToListMappable for EventarcPipeline_Timeouts {
    type O = terrars::BlockAssignable<EventarcPipeline_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcPipeline_Timeouts {}

impl BuildEventarcPipeline_Timeouts {
    pub fn build(self) -> EventarcPipeline_Timeouts {
        EventarcPipeline_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EventarcPipeline_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcPipeline_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcPipeline_TimeoutsRef {
        EventarcPipeline_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcPipeline_TimeoutsRef {
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
