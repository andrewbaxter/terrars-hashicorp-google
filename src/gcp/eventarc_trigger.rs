use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EventarcTriggerData {
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
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_data_content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<EventarcTriggerDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matching_criteria: std::option::Option<std::vec::Vec<EventarcTriggerMatchingCriteriaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EventarcTriggerTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transport: std::option::Option<std::vec::Vec<EventarcTriggerTransportEl>>,
    dynamic: EventarcTriggerDynamic,
}

struct EventarcTrigger_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EventarcTriggerData>,
}

#[derive(Clone)]
pub struct EventarcTrigger(std::rc::Rc<EventarcTrigger_>);

impl EventarcTrigger {
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
        "Set the field `channel`.\nOptional. The name of the channel associated with the trigger in `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from Eventarc SaaS partners."]
    pub fn set_channel(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().channel = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `event_data_content_type`.\nOptional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to `application/json` if the value is not defined."]
    pub fn set_event_data_content_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().event_data_content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. User labels attached to the triggers that can be used to group resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
        "Set the field `service_account`.\nOptional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have `roles/eventarc.eventReceiver` IAM role."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcTriggerDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `matching_criteria`.\n"]
    pub fn set_matching_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcTriggerMatchingCriteriaEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().matching_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.matching_criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EventarcTriggerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transport`.\n"]
    pub fn set_transport(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcTriggerTransportEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transport = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transport = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nOptional. The name of the channel associated with the trigger in `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from Eventarc SaaS partners."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nOutput only. The reason(s) why a trigger is in FAILED state."]
    pub fn conditions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_data_content_type` after provisioning.\nOptional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to `application/json` if the value is not defined."]
    pub fn event_data_content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_data_content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. User labels attached to the triggers that can be used to group resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nRequired. The resource name of the trigger. Must be unique within the location on the project."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have `roles/eventarc.eventReceiver` IAM role."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<EventarcTriggerDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcTriggerTimeoutsElRef {
        EventarcTriggerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport` after provisioning.\n"]
    pub fn transport(&self) -> terrars::ListRef<EventarcTriggerTransportElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transport", self.extract_ref()))
    }
}

