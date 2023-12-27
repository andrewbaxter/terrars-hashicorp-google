use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LoggingOrganizationSinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_children: Option<PrimField<bool>>,
    name: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_options: Option<Vec<LoggingOrganizationSinkBigqueryOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<Vec<LoggingOrganizationSinkExclusionsEl>>,
    dynamic: LoggingOrganizationSinkDynamic,
}

struct LoggingOrganizationSink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoggingOrganizationSinkData>,
}

#[derive(Clone)]
pub struct LoggingOrganizationSink(Rc<LoggingOrganizationSink_>);

impl LoggingOrganizationSink {
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

    #[doc= "Set the field `description`.\nA description of this sink. The maximum length of the description is 8000 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nIf set to True, then this sink is disabled and it does not export any log entries."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `include_children`.\nWhether or not to include children organizations in the sink export. If true, logs associated with child projects are also exported; otherwise only logs relating to the provided organization are included."]
    pub fn set_include_children(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_children = Some(v.into());
        self
    }

    #[doc= "Set the field `bigquery_options`.\n"]
    pub fn set_bigquery_options(self, v: impl Into<BlockAssignable<LoggingOrganizationSinkBigqueryOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `exclusions`.\n"]
    pub fn set_exclusions(self, v: impl Into<BlockAssignable<LoggingOrganizationSinkExclusionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclusions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclusions = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of this sink. The maximum length of the description is 8000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\nThe destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, or a BigQuery dataset. Examples: \"storage.googleapis.com/[GCS_BUCKET]\" \"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]\" \"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]\" The writer associated with the sink must have access to write to the above resource."]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this sink is disabled and it does not export any log entries."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_children` after provisioning.\nWhether or not to include children organizations in the sink export. If true, logs associated with child projects are also exported; otherwise only logs relating to the provided organization are included."]
    pub fn include_children(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_children", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the logging sink."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe numeric ID of the organization to be exported to the sink."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `writer_identity` after provisioning.\nThe identity associated with this sink. This identity must be granted write access to the configured destination."]
    pub fn writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.writer_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bigquery_options` after provisioning.\n"]
    pub fn bigquery_options(&self) -> ListRef<LoggingOrganizationSinkBigqueryOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bigquery_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<LoggingOrganizationSinkExclusionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusions", self.extract_ref()))
    }
}

impl Referable for LoggingOrganizationSink {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LoggingOrganizationSink { }

impl ToListMappable for LoggingOrganizationSink {
    type O = ListRef<LoggingOrganizationSinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LoggingOrganizationSink_ {
    fn extract_resource_type(&self) -> String {
        "google_logging_organization_sink".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoggingOrganizationSink {
    pub tf_id: String,
    #[doc= "The destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, or a BigQuery dataset. Examples: \"storage.googleapis.com/[GCS_BUCKET]\" \"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]\" \"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]\" The writer associated with the sink must have access to write to the above resource."]
    pub destination: PrimField<String>,
    #[doc= "The name of the logging sink."]
    pub name: PrimField<String>,
    #[doc= "The numeric ID of the organization to be exported to the sink."]
    pub org_id: PrimField<String>,
}

impl BuildLoggingOrganizationSink {
    pub fn build(self, stack: &mut Stack) -> LoggingOrganizationSink {
        let out = LoggingOrganizationSink(Rc::new(LoggingOrganizationSink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoggingOrganizationSinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                destination: self.destination,
                disabled: core::default::Default::default(),
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                include_children: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                bigquery_options: core::default::Default::default(),
                exclusions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoggingOrganizationSinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoggingOrganizationSinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoggingOrganizationSinkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of this sink. The maximum length of the description is 8000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\nThe destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, or a BigQuery dataset. Examples: \"storage.googleapis.com/[GCS_BUCKET]\" \"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]\" \"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]\" The writer associated with the sink must have access to write to the above resource."]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this sink is disabled and it does not export any log entries."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_children` after provisioning.\nWhether or not to include children organizations in the sink export. If true, logs associated with child projects are also exported; otherwise only logs relating to the provided organization are included."]
    pub fn include_children(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_children", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the logging sink."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe numeric ID of the organization to be exported to the sink."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `writer_identity` after provisioning.\nThe identity associated with this sink. This identity must be granted write access to the configured destination."]
    pub fn writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.writer_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bigquery_options` after provisioning.\n"]
    pub fn bigquery_options(&self) -> ListRef<LoggingOrganizationSinkBigqueryOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bigquery_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<LoggingOrganizationSinkExclusionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoggingOrganizationSinkBigqueryOptionsEl {
    use_partitioned_tables: PrimField<bool>,
}

impl LoggingOrganizationSinkBigqueryOptionsEl { }

impl ToListMappable for LoggingOrganizationSinkBigqueryOptionsEl {
    type O = BlockAssignable<LoggingOrganizationSinkBigqueryOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingOrganizationSinkBigqueryOptionsEl {
    #[doc= "Whether to use BigQuery's partition tables. By default, Logging creates dated tables based on the log entries' timestamps, e.g. syslog_20170523. With partitioned tables the date suffix is no longer present and special query syntax has to be used instead. In both cases, tables are sharded based on UTC timezone."]
    pub use_partitioned_tables: PrimField<bool>,
}

impl BuildLoggingOrganizationSinkBigqueryOptionsEl {
    pub fn build(self) -> LoggingOrganizationSinkBigqueryOptionsEl {
        LoggingOrganizationSinkBigqueryOptionsEl { use_partitioned_tables: self.use_partitioned_tables }
    }
}

pub struct LoggingOrganizationSinkBigqueryOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoggingOrganizationSinkBigqueryOptionsElRef {
    fn new(shared: StackShared, base: String) -> LoggingOrganizationSinkBigqueryOptionsElRef {
        LoggingOrganizationSinkBigqueryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingOrganizationSinkBigqueryOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `use_partitioned_tables` after provisioning.\nWhether to use BigQuery's partition tables. By default, Logging creates dated tables based on the log entries' timestamps, e.g. syslog_20170523. With partitioned tables the date suffix is no longer present and special query syntax has to be used instead. In both cases, tables are sharded based on UTC timezone."]
    pub fn use_partitioned_tables(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_partitioned_tables", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingOrganizationSinkExclusionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    filter: PrimField<String>,
    name: PrimField<String>,
}

impl LoggingOrganizationSinkExclusionsEl {
    #[doc= "Set the field `description`.\nA description of this exclusion."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nIf set to True, then this exclusion is disabled and it does not exclude any log entries"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for LoggingOrganizationSinkExclusionsEl {
    type O = BlockAssignable<LoggingOrganizationSinkExclusionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingOrganizationSinkExclusionsEl {
    #[doc= "An advanced logs filter that matches the log entries to be excluded. By using the sample function, you can exclude less than 100% of the matching log entries"]
    pub filter: PrimField<String>,
    #[doc= "A client-assigned identifier, such as \"load-balancer-exclusion\". Identifiers are limited to 100 characters and can include only letters, digits, underscores, hyphens, and periods. First character has to be alphanumeric."]
    pub name: PrimField<String>,
}

impl BuildLoggingOrganizationSinkExclusionsEl {
    pub fn build(self) -> LoggingOrganizationSinkExclusionsEl {
        LoggingOrganizationSinkExclusionsEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: self.filter,
            name: self.name,
        }
    }
}

pub struct LoggingOrganizationSinkExclusionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoggingOrganizationSinkExclusionsElRef {
    fn new(shared: StackShared, base: String) -> LoggingOrganizationSinkExclusionsElRef {
        LoggingOrganizationSinkExclusionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingOrganizationSinkExclusionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of this exclusion."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this exclusion is disabled and it does not exclude any log entries"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nAn advanced logs filter that matches the log entries to be excluded. By using the sample function, you can exclude less than 100% of the matching log entries"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA client-assigned identifier, such as \"load-balancer-exclusion\". Identifiers are limited to 100 characters and can include only letters, digits, underscores, hyphens, and periods. First character has to be alphanumeric."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoggingOrganizationSinkDynamic {
    bigquery_options: Option<DynamicBlock<LoggingOrganizationSinkBigqueryOptionsEl>>,
    exclusions: Option<DynamicBlock<LoggingOrganizationSinkExclusionsEl>>,
}
