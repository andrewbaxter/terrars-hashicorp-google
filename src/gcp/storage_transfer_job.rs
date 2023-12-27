use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageTransferJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_stream: Option<Vec<StorageTransferJobEventStreamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config: Option<Vec<StorageTransferJobNotificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<StorageTransferJobScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_spec: Option<Vec<StorageTransferJobTransferSpecEl>>,
    dynamic: StorageTransferJobDynamic,
}

struct StorageTransferJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageTransferJobData>,
}

#[derive(Clone)]
pub struct StorageTransferJob(Rc<StorageTransferJob_>);

impl StorageTransferJob {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc= "Set the field `event_stream`.\n"]
    pub fn set_event_stream(self, v: impl Into<BlockAssignable<StorageTransferJobEventStreamEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_stream = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_stream = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `notification_config`.\n"]
    pub fn set_notification_config(self, v: impl Into<BlockAssignable<StorageTransferJobNotificationConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<BlockAssignable<StorageTransferJobScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `transfer_spec`.\n"]
    pub fn set_transfer_spec(self, v: impl Into<BlockAssignable<StorageTransferJobTransferSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transfer_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transfer_spec = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\nWhen the Transfer Job was created."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_time` after provisioning.\nWhen the Transfer Job was deleted."]
    pub fn deletion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUnique description to identify the Transfer Job."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modification_time` after provisioning.\nWhen the Transfer Job was last modified."]
    pub fn last_modification_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modification_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Transfer Job."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_stream` after provisioning.\n"]
    pub fn event_stream(&self) -> ListRef<StorageTransferJobEventStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_stream", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<StorageTransferJobNotificationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<StorageTransferJobScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transfer_spec` after provisioning.\n"]
    pub fn transfer_spec(&self) -> ListRef<StorageTransferJobTransferSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transfer_spec", self.extract_ref()))
    }
}

impl Referable for StorageTransferJob {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for StorageTransferJob { }

impl ToListMappable for StorageTransferJob {
    type O = ListRef<StorageTransferJobRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for StorageTransferJob_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_transfer_job".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageTransferJob {
    pub tf_id: String,
    #[doc= "Unique description to identify the Transfer Job."]
    pub description: PrimField<String>,
}

impl BuildStorageTransferJob {
    pub fn build(self, stack: &mut Stack) -> StorageTransferJob {
        let out = StorageTransferJob(Rc::new(StorageTransferJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageTransferJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: self.description,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                status: core::default::Default::default(),
                event_stream: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                schedule: core::default::Default::default(),
                transfer_spec: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageTransferJobRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageTransferJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\nWhen the Transfer Job was created."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_time` after provisioning.\nWhen the Transfer Job was deleted."]
    pub fn deletion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUnique description to identify the Transfer Job."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modification_time` after provisioning.\nWhen the Transfer Job was last modified."]
    pub fn last_modification_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modification_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Transfer Job."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_stream` after provisioning.\n"]
    pub fn event_stream(&self) -> ListRef<StorageTransferJobEventStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_stream", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<StorageTransferJobNotificationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<StorageTransferJobScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transfer_spec` after provisioning.\n"]
    pub fn transfer_spec(&self) -> ListRef<StorageTransferJobTransferSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transfer_spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobEventStreamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_stream_expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_stream_start_time: Option<PrimField<String>>,
    name: PrimField<String>,
}

impl StorageTransferJobEventStreamEl {
    #[doc= "Set the field `event_stream_expiration_time`.\nSpecifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated"]
    pub fn set_event_stream_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_stream_expiration_time = Some(v.into());
        self
    }

    #[doc= "Set the field `event_stream_start_time`.\nSpecifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately"]
    pub fn set_event_stream_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_stream_start_time = Some(v.into());
        self
    }
}