impl terrars::Referable for EventarcTrigger {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EventarcTrigger { }

impl terrars::ToListMappable for EventarcTrigger {
    type O = terrars::ListRef<EventarcTriggerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EventarcTrigger_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_eventarc_trigger".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEventarcTrigger {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Required. The resource name of the trigger. Must be unique within the location on the project."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEventarcTrigger {
    pub fn build(self, stack: &mut terrars::Stack) -> EventarcTrigger {
        let out = EventarcTrigger(std::rc::Rc::new(EventarcTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EventarcTriggerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                channel: core::default::Default::default(),
                event_data_content_type: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                destination: core::default::Default::default(),
                matching_criteria: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transport: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EventarcTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EventarcTriggerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nOptional. The name of the channel associated with the trigger in `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from Eventarc SaaS partners."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nOutput only. The reason(s) why a trigger is in FAILED state."]
    pub fn conditions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_data_content_type` after provisioning.\nOptional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to `application/json` if the value is not defined."]
    pub fn event_data_content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_data_content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. User labels attached to the triggers that can be used to group resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nRequired. The resource name of the trigger. Must be unique within the location on the project."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have `roles/eventarc.eventReceiver` IAM role."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last-modified time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<EventarcTriggerDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcTriggerTimeoutsElRef {
        EventarcTriggerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport` after provisioning.\n"]
    pub fn transport(&self) -> terrars::ListRef<EventarcTriggerTransportElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transport", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EventarcTriggerDestinationElCloudRunServiceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl EventarcTriggerDestinationElCloudRunServiceEl {
    #[doc =
        "Set the field `path`.\nOptional. The relative path on the Cloud Run service the events should be sent to. The value must conform to the definition of URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRequired. The region the Cloud Run service is deployed in."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcTriggerDestinationElCloudRunServiceEl {
    type O = terrars::BlockAssignable<EventarcTriggerDestinationElCloudRunServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcTriggerDestinationElCloudRunServiceEl {
    #[doc =
        "Required. The name of the Cloud Run service being addressed. See https://cloud.google.com/run/docs/reference/rest/v1/namespaces.services. Only services located in the same project of the trigger object can be addressed."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildEventarcTriggerDestinationElCloudRunServiceEl {
    pub fn build(self) -> EventarcTriggerDestinationElCloudRunServiceEl {
        EventarcTriggerDestinationElCloudRunServiceEl {
            path: core::default::Default::default(),
            region: core::default::Default::default(),
            service: self.service,
        }
    }
}

pub struct EventarcTriggerDestinationElCloudRunServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerDestinationElCloudRunServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EventarcTriggerDestinationElCloudRunServiceElRef {
        EventarcTriggerDestinationElCloudRunServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcTriggerDestinationElCloudRunServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nOptional. The relative path on the Cloud Run service the events should be sent to. The value must conform to the definition of URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRequired. The region the Cloud Run service is deployed in."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nRequired. The name of the Cloud Run service being addressed. See https://cloud.google.com/run/docs/reference/rest/v1/namespaces.services. Only services located in the same project of the trigger object can be addressed."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcTriggerDestinationElGkeEl {
    cluster: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl EventarcTriggerDestinationElGkeEl {
    #[doc =
        "Set the field `path`.\nOptional. The relative path on the GKE service the events should be sent to. The value must conform to the definition of a URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcTriggerDestinationElGkeEl {
    type O = terrars::BlockAssignable<EventarcTriggerDestinationElGkeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcTriggerDestinationElGkeEl {
    #[doc =
        "Required. The name of the cluster the GKE service is running in. The cluster must be running in the same project as the trigger being created."]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The name of the Google Compute Engine in which the cluster resides, which can either be compute zone (for example, us-central1-a) for the zonal clusters or region (for example, us-central1) for regional clusters."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Required. The namespace the GKE service is running in."]
    pub namespace: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of the GKE service."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildEventarcTriggerDestinationElGkeEl {
    pub fn build(self) -> EventarcTriggerDestinationElGkeEl {
        EventarcTriggerDestinationElGkeEl {
            cluster: self.cluster,
            location: self.location,
            namespace: self.namespace,
            path: core::default::Default::default(),
            service: self.service,
        }
    }
}

pub struct EventarcTriggerDestinationElGkeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerDestinationElGkeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcTriggerDestinationElGkeElRef {
        EventarcTriggerDestinationElGkeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcTriggerDestinationElGkeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nRequired. The name of the cluster the GKE service is running in. The cluster must be running in the same project as the trigger being created."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nRequired. The name of the Google Compute Engine in which the cluster resides, which can either be compute zone (for example, us-central1-a) for the zonal clusters or region (for example, us-central1) for regional clusters."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nRequired. The namespace the GKE service is running in."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nOptional. The relative path on the GKE service the events should be sent to. The value must conform to the definition of a URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the GKE service."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcTriggerDestinationElDynamic {
    cloud_run_service: std::option::Option<terrars::DynamicBlock<EventarcTriggerDestinationElCloudRunServiceEl>>,
    gke: std::option::Option<terrars::DynamicBlock<EventarcTriggerDestinationElGkeEl>>,
}

#[derive(Serialize)]
pub struct EventarcTriggerDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_function: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_run_service: std::option::Option<std::vec::Vec<EventarcTriggerDestinationElCloudRunServiceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke: std::option::Option<std::vec::Vec<EventarcTriggerDestinationElGkeEl>>,
    dynamic: EventarcTriggerDestinationElDynamic,
}

impl EventarcTriggerDestinationEl {
    #[doc =
        "Set the field `cloud_function`.\n[WARNING] Configuring a Cloud Function in Trigger is not supported as of today. The Cloud Function resource name. Format: projects/{project}/locations/{location}/functions/{function}"]
    pub fn set_cloud_function(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_function = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `workflow`.\nThe resource name of the Workflow whose Executions are triggered by the events. The Workflow resource should be deployed in the same project as the trigger. Format: `projects/{project}/locations/{location}/workflows/{workflow}`"]
    pub fn set_workflow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workflow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_run_service`.\n"]
    pub fn set_cloud_run_service(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcTriggerDestinationElCloudRunServiceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_run_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_run_service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gke`.\n"]
    pub fn set_gke(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcTriggerDestinationElGkeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gke = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gke = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EventarcTriggerDestinationEl {
    type O = terrars::BlockAssignable<EventarcTriggerDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcTriggerDestinationEl {}

impl BuildEventarcTriggerDestinationEl {
    pub fn build(self) -> EventarcTriggerDestinationEl {
        EventarcTriggerDestinationEl {
            cloud_function: core::default::Default::default(),
            workflow: core::default::Default::default(),
            cloud_run_service: core::default::Default::default(),
            gke: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EventarcTriggerDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcTriggerDestinationElRef {
        EventarcTriggerDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcTriggerDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_function` after provisioning.\n[WARNING] Configuring a Cloud Function in Trigger is not supported as of today. The Cloud Function resource name. Format: projects/{project}/locations/{location}/functions/{function}"]
    pub fn cloud_function(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_function", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workflow` after provisioning.\nThe resource name of the Workflow whose Executions are triggered by the events. The Workflow resource should be deployed in the same project as the trigger. Format: `projects/{project}/locations/{location}/workflows/{workflow}`"]
    pub fn workflow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_run_service` after provisioning.\n"]
    pub fn cloud_run_service(&self) -> terrars::ListRef<EventarcTriggerDestinationElCloudRunServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run_service", self.base))
    }

    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> terrars::ListRef<EventarcTriggerDestinationElGkeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcTriggerMatchingCriteriaEl {
    attribute: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl EventarcTriggerMatchingCriteriaEl {
    #[doc =
        "Set the field `operator`.\nOptional. The operator used for matching the events with the value of the filter. If not specified, only events that have an exact key-value pair specified in the filter are matched. The only allowed value is `match-path-pattern`."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcTriggerMatchingCriteriaEl {
    type O = terrars::BlockAssignable<EventarcTriggerMatchingCriteriaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcTriggerMatchingCriteriaEl {
    #[doc =
        "Required. The name of a CloudEvents attribute. Currently, only a subset of attributes are supported for filtering. All triggers MUST provide a filter for the 'type' attribute."]
    pub attribute: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The value for the attribute. See https://cloud.google.com/eventarc/docs/creating-triggers#trigger-gcloud for available values."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildEventarcTriggerMatchingCriteriaEl {
    pub fn build(self) -> EventarcTriggerMatchingCriteriaEl {
        EventarcTriggerMatchingCriteriaEl {
            attribute: self.attribute,
            operator: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct EventarcTriggerMatchingCriteriaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerMatchingCriteriaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcTriggerMatchingCriteriaElRef {
        EventarcTriggerMatchingCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcTriggerMatchingCriteriaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute` after provisioning.\nRequired. The name of a CloudEvents attribute. Currently, only a subset of attributes are supported for filtering. All triggers MUST provide a filter for the 'type' attribute."]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nOptional. The operator used for matching the events with the value of the filter. If not specified, only events that have an exact key-value pair specified in the filter are matched. The only allowed value is `match-path-pattern`."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nRequired. The value for the attribute. See https://cloud.google.com/eventarc/docs/creating-triggers#trigger-gcloud for available values."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EventarcTriggerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcTriggerTimeoutsEl {
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

impl terrars::ToListMappable for EventarcTriggerTimeoutsEl {
    type O = terrars::BlockAssignable<EventarcTriggerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcTriggerTimeoutsEl {}

impl BuildEventarcTriggerTimeoutsEl {
    pub fn build(self) -> EventarcTriggerTimeoutsEl {
        EventarcTriggerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EventarcTriggerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcTriggerTimeoutsElRef {
        EventarcTriggerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcTriggerTimeoutsElRef {
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

#[derive(Serialize)]
pub struct EventarcTriggerTransportElPubsubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EventarcTriggerTransportElPubsubEl {
    #[doc =
        "Set the field `topic`.\nOptional. The name of the Pub/Sub topic created and managed by Eventarc system as a transport for the event delivery. Format: `projects/{PROJECT_ID}/topics/{TOPIC_NAME}. You may set an existing topic for triggers of the type google.cloud.pubsub.topic.v1.messagePublished` only. The topic you provide here will not be deleted by Eventarc at trigger deletion."]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EventarcTriggerTransportElPubsubEl {
    type O = terrars::BlockAssignable<EventarcTriggerTransportElPubsubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcTriggerTransportElPubsubEl {}

impl BuildEventarcTriggerTransportElPubsubEl {
    pub fn build(self) -> EventarcTriggerTransportElPubsubEl {
        EventarcTriggerTransportElPubsubEl { topic: core::default::Default::default() }
    }
}

pub struct EventarcTriggerTransportElPubsubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerTransportElPubsubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcTriggerTransportElPubsubElRef {
        EventarcTriggerTransportElPubsubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcTriggerTransportElPubsubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `subscription` after provisioning.\nOutput only. The name of the Pub/Sub subscription created and managed by Eventarc system as a transport for the event delivery. Format: `projects/{PROJECT_ID}/subscriptions/{SUBSCRIPTION_NAME}`."]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nOptional. The name of the Pub/Sub topic created and managed by Eventarc system as a transport for the event delivery. Format: `projects/{PROJECT_ID}/topics/{TOPIC_NAME}. You may set an existing topic for triggers of the type google.cloud.pubsub.topic.v1.messagePublished` only. The topic you provide here will not be deleted by Eventarc at trigger deletion."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcTriggerTransportElDynamic {
    pubsub: std::option::Option<terrars::DynamicBlock<EventarcTriggerTransportElPubsubEl>>,
}

#[derive(Serialize)]
pub struct EventarcTriggerTransportEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub: std::option::Option<std::vec::Vec<EventarcTriggerTransportElPubsubEl>>,
    dynamic: EventarcTriggerTransportElDynamic,
}

impl EventarcTriggerTransportEl {
    #[doc = "Set the field `pubsub`.\n"]
    pub fn set_pubsub(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EventarcTriggerTransportElPubsubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EventarcTriggerTransportEl {
    type O = terrars::BlockAssignable<EventarcTriggerTransportEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEventarcTriggerTransportEl {}

impl BuildEventarcTriggerTransportEl {
    pub fn build(self) -> EventarcTriggerTransportEl {
        EventarcTriggerTransportEl {
            pubsub: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EventarcTriggerTransportElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EventarcTriggerTransportElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EventarcTriggerTransportElRef {
        EventarcTriggerTransportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EventarcTriggerTransportElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pubsub` after provisioning.\n"]
    pub fn pubsub(&self) -> terrars::ListRef<EventarcTriggerTransportElPubsubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub", self.base))
    }
}

#[derive(Serialize, Default)]
struct EventarcTriggerDynamic {
    destination: std::option::Option<terrars::DynamicBlock<EventarcTriggerDestinationEl>>,
    matching_criteria: std::option::Option<terrars::DynamicBlock<EventarcTriggerMatchingCriteriaEl>>,
    transport: std::option::Option<terrars::DynamicBlock<EventarcTriggerTransportEl>>,
}