impl ToListMappable for StorageTransferJobEventStreamEl {
    type O = BlockAssignable<StorageTransferJobEventStreamEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobEventStreamEl {
    #[doc= "Specifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'"]
    pub name: PrimField<String>,
}

impl BuildStorageTransferJobEventStreamEl {
    pub fn build(self) -> StorageTransferJobEventStreamEl {
        StorageTransferJobEventStreamEl {
            event_stream_expiration_time: core::default::Default::default(),
            event_stream_start_time: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct StorageTransferJobEventStreamElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobEventStreamElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobEventStreamElRef {
        StorageTransferJobEventStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobEventStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `event_stream_expiration_time` after provisioning.\nSpecifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated"]
    pub fn event_stream_expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_stream_expiration_time", self.base))
    }

    #[doc= "Get a reference to the value of field `event_stream_start_time` after provisioning.\nSpecifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately"]
    pub fn event_stream_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_stream_start_time", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nSpecifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_types: Option<SetField<PrimField<String>>>,
    payload_format: PrimField<String>,
    pubsub_topic: PrimField<String>,
}

impl StorageTransferJobNotificationConfigEl {
    #[doc= "Set the field `event_types`.\nEvent types for which a notification is desired. If empty, send notifications for all event types. The valid types are \"TRANSFER_OPERATION_SUCCESS\", \"TRANSFER_OPERATION_FAILED\", \"TRANSFER_OPERATION_ABORTED\"."]
    pub fn set_event_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.event_types = Some(v.into());
        self
    }
}

impl ToListMappable for StorageTransferJobNotificationConfigEl {
    type O = BlockAssignable<StorageTransferJobNotificationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobNotificationConfigEl {
    #[doc= "The desired format of the notification message payloads. One of \"NONE\" or \"JSON\"."]
    pub payload_format: PrimField<String>,
    #[doc= "The Topic.name of the Pub/Sub topic to which to publish notifications."]
    pub pubsub_topic: PrimField<String>,
}

impl BuildStorageTransferJobNotificationConfigEl {
    pub fn build(self) -> StorageTransferJobNotificationConfigEl {
        StorageTransferJobNotificationConfigEl {
            event_types: core::default::Default::default(),
            payload_format: self.payload_format,
            pubsub_topic: self.pubsub_topic,
        }
    }
}

pub struct StorageTransferJobNotificationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobNotificationConfigElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobNotificationConfigElRef {
        StorageTransferJobNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `event_types` after provisioning.\nEvent types for which a notification is desired. If empty, send notifications for all event types. The valid types are \"TRANSFER_OPERATION_SUCCESS\", \"TRANSFER_OPERATION_FAILED\", \"TRANSFER_OPERATION_ABORTED\"."]
    pub fn event_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.event_types", self.base))
    }

    #[doc= "Get a reference to the value of field `payload_format` after provisioning.\nThe desired format of the notification message payloads. One of \"NONE\" or \"JSON\"."]
    pub fn payload_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload_format", self.base))
    }

    #[doc= "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Topic.name of the Pub/Sub topic to which to publish notifications."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobScheduleElScheduleEndDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}

impl StorageTransferJobScheduleElScheduleEndDateEl { }

impl ToListMappable for StorageTransferJobScheduleElScheduleEndDateEl {
    type O = BlockAssignable<StorageTransferJobScheduleElScheduleEndDateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobScheduleElScheduleEndDateEl {
    #[doc= "Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: PrimField<f64>,
    #[doc= "Month of year. Must be from 1 to 12."]
    pub month: PrimField<f64>,
    #[doc= "Year of date. Must be from 1 to 9999."]
    pub year: PrimField<f64>,
}

impl BuildStorageTransferJobScheduleElScheduleEndDateEl {
    pub fn build(self) -> StorageTransferJobScheduleElScheduleEndDateEl {
        StorageTransferJobScheduleElScheduleEndDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

pub struct StorageTransferJobScheduleElScheduleEndDateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobScheduleElScheduleEndDateElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobScheduleElScheduleEndDateElRef {
        StorageTransferJobScheduleElScheduleEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobScheduleElScheduleEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day` after provisioning.\nDay of month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc= "Get a reference to the value of field `month` after provisioning.\nMonth of year. Must be from 1 to 12."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc= "Get a reference to the value of field `year` after provisioning.\nYear of date. Must be from 1 to 9999."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobScheduleElScheduleStartDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}

impl StorageTransferJobScheduleElScheduleStartDateEl { }

impl ToListMappable for StorageTransferJobScheduleElScheduleStartDateEl {
    type O = BlockAssignable<StorageTransferJobScheduleElScheduleStartDateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobScheduleElScheduleStartDateEl {
    #[doc= "Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: PrimField<f64>,
    #[doc= "Month of year. Must be from 1 to 12."]
    pub month: PrimField<f64>,
    #[doc= "Year of date. Must be from 1 to 9999."]
    pub year: PrimField<f64>,
}

impl BuildStorageTransferJobScheduleElScheduleStartDateEl {
    pub fn build(self) -> StorageTransferJobScheduleElScheduleStartDateEl {
        StorageTransferJobScheduleElScheduleStartDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

pub struct StorageTransferJobScheduleElScheduleStartDateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobScheduleElScheduleStartDateElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobScheduleElScheduleStartDateElRef {
        StorageTransferJobScheduleElScheduleStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobScheduleElScheduleStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day` after provisioning.\nDay of month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc= "Get a reference to the value of field `month` after provisioning.\nMonth of year. Must be from 1 to 12."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc= "Get a reference to the value of field `year` after provisioning.\nYear of date. Must be from 1 to 9999."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobScheduleElStartTimeOfDayEl {
    hours: PrimField<f64>,
    minutes: PrimField<f64>,
    nanos: PrimField<f64>,
    seconds: PrimField<f64>,
}

impl StorageTransferJobScheduleElStartTimeOfDayEl { }

impl ToListMappable for StorageTransferJobScheduleElStartTimeOfDayEl {
    type O = BlockAssignable<StorageTransferJobScheduleElStartTimeOfDayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobScheduleElStartTimeOfDayEl {
    #[doc= "Hours of day in 24 hour format. Should be from 0 to 23."]
    pub hours: PrimField<f64>,
    #[doc= "Minutes of hour of day. Must be from 0 to 59."]
    pub minutes: PrimField<f64>,
    #[doc= "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub nanos: PrimField<f64>,
    #[doc= "Seconds of minutes of the time. Must normally be from 0 to 59."]
    pub seconds: PrimField<f64>,
}

impl BuildStorageTransferJobScheduleElStartTimeOfDayEl {
    pub fn build(self) -> StorageTransferJobScheduleElStartTimeOfDayEl {
        StorageTransferJobScheduleElStartTimeOfDayEl {
            hours: self.hours,
            minutes: self.minutes,
            nanos: self.nanos,
            seconds: self.seconds,
        }
    }
}

pub struct StorageTransferJobScheduleElStartTimeOfDayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobScheduleElStartTimeOfDayElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobScheduleElStartTimeOfDayElRef {
        StorageTransferJobScheduleElStartTimeOfDayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobScheduleElStartTimeOfDayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc= "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJobScheduleElDynamic {
    schedule_end_date: Option<DynamicBlock<StorageTransferJobScheduleElScheduleEndDateEl>>,
    schedule_start_date: Option<DynamicBlock<StorageTransferJobScheduleElScheduleStartDateEl>>,
    start_time_of_day: Option<DynamicBlock<StorageTransferJobScheduleElStartTimeOfDayEl>>,
}

#[derive(Serialize)]
pub struct StorageTransferJobScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_end_date: Option<Vec<StorageTransferJobScheduleElScheduleEndDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_start_date: Option<Vec<StorageTransferJobScheduleElScheduleStartDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_of_day: Option<Vec<StorageTransferJobScheduleElStartTimeOfDayEl>>,
    dynamic: StorageTransferJobScheduleElDynamic,
}

impl StorageTransferJobScheduleEl {
    #[doc= "Set the field `repeat_interval`.\nInterval between the start of each scheduled transfer. If unspecified, the default value is 24 hours. This value may not be less than 1 hour. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_repeat_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repeat_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `schedule_end_date`.\n"]
    pub fn set_schedule_end_date(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobScheduleElScheduleEndDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule_end_date = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_end_date = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `schedule_start_date`.\n"]
    pub fn set_schedule_start_date(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobScheduleElScheduleStartDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule_start_date = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_start_date = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `start_time_of_day`.\n"]
    pub fn set_start_time_of_day(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobScheduleElStartTimeOfDayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time_of_day = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time_of_day = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for StorageTransferJobScheduleEl {
    type O = BlockAssignable<StorageTransferJobScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobScheduleEl {}

impl BuildStorageTransferJobScheduleEl {
    pub fn build(self) -> StorageTransferJobScheduleEl {
        StorageTransferJobScheduleEl {
            repeat_interval: core::default::Default::default(),
            schedule_end_date: core::default::Default::default(),
            schedule_start_date: core::default::Default::default(),
            start_time_of_day: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct StorageTransferJobScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobScheduleElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobScheduleElRef {
        StorageTransferJobScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `repeat_interval` after provisioning.\nInterval between the start of each scheduled transfer. If unspecified, the default value is 24 hours. This value may not be less than 1 hour. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn repeat_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repeat_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `schedule_end_date` after provisioning.\n"]
    pub fn schedule_end_date(&self) -> ListRef<StorageTransferJobScheduleElScheduleEndDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule_end_date", self.base))
    }

    #[doc= "Get a reference to the value of field `schedule_start_date` after provisioning.\n"]
    pub fn schedule_start_date(&self) -> ListRef<StorageTransferJobScheduleElScheduleStartDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule_start_date", self.base))
    }

    #[doc= "Get a reference to the value of field `start_time_of_day` after provisioning.\n"]
    pub fn start_time_of_day(&self) -> ListRef<StorageTransferJobScheduleElStartTimeOfDayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time_of_day", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    access_key_id: PrimField<String>,
    secret_access_key: PrimField<String>,
}

impl StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl { }

impl ToListMappable for StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    #[doc= "AWS Key ID."]
    pub access_key_id: PrimField<String>,
    #[doc= "AWS Secret Access Key."]
    pub secret_access_key: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
        StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
        }
    }
}

pub struct StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
        StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_key_id` after provisioning.\nAWS Key ID."]
    pub fn access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_access_key` after provisioning.\nAWS Secret Access Key."]
    pub fn secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElAwsS3DataSourceElDynamic {
    aws_access_key: Option<DynamicBlock<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>>,
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAwsS3DataSourceEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_access_key: Option<Vec<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>>,
    dynamic: StorageTransferJobTransferSpecElAwsS3DataSourceElDynamic,
}

impl StorageTransferJobTransferSpecElAwsS3DataSourceEl {
    #[doc= "Set the field `path`.\nS3 Bucket path in bucket to transfer."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `role_arn`.\nThe Amazon Resource Name (ARN) of the role to support temporary credentials via 'AssumeRoleWithWebIdentity'. For more information about ARNs, see [IAM ARNs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns). When a role ARN is provided, Transfer Service fetches temporary credentials for the session using a 'AssumeRoleWithWebIdentity' call for the provided role using the [GoogleServiceAccount][] for this project."]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `aws_access_key`.\n"]
    pub fn set_aws_access_key(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_access_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_access_key = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for StorageTransferJobTransferSpecElAwsS3DataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElAwsS3DataSourceEl {
    #[doc= "S3 Bucket name."]
    pub bucket_name: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElAwsS3DataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAwsS3DataSourceEl {
        StorageTransferJobTransferSpecElAwsS3DataSourceEl {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            aws_access_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
        StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\nS3 Bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nS3 Bucket path in bucket to transfer."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the role to support temporary credentials via 'AssumeRoleWithWebIdentity'. For more information about ARNs, see [IAM ARNs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns). When a role ARN is provided, Transfer Service fetches temporary credentials for the session using a 'AssumeRoleWithWebIdentity' call for the provided role using the [GoogleServiceAccount][] for this project."]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `aws_access_key` after provisioning.\n"]
    pub fn aws_access_key(&self) -> ListRef<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_access_key", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
    sas_token: PrimField<String>,
}

impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl { }

impl ToListMappable for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
    #[doc= "Azure shared access signature."]
    pub sas_token: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl { sas_token: self.sas_token }
    }
}

pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `sas_token` after provisioning.\nAzure shared access signature."]
    pub fn sas_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sas_token", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElDynamic {
    azure_credentials: Option<
        DynamicBlock<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl>,
    >,
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    container: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    storage_account: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure_credentials: Option<Vec<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl>>,
    dynamic: StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElDynamic,
}

impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    #[doc= "Set the field `path`.\nRoot path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `azure_credentials`.\n"]
    pub fn set_azure_credentials(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure_credentials = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_credentials = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    #[doc= "The container to transfer from the Azure Storage account."]
    pub container: PrimField<String>,
    #[doc= "The name of the Azure Storage account."]
    pub storage_account: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
            container: self.container,
            path: core::default::Default::default(),
            storage_account: self.storage_account,
            azure_credentials: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `container` after provisioning.\nThe container to transfer from the Azure Storage account."]
    pub fn container(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nRoot path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `storage_account` after provisioning.\nThe name of the Azure Storage account."]
    pub fn storage_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_account", self.base))
    }

    #[doc= "Get a reference to the value of field `azure_credentials` after provisioning.\n"]
    pub fn azure_credentials(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElGcsDataSinkEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl StorageTransferJobTransferSpecElGcsDataSinkEl {
    #[doc= "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for StorageTransferJobTransferSpecElGcsDataSinkEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElGcsDataSinkEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElGcsDataSinkEl {
    #[doc= "Google Cloud Storage bucket name."]
    pub bucket_name: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElGcsDataSinkEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElGcsDataSinkEl {
        StorageTransferJobTransferSpecElGcsDataSinkEl {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}

pub struct StorageTransferJobTransferSpecElGcsDataSinkElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElGcsDataSinkElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElGcsDataSinkElRef {
        StorageTransferJobTransferSpecElGcsDataSinkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElGcsDataSinkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElGcsDataSourceEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl StorageTransferJobTransferSpecElGcsDataSourceEl {
    #[doc= "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for StorageTransferJobTransferSpecElGcsDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElGcsDataSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElGcsDataSourceEl {
    #[doc= "Google Cloud Storage bucket name."]
    pub bucket_name: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElGcsDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElGcsDataSourceEl {
        StorageTransferJobTransferSpecElGcsDataSourceEl {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}

pub struct StorageTransferJobTransferSpecElGcsDataSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElGcsDataSourceElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElGcsDataSourceElRef {
        StorageTransferJobTransferSpecElGcsDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElGcsDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElHttpDataSourceEl {
    list_url: PrimField<String>,
}

impl StorageTransferJobTransferSpecElHttpDataSourceEl { }

impl ToListMappable for StorageTransferJobTransferSpecElHttpDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElHttpDataSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElHttpDataSourceEl {
    #[doc= "The URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported."]
    pub list_url: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElHttpDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElHttpDataSourceEl {
        StorageTransferJobTransferSpecElHttpDataSourceEl { list_url: self.list_url }
    }
}

pub struct StorageTransferJobTransferSpecElHttpDataSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElHttpDataSourceElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElHttpDataSourceElRef {
        StorageTransferJobTransferSpecElHttpDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElHttpDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `list_url` after provisioning.\nThe URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported."]
    pub fn list_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_url", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElObjectConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_since: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_time_elapsed_since_last_modification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_time_elapsed_since_last_modification: Option<PrimField<String>>,
}

impl StorageTransferJobTransferSpecElObjectConditionsEl {
    #[doc= "Set the field `exclude_prefixes`.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn set_exclude_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_prefixes = Some(v.into());
        self
    }

    #[doc= "Set the field `include_prefixes`.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn set_include_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_prefixes = Some(v.into());
        self
    }

    #[doc= "Set the field `last_modified_before`.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified_before = Some(v.into());
        self
    }

    #[doc= "Set the field `last_modified_since`.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_since(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified_since = Some(v.into());
        self
    }

    #[doc= "Set the field `max_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_max_time_elapsed_since_last_modification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_time_elapsed_since_last_modification = Some(v.into());
        self
    }

    #[doc= "Set the field `min_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_min_time_elapsed_since_last_modification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_time_elapsed_since_last_modification = Some(v.into());
        self
    }
}

impl ToListMappable for StorageTransferJobTransferSpecElObjectConditionsEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElObjectConditionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElObjectConditionsEl {}

impl BuildStorageTransferJobTransferSpecElObjectConditionsEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElObjectConditionsEl {
        StorageTransferJobTransferSpecElObjectConditionsEl {
            exclude_prefixes: core::default::Default::default(),
            include_prefixes: core::default::Default::default(),
            last_modified_before: core::default::Default::default(),
            last_modified_since: core::default::Default::default(),
            max_time_elapsed_since_last_modification: core::default::Default::default(),
            min_time_elapsed_since_last_modification: core::default::Default::default(),
        }
    }
}

pub struct StorageTransferJobTransferSpecElObjectConditionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElObjectConditionsElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElObjectConditionsElRef {
        StorageTransferJobTransferSpecElObjectConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElObjectConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exclude_prefixes` after provisioning.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn exclude_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclude_prefixes", self.base))
    }

    #[doc= "Get a reference to the value of field `include_prefixes` after provisioning.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn include_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include_prefixes", self.base))
    }

    #[doc= "Get a reference to the value of field `last_modified_before` after provisioning.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_before", self.base))
    }

    #[doc= "Get a reference to the value of field `last_modified_since` after provisioning.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_since(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_since", self.base))
    }

    #[doc= "Get a reference to the value of field `max_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_time_elapsed_since_last_modification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_time_elapsed_since_last_modification", self.base))
    }

    #[doc= "Get a reference to the value of field `min_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn min_time_elapsed_since_last_modification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_time_elapsed_since_last_modification", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElPosixDataSinkEl {
    root_directory: PrimField<String>,
}

impl StorageTransferJobTransferSpecElPosixDataSinkEl { }

impl ToListMappable for StorageTransferJobTransferSpecElPosixDataSinkEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElPosixDataSinkEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElPosixDataSinkEl {
    #[doc= "Root directory path to the filesystem."]
    pub root_directory: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElPosixDataSinkEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElPosixDataSinkEl {
        StorageTransferJobTransferSpecElPosixDataSinkEl { root_directory: self.root_directory }
    }
}

pub struct StorageTransferJobTransferSpecElPosixDataSinkElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElPosixDataSinkElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElPosixDataSinkElRef {
        StorageTransferJobTransferSpecElPosixDataSinkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElPosixDataSinkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `root_directory` after provisioning.\nRoot directory path to the filesystem."]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElPosixDataSourceEl {
    root_directory: PrimField<String>,
}

impl StorageTransferJobTransferSpecElPosixDataSourceEl { }

impl ToListMappable for StorageTransferJobTransferSpecElPosixDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElPosixDataSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElPosixDataSourceEl {
    #[doc= "Root directory path to the filesystem."]
    pub root_directory: PrimField<String>,
}

impl BuildStorageTransferJobTransferSpecElPosixDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElPosixDataSourceEl {
        StorageTransferJobTransferSpecElPosixDataSourceEl { root_directory: self.root_directory }
    }
}

pub struct StorageTransferJobTransferSpecElPosixDataSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElPosixDataSourceElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElPosixDataSourceElRef {
        StorageTransferJobTransferSpecElPosixDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElPosixDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `root_directory` after provisioning.\nRoot directory path to the filesystem."]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElTransferOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_objects_from_source_after_transfer: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_objects_unique_in_sink: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_objects_already_existing_in_sink: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_when: Option<PrimField<String>>,
}

impl StorageTransferJobTransferSpecElTransferOptionsEl {
    #[doc= "Set the field `delete_objects_from_source_after_transfer`.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn set_delete_objects_from_source_after_transfer(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_objects_from_source_after_transfer = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_objects_unique_in_sink`.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn set_delete_objects_unique_in_sink(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_objects_unique_in_sink = Some(v.into());
        self
    }

    #[doc= "Set the field `overwrite_objects_already_existing_in_sink`.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn set_overwrite_objects_already_existing_in_sink(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.overwrite_objects_already_existing_in_sink = Some(v.into());
        self
    }

    #[doc= "Set the field `overwrite_when`.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn set_overwrite_when(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_when = Some(v.into());
        self
    }
}

impl ToListMappable for StorageTransferJobTransferSpecElTransferOptionsEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElTransferOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecElTransferOptionsEl {}

impl BuildStorageTransferJobTransferSpecElTransferOptionsEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElTransferOptionsEl {
        StorageTransferJobTransferSpecElTransferOptionsEl {
            delete_objects_from_source_after_transfer: core::default::Default::default(),
            delete_objects_unique_in_sink: core::default::Default::default(),
            overwrite_objects_already_existing_in_sink: core::default::Default::default(),
            overwrite_when: core::default::Default::default(),
        }
    }
}

pub struct StorageTransferJobTransferSpecElTransferOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElTransferOptionsElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElTransferOptionsElRef {
        StorageTransferJobTransferSpecElTransferOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElTransferOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delete_objects_from_source_after_transfer` after provisioning.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn delete_objects_from_source_after_transfer(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_objects_from_source_after_transfer", self.base))
    }

    #[doc= "Get a reference to the value of field `delete_objects_unique_in_sink` after provisioning.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn delete_objects_unique_in_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_objects_unique_in_sink", self.base))
    }

    #[doc= "Get a reference to the value of field `overwrite_objects_already_existing_in_sink` after provisioning.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn overwrite_objects_already_existing_in_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.overwrite_objects_already_existing_in_sink", self.base))
    }

    #[doc= "Get a reference to the value of field `overwrite_when` after provisioning.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn overwrite_when(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.overwrite_when", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElDynamic {
    aws_s3_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElAwsS3DataSourceEl>>,
    azure_blob_storage_data_source: Option<
        DynamicBlock<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>,
    >,
    gcs_data_sink: Option<DynamicBlock<StorageTransferJobTransferSpecElGcsDataSinkEl>>,
    gcs_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElGcsDataSourceEl>>,
    http_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElHttpDataSourceEl>>,
    object_conditions: Option<DynamicBlock<StorageTransferJobTransferSpecElObjectConditionsEl>>,
    posix_data_sink: Option<DynamicBlock<StorageTransferJobTransferSpecElPosixDataSinkEl>>,
    posix_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElPosixDataSourceEl>>,
    transfer_options: Option<DynamicBlock<StorageTransferJobTransferSpecElTransferOptionsEl>>,
}

#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sink_agent_pool_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_agent_pool_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_s3_data_source: Option<Vec<StorageTransferJobTransferSpecElAwsS3DataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure_blob_storage_data_source: Option<Vec<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_data_sink: Option<Vec<StorageTransferJobTransferSpecElGcsDataSinkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_data_source: Option<Vec<StorageTransferJobTransferSpecElGcsDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_data_source: Option<Vec<StorageTransferJobTransferSpecElHttpDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_conditions: Option<Vec<StorageTransferJobTransferSpecElObjectConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_data_sink: Option<Vec<StorageTransferJobTransferSpecElPosixDataSinkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_data_source: Option<Vec<StorageTransferJobTransferSpecElPosixDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_options: Option<Vec<StorageTransferJobTransferSpecElTransferOptionsEl>>,
    dynamic: StorageTransferJobTransferSpecElDynamic,
}

impl StorageTransferJobTransferSpecEl {
    #[doc= "Set the field `sink_agent_pool_name`.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn set_sink_agent_pool_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sink_agent_pool_name = Some(v.into());
        self
    }

    #[doc= "Set the field `source_agent_pool_name`.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn set_source_agent_pool_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_agent_pool_name = Some(v.into());
        self
    }

    #[doc= "Set the field `aws_s3_data_source`.\n"]
    pub fn set_aws_s3_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_s3_data_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_s3_data_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `azure_blob_storage_data_source`.\n"]
    pub fn set_azure_blob_storage_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure_blob_storage_data_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_blob_storage_data_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gcs_data_sink`.\n"]
    pub fn set_gcs_data_sink(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElGcsDataSinkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_data_sink = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_sink = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gcs_data_source`.\n"]
    pub fn set_gcs_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElGcsDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_data_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `http_data_source`.\n"]
    pub fn set_http_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElHttpDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_data_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_data_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `object_conditions`.\n"]
    pub fn set_object_conditions(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElObjectConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_conditions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_conditions = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `posix_data_sink`.\n"]
    pub fn set_posix_data_sink(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElPosixDataSinkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.posix_data_sink = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_data_sink = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `posix_data_source`.\n"]
    pub fn set_posix_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElPosixDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.posix_data_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_data_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `transfer_options`.\n"]
    pub fn set_transfer_options(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElTransferOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transfer_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for StorageTransferJobTransferSpecEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageTransferJobTransferSpecEl {}

impl BuildStorageTransferJobTransferSpecEl {
    pub fn build(self) -> StorageTransferJobTransferSpecEl {
        StorageTransferJobTransferSpecEl {
            sink_agent_pool_name: core::default::Default::default(),
            source_agent_pool_name: core::default::Default::default(),
            aws_s3_data_source: core::default::Default::default(),
            azure_blob_storage_data_source: core::default::Default::default(),
            gcs_data_sink: core::default::Default::default(),
            gcs_data_source: core::default::Default::default(),
            http_data_source: core::default::Default::default(),
            object_conditions: core::default::Default::default(),
            posix_data_sink: core::default::Default::default(),
            posix_data_source: core::default::Default::default(),
            transfer_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct StorageTransferJobTransferSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StorageTransferJobTransferSpecElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElRef {
        StorageTransferJobTransferSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageTransferJobTransferSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `sink_agent_pool_name` after provisioning.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn sink_agent_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sink_agent_pool_name", self.base))
    }

    #[doc= "Get a reference to the value of field `source_agent_pool_name` after provisioning.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn source_agent_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_agent_pool_name", self.base))
    }

    #[doc= "Get a reference to the value of field `aws_s3_data_source` after provisioning.\n"]
    pub fn aws_s3_data_source(&self) -> ListRef<StorageTransferJobTransferSpecElAwsS3DataSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_s3_data_source", self.base))
    }

    #[doc= "Get a reference to the value of field `azure_blob_storage_data_source` after provisioning.\n"]
    pub fn azure_blob_storage_data_source(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure_blob_storage_data_source", self.base))
    }

    #[doc= "Get a reference to the value of field `gcs_data_sink` after provisioning.\n"]
    pub fn gcs_data_sink(&self) -> ListRef<StorageTransferJobTransferSpecElGcsDataSinkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs_data_sink", self.base))
    }

    #[doc= "Get a reference to the value of field `gcs_data_source` after provisioning.\n"]
    pub fn gcs_data_source(&self) -> ListRef<StorageTransferJobTransferSpecElGcsDataSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs_data_source", self.base))
    }

    #[doc= "Get a reference to the value of field `http_data_source` after provisioning.\n"]
    pub fn http_data_source(&self) -> ListRef<StorageTransferJobTransferSpecElHttpDataSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_data_source", self.base))
    }

    #[doc= "Get a reference to the value of field `object_conditions` after provisioning.\n"]
    pub fn object_conditions(&self) -> ListRef<StorageTransferJobTransferSpecElObjectConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.object_conditions", self.base))
    }

    #[doc= "Get a reference to the value of field `posix_data_sink` after provisioning.\n"]
    pub fn posix_data_sink(&self) -> ListRef<StorageTransferJobTransferSpecElPosixDataSinkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.posix_data_sink", self.base))
    }

    #[doc= "Get a reference to the value of field `posix_data_source` after provisioning.\n"]
    pub fn posix_data_source(&self) -> ListRef<StorageTransferJobTransferSpecElPosixDataSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.posix_data_source", self.base))
    }

    #[doc= "Get a reference to the value of field `transfer_options` after provisioning.\n"]
    pub fn transfer_options(&self) -> ListRef<StorageTransferJobTransferSpecElTransferOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transfer_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageTransferJobDynamic {
    event_stream: Option<DynamicBlock<StorageTransferJobEventStreamEl>>,
    notification_config: Option<DynamicBlock<StorageTransferJobNotificationConfigEl>>,
    schedule: Option<DynamicBlock<StorageTransferJobScheduleEl>>,
    transfer_spec: Option<DynamicBlock<StorageTransferJobTransferSpecEl>>,
}
